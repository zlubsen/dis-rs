use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use bytes::{Bytes, BytesMut};
use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;
use tokio::select;

use cdis_assemble::entity_state::model::EntityState;
use cdis_assemble::{BitBuffer, CdisPdu, Codec, create_bit_buffer, SerializeCdisPdu};

use dis_rs::entity_state::model::EntityMarking;
use dis_rs::enumerations::{Country, EntityKind, EntityMarkingCharacterSet, PduType, PlatformDomain};
use dis_rs::model::{EntityId, EntityType, Location, Pdu, PduHeader};
use dis_rs::parse;

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

    let dis_socket = create_udp_socket(config.dis_socket);

    tokio::spawn(reader_socket(config.clone(), cmd_rx, event_tx));
    tokio::spawn(encoder(config.clone()));
    tokio::spawn(decoder(config.clone()));
    tokio::spawn(writer_socket(config.clone(), cmd_rx, event_tx));
}

async fn create_udp_socket(endpoint: &UdpEndpoint) -> Arc<UdpSocket> {
    let is_ipv4 = endpoint.address.is_ipv4();
    let socket_domain = if is_ipv4 { Domain::IPV4 } else { Domain::IPV6 };
    let socket_type = Type::DGRAM;
    let socket_protocol = Protocol::UDP;
    let socket = Socket::new(socket_domain, socket_type, Some(socket_protocol)).expect("Error creating socket.");

    socket.set_reuse_address(true).expect("Failed to set SO_REUSEADDR.");
    socket.set_reuse_port(true).expect("Failed to set SO_REUSEPORT.");

    match (is_ipv4, endpoint.mode) {
        (true, UdpMode::UniCast) => {
            socket.bind(&endpoint.address.into()).expect(format!("Failed to bind to IPv4 address {:?}", endpoint.address).as_str());
        }
        (true, UdpMode::BroadCast) => {
            socket.set_broadcast(true).expect("Failed to set SO_BROADCAST.");
        }
        (true, UdpMode::MultiCast) => {
            socket.join_multicast_v4(&endpoint.address.into(), endpoint.interface.into()).expect("Failed to join multicast group.");
        }
        (false, UdpMode::UniCast) => {
            socket.bind(&endpoint.address.into()).expect(format!("Failed to bind to IPv6 address {:?}", endpoint.address).as_str())
        }
        (false, UdpMode::BroadCast) => {
            socket.set_broadcast(true).expect("Failed to set SO_BROADCAST.");
        }
        (false, UdpMode::MultiCast) => {

        }
    }
    let socket = std::net::UdpSocket::from(socket);
    let socket = UdpSocket::try_from(socket).expect("Failed to convert std::net::UdpSocket to tokio::net::UdpSocket.");

    Arc::new(socket)
}

pub(crate) enum Payload {
    DIS(Pdu),
    CDIS(CdisPdu)
}

async fn reader_socket(socket: Arc<UdpSocket>,
                       to_codec: tokio::sync::mpsc::Sender<Pdu>,
                       mut cmd_rx: tokio::sync::broadcast::Receiver<Command>,
                       event_tx: tokio::sync::mpsc::Sender<Event>) {
    let mut buf = BytesMut::with_capacity(1500);

    enum Action {
        Received(usize, SocketAddr),
        Quit
    };

    loop {
        let action = select! {
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap();
                match cmd {
                    Command::Quit => { Action::Quit }
                }
            }
            (bytes_received, from_address) = socket.recv_from(&mut buf) => {
                Action::Reveived(bytes_received, from_address)
            }
        };

        match action {
            Action::Received(bytes_received, from_address) => {
                let pdus = parse(&buf).expect("Error parsing PDUs");
                // pdus.iter().for_each(|pdu| to_codec.send(*pdu))
            }
            Action::Quit => {

            }
        }
    }
}

fn handle_bytes(bytes_received: usize, bytes: &mut BytesMut) {

}

async fn writer_socket(config: Config, mut cmd_rx: tokio::sync::broadcast::Receiver<Command>, event_tx: tokio::sync::mpsc::Sender<Event>) {
    let socket = UdpSocket::bind(config.dis_socket.address).await.unwrap();

    let mut buf = BytesMut::with_capacity(1500);
    loop {
        select! {
            cmd = cmd_rx.recv() => {
                let cmd = cmd.unwrap();
                // received a command
            }
            _ = socket.recv(&mut buf) => {
                // received bytes
            }
        }
    }
}

async fn encoder(config: Config, channel_in: tokio::sync::mpsc::Receiver<&Bytes>, channel_out: tokio::sync::mpsc::Sender<&Bytes>) {
    let r = channel_in.recv().await.unwrap()
}

async fn decoder(config: Config) {
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
