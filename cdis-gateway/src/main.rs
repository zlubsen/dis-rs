#![forbid(unsafe_code)]
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;
use std::net::{IpAddr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use bytes::{Bytes, BytesMut};
use tokio::net::UdpSocket;
use tokio::select;
use tracing::{error, event, info, Level};
use clap::Parser;
use tracing::log::trace;
use dis_rs::enumerations::PduType;

use crate::config::{Arguments, Config, ConfigError, ConfigSpec, UdpEndpoint, UdpMode};
use crate::codec::{Decoder, Encoder};
use crate::site::run_site;
use crate::stats::{GatewayStats, run_stats, SseStat};

mod config;
mod codec;
mod site;
mod stats;

const DATA_CHANNEL_BUFFER_SIZE: usize = 20;
const COMMAND_CHANNEL_BUFFER_SIZE: usize = 10;
const EVENT_CHANNEL_BUFFER_SIZE: usize = 50;
const STATS_CHANNEL_BUFFER_SIZE: usize = 50;
const READER_SOCKET_BUFFER_SIZE_BYTES: usize = 1500;

fn main() -> Result<(), GatewayError>{
    tracing_subscriber::fmt()
        .pretty()
        // enable everything
        .with_max_level(tracing::Level::TRACE)
        // sets this to be the default, global collector for this application.
        .init();

    let arguments = Arguments::parse();

    let mut file = File::open(arguments.config).map_err(|err| GatewayError::ConfigFileLoad(err))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).map_err(|err| GatewayError::ConfigFileRead(err) )?;

    let config_spec : ConfigSpec = toml::from_str(buffer.as_str()).map_err(| err | GatewayError::ConfigFileParse(err) )?;
    let config = Config::try_from(&config_spec).map_err(|e| GatewayError::ConfigInvalid(e))?;

    cli_print_config(&config, &config_spec);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build().map_err(|_err| GatewayError::RuntimeStart)?;
    let _guard = runtime.enter();
    runtime.block_on( async { start_gateway(config).await } );
    runtime.shutdown_background();

    Ok(())
}

#[derive(Debug)]
enum GatewayError {
    ConfigFileLoad(io::Error),
    ConfigFileRead(io::Error),
    ConfigFileParse(toml::de::Error),
    ConfigInvalid(ConfigError),
    RuntimeStart,
}

impl std::error::Error for GatewayError {}

impl Display for GatewayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GatewayError::ConfigFileLoad(err) => { write!(f, "Error loading config file - {}.", err) }
            GatewayError::ConfigFileRead(err) => { write!(f, "Error reading config file contents - {}.", err) }
            GatewayError::ConfigFileParse(err) => { write!(f, "Error parsing config file - {}.", err) }
            GatewayError::ConfigInvalid(err) => { write!(f, "{}", err) }
            GatewayError::RuntimeStart => { write!(f, "Error starting Tokio runtime.") }
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
enum Command {
    #[default]
    NoOp, // Handles potentially closed channels
    Quit
}

#[derive(Clone, Debug, PartialEq)]
enum Event {
    ReceivedBytesDis(usize), // bytes received through socket
    ReceivedBytesCDis(usize), // bytes received through socket
    ReceivedDis(PduType, u64), // type of the pdu and size of that pdu in bytes
    ReceivedCDis(PduType, u64),
    EncodedPdu(PduType, u64),
    DecodedPdu(PduType, u64),
    RejectedUnsupportedDisPdu(PduType, u64),
    RejectedUnsupportedCDisPdu(PduType, u64),
    UnimplementedEncodedPdu(PduType, u64),
    UnimplementedDecodedPdu(PduType, u64),
    SentDis(usize), // bytes send through socket
    SentCDis(usize), // bytes send through socket
}

/// Prints basic config information to the terminal
fn cli_print_config(config: &Config, config_spec: &ConfigSpec) {
    info!("*** C-DIS Gateway ***");
    if let Some(meta) = &config_spec.metadata {
        info!("Configuration `{}` - {} - {}", meta.name, meta.version, meta.author);
    }
    info!("Running in {} mode.", config.mode);
    info!("Encoder options - use guise: {} - {}", config.use_guise, config.optimization);
    info!("Hosting templates at port {}.", config.site_host);
    info!("DIS socket: {:?}.", config.dis_socket);
    info!("C-DIS socket: {:?}.", config.cdis_socket);
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
    let (cmd_tx, _cmd_rx) = tokio::sync::broadcast::channel::<Command>(COMMAND_CHANNEL_BUFFER_SIZE);
    let (event_tx, event_rx) = tokio::sync::mpsc::channel::<Event>(EVENT_CHANNEL_BUFFER_SIZE);
    let (dis_socket_out_tx, dis_socket_out_rx) = tokio::sync::mpsc::channel::<Bytes>(DATA_CHANNEL_BUFFER_SIZE);
    let (dis_socket_in_tx, dis_socket_in_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(DATA_CHANNEL_BUFFER_SIZE);
    let (cdis_socket_out_tx, cdis_socket_out_rx) = tokio::sync::mpsc::channel::<Bytes>(DATA_CHANNEL_BUFFER_SIZE);
    let (cdis_socket_in_tx, cdis_socket_in_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(DATA_CHANNEL_BUFFER_SIZE);
    let (stats_tx, stats_rx) = tokio::sync::broadcast::channel::<SseStat>(STATS_CHANNEL_BUFFER_SIZE);

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
            config.clone(), dis_socket_out_rx, cdis_socket_in_tx,
            cmd_tx.subscribe(), event_tx.clone())),
        tokio::spawn(decoder(
            config.clone(), cdis_socket_out_rx, dis_socket_in_tx,
            cmd_tx.subscribe(), event_tx.clone())),
        tokio::spawn(run_site(config.clone(), stats_tx.clone(), cmd_tx.clone())),
        tokio::spawn(run_stats(stats_tx, cmd_tx.subscribe(), event_rx))
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

    // Create socket using socket2 crate, to be able to set required socket options (SO_REUSEADDR, SO_REUSEPORT, ...)
    let is_ipv4 = endpoint.address.is_ipv4();
    let socket_domain = if is_ipv4 { Domain::IPV4 } else { Domain::IPV6 };
    let socket_type = Type::DGRAM;
    let socket_protocol = Protocol::UDP;
    let socket = Socket::new(socket_domain, socket_type, Some(socket_protocol)).expect("Error creating socket.");

    if let Err(err) = socket.set_reuse_address(true) {
        error!("Failed to set SO_REUSEADDR for endpoint address {} - {}.", endpoint.address, err);
    }
    if let Err(err) = socket.set_reuse_port(true) {
        error!("Failed to set SO_REUSEPORT for endpoint address {} - {}.", endpoint.address, err);
    }
    if let Err(err) = socket.set_nonblocking(true) {
        error!("Failed to set nonblocking mode for endpoint address {} - {}", endpoint.address, err);
    }

    match (is_ipv4, endpoint.mode) {
        (true, UdpMode::UniCast) => {
            if let Err(err) = socket.bind(&endpoint.interface.into()) {
            error!("Failed to bind to IPv4 address {:?} - {}", endpoint.address, err);
            }
        }
        (true, UdpMode::BroadCast) => {
            if let Err(err) = socket.set_broadcast(true) {
                error!("Failed to set SO_BROADCAST for endpoint address {} - {}.", endpoint.interface, err);
            }
            if let Err(err) = socket.bind(&endpoint.interface.into()) {
                error!("Failed to bind to IPv4 address {:?} - {}", endpoint.address, err);
            }
            if let Err(err) = socket.set_ttl(1) {
                error!("Failed to set TTL - {err}.");
            }
        }
        (true, UdpMode::MultiCast) => {
            if let IpAddr::V4(ip_address_v4) = endpoint.address.ip() {
                if let IpAddr::V4(interface_v4) = endpoint.interface.ip() {
                    socket.join_multicast_v4(&ip_address_v4, &interface_v4)
                        .unwrap_or_else(|_| panic!("Failed to join multicast group {} using interface {}.", ip_address_v4, interface_v4));
                }
            }
        }
        (false, UdpMode::UniCast) => {
            // TODO use .inspect_err() ?
            socket.bind(&endpoint.interface.into()).unwrap_or_else(|_| panic!("Failed to bind to IPv6 address {:?}", endpoint.address));
        }
        (false, UdpMode::BroadCast) => {
            socket.set_broadcast(true).expect("Failed to set SO_BROADCAST.");
            socket.set_ttl(1).expect("Failed to set TTL.");
            socket.bind(&endpoint.interface.into()).unwrap_or_else(|_| panic!("Failed to bind to IPv6 address {:?}", endpoint.address));
        }
        (false, UdpMode::MultiCast) => {
            if let IpAddr::V6(ip_address_v6) = endpoint.address.ip() {
                if let IpAddr::V6(interface_v6) = endpoint.interface.ip() {
                    // TODO how does IPv6 work with u32 interface numbers - pick 'any' for now.
                    socket.join_multicast_v6(&ip_address_v6, 0)
                        .unwrap_or_else(|_| panic!("Failed to join multicast group {} using interface 0 ({}).", ip_address_v6, interface_v6));
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
        NoOp,
        ReceivedPacket(Bytes, SocketAddr),
        BlockedPacket,
        Error(io::Error),
        Quit
    }

    loop {
        let action = select! {
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap_or_default();
                match cmd {
                    Command::NoOp => { Action::NoOp }
                    Command::Quit => { Action::Quit }
                }
            }
            received = socket.recv_from(&mut buf) => {
                match received {
                    Ok((bytes_received, from_address)) => {
                        if socket.local_addr().unwrap() != from_address { // FIXME potential panic, provide a default value...
                            Action::ReceivedPacket(Bytes::copy_from_slice(&buf[..bytes_received]), from_address)
                        } else { Action::BlockedPacket }
                    }
                    Err(err) => { Action::Error(err) }
                }
            }
        };

        match action {
            Action::NoOp => { }
            Action::ReceivedPacket(bytes, _from_address) => {
                if to_codec.send(bytes).await.is_err() {
                    event!(Level::ERROR, "Reader socket to codec channel dropped.");
                    return;
                }
            }
            Action::BlockedPacket => { }
            Action::Error(io_error) => {
                event!(Level::ERROR, "{io_error}");
                return;
            }
            Action::Quit => {
                trace!("Reader socket stopping due to receiving Command::Quit.");
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
        NoOp,
        Send(io::Result<usize>),
        SocketError,
        ChannelError,
        Quit
    }

    loop {
        let action = select! {
            cmd = cmd_rx.recv() => {
                match cmd {
                    Ok(cmd) => {
                        match cmd {
                            Command::Quit => { Action::Quit }
                            Command::NoOp => { Action::NoOp}
                        }
                    }
                    Err(_) => Action::ChannelError
                }

            }
            bytes = from_codec.recv() => {
                match bytes {
                    Some(bytes) => {
                        match socket.send_to(&bytes, to_address).await {
                            Ok(bytes_send) => Action::Send(Ok(bytes_send)),
                            Err(_err) => Action::SocketError,
                        }
                    }
                    None => {
                        Action::ChannelError
                    }
                }

            }
        };

        match action {
            Action::NoOp => { }
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
            Action::SocketError => { event!(Level::ERROR, "Failed to write through socket: {socket:?}"); return; }
            Action::ChannelError => { event!(Level::ERROR, "Internal channel failure in write socket task"); return; }
            Action::Quit => {
                trace!("Writer socket stopping due to receiving Command::Quit.");
                return;
            }
        }
    }
}

/// Task that runs the encoder part of the gateway, being connected to the DIS socket for input, and outputs to the C-DIS socket.
async fn encoder(config: Config, mut channel_in: tokio::sync::mpsc::Receiver<Bytes>,
                 channel_out: tokio::sync::mpsc::Sender<Vec<u8>>,
                 mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                 event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut encoder = Encoder::new(&config, event_tx);

    loop {
        select! {
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap_or_default();
                match cmd {
                    Command::NoOp => { }
                    Command::Quit => {
                        trace!("Encoder task stopping due to receiving Command::Quit.");
                        return;
                    }
                }
            }
            bytes = channel_in.recv() => {
                if let Some(bytes) = bytes {
                    let bytes = encoder.encode_buffer(bytes);
                    channel_out.send(bytes).await.expect("Error sending encoded bytes to socket.");
                } else {
                    trace!("Encoder task received zero bytes through channel.");
                }
            }
        }
    }
}

/// Task that runs the decoder part of the gateway, being connected to the C-DIS socket for input, and outputs to the DIS socket.
async fn decoder(config: Config,
                 mut channel_in: tokio::sync::mpsc::Receiver<Bytes>,
                 channel_out: tokio::sync::mpsc::Sender<Vec<u8>>,
                 mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                 event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut decoder = Decoder::new(&config, event_tx);

    loop {
        select! {
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap_or_default();
                match cmd {
                    Command::NoOp => { }
                    Command::Quit => {
                        trace!("Decoder task stopping due to receiving Command::Quit.");
                        return;
                    }
                }
            }
            bytes = channel_in.recv() => {
                if let Some(bytes) = bytes {
                    let bytes = decoder.decode_buffer(bytes);
                    channel_out.send(bytes).await.expect("Error sending encoded bytes to socket.");
                } else {
                    trace!("Decoder task received zero bytes through channel.");
                }
            }
        }
    }
}
