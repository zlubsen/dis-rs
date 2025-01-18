use crate::core::{
    BaseNode, BaseStatistics, InstanceId, NodeConstructor, NodeData, NodeRunner, UntypedNode,
    DEFAULT_AGGREGATE_STATS_INTERVAL_MS, DEFAULT_NODE_CHANNEL_CAPACITY,
    DEFAULT_OUTPUT_STATS_INTERVAL_MS,
};
use crate::error::InfraError;
use crate::runtime::{Command, Event};
use bytes::{Bytes, BytesMut};
use serde_derive::{Deserialize, Serialize};
use std::any::Any;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpSocket, UdpSocket};
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use tracing::{error, trace};

const SOCKET_BUFFER_CAPACITY: usize = 32_768;

const DEFAULT_TTL: u32 = 1;
const DEFAULT_BLOCK_OWN_SOCKET: bool = true;
const DEFAULT_OWN_ADDRESS: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
const DEFAULT_TCP_MAX_CONNECTIONS: usize = 15;
const DEFAULT_TCP_CLIENT_CONNECT_TIMEOUT_MS: u64 = 5000;

const SPEC_UDP_NODE_TYPE: &str = "udp";
const SPEC_UDP_MODE_UNICAST: &str = "unicast";
const SPEC_UDP_MODE_BROADCAST: &str = "broadcast";
const SPEC_UDP_MODE_MULTICAST: &str = "multicast";
const SPEC_TCP_SERVER_NODE_TYPE: &str = "tcp_server";
const SPEC_TCP_CLIENT_NODE_TYPE: &str = "tcp_client";

pub fn available_nodes() -> Vec<(&'static str, NodeConstructor)> {
    let network_nodes_constructor: NodeConstructor = node_from_spec;

    let mut items = Vec::new();
    items.push((SPEC_UDP_NODE_TYPE, network_nodes_constructor));
    items.push((SPEC_TCP_SERVER_NODE_TYPE, network_nodes_constructor));
    items.push((SPEC_TCP_CLIENT_NODE_TYPE, network_nodes_constructor));
    items
}

pub fn node_from_spec(
    instance_id: InstanceId,
    cmd_rx: Receiver<Command>,
    event_tx: Sender<Event>,
    type_value: &str,
    spec: &toml::Table,
) -> Result<UntypedNode, InfraError> {
    match type_value {
        SPEC_UDP_NODE_TYPE => {
            let node = UdpNodeData::new(instance_id, cmd_rx, event_tx, spec)?.to_dyn();
            Ok(node)
        }
        SPEC_TCP_SERVER_NODE_TYPE => {
            let node = TcpServerNodeData::new(instance_id, cmd_rx, event_tx, spec)?.to_dyn();
            Ok(node)
        }
        SPEC_TCP_CLIENT_NODE_TYPE => {
            let node = TcpClientNodeData::new(instance_id, cmd_rx, event_tx, spec)?.to_dyn();
            Ok(node)
        }
        unknown_value => Err(InfraError::InvalidSpec {
            message: format!("Unknown node type '{unknown_value}' for module 'network'"),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UdpNodeSpec {
    pub name: String,
    pub uri: String,
    pub interface: String,
    pub mode: Option<String>,
    pub ttl: Option<u32>,
    pub block_own_socket: Option<bool>,
}

#[derive(Debug)]
pub struct UdpNodeData {
    base: BaseNode,
    buffer: BytesMut,
    mode: UdpMode,
    interface: SocketAddr,
    address: SocketAddr,
    ttl: u32,
    block_own_socket: bool,
    incoming: Option<Receiver<Bytes>>,
    outgoing: Sender<Bytes>,
}

pub struct UdpNodeRunner {
    instance_id: InstanceId,
    name: String,
    buffer: BytesMut,
    address: SocketAddr,
    socket: UdpSocket,
    block_own_socket: bool,
    statistics: SocketStatistics,
}

#[derive(Debug)]
enum UdpNodeEvent {
    NoEvent,
    ReceivedPacket(Bytes),
    BlockedPacket,
    ReceivedIncoming(Bytes),
    SocketError(std::io::Error),
    ReceiveIncomingError(RecvError),
    SendOutgoingChannelError,
    SendEventChannelError,
    OutputStatistics,
    Quit,
}

#[derive(Copy, Clone, Debug, Default)]
struct SocketStatistics {
    base: BaseStatistics,
    total: SocketStatisticsItems,
    running_interval: SocketStatisticsItems,
    latest_interval: SocketStatisticsItems,
}

#[derive(Copy, Clone, Debug, Default)]
struct SocketStatisticsItems {
    packets_socket_in: u64,
    packets_socket_in_blocked: u64,
    packets_socket_out: u64,
    bytes_socket_in: u64,
    bytes_socket_out: u64,
    bytes_in: u64,
    bytes_out: u64,
}

impl SocketStatistics {
    fn received_packet(&mut self, number_of_bytes: usize) {
        self.total.packets_socket_in += 1;
        self.total.bytes_socket_in += number_of_bytes as u64;
        self.total.bytes_out += number_of_bytes as u64;
        self.running_interval.packets_socket_in += 1;
        self.running_interval.bytes_socket_in += number_of_bytes as u64;
        self.running_interval.bytes_out += number_of_bytes as u64;
        self.base.outgoing_message();
    }

    fn blocked_packet(&mut self, number_of_bytes: usize) {
        self.total.packets_socket_in += 1;
        self.total.packets_socket_in_blocked += 1;
        self.total.bytes_socket_in += number_of_bytes as u64;
        self.running_interval.packets_socket_in += 1;
        self.running_interval.packets_socket_in_blocked += 1;
    }

    fn received_incoming(&mut self, number_of_bytes: usize) {
        self.total.packets_socket_out += 1;
        self.total.bytes_socket_out += number_of_bytes as u64;
        self.total.bytes_in += number_of_bytes as u64;
        self.running_interval.packets_socket_out += 1;
        self.running_interval.bytes_socket_out += number_of_bytes as u64;
        self.running_interval.bytes_in += number_of_bytes as u64;
        self.base.incoming_message();
    }

    fn aggregate_interval(&mut self) {
        self.latest_interval = self.running_interval;
        self.running_interval = Default::default();
        self.base.aggregate_interval();
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum UdpMode {
    #[default]
    UniCast,
    BroadCast,
    MultiCast,
}

impl TryFrom<&str> for UdpMode {
    type Error = InfraError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            SPEC_UDP_MODE_UNICAST => Ok(Self::UniCast),
            SPEC_UDP_MODE_BROADCAST => Ok(Self::BroadCast),
            SPEC_UDP_MODE_MULTICAST => Ok(Self::MultiCast),
            _ => Err(InfraError::InvalidSpec {
                message: format!(
                    "Configured UDP mode is invalid. Valid values are '{}', '{}' and '{}'.",
                    SPEC_UDP_MODE_UNICAST, SPEC_UDP_MODE_BROADCAST, SPEC_UDP_MODE_MULTICAST
                ),
            }),
        }
    }
}

impl NodeData for UdpNodeData {
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<Self, InfraError> {
        let node_spec: UdpNodeSpec =
            toml::from_str(&spec.to_string()).map_err(|err| InfraError::InvalidSpec {
                message: err.to_string(),
            })?;

        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let mut buffer = BytesMut::with_capacity(SOCKET_BUFFER_CAPACITY);
        buffer.resize(SOCKET_BUFFER_CAPACITY, 0);

        let mode = if let Some(mode) = &node_spec.mode {
            UdpMode::try_from(mode.as_str())
        } else {
            Ok(UdpMode::default())
        }?;
        let ttl = node_spec.ttl.unwrap_or(DEFAULT_TTL);
        let block_own_socket = node_spec
            .block_own_socket
            .unwrap_or(DEFAULT_BLOCK_OWN_SOCKET);

        let interface =
            node_spec
                .interface
                .parse::<SocketAddr>()
                .map_err(|_err| InfraError::InvalidSpec {
                    message: format!(
                        "Node {instance_id} - Cannot parse socket address for interface {}",
                        node_spec.interface
                    ),
                })?;
        let address =
            node_spec
                .uri
                .parse::<SocketAddr>()
                .map_err(|_err| InfraError::InvalidSpec {
                    message: format!(
                        "Node {instance_id} - Cannot parse socket address for uri {}",
                        node_spec.uri
                    ),
                })?;

        Ok(Self {
            base: BaseNode::new(instance_id, node_spec.name.clone(), cmd_rx, event_tx),
            buffer,
            mode,
            interface,
            address,
            ttl,
            block_own_socket,
            incoming: None,
            outgoing: out_tx,
        })
    }

    fn request_subscription(&self) -> Box<dyn Any> {
        let client = self.outgoing.subscribe();
        Box::new(client)
    }

    fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError> {
        if let Ok(receiver) = receiver.downcast::<Receiver<Bytes>>() {
            self.incoming = Some(*receiver);
            Ok(())
        } else {
            Err(InfraError::SubscribeToChannel {
                instance_id: self.base.instance_id,
                node_name: self.base.name.clone(),
                data_type_expected: "Bytes".to_string(),
            })
        }
    }

    fn request_external_sender(&mut self) -> Result<Box<dyn Any>, InfraError> {
        let (incoming_tx, incoming_rx) = channel::<Bytes>(DEFAULT_NODE_CHANNEL_CAPACITY);
        self.register_subscription(Box::new(incoming_rx))?;
        Ok(Box::new(incoming_tx))
    }

    fn id(&self) -> InstanceId {
        self.base.instance_id
    }

    fn name(&self) -> &str {
        self.base.name.as_str()
    }

    fn spawn_into_runner(self: Box<Self>) -> Result<JoinHandle<()>, InfraError> {
        UdpNodeRunner::spawn_with_data(*self)
    }
}

impl NodeRunner for UdpNodeRunner {
    type Data = UdpNodeData;
    type Incoming = Bytes;
    type Outgoing = Bytes;

    fn id(&self) -> InstanceId {
        self.instance_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, InfraError> {
        let socket = create_udp_socket(&data)?;

        let mut node_runner = Self {
            instance_id: data.base.instance_id,
            name: data.base.name,
            buffer: data.buffer,
            address: data.address,
            socket,
            block_own_socket: data.block_own_socket,
            statistics: SocketStatistics::default(),
        };

        Ok(tokio::spawn(async move {
            node_runner
                .run(
                    data.base.cmd_rx,
                    data.base.event_tx,
                    data.incoming,
                    data.outgoing,
                )
                .await
        }))
    }

    async fn run(
        &mut self,
        mut cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        mut incoming: Option<Receiver<Self::Incoming>>,
        outgoing: Sender<Self::Outgoing>,
    ) {
        let mut aggregate_stats_interval =
            tokio::time::interval(Duration::from_millis(DEFAULT_AGGREGATE_STATS_INTERVAL_MS));
        let mut output_stats_interval =
            tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

        let default_own_socketaddr = DEFAULT_OWN_ADDRESS;
        let local_address = self.socket.local_addr().unwrap_or(default_own_socketaddr);

        loop {
            let event: UdpNodeEvent = tokio::select! {
                // receiving commands
                Ok(cmd) = cmd_rx.recv() => {
                    map_command_to_event(&cmd)
                }
                // receiving from the incoming channel
                Some(message) = Self::receive_incoming(self.instance_id, &mut incoming) => {
                    self.statistics.received_incoming(message.len());
                    UdpNodeEvent::ReceivedIncoming(message)
                }
                // receiving from the socket
                Ok((bytes_received, from_address)) = self.socket.recv_from(&mut self.buffer) => {
                    if self.block_own_socket && (local_address == from_address) {
                        self.statistics.blocked_packet(bytes_received);
                        UdpNodeEvent::BlockedPacket
                    } else {
                        self.statistics.received_packet(bytes_received);
                        Bytes::copy_from_slice(&self.buffer[..bytes_received]);
                        UdpNodeEvent::ReceivedPacket(Bytes::copy_from_slice(&self.buffer[..bytes_received]))
                    }
                }
                // aggregate statistics for the interval
                _ = aggregate_stats_interval.tick() => {
                    self.statistics.aggregate_interval();
                    UdpNodeEvent::NoEvent
                }
                // output current state of the stats
                _ = output_stats_interval.tick() => {
                    UdpNodeEvent::OutputStatistics
                }
            };

            match event {
                UdpNodeEvent::NoEvent => {}
                UdpNodeEvent::ReceivedPacket(bytes) => {
                    if let Ok(_num_receivers) = outgoing.send(bytes) {
                    } else {
                        Self::emit_event(
                            &event_tx,
                            Event::NodeError(InfraError::RuntimeNode {
                                instance_id: self.id(),
                                message: "Outgoing channel send failed.".to_string(),
                            }),
                        );
                    };
                }
                UdpNodeEvent::BlockedPacket => {}
                UdpNodeEvent::ReceivedIncoming(incoming_data) => {
                    match self.socket.send_to(&incoming_data, self.address).await {
                        Ok(_bytes_send) => {}
                        Err(err) => Self::emit_event(
                            &event_tx,
                            Event::NodeError(InfraError::RuntimeNode {
                                instance_id: self.id(),
                                message: err.to_string(),
                            }),
                        ),
                    }
                }
                UdpNodeEvent::SocketError(err) => Self::emit_event(
                    &event_tx,
                    Event::NodeError(InfraError::RuntimeNode {
                        instance_id: self.id(),
                        message: err.to_string(),
                    }),
                ),
                UdpNodeEvent::OutputStatistics => {
                    // TODO send statistics out
                }
                UdpNodeEvent::Quit => {
                    break;
                }
                UdpNodeEvent::ReceiveIncomingError(_) => {} // TODO handle channel error
                UdpNodeEvent::SendOutgoingChannelError => {} // TODO handle channel error
                UdpNodeEvent::SendEventChannelError => {}   // TODO handle channel error
            }
        }
    }
}

fn map_command_to_event(command: &Command) -> UdpNodeEvent {
    match command {
        Command::Quit => UdpNodeEvent::Quit,
    }
}

/// Creates an UDP socket based on the settings contained in `endpoint`.
/// The created `tokio::net::udp::UdpSocket` is returned wrapped in an `Arc`
/// so that it can be used by multiple tasks (i.e., for both writing and sending).
#[allow(clippy::too_many_lines)]
fn create_udp_socket(endpoint: &UdpNodeData) -> Result<UdpSocket, InfraError> {
    use socket2::{Domain, Protocol, Socket, Type};

    // Create socket using socket2 crate, to be able to set required socket options (SO_REUSEADDR, SO_REUSEPORT, ...)
    let is_ipv4 = endpoint.address.is_ipv4();
    let socket_domain = if is_ipv4 { Domain::IPV4 } else { Domain::IPV6 };
    let socket_type = Type::DGRAM;
    let socket_protocol = Protocol::UDP;
    let socket = Socket::new(socket_domain, socket_type, Some(socket_protocol))
        .expect("Error creating socket.");

    if let Err(err) = socket.set_reuse_address(true) {
        error!(
            "Failed to set SO_REUSEADDR for endpoint address {} - {}.",
            endpoint.address, err
        );
    }
    #[cfg(all(
        target_family = "unix",
        not(any(target_os = "solaris", target_os = "illumos"))
    ))]
    if let Err(err) = socket.set_reuse_port(true) {
        error!(
            "Failed to set SO_REUSEPORT for endpoint address {} - {}.",
            endpoint.address, err
        );
    }
    if let Err(err) = socket.set_nonblocking(true) {
        error!(
            "Failed to set nonblocking mode for endpoint address {} - {}",
            endpoint.address, err
        );
    }

    match (is_ipv4, endpoint.mode) {
        (true, UdpMode::UniCast) => {
            if let Err(err) = socket.bind(&endpoint.interface.into()) {
                return Err(InfraError::CreateNode {
                    instance_id: endpoint.base.instance_id,
                    message: format!(
                        "Failed to bind to IPv4 address {:?} - {}",
                        endpoint.address, err
                    ),
                });
            }
        }
        (true, UdpMode::BroadCast) => {
            if let Err(err) = socket.set_broadcast(true) {
                return Err(InfraError::CreateNode {
                    instance_id: endpoint.base.instance_id,
                    message: format!(
                        "Failed to set SO_BROADCAST for endpoint address {} - {}.",
                        endpoint.interface, err
                    ),
                });
            }
            if let Err(err) = socket.bind(&endpoint.interface.into()) {
                return Err(InfraError::CreateNode {
                    instance_id: endpoint.base.instance_id,
                    message: format!(
                        "Failed to bind to IPv4 address {:?} - {}",
                        endpoint.address, err
                    ),
                });
            }
            if let Err(err) = socket.set_ttl(endpoint.ttl) {
                return Err(InfraError::CreateNode {
                    instance_id: endpoint.base.instance_id,
                    message: format!("Failed to set TTL - {err}."),
                });
            }
        }
        (true, UdpMode::MultiCast) => {
            if let IpAddr::V4(ip_address_v4) = endpoint.address.ip() {
                if let IpAddr::V4(interface_v4) = endpoint.interface.ip() {
                    if socket
                        .join_multicast_v4(&ip_address_v4, &interface_v4)
                        .is_err()
                    {
                        return Err(InfraError::CreateNode {
                            instance_id: endpoint.base.instance_id,
                            message: format!("Failed to join multicast group {ip_address_v4} using interface {interface_v4}."),
                        });
                    }
                }
            }
        }
        (false, UdpMode::UniCast) => {
            if socket.bind(&endpoint.interface.into()).is_err() {
                return Err(InfraError::CreateNode {
                    instance_id: endpoint.base.instance_id,
                    message: format!("Failed to bind to IPv6 address {:?}", endpoint.address),
                });
            }
        }
        (false, UdpMode::BroadCast) => {
            socket
                .set_broadcast(true)
                .map_err(|_| InfraError::CreateNode {
                    instance_id: endpoint.base.instance_id,
                    message: "Failed to set SO_BROADCAST.".to_string(),
                })?;
            socket.set_ttl(1).map_err(|_| InfraError::CreateNode {
                instance_id: endpoint.base.instance_id,
                message: "Failed to set TTL.".to_string(),
            })?;
            socket
                .bind(&endpoint.interface.into())
                .map_err(|_| InfraError::CreateNode {
                    instance_id: endpoint.base.instance_id,
                    message: format!("Failed to bind to IPv6 address {:?}", endpoint.address),
                })?;
        }
        (false, UdpMode::MultiCast) => {
            if let IpAddr::V6(ip_address_v6) = endpoint.address.ip() {
                if let IpAddr::V6(interface_v6) = endpoint.interface.ip() {
                    // TODO how does IPv6 work with u32 interface numbers - pick 'any' for now.
                    socket
                        .join_multicast_v6(&ip_address_v6, 0)
                        .map_err(|_| InfraError::CreateNode {
                            instance_id: endpoint.base.instance_id,
                            message: format!("Failed to join multicast group {ip_address_v6} using interface 0 ({interface_v6})."),
                        })?;
                }
            }
        }
    }

    // Convert socket2::Socket to tokio::net::UdpSocket via std::net::UdpSocket
    let socket = std::net::UdpSocket::from(socket);
    let socket = UdpSocket::try_from(socket).map_err(|_| InfraError::CreateNode {
        instance_id: endpoint.base.instance_id,
        message: "Failed to convert std::net::UdpSocket to tokio::net::UdpSocket.".to_string(),
    })?;

    Ok(socket)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpServerNodeSpec {
    name: String,
    interface: String,
    max_connections: Option<usize>,
}

#[derive(Debug)]
pub struct TcpServerNodeData {
    base: BaseNode,
    buffer: BytesMut,
    interface: SocketAddr,
    max_connections: usize,
    incoming: Option<Receiver<Bytes>>,
    outgoing: Sender<Bytes>,
}

pub struct TcpServerNodeRunner {
    instance_id: InstanceId,
    name: String,
    buffer: BytesMut,
    interface: SocketAddr,
    max_connections: usize,
    statistics: SocketStatistics,
}

impl NodeData for TcpServerNodeData {
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<Self, InfraError> {
        let node_spec: TcpServerNodeSpec =
            toml::from_str(&spec.to_string()).map_err(|err| InfraError::InvalidSpec {
                message: err.to_string(),
            })?;

        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let mut buffer = BytesMut::with_capacity(SOCKET_BUFFER_CAPACITY);
        buffer.resize(SOCKET_BUFFER_CAPACITY, 0);

        let interface =
            node_spec
                .interface
                .parse::<SocketAddr>()
                .map_err(|_err| InfraError::InvalidSpec {
                    message: format!(
                        "Node {instance_id} - Cannot parse socket address for interface {}",
                        node_spec.interface
                    ),
                })?;
        let max_connections = node_spec
            .max_connections
            .unwrap_or(DEFAULT_TCP_MAX_CONNECTIONS);

        Ok(Self {
            base: BaseNode {
                instance_id,
                name: node_spec.name.clone(),
                cmd_rx,
                event_tx,
            },
            buffer,
            interface,
            max_connections,
            incoming: None,
            outgoing: out_tx,
        })
    }

    fn request_subscription(&self) -> Box<dyn Any> {
        let client = self.outgoing.subscribe();
        Box::new(client)
    }

    fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError> {
        if let Ok(receiver) = receiver.downcast::<Receiver<Bytes>>() {
            self.incoming = Some(*receiver);
            Ok(())
        } else {
            Err(InfraError::SubscribeToChannel {
                instance_id: self.base.instance_id,
                node_name: self.base.name.clone(),
                data_type_expected: "Bytes".to_string(),
            })
        }
    }

    fn request_external_sender(&mut self) -> Result<Box<dyn Any>, InfraError> {
        let (incoming_tx, incoming_rx) = channel::<Bytes>(DEFAULT_NODE_CHANNEL_CAPACITY);
        self.register_subscription(Box::new(incoming_rx))?;
        Ok(Box::new(incoming_tx))
    }

    fn id(&self) -> InstanceId {
        self.base.instance_id
    }

    fn name(&self) -> &str {
        self.base.name.as_str()
    }

    fn spawn_into_runner(self: Box<Self>) -> Result<JoinHandle<()>, InfraError> {
        TcpServerNodeRunner::spawn_with_data(*self)
    }
}

impl NodeRunner for TcpServerNodeRunner {
    type Data = TcpServerNodeData;
    type Incoming = Bytes;
    type Outgoing = Bytes;

    fn id(&self) -> InstanceId {
        self.instance_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, InfraError> {
        let mut node_runner = Self {
            instance_id: data.base.instance_id,
            name: data.base.name,
            buffer: data.buffer,
            interface: data.interface,
            max_connections: data.max_connections,
            statistics: SocketStatistics::default(),
        };

        Ok(tokio::spawn(async move {
            node_runner
                .run(
                    data.base.cmd_rx,
                    data.base.event_tx,
                    data.incoming,
                    data.outgoing,
                )
                .await
        }))
    }

    async fn run(
        &mut self,
        mut cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        mut incoming: Option<Receiver<Self::Incoming>>,
        outgoing: Sender<Self::Outgoing>,
    ) {
        let (tcp_read_tx, mut tcp_read_rx) =
            tokio::sync::mpsc::channel::<Bytes>(DEFAULT_NODE_CHANNEL_CAPACITY);
        let (tcp_write_tx, _tcp_write_rx) = channel::<Bytes>(DEFAULT_NODE_CHANNEL_CAPACITY);

        let socket = match TcpListener::bind(self.interface).await {
            Ok(socket) => socket,
            Err(err) => {
                trace!("Cannot bind TCP socket to {}", self.interface);
                return;
            }
        };

        loop {
            tokio::select! {
                Ok(command) = cmd_rx.recv() => {
                    if command == Command::Quit { break; }
                }
                Ok((mut stream, remote_addr)) = socket.accept() => {
                    // TODO add semaphore for tracking max number of connections
                    // TODO whitelist/blacklist of remote addresses
                    let (reader, writer) = stream.into_split();
                    let closer = Arc::new(Notify::new());
                    let read_handle = tokio::spawn(run_tcp_reader(reader, tcp_read_tx.clone(), closer.clone()));
                    let write_handle = tokio::spawn(run_tcp_write(writer, tcp_write_tx.subscribe(), closer.clone()));
                    tokio::spawn( async move {
                        read_handle.await.unwrap();
                        write_handle.await.unwrap();
                    });
                }
                Some(message) = Self::receive_incoming(self.instance_id, &mut incoming) => {
                    self.statistics.received_incoming(message.len());
                    let _ = tcp_write_tx.send(message);
                }
                Some(message) = tcp_read_rx.recv() => {
                    self.statistics.received_packet(message.len());
                    let _ = outgoing.send(message);
                }
            }
        }
    }
}

async fn run_tcp_reader(
    mut reader: OwnedReadHalf,
    to_node: tokio::sync::mpsc::Sender<Bytes>,
    closer: Arc<Notify>,
) {
    let mut buf = BytesMut::with_capacity(SOCKET_BUFFER_CAPACITY);
    buf.resize(SOCKET_BUFFER_CAPACITY, 0);
    loop {
        match reader.read(&mut buf).await {
            Ok(0) => {
                closer.notify_one();
                break;
            }
            Ok(bytes_received) => {
                let buf_to_send = Bytes::copy_from_slice(&buf[..bytes_received]);
                let _ = to_node.send(buf_to_send).await;
            }
            Err(_err) => {
                break;
            }
        }
    }
}

async fn run_tcp_write(
    mut writer: OwnedWriteHalf,
    mut from_node: Receiver<Bytes>,
    closer: Arc<Notify>,
) {
    loop {
        tokio::select! {
            _ = closer.notified() => {
                break;
            }
            Ok(message) = from_node.recv() => {
                match writer.write(&message[..]).await {
                    Ok(0) => {}
                    Ok(_bytes_sent) => { }
                    Err(_err) => {

                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpClientNodeSpec {
    name: String,
    interface: String,
    address: String,
}

#[derive(Debug)]
pub struct TcpClientNodeData {
    base: BaseNode,
    buffer: BytesMut,
    interface: SocketAddr,
    address: SocketAddr,
    incoming: Option<Receiver<Bytes>>,
    outgoing: Sender<Bytes>,
}

pub struct TcpClientNodeRunner {
    instance_id: InstanceId,
    name: String,
    buffer: BytesMut,
    interface: SocketAddr,
    address: SocketAddr,
    statistics: SocketStatistics,
}

impl NodeData for TcpClientNodeData {
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<Self, InfraError> {
        let node_spec: TcpClientNodeSpec =
            toml::from_str(&spec.to_string()).map_err(|err| InfraError::InvalidSpec {
                message: err.to_string(),
            })?;

        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let mut buffer = BytesMut::with_capacity(SOCKET_BUFFER_CAPACITY);
        buffer.resize(SOCKET_BUFFER_CAPACITY, 0);

        let interface =
            node_spec
                .interface
                .parse::<SocketAddr>()
                .map_err(|_err| InfraError::InvalidSpec {
                    message: format!(
                        "Node {} - Cannot parse socket address for interface {}",
                        node_spec.name, node_spec.interface
                    ),
                })?;
        let address =
            node_spec
                .address
                .parse::<SocketAddr>()
                .map_err(|_err| InfraError::InvalidSpec {
                    message: format!(
                        "Node {} - Cannot parse socket address for remote TCP server {}",
                        node_spec.name, node_spec.interface
                    ),
                })?;

        Ok(Self {
            base: BaseNode {
                instance_id,
                name: node_spec.name.clone(),
                cmd_rx,
                event_tx,
            },
            buffer,
            interface,
            address,
            incoming: None,
            outgoing: out_tx,
        })
    }

    fn request_subscription(&self) -> Box<dyn Any> {
        let client = self.outgoing.subscribe();
        Box::new(client)
    }

    fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError> {
        if let Ok(receiver) = receiver.downcast::<Receiver<Bytes>>() {
            self.incoming = Some(*receiver);
            Ok(())
        } else {
            Err(InfraError::SubscribeToChannel {
                instance_id: self.base.instance_id,
                node_name: self.base.name.clone(),
                data_type_expected: "Bytes".to_string(),
            })
        }
    }

    fn request_external_sender(&mut self) -> Result<Box<dyn Any>, InfraError> {
        let (incoming_tx, incoming_rx) = channel::<Bytes>(DEFAULT_NODE_CHANNEL_CAPACITY);
        self.register_subscription(Box::new(incoming_rx))?;
        Ok(Box::new(incoming_tx))
    }

    fn id(&self) -> InstanceId {
        self.base.instance_id
    }

    fn name(&self) -> &str {
        self.base.name.as_str()
    }

    fn spawn_into_runner(self: Box<Self>) -> Result<JoinHandle<()>, InfraError> {
        TcpClientNodeRunner::spawn_with_data(*self)
    }
}

impl NodeRunner for TcpClientNodeRunner {
    type Data = TcpClientNodeData;
    type Incoming = Bytes;
    type Outgoing = Bytes;

    fn id(&self) -> InstanceId {
        self.instance_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, InfraError> {
        let mut node_runner = Self {
            instance_id: data.base.instance_id,
            name: data.base.name,
            buffer: data.buffer,
            interface: data.interface,
            address: data.address,
            statistics: SocketStatistics::default(),
        };

        Ok(tokio::spawn(async move {
            node_runner
                .run(
                    data.base.cmd_rx,
                    data.base.event_tx,
                    data.incoming,
                    data.outgoing,
                )
                .await
        }))
    }

    async fn run(
        &mut self,
        mut cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        mut incoming: Option<Receiver<Self::Incoming>>,
        outgoing: Sender<Self::Outgoing>,
    ) {
        let mut aggregate_stats_interval =
            tokio::time::interval(Duration::from_millis(DEFAULT_AGGREGATE_STATS_INTERVAL_MS));
        let mut output_stats_interval =
            tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

        let socket = if self.interface.is_ipv4() {
            TcpSocket::new_v4()
        } else {
            TcpSocket::new_v6()
        };
        let socket = match socket {
            Ok(socket) => socket,
            Err(err) => {
                Self::emit_event(
                    &event_tx,
                    Event::NodeError(InfraError::CreateNode {
                        instance_id: self.id(),
                        message: err.to_string(),
                    }),
                );
                return;
            }
        };

        if let Err(err) = socket.bind(self.interface) {
            Self::emit_event(
                &event_tx,
                Event::NodeError(InfraError::CreateNode {
                    instance_id: self.id(),
                    message: err.to_string(),
                }),
            );
        }
        let mut tcp_stream = match socket.connect(self.address).await {
            Ok(stream) => stream,
            Err(err) => {
                Self::emit_event(
                    &event_tx,
                    Event::NodeError(InfraError::CreateNode {
                        instance_id: self.id(),
                        message: err.to_string(),
                    }),
                );
                return;
            }
        };

        let (mut reader, mut writer) = tcp_stream.split();

        loop {
            tokio::select! {
                // receiving commands
                Ok(cmd) = cmd_rx.recv() => {
                    if cmd == Command::Quit { break; }
                }
                // receiving from the incoming channel
                Some(message) = Self::receive_incoming(self.instance_id, &mut incoming) => {
                    let _send_result = writer.write_all(&message).await;
                    self.statistics.received_incoming(message.len());
                }
                // receiving from the socket
                Ok(bytes_received) = reader.read(&mut self.buffer) => {
                    if bytes_received == 0 {
                        Self::emit_event(&event_tx, Event::NodeError(InfraError::RuntimeNode {
                            instance_id: self.id(),
                            message: "TCP client node disconnected.".to_string(),
                        }));
                    } else if let Ok(_num_receivers) = outgoing
                        .send(Bytes::copy_from_slice(&self.buffer[..bytes_received])) {
                        self.statistics.received_packet(bytes_received);
                    } else {
                        Self::emit_event(&event_tx, Event::NodeError(
                            InfraError::RuntimeNode {
                                instance_id: self.id(),
                                message: "Outgoing channel send failed.".to_string(),
                            },
                        ));
                        break;
                    };
                }
                // aggregate statistics for the interval
                _ = aggregate_stats_interval.tick() => {
                    self.statistics.aggregate_interval();
                }
                // output current state of the stats
                _ = output_stats_interval.tick() => {
                    // TODO
                }
            };
        }
    }
}
