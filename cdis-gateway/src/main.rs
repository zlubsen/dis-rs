use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use bytes::{BufMut, Bytes, BytesMut};
use tokio::net::UdpSocket;
use tokio::select;
use tracing::{event, Level};

use cdis_assemble::{CdisPdu};
use dis_rs::model::{Pdu};

use crate::config::{Config, UdpEndpoint, UdpMode};
use crate::codec::{Decoder, Encoder};

mod config;
mod codec;

fn main() {
    let config = Config {
        dis_socket: UdpEndpoint {
            mode: UdpMode::BroadCast,
            interface: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192,168,178,11), 3000)),
            address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192,168,178,255), 3000)),
            // address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::BROADCAST, 3000)),
            ttl: 1,
            block_own_socket: true,
        },
            cdis_socket: UdpEndpoint {
            mode: UdpMode::BroadCast,
                interface: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192,168,178,11), 3001)),
            address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192,168,178,255), 3001)),
            ttl: 1,
            block_own_socket: true,
        },
        mode: Default::default(),
    };

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build().unwrap();
    let _guard = runtime.enter();
    runtime.block_on( async { start_gateway(config).await } );
    runtime.shutdown_background();
}

#[derive(Clone, Debug, PartialEq)]
enum Command {
    Quit
}

#[derive(Clone, Debug, PartialEq)]
enum Event {
    Nop
}

async fn start_gateway(config: Config) {
    let (cmd_tx, cmd_rx) = tokio::sync::broadcast::channel::<Command>(10);
    let (event_tx, _event_rx) = tokio::sync::mpsc::channel::<Event>(100);
    let (dis_socket_out_tx, dis_socket_out_rx) = tokio::sync::mpsc::channel::<Bytes>(10);
    let (dis_socket_in_tx, dis_socket_in_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(10);
    let (cdis_socket_out_tx, cdis_socket_out_rx) = tokio::sync::mpsc::channel::<Bytes>(10);
    let (cdis_socket_in_tx, cdis_socket_in_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(10);

    let dis_socket = create_udp_socket(&config.dis_socket).await;
    let cdis_socket = create_udp_socket(&config.cdis_socket).await;

    let dis_read_socket = reader_socket(dis_socket.clone(), dis_socket_out_tx, cmd_tx.subscribe(), event_tx.clone());
    let dis_write_socket = writer_socket(dis_socket.clone(), config.dis_socket.address, dis_socket_in_rx, cmd_tx.subscribe(), event_tx.clone());
    let cdis_read_socket = reader_socket(cdis_socket.clone(), cdis_socket_out_tx, cmd_tx.subscribe(), event_tx.clone());
    let cdis_write_socket = writer_socket(cdis_socket.clone(), config.cdis_socket.address, cdis_socket_in_rx, cmd_tx.subscribe(), event_tx.clone());
    let h1 = tokio::spawn(dis_read_socket);
    let h2 = tokio::spawn(dis_write_socket);
    let h3 = tokio::spawn(cdis_read_socket);
    let h4 = tokio::spawn(cdis_write_socket);
    let h5 = tokio::spawn(encoder(config, dis_socket_out_rx, cdis_socket_in_tx, cmd_tx.subscribe(), event_tx.clone()));
    let h6 = tokio::spawn(decoder(config, cdis_socket_out_rx, dis_socket_in_tx, cmd_tx.subscribe(), event_tx.clone()));

    let (h1, h2, h3, h4, h5, h6) =
        tokio::join!(h1, h2, h3, h4, h5, h6);
    h1.unwrap();
    h2.unwrap();
    h3.unwrap();
    h4.unwrap();
    h5.unwrap();
    h6.unwrap();
}

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

async fn reader_socket(socket: Arc<UdpSocket>,
                       to_codec: tokio::sync::mpsc::Sender<Bytes>,
                       mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                       _event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut buf = BytesMut::with_capacity(1500);
    buf.resize(1500, 0);

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

async fn encoder(config: Config, mut channel_in: tokio::sync::mpsc::Receiver<Bytes>,
                 channel_out: tokio::sync::mpsc::Sender<Vec<u8>>,
                 mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                 _event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut encoder = Encoder::new(config.mode);

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

async fn decoder(config: Config,
                 mut channel_in: tokio::sync::mpsc::Receiver<Bytes>,
                 channel_out: tokio::sync::mpsc::Sender<Vec<u8>>,
                 mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                 _event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut decoder = Decoder::new(config.mode);

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
