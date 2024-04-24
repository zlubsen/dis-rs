use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use bytes::{BufMut, Bytes, BytesMut};
use tokio::net::UdpSocket;
use tokio::select;
use tracing::{error, event, info, Level, trace};
use clap::Parser;

use crate::config::{Arguments, Config, ConfigError, ConfigSpec, GatewayMode, UdpEndpoint, UdpMode};
use crate::codec::{Decoder, Encoder};

mod config;
mod codec;

const DATA_CHANNEL_BUFFER_SIZE: usize = 20;
const COMMAND_CHANNEL_BUFFER_SIZE: usize = 10;
const EVENT_CHANNEL_BUFFER_SIZE: usize = 50;
const READER_SOCKET_BUFFER_SIZE_BYTES: usize = 1500;

fn main() -> Result<(), GatewayError>{
    tracing_subscriber::fmt()
        .pretty()
        // enable everything
        .with_max_level(tracing::Level::TRACE)
        // sets this to be the default, global collector for this application.
        .init();

    let arguments = Arguments::parse();

    let mut file = File::open(arguments.config).map_err(|err| GatewayError::ConfigFileLoadError(err))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).map_err(|err| GatewayError::ConfigFileReadError(err) )?;

    let config_spec : ConfigSpec = toml::from_str(buffer.as_str()).map_err(| err |GatewayError::ConfigFileParseError(err) )?;
    let config = Config::try_from(&config_spec).map_err(|e| GatewayError::ConfigError(e))?;
    // TODO print the used configuration
    info!("Running C-DIS Gateway");
    if let Some(meta) = config_spec.metadata {
        info!("Configuration `{}` - {} - {}", meta.name, meta.version, meta.author);
    }
    info!("Running in {} mode.", config.mode);
    info!("Hosting site at port {}.", config.site_host);
    info!("DIS socket: {:?}.", config.dis_socket);
    info!("C-DIS socket: {:?}.", config.cdis_socket);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build().map_err(|_err| GatewayError::RuntimeStartError)?;
    let _guard = runtime.enter();
    runtime.block_on( async { start_gateway(config).await } );
    runtime.shutdown_background();

    Ok(())
}

#[derive(Debug)]
enum GatewayError {
    ConfigFileLoadError(io::Error),
    ConfigFileReadError(io::Error),
    ConfigFileParseError(toml::de::Error),
    ConfigError(ConfigError),
    RuntimeStartError,
}

impl std::error::Error for GatewayError {
}

impl Display for GatewayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GatewayError::ConfigFileLoadError(err) => { write!(f, "Error loading config file - {}.", err) }
            GatewayError::ConfigFileReadError(err) => { write!(f, "Error reading config file contents - {}.", err) }
            GatewayError::ConfigFileParseError(err) => { write!(f, "Error parsing config file - {}.", err) }
            GatewayError::ConfigError(err) => { write!(f, "{}", err) }
            GatewayError::RuntimeStartError => { write!(f, "Error starting Tokio runtime.") }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Command {
    Quit
}

#[derive(Clone, Debug, PartialEq)]
enum Event {
    Nop
}

/// Starts all necessary tasks and channels for the gateway.
///
/// A gateway consists of two sockets, both able to send and receive data.
/// Either is the start or end of a encode/decode flow.
/// In each flow there is either an encoder, or a decoder performing the DIS to C-DIS conversion or vice versa.
/// Each flow of read-socket to encoder/decoder to write-socket is connected via mpsc channels.
///
/// Each task takes the receiver of a broadcast channel to receive `Command`s, and can emit `Event`s via an mpsc channel.
async fn start_gateway(config: Config) {
    let (cmd_tx, cmd_rx) = tokio::sync::broadcast::channel::<Command>(COMMAND_CHANNEL_BUFFER_SIZE);
    let (event_tx, _event_rx) = tokio::sync::mpsc::channel::<Event>(EVENT_CHANNEL_BUFFER_SIZE);
    let (dis_socket_out_tx, dis_socket_out_rx) = tokio::sync::mpsc::channel::<Bytes>(DATA_CHANNEL_BUFFER_SIZE);
    let (dis_socket_in_tx, dis_socket_in_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(DATA_CHANNEL_BUFFER_SIZE);
    let (cdis_socket_out_tx, cdis_socket_out_rx) = tokio::sync::mpsc::channel::<Bytes>(DATA_CHANNEL_BUFFER_SIZE);
    let (cdis_socket_in_tx, cdis_socket_in_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(DATA_CHANNEL_BUFFER_SIZE);

    let dis_socket = create_udp_socket(&config.dis_socket).await;
    let cdis_socket = create_udp_socket(&config.cdis_socket).await;

    let dis_read_socket = reader_socket(
        dis_socket.clone(), dis_socket_out_tx, cmd_tx.subscribe(), event_tx.clone());
    let dis_write_socket = writer_socket(
        dis_socket.clone(), config.dis_socket.address, dis_socket_in_rx,
        cmd_tx.subscribe(), event_tx.clone());
    let cdis_read_socket = reader_socket(
        cdis_socket.clone(), cdis_socket_out_tx, cmd_tx.subscribe(), event_tx.clone());
    let cdis_write_socket = writer_socket(
        cdis_socket.clone(), config.cdis_socket.address, cdis_socket_in_rx,
        cmd_tx.subscribe(), event_tx.clone());

    let handles = tokio::try_join!(
        tokio::spawn(dis_read_socket),
        tokio::spawn(dis_write_socket),
        tokio::spawn(cdis_read_socket),
        tokio::spawn(cdis_write_socket),
        tokio::spawn(encoder(
            config.mode, dis_socket_out_rx, cdis_socket_in_tx,
            cmd_tx.subscribe(), event_tx.clone())),
        tokio::spawn(decoder(
            config.mode, cdis_socket_out_rx, dis_socket_in_tx,
            cmd_tx.subscribe(), event_tx.clone()))
    );

    match handles {
        Ok(_handles) => {}
        Err(err) => {
            error!("Processing failed; error = {}", err);
        }
    }
}

/// Creates an UDP socket based on the settings contained in `endpoint`.
/// The created `tokio::net::udp::UdpSocket` is returned wrapped in an `Arc`
/// so that it can be used by multiple tasks (i.e., for both writing and sending).
async fn create_udp_socket(endpoint: &UdpEndpoint) -> Arc<UdpSocket> {
    use socket2::{Domain, Protocol, Socket, Type};

    let is_ipv4 = endpoint.address.is_ipv4();
    let socket_domain = if is_ipv4 { Domain::IPV4 } else { Domain::IPV6 };
    let socket_type = Type::DGRAM;
    let socket_protocol = Protocol::UDP;
    let socket = Socket::new(socket_domain, socket_type, Some(socket_protocol)).expect("Error creating socket.");

    socket.set_reuse_address(true).expect(format!("Failed to set SO_REUSEADDR for endpoint address {}.", endpoint.address).as_str());
    socket.set_reuse_port(true).expect(format!("Failed to set SO_REUSEPORT for endpoint address {}.", endpoint.address).as_str());
    socket.set_nonblocking(true).expect(format!("Failed to set nonblocking mode for endpoint address {}", endpoint.address).as_str());

    match (is_ipv4, endpoint.mode) {
        (true, UdpMode::UniCast) => {
            socket.bind(&endpoint.interface.into()).expect(format!("Failed to bind to IPv4 address {:?}", endpoint.address).as_str());
        }
        (true, UdpMode::BroadCast) => {
            socket.set_broadcast(true).expect(format!("Failed to set SO_BROADCAST for endpoint address {}.", endpoint.interface).as_str());
            socket.bind(&endpoint.interface.into()).expect(format!("Failed to bind to IPv4 address {:?}", endpoint.address).as_str());
            socket.set_ttl(1).expect("Failed to set TTL.");
        }
        (true, UdpMode::MultiCast) => {
            if let IpAddr::V4(ip_address_v4) = endpoint.address.ip() {
                if let IpAddr::V4(interface_v4) = endpoint.interface.ip() {
                    socket.join_multicast_v4(&ip_address_v4, &interface_v4)
                        .expect(format!("Failed to join multicast group {} using interface {}.", ip_address_v4, interface_v4).as_str());
                }
            }
        }
        (false, UdpMode::UniCast) => {
            socket.bind(&endpoint.interface.into()).expect(format!("Failed to bind to IPv6 address {:?}", endpoint.address).as_str())
        }
        (false, UdpMode::BroadCast) => {
            socket.set_broadcast(true).expect("Failed to set SO_BROADCAST.");
            socket.set_ttl(1).expect("Failed to set TTL.");
            socket.bind(&endpoint.interface.into()).expect(format!("Failed to bind to IPv6 address {:?}", endpoint.address).as_str());
        }
        (false, UdpMode::MultiCast) => {
            if let IpAddr::V6(ip_address_v6) = endpoint.address.ip() {
                if let IpAddr::V6(interface_v6) = endpoint.interface.ip() {
                    // TODO how does IPv6 work with u32 interface numbers - pick 'any' for now.
                    socket.join_multicast_v6(&ip_address_v6, 0)
                        .expect(format!("Failed to join multicast group {} using interface 0 ({}).", ip_address_v6, interface_v6).as_str());
                }
            }
        }
    }

    // Convert socket2::Socket to tokio::net::UdpSocket via std::net::UdpSocket
    let socket = std::net::UdpSocket::from(socket);
    let socket = UdpSocket::try_from(socket).expect("Failed to convert std::net::UdpSocket to tokio::net::UdpSocket.");

    Arc::new(socket)
}

/// Task that runs an UdpSocket for reading UDP packets from the network.
/// Received packets will be send to the encoder/decoder task to which the `to_codec` channel is connected to.
async fn reader_socket(socket: Arc<UdpSocket>,
                       to_codec: tokio::sync::mpsc::Sender<Bytes>,
                       mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                       _event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut buf = BytesMut::with_capacity(READER_SOCKET_BUFFER_SIZE_BYTES);
    buf.resize(READER_SOCKET_BUFFER_SIZE_BYTES, 0);

    #[derive(Debug)]
    enum Action {
        ReceivedPacket(Bytes, SocketAddr),
        BlockedPacket,
        Error(std::io::Error),
        Quit
    }

    loop {
        let action = select! {
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap();
                match cmd {
                    Command::Quit => { Action::Quit }
                }
            }
            received = socket.recv_from(&mut buf) => {
                match received {
                    Ok((bytes_received, from_address)) => {
                        if socket.local_addr().unwrap() != from_address {
                            Action::ReceivedPacket(Bytes::copy_from_slice(&buf[..bytes_received]), from_address)
                        } else { Action::BlockedPacket }
                    }
                    Err(err) => { Action::Error(err) }
                }
            }
        };

        match action {
            Action::ReceivedPacket(bytes, _from_address) => {
                if to_codec.send(bytes).await.is_err() {
                    event!(Level::ERROR, "Reader socket to codec channel dropped.");
                    return;
                }
            }
            Action::BlockedPacket => {
            }
            Action::Error(io_error) => {
                event!(Level::ERROR, "{io_error}");
                return;
            }
            Action::Quit => {
                return;
            }
        }
    }
}

/// Task that runs an UdpSocket for writing UDP packets to the network.
/// Packets will be received from the encoder/decoder task to which the `to_codec` channel is connected to.
async fn writer_socket(socket: Arc<UdpSocket>, to_address: SocketAddr,
                       mut from_codec: tokio::sync::mpsc::Receiver<Vec<u8>>,
                       mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                       _event_tx: tokio::sync::mpsc::Sender<Event>) {
    #[derive(Debug)]
    enum Action {
        Send(std::io::Result<usize>),
        Error(std::io::Error),
        Quit
    }

    loop {
        let action = select! {
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap();
                match cmd {
                    Command::Quit => { Action::Quit }
                }
            }
            bytes = from_codec.recv() => {
                let bytes = bytes.unwrap();
                let bytes_send = socket.send_to(&bytes, to_address).await;

                Action::Send(Ok(bytes_send.unwrap()))
            }
        };

        match action {
            Action::Send(result) => {
                match result {
                    Ok(_bytes_send) => {
                        // keep some statistics
                    }
                    Err(io_error) => {
                        event!(Level::ERROR, "{io_error}");
                        return;
                    }
                }
            }
            Action::Error(io_error) => { event!(Level::ERROR, "{io_error}"); return; }
            Action::Quit => { return; }
        }
    }
}

async fn encoder(mode: GatewayMode, mut channel_in: tokio::sync::mpsc::Receiver<Bytes>,
                 channel_out: tokio::sync::mpsc::Sender<Vec<u8>>,
                 mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                 _event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut encoder = Encoder::new(mode);

    loop {
        select! {
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap();
                match cmd {
                    Command::Quit => { return; }
                }
            }
            bytes = channel_in.recv() => {
                let bytes = bytes.unwrap();
                let bytes = encoder.encode_buffer(bytes);
                channel_out.send(bytes).await.expect("Error sending encoded bytes to socket.");
            }
        }
    }
}

async fn decoder(mode: GatewayMode,
                 mut channel_in: tokio::sync::mpsc::Receiver<Bytes>,
                 channel_out: tokio::sync::mpsc::Sender<Vec<u8>>,
                 mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                 _event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut decoder = Decoder::new(mode);

    loop {
        select! {
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap();
                match cmd {
                    Command::Quit => { return; }
                }
            }
            bytes = channel_in.recv() => {
                let bytes = bytes.unwrap();
                let bytes = decoder.decode_buffer(bytes);
                channel_out.send(bytes).await.expect("Error sending encoded bytes to socket.");
            }
        }
    }
}
