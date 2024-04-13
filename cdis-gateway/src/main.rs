use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use bytes::{Bytes, BytesMut};
use tokio::net::UdpSocket;
use tokio::select;
use tracing::{event, Level};

use cdis_assemble::entity_state::model::EntityState;
use cdis_assemble::{BitBuffer, CdisPdu, Codec, create_bit_buffer, SerializeCdisPdu};

use dis_rs::entity_state::model::EntityMarking;
use dis_rs::enumerations::{Country, EntityKind, EntityMarkingCharacterSet, PduType, PlatformDomain};
use dis_rs::model::{EntityId, EntityType, Location, Pdu, PduHeader};
use crate::codec::Encoder;

use crate::config::{Config, UdpEndpoint, UdpMode};

mod config;
mod codec;

fn main() {
    let config = Config {
        dis_socket: UdpEndpoint {
            mode: UdpMode::UniCast,
            interface: IpAddr::V4(Ipv4Addr::new(127,0,0,1)),
            address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 3000)),
            ttl: 1,
        },
        cdis_socket: UdpEndpoint {
            mode: UdpMode::UniCast,
            interface: IpAddr::V4(Ipv4Addr::new(127,0,0,1)),
            address: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 3001)),
            ttl: 1,
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
    let (dis_socket_out_tx, dis_socket_out_rx) = tokio::sync::mpsc::channel::<Bytes>(100);
    let (dis_socket_in_tx, dis_socket_in_rx) = tokio::sync::mpsc::channel::<Bytes>(100);
    let (cdis_socket_out_tx, cdis_socket_out_rx) = tokio::sync::mpsc::channel::<Bytes>(100);
    let (cdis_socket_in_tx, cdis_socket_in_rx) = tokio::sync::mpsc::channel::<Bytes>(100);

    let dis_socket = create_udp_socket(&config.dis_socket).await;
    let cdis_socket = create_udp_socket(&config.cdis_socket).await;

    let dis_read_socket = reader_socket(dis_socket.clone(), dis_socket_out_tx, cmd_tx.subscribe(), event_tx.clone());
    let dis_write_socket = writer_socket(dis_socket.clone(), dis_socket_in_rx, cmd_tx.subscribe(), event_tx.clone());
    let cdis_read_socket = reader_socket(cdis_socket.clone(), cdis_socket_out_tx, cmd_tx.subscribe(), event_tx.clone());
    let cdis_write_socket = writer_socket(cdis_socket.clone(), cdis_socket_in_rx, cmd_tx.subscribe(), event_tx.clone());
    tokio::spawn(dis_read_socket);
    tokio::spawn(dis_write_socket);
    tokio::spawn(cdis_read_socket);
    tokio::spawn(cdis_write_socket);
    tokio::spawn(encoder(config.clone(), dis_socket_out_rx, cdis_socket_in_tx, cmd_tx.subscribe(), event_tx.clone()));
    tokio::spawn(decoder(config.clone(), cdis_socket_out_rx, dis_socket_in_tx, cmd_tx.subscribe(), event_tx.clone()));
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

    match (is_ipv4, endpoint.mode) {
        (true, UdpMode::UniCast) => {
            socket.bind(&endpoint.address.into()).expect(format!("Failed to bind to IPv4 address {:?}", endpoint.address).as_str());
        }
        (true, UdpMode::BroadCast) => {
            socket.set_broadcast(true).expect(format!("Failed to set SO_BROADCAST for endpoint address {}.", endpoint.address).as_str());
        }
        (true, UdpMode::MultiCast) => {
            if let IpAddr::V4(ip_address_v4) = endpoint.address.ip() {
                if let IpAddr::V4(interface_v4) = endpoint.interface {
                    socket.join_multicast_v4(&ip_address_v4, &interface_v4)
                        .expect(format!("Failed to join multicast group {} using interface {}.", ip_address_v4, interface_v4).as_str());
                }
            }
        }
        (false, UdpMode::UniCast) => {
            socket.bind(&endpoint.address.into()).expect(format!("Failed to bind to IPv6 address {:?}", endpoint.address).as_str())
        }
        (false, UdpMode::BroadCast) => {
            socket.set_broadcast(true).expect("Failed to set SO_BROADCAST.");
        }
        (false, UdpMode::MultiCast) => {
            if let IpAddr::V6(ip_address_v6) = endpoint.address.ip() {
                if let IpAddr::V6(interface_v6) = endpoint.interface {
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

pub(crate) enum Payload {
    DIS(Pdu),
    CDIS(CdisPdu)
}

async fn reader_socket<T>(socket: Arc<UdpSocket>,
                       to_codec: tokio::sync::mpsc::Sender<Bytes>,
                       mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                       event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut buf = BytesMut::with_capacity(1500);

    enum Action {
        Received(Bytes, SocketAddr),
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
                        Action::Received(Bytes::copy_from_slice(&buf[..bytes_received]), from_address)
                    }
                    Err(err) => { Action::Error(err) }
                }
            }
        };

        match action {
            Action::Received(bytes, from_address) => {
                if let Err(_) = to_codec.send(bytes).await {
                    event!(Level::ERROR, "Reader socket to codec channel dropped.");
                    return;
                }
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

async fn writer_socket<T>(socket: Arc<UdpSocket>, mut from_codec: tokio::sync::mpsc::Receiver<Bytes>, mut cmd_rx: tokio::sync::broadcast::Receiver<Command>, event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut buf = BytesMut::with_capacity(1500);

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
                let bytes_send = socket.send(&bytes).await;
                Action::Send(bytes_send)
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
                 channel_out: tokio::sync::mpsc::Sender<Bytes>,
                 mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                 event_tx: tokio::sync::mpsc::Sender<Event>) {
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
                 channel_in: tokio::sync::mpsc::Receiver<Bytes>,
                 channel_out: tokio::sync::mpsc::Sender<Bytes>,
                 mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                 event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut decoder = Decoder::new(config.mode);
}

fn test() {
    let mut write_buf: BitBuffer = create_bit_buffer();
    let mut read_buf = BytesMut::with_capacity(1400);

    let dis_entity_state_body = dis_rs::entity_state::model::EntityState::builder()
        .with_entity_id(EntityId::new(8, 8, 8))
        .with_entity_type(EntityType::default()
            .with_kind(EntityKind::Platform)
            .with_domain(PlatformDomain::Air)
            .with_country(Country::Netherlands_NLD_))
        .with_location(Location::new(35000.0, 10000.0, 30000.0))
        .with_marking(EntityMarking::new("My1stPlane", EntityMarkingCharacterSet::ASCII))
        .build().into_pdu_body();
    let header = PduHeader::new_v7(1, PduType::EntityState);
    let dis_entity_state = Pdu::finalize_from_parts(header, dis_entity_state_body, 500);
    // TODO encode entire PDU...
    let cdis_entity_state = CdisPdu::encode(&dis_entity_state);

    let cursor = cdis_entity_state.serialize(&mut write_buf, 0);

    let cdis_wire: Vec<u8> = write_buf.data[0..cursor].chunks_exact(8).map(|ch| { ch[0] } ).collect();
    println!("{}", cdis_wire.len());
    dbg!(cdis_wire.clone());

    let parsed_cdis = cdis_assemble::parse(cdis_wire.as_slice()).unwrap();
    dbg!(parsed_cdis);

    let decoded_es = cdis_entity_state.decode();
    dbg!(decoded_es);
}
