use crate::core::{
    BaseNode, BaseStatistics, InstanceId, NodeConstructor, NodeConstructorPointer, NodeData,
    NodeRunner, UntypedNode, DEFAULT_AGGREGATE_STATS_INTERVAL_MS, DEFAULT_NODE_CHANNEL_CAPACITY,
    DEFAULT_OUTPUT_STATS_INTERVAL_MS,
};
use crate::error::{CreationError, ExecutionError, NodeError, SpecificationError};
use crate::node_data_impl;
use crate::runtime::{Command, Event};
use bytes::{Bytes, BytesMut};
use serde_derive::{Deserialize, Serialize};
use std::any::Any;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpSocket, UdpSocket};
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use tracing::error;

const DEFAULT_SOCKET_BUFFER_CAPACITY: usize = 32_768;
const DEFAULT_TTL: u32 = 1;
const DEFAULT_BLOCK_OWN_SOCKET: bool = true;
const DEFAULT_OWN_ADDRESS: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
// const DEFAULT_TCP_MAX_CONNECTIONS: usize = 15;
// const DEFAULT_TCP_CLIENT_CONNECT_TIMEOUT_MS: u64 = 5000;

const SPEC_UDP_NODE_TYPE: &str = "udp";
const SPEC_UDP_MODE_UNICAST: &str = "unicast";
const SPEC_UDP_MODE_BROADCAST: &str = "broadcast";
const SPEC_UDP_MODE_MULTICAST: &str = "multicast";
const SPEC_TCP_SERVER_NODE_TYPE: &str = "tcp_server";
const SPEC_TCP_CLIENT_NODE_TYPE: &str = "tcp_client";

pub fn available_nodes() -> Vec<NodeConstructorPointer> {
    let network_nodes_constructor: NodeConstructor = node_from_spec;

    let items = vec![
        (SPEC_UDP_NODE_TYPE, network_nodes_constructor),
        (SPEC_TCP_SERVER_NODE_TYPE, network_nodes_constructor),
        (SPEC_TCP_CLIENT_NODE_TYPE, network_nodes_constructor),
    ];
    items
}

pub fn node_from_spec(
    instance_id: InstanceId,
    cmd_rx: Receiver<Command>,
    event_tx: Sender<Event>,
    type_value: &str,
    spec: &toml::Table,
) -> Result<UntypedNode, SpecificationError> {
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
        unknown_value => Err(SpecificationError::UnknownNodeTypeForModule {
            node_type: unknown_value.to_string(),
            module_name: "network",
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
    pub buffer_size: Option<usize>,
    pub block_own_socket: Option<bool>,
}

#[derive(Debug)]
pub struct UdpNodeData {
    base: BaseNode,
    buffer_size: usize,
    // buffer: BytesMut, // TODO convert to buffer_size config, buffer is created on run, like for the TCP server?
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
    #[allow(dead_code)]
    SocketError(std::io::Error),
    OutputStatistics,
    Quit,
}

#[derive(Copy, Clone, Debug, Default, Serialize)]
struct SocketStatistics {
    base: BaseStatistics,
    total: SocketStatisticsItems,
    #[serde(skip)]
    running_interval: SocketStatisticsItems,
    latest_interval: SocketStatisticsItems,
}

#[derive(Copy, Clone, Debug, Default, Serialize)]
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
    fn new(node_id: InstanceId) -> Self {
        Self {
            base: BaseStatistics::new(node_id),
            ..Default::default()
        }
    }

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
    type Error = UdpNodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            SPEC_UDP_MODE_UNICAST => Ok(Self::UniCast),
            SPEC_UDP_MODE_BROADCAST => Ok(Self::BroadCast),
            SPEC_UDP_MODE_MULTICAST => Ok(Self::MultiCast),
            _ => Err(UdpNodeError::IncorrectMode(
                SPEC_UDP_MODE_UNICAST,
                SPEC_UDP_MODE_BROADCAST,
                SPEC_UDP_MODE_MULTICAST,
            )),
        }
    }
}

impl NodeData for UdpNodeData {
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<Self, SpecificationError> {
        let node_spec: UdpNodeSpec =
            toml::from_str(&spec.to_string()).map_err(SpecificationError::ParseSpecification)?;

        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let mode = if let Some(mode) = &node_spec.mode {
            UdpMode::try_from(mode.as_str())
        } else {
            Ok(UdpMode::default())
        }
        .map_err(|node_error| SpecificationError::Module(Box::new(node_error)))?;
        let ttl = node_spec.ttl.unwrap_or(DEFAULT_TTL);
        let buffer_size = node_spec
            .buffer_size
            .unwrap_or(DEFAULT_SOCKET_BUFFER_CAPACITY);
        let block_own_socket = node_spec
            .block_own_socket
            .unwrap_or(DEFAULT_BLOCK_OWN_SOCKET);

        let interface = node_spec.interface.parse::<SocketAddr>().map_err(|_err| {
            SpecificationError::Module(Box::new(UdpNodeError::IncorrectInterface(
                instance_id,
                node_spec.interface,
            )))
        })?;
        let address = node_spec.uri.parse::<SocketAddr>().map_err(|_err| {
            SpecificationError::Module(Box::new(UdpNodeError::IncorrectUri(
                instance_id,
                node_spec.uri,
            )))
        })?;

        Ok(Self {
            base: BaseNode::new(instance_id, node_spec.name.clone(), cmd_rx, event_tx),
            buffer_size,
            mode,
            interface,
            address,
            ttl,
            block_own_socket,
            incoming: None,
            outgoing: out_tx,
        })
    }

    node_data_impl!(
        Bytes,
        self.incoming,
        self.outgoing,
        self.base.instance_id,
        self.base.name,
        UdpNodeRunner
    );
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

    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, CreationError> {
        let socket = create_udp_socket(&data)
            .map_err(|udp_error| CreationError::CreateNode(Box::new(udp_error)))?;

        let mut buffer = BytesMut::with_capacity(data.buffer_size);
        buffer.resize(data.buffer_size, 0);

        let mut node_runner = Self {
            instance_id: data.base.instance_id,
            name: data.base.name,
            buffer,
            address: data.address,
            socket,
            block_own_socket: data.block_own_socket,
            statistics: SocketStatistics::new(data.base.instance_id),
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
                            Event::RuntimeError(ExecutionError::OutputChannelSend(self.id())),
                        );
                    };
                }
                UdpNodeEvent::BlockedPacket => {}
                UdpNodeEvent::ReceivedIncoming(incoming_data) => {
                    match self.socket.send_to(&incoming_data, self.address).await {
                        Ok(_bytes_send) => {}
                        Err(err) => Self::emit_event(
                            &event_tx,
                            Event::RuntimeError(ExecutionError::NodeExecution {
                                node_id: self.id(),
                                message: err.to_string(),
                            }),
                        ),
                    }
                }
                UdpNodeEvent::SocketError(err) => Self::emit_event(
                    &event_tx,
                    Event::RuntimeError(ExecutionError::NodeExecution {
                        node_id: self.id(),
                        message: err.to_string(),
                    }),
                ),
                UdpNodeEvent::OutputStatistics => {
                    if let Ok(json) = serde_json::to_string_pretty(&self.statistics) {
                        Self::emit_event(&event_tx, Event::SendStatistics(json))
                    }
                }
                UdpNodeEvent::Quit => {
                    break;
                }
            }
        }
    }
}

fn map_command_to_event(command: &Command) -> UdpNodeEvent {
    match command {
        Command::Quit => UdpNodeEvent::Quit,
    }
}

#[derive(Debug, Error)]
pub enum UdpNodeError {
    #[error("Node {0}, Cannot parse socket address for interface \"{1}\"")]
    IncorrectInterface(InstanceId, String),
    #[error("Node {0}, Cannot parse socket address for remote uri \"{1}\"")]
    IncorrectUri(InstanceId, String),
    #[error("Configured UDP mode is invalid. Valid values are '{0}', '{1}' and '{2}'.")]
    IncorrectMode(&'static str, &'static str, &'static str),
    #[error("Node {0}, failed to create socket.")]
    CreateSocket(InstanceId),
    #[error("Node {0}, failed to set SO_REUSEADDR for endpoint address {1}.")]
    SetReuseAddress(InstanceId, SocketAddr),
    #[error("Node {0}, failed to set SO_REUSEPORT for endpoint address {1}.")]
    SetReusePort(InstanceId, SocketAddr),
    #[error("Node {0}, failed to set non-blocking mode for endpoint address {1}.")]
    SetNonblocking(InstanceId, SocketAddr),
    #[error("Node {0}, failed to set SO_BROADCAST for endpoint address {1}.")]
    SetBroadcast(InstanceId, SocketAddr),
    #[error("Node {0}, failed to set TTL for endpoint address {1}.")]
    SetTtl(InstanceId, SocketAddr),
    #[error("Node {0}, failed to join multicast group {0} using interface {1} (IPv4).")]
    JoinMulticastV4(InstanceId, Ipv4Addr, Ipv4Addr),
    #[error("Node {0}, failed to join multicast group {0} using interface {1} (IPv6).")]
    JoinMulticastV6(InstanceId, Ipv6Addr, Ipv6Addr),
    #[error("Node {0}, failed to bind to address {0:?}")]
    BindToAddress(InstanceId, SocketAddr),
    #[error("Node {0}, failed to convert std::net::UdpSocket to tokio::net::UdpSocket.")]
    ConvertToAsync(InstanceId),
}

impl NodeError for UdpNodeError {}

/// Creates an UDP socket based on the settings contained in `endpoint`.
/// The created `tokio::net::udp::UdpSocket` is returned wrapped in an `Arc`
/// so that it can be used by multiple tasks (i.e., for both writing and sending).
#[allow(clippy::too_many_lines)]
fn create_udp_socket(endpoint: &UdpNodeData) -> Result<UdpSocket, UdpNodeError> {
    use socket2::{Domain, Protocol, Socket, Type};

    // Create socket using socket2 crate, to be able to set required socket options (SO_REUSEADDR, SO_REUSEPORT, ...)
    let is_ipv4 = endpoint.address.is_ipv4();
    let socket_domain = if is_ipv4 { Domain::IPV4 } else { Domain::IPV6 };
    let socket_type = Type::DGRAM;
    let socket_protocol = Protocol::UDP;
    let socket = Socket::new(socket_domain, socket_type, Some(socket_protocol))
        .map_err(|_err| UdpNodeError::CreateSocket(endpoint.base.instance_id))?;

    socket
        .set_reuse_address(true)
        .map_err(|_| UdpNodeError::SetReuseAddress(endpoint.base.instance_id, endpoint.address))?;

    #[cfg(all(
        target_family = "unix",
        not(any(target_os = "solaris", target_os = "illumos"))
    ))]
    socket
        .set_reuse_port(true)
        .map_err(|_| UdpNodeError::SetReusePort(endpoint.base.instance_id, endpoint.address))?;
    socket
        .set_nonblocking(true)
        .map_err(|_| UdpNodeError::SetNonblocking(endpoint.base.instance_id, endpoint.address))?;

    match (is_ipv4, endpoint.mode) {
        (true, UdpMode::UniCast) => {
            socket.bind(&endpoint.interface.into()).map_err(|_| {
                UdpNodeError::BindToAddress(endpoint.base.instance_id, endpoint.address)
            })?;
        }
        (true, UdpMode::BroadCast) => {
            socket.set_broadcast(true).map_err(|_| {
                UdpNodeError::SetBroadcast(endpoint.base.instance_id, endpoint.interface)
            })?;
            socket
                .set_ttl(endpoint.ttl)
                .map_err(|_| UdpNodeError::SetTtl(endpoint.base.instance_id, endpoint.interface))?;
            socket.bind(&endpoint.interface.into()).map_err(|_| {
                UdpNodeError::BindToAddress(endpoint.base.instance_id, endpoint.interface)
            })?;
        }
        (true, UdpMode::MultiCast) => {
            if let IpAddr::V4(ip_address_v4) = endpoint.address.ip() {
                if let IpAddr::V4(interface_v4) = endpoint.interface.ip() {
                    socket
                        .join_multicast_v4(&ip_address_v4, &interface_v4)
                        .map_err(|_| {
                            UdpNodeError::JoinMulticastV4(
                                endpoint.base.instance_id,
                                ip_address_v4,
                                interface_v4,
                            )
                        })?
                }
            }
        }
        (false, UdpMode::UniCast) => socket.bind(&endpoint.interface.into()).map_err(|_| {
            UdpNodeError::BindToAddress(endpoint.base.instance_id, endpoint.address)
        })?,
        (false, UdpMode::BroadCast) => {
            socket.set_broadcast(true).map_err(|_| {
                UdpNodeError::SetBroadcast(endpoint.base.instance_id, endpoint.interface)
            })?;
            socket
                .set_ttl(1)
                .map_err(|_| UdpNodeError::SetTtl(endpoint.base.instance_id, endpoint.interface))?;
            socket.bind(&endpoint.interface.into()).map_err(|_| {
                UdpNodeError::BindToAddress(endpoint.base.instance_id, endpoint.interface)
            })?;
        }
        (false, UdpMode::MultiCast) => {
            if let IpAddr::V6(ip_address_v6) = endpoint.address.ip() {
                if let IpAddr::V6(interface_v6) = endpoint.interface.ip() {
                    // TODO how does IPv6 work with u32 interface numbers - pick 'any' for now.
                    socket.join_multicast_v6(&ip_address_v6, 0).map_err(|_| {
                        UdpNodeError::JoinMulticastV6(
                            endpoint.base.instance_id,
                            ip_address_v6,
                            interface_v6,
                        )
                    })?;
                }
            }
        }
    }

    // Convert socket2::Socket to tokio::net::UdpSocket via std::net::UdpSocket
    let socket = std::net::UdpSocket::from(socket);
    let socket = UdpSocket::try_from(socket)
        .map_err(|_| UdpNodeError::ConvertToAsync(endpoint.base.instance_id))?;

    Ok(socket)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpServerNodeSpec {
    name: String,
    interface: String,
    buffer_size: Option<usize>,
    max_connections: Option<usize>,
}

#[derive(Debug)]
pub struct TcpServerNodeData {
    base: BaseNode,
    interface: SocketAddr,
    buffer_size: usize,
    // max_connections: usize,
    incoming: Option<Receiver<Bytes>>,
    outgoing: Sender<Bytes>,
}

pub struct TcpServerNodeRunner {
    instance_id: InstanceId,
    name: String,
    interface: SocketAddr,
    buffer_size: usize,
    // max_connections: usize,
    statistics: SocketStatistics,
}

impl NodeData for TcpServerNodeData {
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<Self, SpecificationError> {
        let node_spec: TcpServerNodeSpec =
            toml::from_str(&spec.to_string()).map_err(SpecificationError::ParseSpecification)?;

        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let interface = node_spec.interface.parse::<SocketAddr>().map_err(|_err| {
            SpecificationError::Module(Box::new(UdpNodeError::IncorrectInterface(
                instance_id,
                node_spec.interface,
            )))
        })?;
        let buffer_size = node_spec
            .buffer_size
            .unwrap_or(DEFAULT_SOCKET_BUFFER_CAPACITY);
        // let max_connections = node_spec
        //     .max_connections
        //     .unwrap_or(DEFAULT_TCP_MAX_CONNECTIONS);

        Ok(Self {
            base: BaseNode {
                instance_id,
                name: node_spec.name.clone(),
                cmd_rx,
                event_tx,
            },
            interface,
            buffer_size,
            // max_connections,
            incoming: None,
            outgoing: out_tx,
        })
    }

    node_data_impl!(
        Bytes,
        self.incoming,
        self.outgoing,
        self.base.instance_id,
        self.base.name,
        TcpServerNodeRunner
    );
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

    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, CreationError> {
        let mut node_runner = Self {
            instance_id: data.base.instance_id,
            name: data.base.name,
            interface: data.interface,
            buffer_size: data.buffer_size,
            // max_connections: data.max_connections,
            statistics: SocketStatistics::new(data.base.instance_id),
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

        let (tcp_read_tx, mut tcp_read_rx) =
            tokio::sync::mpsc::channel::<Bytes>(DEFAULT_NODE_CHANNEL_CAPACITY);
        let (tcp_write_tx, _tcp_write_rx) = channel::<Bytes>(DEFAULT_NODE_CHANNEL_CAPACITY);

        let socket = match TcpListener::bind(self.interface).await {
            Ok(socket) => socket,
            Err(_) => {
                Self::emit_event(
                    &event_tx,
                    Event::RuntimeError(ExecutionError::NodeExecution {
                        node_id: self.id(),
                        message: format!("Cannot bind TCP socket to {}", self.interface),
                    }),
                );
                return;
            }
        };

        loop {
            tokio::select! {
                Ok(command) = cmd_rx.recv() => {
                    if command == Command::Quit { break; }
                }
                Ok((stream, _remote_addr)) = socket.accept() => {
                    // TODO add semaphore for tracking max number of connections
                    // TODO whitelist/blacklist of remote addresses
                    let (reader, writer) = stream.into_split();
                    let closer = Arc::new(Notify::new());
                    let read_handle = tokio::spawn(run_tcp_reader(reader, tcp_read_tx.clone(), closer.clone(), self.buffer_size));
                    let write_handle = tokio::spawn(run_tcp_write(writer, tcp_write_tx.subscribe(), closer.clone()));
                    tokio::spawn( async move {
                        let _ = read_handle.await;
                        let _ = write_handle.await;
                    });
                }
                Some(message) = Self::receive_incoming(self.instance_id, &mut incoming) => {
                    self.statistics.received_incoming(message.len());
                    let _ = tcp_write_tx.send(message).inspect_err(|err|
                        Self::emit_event(&event_tx, Event::RuntimeError(ExecutionError::NodeExecution {
                            node_id: self.id(),
                            message: err.to_string()
                        }))
                    );
                }
                Some(message) = tcp_read_rx.recv() => {
                    self.statistics.received_packet(message.len());
                    let _ = outgoing.send(message).inspect_err(|_|
                        Self::emit_event(&event_tx, Event::RuntimeError(ExecutionError::OutputChannelSend(self.id()))));
                }
                // aggregate statistics for the interval
                _ = aggregate_stats_interval.tick() => {
                    self.statistics.aggregate_interval();
                }
                // output current state of the stats
                _ = output_stats_interval.tick() => {
                    if let Ok(json) = serde_json::to_string_pretty(&self.statistics) {
                        Self::emit_event(&event_tx,
                            Event::SendStatistics(json))
                    }
                }
            }
        }
    }
}

async fn run_tcp_reader(
    mut reader: OwnedReadHalf,
    to_node: tokio::sync::mpsc::Sender<Bytes>,
    closer: Arc<Notify>,
    buffer_size: usize,
) {
    let mut buf = BytesMut::with_capacity(buffer_size);
    buf.resize(buffer_size, 0);

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
                    Ok(0) => { }
                    Ok(_bytes_sent) => { }
                    Err(_err) => { }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpClientNodeSpec {
    name: String,
    interface: String,
    uri: String,
    buffer_size: Option<usize>,
}

#[derive(Debug)]
pub struct TcpClientNodeData {
    base: BaseNode,
    interface: SocketAddr,
    address: SocketAddr,
    buffer_size: usize,
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
    ) -> Result<Self, SpecificationError> {
        let node_spec: TcpClientNodeSpec =
            toml::from_str(&spec.to_string()).map_err(SpecificationError::ParseSpecification)?;

        let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

        let interface = node_spec.interface.parse::<SocketAddr>().map_err(|_err| {
            SpecificationError::Module(Box::new(UdpNodeError::IncorrectInterface(
                instance_id,
                node_spec.interface,
            )))
        })?;
        let address = node_spec.uri.parse::<SocketAddr>().map_err(|_err| {
            SpecificationError::Module(Box::new(UdpNodeError::IncorrectUri(
                instance_id,
                node_spec.uri,
            )))
        })?;

        let buffer_size = node_spec
            .buffer_size
            .unwrap_or(DEFAULT_SOCKET_BUFFER_CAPACITY);

        Ok(Self {
            base: BaseNode {
                instance_id,
                name: node_spec.name.clone(),
                cmd_rx,
                event_tx,
            },
            interface,
            address,
            buffer_size,
            incoming: None,
            outgoing: out_tx,
        })
    }

    node_data_impl!(
        Bytes,
        self.incoming,
        self.outgoing,
        self.base.instance_id,
        self.base.name,
        TcpClientNodeRunner
    );
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

    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, CreationError> {
        let mut buffer = BytesMut::with_capacity(data.buffer_size);
        buffer.resize(data.buffer_size, 0);

        let mut node_runner = Self {
            instance_id: data.base.instance_id,
            name: data.base.name,
            buffer,
            interface: data.interface,
            address: data.address,
            statistics: SocketStatistics::new(data.base.instance_id),
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
                    Event::RuntimeError(ExecutionError::NodeExecution {
                        node_id: self.id(),
                        message: err.to_string(),
                    }),
                );
                return;
            }
        };

        if let Err(err) = socket.bind(self.interface) {
            Self::emit_event(
                &event_tx,
                Event::RuntimeError(ExecutionError::NodeExecution {
                    node_id: self.id(),
                    message: err.to_string(),
                }),
            );
        }
        let mut tcp_stream = match socket.connect(self.address).await {
            Ok(stream) => stream,
            Err(err) => {
                Self::emit_event(
                    &event_tx,
                    Event::RuntimeError(ExecutionError::NodeExecution {
                        node_id: self.id(),
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
                        Self::emit_event(&event_tx, Event::RuntimeError(ExecutionError::NodeExecution {
                            node_id: self.id(),
                            message: "TCP client node disconnected.".to_string(),
                        }));
                    } else if let Ok(_num_receivers) = outgoing
                        .send(Bytes::copy_from_slice(&self.buffer[..bytes_received])) {
                        self.statistics.received_packet(bytes_received);
                    } else {
                        Self::emit_event(&event_tx, Event::RuntimeError(
                            ExecutionError::OutputChannelSend(self.id())
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
                    if let Ok(json) = serde_json::to_string_pretty(&self.statistics) {
                        Self::emit_event(&event_tx,
                            Event::SendStatistics(json)
                        );
                    }
                }
            }
        }
    }
}
