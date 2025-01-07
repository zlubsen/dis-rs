use crate::core::{NodeConstructor, NodeData};
use crate::error::InfraError;
use crate::infra::network::UdpNodeData;
use crate::runtime::{Command, Event};
use toml::Value;

const NODE_CHANNEL_CAPACITY: usize = 50;
const SOCKET_BUFFER_CAPACITY: usize = 32_768;

pub fn builtin_nodes() -> Vec<(&'static str, NodeConstructor)> {
    let udp_func_ptr: NodeConstructor = node_data_from_spec;

    let mut items = Vec::new();
    let mod_udp = ("udp", udp_func_ptr);
    let mod_dis = ("dis", udp_func_ptr); // FIXME actual function
    items.push(mod_udp);
    items.push(mod_dis);
    items
}

pub fn node_data_from_spec(
    instance_id: u64,
    cmd_rx: tokio::sync::broadcast::Receiver<Command>,
    event_tx: tokio::sync::broadcast::Sender<Event>,
    spec: &toml::Table,
) -> Result<Box<dyn NodeData>, InfraError> {
    if !spec.contains_key("type") {
        return Err(InfraError::InvalidSpec {
            message: "Node specification does not contain the 'type' of the node.".to_string(),
        });
    }
    println!("creating node for {:?}", &spec["type"].as_str());
    match &spec["type"] {
        Value::String(value) => match value.as_str() {
            "udp" => {
                let spec: network::UdpNodeSpec = toml::from_str(&spec.to_string()).unwrap();
                let node = UdpNodeData::new(instance_id, cmd_rx, event_tx, &spec)?.to_dyn();
                println!("constructed an UDP node");
                Ok(node)
            }
            "dis" => Err(InfraError::InvalidSpec {
                message: "Unimplemented".to_string(),
            }),
            unknown_value => Err(InfraError::InvalidSpec {
                message: format!("Node type is not known '{unknown_value}'"),
            }),
        },
        invalid_value => Err(InfraError::InvalidSpec {
            message: format!(
                "Node type is of an invalid data type ('{}')",
                invalid_value.to_string()
            ),
        }),
    }
}

pub fn register_channel_from_spec(
    spec: &toml::Table,
    nodes: &mut Vec<Box<dyn NodeData>>,
) -> Result<(), InfraError> {
    let from = spec
        .get("from")
        .ok_or(InfraError::InvalidSpec {
            message: "Channel spec misses field 'from'.".to_string(),
        })?
        .as_str()
        .ok_or(InfraError::InvalidSpec {
            message: "Channel spec field 'from' is not a string value.".to_string(),
        })?;
    let to = spec
        .get("to")
        .ok_or(InfraError::InvalidSpec {
            message: "Channel spec misses field 'to'.".to_string(),
        })?
        .as_str()
        .ok_or(InfraError::InvalidSpec {
            message: "Channel spec field 'to' is not a string value.".to_string(),
        })?;

    let from_id = nodes
        .iter()
        .find(|node| node.name() == from)
        .ok_or(InfraError::InvalidSpec {
            message: format!(
                "Invalid channel spec, no correct (from) node with name '{from}' is defined."
            ),
        })?
        .id();
    let to_id = nodes
        .iter()
        .find(|node| node.name() == to)
        .ok_or(InfraError::InvalidSpec {
            message: format!(
                "Invalid channel spec, no correct (to) node with name '{to}' is defined."
            ),
        })?
        .id();

    let from_node = nodes.get(from_id as usize).unwrap();
    let sub = from_node.request_subscription();
    let to_node = nodes.get_mut(to_id as usize).unwrap();
    to_node.register_subscription(sub)?;

    Ok(())
}

pub mod network {
    use crate::core::{BaseNode, NodeData, NodeRunner};
    use crate::error::InfraError;
    use crate::infra::{NODE_CHANNEL_CAPACITY, SOCKET_BUFFER_CAPACITY};
    use crate::runtime::{Command, Event};
    use bytes::{Bytes, BytesMut};
    use serde_derive::{Deserialize, Serialize};
    use std::any::Any;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::time::Duration;
    use tokio::net::UdpSocket;
    use tokio::select;
    use tokio::sync::broadcast::error::RecvError;
    use tokio::sync::broadcast::{channel, Receiver, Sender};
    use tokio::task::JoinHandle;
    use tracing::error;

    const DEFAULT_TTL: u32 = 1;
    const DEFAULT_BLOCK_OWN_SOCKET: bool = true;
    const DEFAULT_AGGREGATE_STATS_INTERVAL_MS: u64 = 1000;
    const DEFAULT_OWN_ADDRESS: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

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
        data: UdpNodeData,
        socket: UdpSocket,
        statistics: UdpNodeStatistics,
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
    struct UdpNodeStatistics {
        total: UdpNodeStatisticsItems,
        running_interval: UdpNodeStatisticsItems,
        latest_interval: UdpNodeStatisticsItems,
    }

    #[derive(Copy, Clone, Debug, Default)]
    struct UdpNodeStatisticsItems {
        messages_in: u64,
        messages_out: u64,
        packets_socket_in: u64,
        packets_socket_in_blocked: u64,
        packets_socket_out: u64,
        bytes_in: u64,
        bytes_out: u64,
        bytes_socket_in: u64,
        bytes_socket_out: u64,
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
                "unicast" => Ok(Self::UniCast),
                "broadcast" => Ok(Self::BroadCast),
                "multicast" => Ok(Self::MultiCast),
                _ => Err(InfraError::InvalidSpec { message: "Configured UDP mode is invalid. Valid values are 'unicast', 'broadcast' and 'multicast'.".to_string() }),
            }
        }
    }

    impl UdpNodeData {
        pub fn new(
            instance_id: u64,
            cmd_rx: Receiver<Command>,
            event_tx: Sender<Event>,
            node_spec: &UdpNodeSpec,
        ) -> Result<Self, InfraError> {
            let (out_tx, _out_rx) = channel(NODE_CHANNEL_CAPACITY);

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

            let interface = node_spec.interface.parse::<SocketAddr>().map_err(|_err| {
                InfraError::InvalidSpec {
                    message: format!(
                        "Node {instance_id} - Cannot parse socket address for interface {}",
                        node_spec.interface
                    ),
                }
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
    }

    impl NodeData for UdpNodeData {
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
                })
            }
        }

        fn id(&self) -> u64 {
            self.base.instance_id
        }

        fn name(&self) -> &str {
            self.base.name.as_str()
        }

        fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
            UdpNodeRunner::spawn_with_data(*self)
        }
    }

    impl NodeRunner for UdpNodeRunner {
        type Data = UdpNodeData;

        fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
            let socket = create_udp_socket(&data);
            let mut node_runner = Self {
                data,
                socket,
                statistics: UdpNodeStatistics::default(),
            };

            tokio::spawn(async move { node_runner.run().await })
        }

        async fn run(&mut self) {
            let mut aggregate_stats_interval =
                tokio::time::interval(Duration::from_millis(DEFAULT_AGGREGATE_STATS_INTERVAL_MS));

            let default_own_socketaddr = DEFAULT_OWN_ADDRESS;
            let local_address = self.socket.local_addr().unwrap_or(default_own_socketaddr);

            loop {
                let event: UdpNodeEvent = select! {
                    // receiving commands
                    Ok(cmd) = self.data.base.cmd_rx.recv() => {
                        map_command_to_event(&cmd)
                    }
                    // receiving from the incoming channel
                    incoming = async {
                        match &mut self.data.incoming {
                            Some(channel) => match channel
                                .recv()
                                .await {
                                    Ok(received) => { UdpNodeEvent::ReceivedIncoming(received) }
                                    Err(err) => {
                                        UdpNodeEvent::ReceiveIncomingError(err)
                                }
                            }
                            None => UdpNodeEvent::NoEvent
                        }
                    } => { incoming }
                    // receiving from the socket
                    Ok((bytes_received, from_address)) = self.socket.recv_from(&mut self.data.buffer) => {
                        if self.data.block_own_socket && (local_address == from_address) {
                            UdpNodeEvent::BlockedPacket
                        } else {
                            Bytes::copy_from_slice(&self.data.buffer[..bytes_received]);
                            UdpNodeEvent::ReceivedPacket(Bytes::copy_from_slice(&self.data.buffer[..bytes_received]))
                        }
                    }
                    // aggregate statistics for the interval
                    _ = aggregate_stats_interval.tick() => {
                        self.aggregate_statistics();
                        UdpNodeEvent::OutputStatistics
                    }
                };

                match event {
                    UdpNodeEvent::NoEvent => {}
                    UdpNodeEvent::ReceivedPacket(bytes) => {
                        if let Ok(_num_receivers) = self.data.outgoing.send(bytes) {
                        } else {
                            if let Err(err) = self.data.base.event_tx.send(Event::NodeError(
                                InfraError::RuntimeNode {
                                    instance_id: self.data.base.instance_id,
                                    message: "Outgoing channel send failed".to_string(),
                                },
                            )) {
                                error!("{err}");
                                break;
                            }
                        };
                    }
                    UdpNodeEvent::BlockedPacket => {}
                    UdpNodeEvent::ReceivedIncoming(incoming_data) => {
                        match self.socket.send_to(&incoming_data, self.data.address).await {
                            Ok(_bytes_send) => {}
                            Err(err) => {
                                if let Err(err) = self.data.base.event_tx.send(Event::NodeError(
                                    InfraError::RuntimeNode {
                                        instance_id: self.data.base.instance_id,
                                        message: err.to_string(),
                                    },
                                )) {
                                    error!("{err}");
                                    break;
                                }
                            }
                        }
                    }
                    UdpNodeEvent::SocketError(err) => {
                        if let Err(err) = self.data.base.event_tx.send(Event::NodeError(
                            InfraError::RuntimeNode {
                                instance_id: self.data.base.instance_id,
                                message: err.to_string(),
                            },
                        )) {
                            error!("{err}");
                            break;
                        }
                    }
                    UdpNodeEvent::OutputStatistics => {
                        // TODO send statistics out
                    }
                    UdpNodeEvent::Quit => {
                        break;
                    }
                    UdpNodeEvent::ReceiveIncomingError(_) => {}
                    UdpNodeEvent::SendOutgoingChannelError => {}
                    UdpNodeEvent::SendEventChannelError => {}
                }
            }
        }
    }

    fn map_command_to_event(command: &Command) -> UdpNodeEvent {
        match command {
            Command::Quit => UdpNodeEvent::Quit,
        }
    }

    impl UdpNodeRunner {
        fn collect_statistics(&mut self, event: &UdpNodeEvent) {
            // TODO
            match event {
                UdpNodeEvent::NoEvent => {}
                UdpNodeEvent::ReceivedPacket(bytes) => {
                    self.statistics.total.packets_socket_in += 1;
                    self.statistics.total.bytes_socket_in += bytes.len() as u64;
                    self.statistics.running_interval.packets_socket_in += 1;
                    self.statistics.running_interval.bytes_socket_in += bytes.len() as u64;
                    self.statistics.total.messages_out += 1;
                    self.statistics.running_interval.messages_out += 1;
                }
                UdpNodeEvent::BlockedPacket => {
                    self.statistics.total.packets_socket_in_blocked += 1;
                    self.statistics.running_interval.packets_socket_in_blocked += 1;
                }
                UdpNodeEvent::ReceivedIncoming(bytes) => {
                    self.statistics.total.packets_socket_out += 1;
                    self.statistics.total.bytes_socket_out += bytes.len() as u64;
                    self.statistics.running_interval.packets_socket_out += 1;
                    self.statistics.running_interval.bytes_socket_out += bytes.len() as u64;
                    self.statistics.total.messages_in += 1;
                    self.statistics.running_interval.messages_in += 1;
                }
                UdpNodeEvent::SocketError(_) => {}
                UdpNodeEvent::OutputStatistics => {}
                UdpNodeEvent::Quit => {}
                UdpNodeEvent::ReceiveIncomingError(_) => {}
                UdpNodeEvent::SendOutgoingChannelError => {}
                UdpNodeEvent::SendEventChannelError => {}
            }
        }

        fn aggregate_statistics(&mut self) {
            self.statistics.latest_interval = self.statistics.running_interval;
            self.statistics.running_interval = UdpNodeStatisticsItems::default();
        }
    }

    /// Creates an UDP socket based on the settings contained in `endpoint`.
    /// The created `tokio::net::udp::UdpSocket` is returned wrapped in an `Arc`
    /// so that it can be used by multiple tasks (i.e., for both writing and sending).
    #[allow(clippy::too_many_lines)]
    fn create_udp_socket(endpoint: &UdpNodeData) -> UdpSocket {
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
                    error!(
                        "Failed to bind to IPv4 address {:?} - {}",
                        endpoint.address, err
                    );
                }
            }
            (true, UdpMode::BroadCast) => {
                if let Err(err) = socket.set_broadcast(true) {
                    error!(
                        "Failed to set SO_BROADCAST for endpoint address {} - {}.",
                        endpoint.interface, err
                    );
                }
                if let Err(err) = socket.bind(&endpoint.interface.into()) {
                    error!(
                        "Failed to bind to IPv4 address {:?} - {}",
                        endpoint.address, err
                    );
                }
                if let Err(err) = socket.set_ttl(endpoint.ttl) {
                    error!("Failed to set TTL - {err}.");
                }
            }
            (true, UdpMode::MultiCast) => {
                if let IpAddr::V4(ip_address_v4) = endpoint.address.ip() {
                    if let IpAddr::V4(interface_v4) = endpoint.interface.ip() {
                        socket
                            .join_multicast_v4(&ip_address_v4, &interface_v4)
                            .unwrap_or_else(|_| {
                                panic!("Failed to join multicast group {ip_address_v4} using interface {interface_v4}.")
                            });
                    }
                }
            }
            (false, UdpMode::UniCast) => {
                // TODO use .inspect_err() ?
                socket.bind(&endpoint.interface.into()).unwrap_or_else(|_| {
                    panic!("Failed to bind to IPv6 address {:?}", endpoint.address)
                });
            }
            (false, UdpMode::BroadCast) => {
                socket
                    .set_broadcast(true)
                    .expect("Failed to set SO_BROADCAST.");
                socket.set_ttl(1).expect("Failed to set TTL.");
                socket.bind(&endpoint.interface.into()).unwrap_or_else(|_| {
                    panic!("Failed to bind to IPv6 address {:?}", endpoint.address)
                });
            }
            (false, UdpMode::MultiCast) => {
                if let IpAddr::V6(ip_address_v6) = endpoint.address.ip() {
                    if let IpAddr::V6(interface_v6) = endpoint.interface.ip() {
                        // TODO how does IPv6 work with u32 interface numbers - pick 'any' for now.
                        socket
                            .join_multicast_v6(&ip_address_v6, 0)
                            .unwrap_or_else(|_| {
                                panic!("Failed to join multicast group {ip_address_v6} using interface 0 ({interface_v6}).")
                            });
                    }
                }
            }
        }

        // Convert socket2::Socket to tokio::net::UdpSocket via std::net::UdpSocket
        let socket = std::net::UdpSocket::from(socket);
        let socket = UdpSocket::try_from(socket)
            .expect("Failed to convert std::net::UdpSocket to tokio::net::UdpSocket.");

        socket
    }
}

pub mod dis {
    use crate::core::BaseNode;
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DisNodeSpec {
        exercise_id: Option<u8>,
        dis_version: Option<u8>,
        // add DIS federation parameters
    }

    #[derive(Debug)]
    pub struct DisNodeData {
        base_node: BaseNode,
    }

    pub struct DisNodeRunner {
        data: DisNodeData,
    }
}
