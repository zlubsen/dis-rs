use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use bytes::BytesMut;
use tokio::net::UdpSocket;
use tokio::select;

use cdis_assemble::entity_state::model::EntityState;
use cdis_assemble::{BitBuffer, CdisPdu, Codec, create_bit_buffer, SerializeCdisPdu};

use dis_rs::entity_state::model::EntityMarking;
use dis_rs::enumerations::{Country, EntityKind, EntityMarkingCharacterSet, PduType, PlatformDomain};
use dis_rs::model::{EntityId, EntityType, Location, Pdu, PduHeader};

use crate::config::{Config, UdpEndpoint, UdpMode};

mod config;

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
    Nop
}

#[derive(Clone, Debug, PartialEq)]
enum Event {
    Nop
}

async fn start_gateway(config: Config) {
    let (cmd_tx, cmd_rx) = tokio::sync::broadcast::channel::<Command>(10);
    let (event_tx, _event_rx) = tokio::sync::mpsc::channel::<Event>(100);

    tokio::spawn(encoder_socket(config.clone(), cmd_rx, event_tx));
    tokio::spawn(encoder(config.clone()));
    tokio::spawn(decoder(config.clone()));
    tokio::spawn(decoder_socket(config.clone(), cmd_rx, event_tx));
}

async fn encoder_socket(config: Config, mut cmd_rx: tokio::sync::broadcast::Receiver<Command>, event_tx: tokio::sync::mpsc::Sender<Event>) {
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

async fn decoder_socket(config: Config, mut cmd_rx: tokio::sync::broadcast::Receiver<Command>, event_tx: tokio::sync::mpsc::Sender<Event>) {
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

async fn encoder(config: Config) {
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

async fn decoder(config: Config) {
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
