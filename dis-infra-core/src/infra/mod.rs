pub mod util {
    use crate::core::{
        BaseNode, BaseNodeSpec, BaseStatistics, InstanceId, NodeConstructor, NodeData, NodeRunner,
        UntypedNode, DEFAULT_AGGREGATE_STATS_INTERVAL_MS, DEFAULT_NODE_CHANNEL_CAPACITY,
        DEFAULT_OUTPUT_STATS_INTERVAL_MS,
    };
    use crate::error::InfraError;
    use crate::runtime::{Command, Event};
    use bytes::Bytes;
    use std::any::Any;
    use std::time::Duration;
    use tokio::sync::broadcast::{channel, Receiver, Sender};
    use tokio::task::JoinHandle;

    const SPEC_PASS_THROUGH_NODE_TYPE: &str = "pass_through";

    pub fn available_nodes() -> Vec<(&'static str, NodeConstructor)> {
        let util_nodes_constructor: NodeConstructor = node_from_spec;

        let mut items = Vec::new();
        items.push((SPEC_PASS_THROUGH_NODE_TYPE, util_nodes_constructor));
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
            SPEC_PASS_THROUGH_NODE_TYPE => {
                let node = PassThroughNodeData::new(instance_id, cmd_rx, event_tx, spec)?.to_dyn();
                Ok(node)
            }
            unknown_value => Err(InfraError::InvalidSpec {
                message: format!("Unknown node type '{unknown_value}' for module 'util'"),
            }),
        }
    }

    #[derive(Debug)]
    pub struct PassThroughNodeData {
        base: BaseNode,
        incoming: Option<Receiver<Bytes>>,
        outgoing: Sender<Bytes>,
    }

    pub struct PassThroughNodeRunner {
        data: PassThroughNodeData,
        statistics: BaseStatistics,
    }

    impl NodeData for PassThroughNodeData {
        fn new(
            instance_id: InstanceId,
            cmd_rx: Receiver<Command>,
            event_tx: Sender<Event>,
            spec: &toml::Table,
        ) -> Result<Self, InfraError> {
            let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

            let node_spec: BaseNodeSpec =
                toml::from_str(&spec.to_string()).map_err(|err| InfraError::InvalidSpec {
                    message: err.to_string(),
                })?;

            Ok(Self {
                base: BaseNode {
                    instance_id,
                    name: node_spec.name.clone(),
                    cmd_rx,
                    event_tx,
                },
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
                })
            }
        }

        fn id(&self) -> InstanceId {
            self.base.instance_id
        }

        fn name(&self) -> &str {
            self.base.name.as_str()
        }

        fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
            PassThroughNodeRunner::spawn_with_data(*self)
        }
    }

    enum NodeEvent<T> {
        ReceivedIncoming(T),
        ReceiveIncomingError,
    }

    impl NodeRunner for PassThroughNodeRunner {
        type Data = PassThroughNodeData;

        fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
            let mut node_runner = Self {
                data,
                statistics: BaseStatistics::default(),
            };

            tokio::spawn(async move { node_runner.run().await })
        }

        async fn run(&mut self) {
            loop {
                let mut aggregate_stats_interval = tokio::time::interval(Duration::from_millis(
                    DEFAULT_AGGREGATE_STATS_INTERVAL_MS,
                ));
                let mut output_stats_interval =
                    tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

                tokio::select! {
                    // receiving commands
                    Ok(cmd) = self.data.base.cmd_rx.recv() => {
                        if cmd == Command::Quit { break; }
                    }
                    // receiving from the incoming channel
                    _ = async {
                        match &mut self.data.incoming {
                            Some(channel) => match channel.recv().await {
                                Ok(message) => {
                                    let _send_result = self.data.outgoing.send(message);
                                }
                                Err(_err) => {}
                            },
                            None => {}
                        }
                    } => { }
                    _ = aggregate_stats_interval.tick() => {
                        self.statistics.aggregate_interval();
                    }
                    _ = output_stats_interval.tick() => {
                        // TODO
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn some_test() {
            assert!(false);
        }
    }
}

pub mod network {
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
    use std::time::Duration;
    use tokio::net::{TcpListener, UdpSocket};
    use tokio::sync::broadcast::error::RecvError;
    use tokio::sync::broadcast::{channel, Receiver, Sender};
    use tokio::task::JoinHandle;
    use toml::Table;
    use tracing::error;

    const SOCKET_BUFFER_CAPACITY: usize = 32_768;

    const DEFAULT_TTL: u32 = 1;
    const DEFAULT_BLOCK_OWN_SOCKET: bool = true;
    const DEFAULT_OWN_ADDRESS: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    const DEFAULT_TCP_MAX_CONNECTIONS: usize = 15;

    const SPEC_UDP_NODE_TYPE: &str = "udp";
    const SPEC_UDP_MODE_UNICAST: &str = "unicast";
    const SPEC_UDP_MODE_BROADCAST: &str = "broadcast";
    const SPEC_UDP_MODE_MULTICAST: &str = "multicast";

    pub fn available_nodes() -> Vec<(&'static str, NodeConstructor)> {
        let network_nodes_constructor: NodeConstructor = node_from_spec;

        let mut items = Vec::new();
        items.push((SPEC_UDP_NODE_TYPE, network_nodes_constructor));
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
                let node = UdpNodeData::new(instance_id, cmd_rx, event_tx, &spec)?.to_dyn();
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
        data: UdpNodeData,
        socket: UdpSocket,
        statistics: UdpNodeStatistics,
    }

    #[derive(Debug)]
    enum UdpNodeEvent {
        NoEvent,
        ReceivedPacket(Bytes),
        BlockedPacket(usize),
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
        base: BaseStatistics,
        total: UdpNodeStatisticsItems,
        running_interval: UdpNodeStatisticsItems,
        latest_interval: UdpNodeStatisticsItems,
    }

    #[derive(Copy, Clone, Debug, Default)]
    struct UdpNodeStatisticsItems {
        packets_socket_in: u64,
        packets_socket_in_blocked: u64,
        packets_socket_out: u64,
        bytes_socket_in: u64,
        bytes_socket_out: u64,
        bytes_in: u64,
        bytes_out: u64,
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

        fn id(&self) -> InstanceId {
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
            let mut output_stats_interval =
                tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

            let default_own_socketaddr = DEFAULT_OWN_ADDRESS;
            let local_address = self.socket.local_addr().unwrap_or(default_own_socketaddr);

            loop {
                let event: UdpNodeEvent = tokio::select! {
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
                            UdpNodeEvent::BlockedPacket(bytes_received)
                        } else {
                            Bytes::copy_from_slice(&self.data.buffer[..bytes_received]);
                            UdpNodeEvent::ReceivedPacket(Bytes::copy_from_slice(&self.data.buffer[..bytes_received]))
                        }
                    }
                    // aggregate statistics for the interval
                    _ = aggregate_stats_interval.tick() => {
                        self.aggregate_statistics_interval();
                        UdpNodeEvent::NoEvent
                    }
                    // output current state of the stats
                    _ = output_stats_interval.tick() => {
                        UdpNodeEvent::OutputStatistics
                    }
                };

                self.collect_statistics(&event);

                match event {
                    UdpNodeEvent::NoEvent => {}
                    UdpNodeEvent::ReceivedPacket(bytes) => {
                        if let Ok(_num_receivers) = self.data.outgoing.send(bytes) {
                        } else {
                            if let Err(err) = self.data.base.event_tx.send(Event::NodeError(
                                InfraError::RuntimeNode {
                                    instance_id: self.data.base.instance_id,
                                    message: "Outgoing channel send failed.".to_string(),
                                },
                            )) {
                                error!("{err}");
                                break;
                            }
                        };
                    }
                    UdpNodeEvent::BlockedPacket(_) => {}
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

    impl UdpNodeRunner {
        fn collect_statistics(&mut self, event: &UdpNodeEvent) {
            match event {
                UdpNodeEvent::ReceivedPacket(bytes) => {
                    self.statistics.total.packets_socket_in += 1;
                    self.statistics.total.bytes_socket_in += bytes.len() as u64;
                    self.statistics.total.bytes_out += bytes.len() as u64;
                    self.statistics.running_interval.packets_socket_in += 1;
                    self.statistics.running_interval.bytes_socket_in += bytes.len() as u64;
                    self.statistics.running_interval.bytes_out += bytes.len() as u64;
                    self.statistics.base.outgoing_message();
                }
                UdpNodeEvent::BlockedPacket(num_bytes) => {
                    self.statistics.total.packets_socket_in += 1;
                    self.statistics.total.packets_socket_in_blocked += 1;
                    self.statistics.total.bytes_socket_in += *num_bytes as u64;
                    self.statistics.running_interval.packets_socket_in += 1;
                    self.statistics.running_interval.packets_socket_in_blocked += 1;
                }
                UdpNodeEvent::ReceivedIncoming(bytes) => {
                    self.statistics.total.packets_socket_out += 1;
                    self.statistics.total.bytes_socket_out += bytes.len() as u64;
                    self.statistics.total.bytes_in += bytes.len() as u64;
                    self.statistics.running_interval.packets_socket_out += 1;
                    self.statistics.running_interval.bytes_socket_out += bytes.len() as u64;
                    self.statistics.running_interval.bytes_in += bytes.len() as u64;
                    self.statistics.base.incoming_message();
                }
                _ => {}
            }
        }

        fn aggregate_statistics_interval(&mut self) {
            self.statistics.latest_interval = self.statistics.running_interval;
            self.statistics.running_interval = Default::default();
            self.statistics.base.aggregate_interval();
        }
    }

    /// Creates an UDP socket based on the settings contained in `endpoint`.
    /// The created `tokio::net::udp::UdpSocket` is returned wrapped in an `Arc`
    /// so that it can be used by multiple tasks (i.e., for both writing and sending).
    #[allow(clippy::too_many_lines)]
    fn create_udp_socket(endpoint: &UdpNodeData) -> UdpSocket {
        // TODO make function fallible
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

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TcpServerNodeSpec {
        name: String,
        interface: String,
        max_connections: Option<usize>,
        block_own_socket: Option<bool>,
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
        data: TcpServerNodeData,
        statistics: TcpNodeStatistics,
    }

    #[derive(Copy, Clone, Default, Debug)]
    pub struct TcpNodeStatistics {
        base: BaseStatistics,
    }

    impl NodeData for TcpServerNodeData {
        fn new(
            instance_id: InstanceId,
            cmd_rx: Receiver<Command>,
            event_tx: Sender<Event>,
            spec: &Table,
        ) -> Result<Self, InfraError> {
            let node_spec: TcpServerNodeSpec =
                toml::from_str(&spec.to_string()).map_err(|err| InfraError::InvalidSpec {
                    message: err.to_string(),
                })?;

            let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

            let mut buffer = BytesMut::with_capacity(SOCKET_BUFFER_CAPACITY);
            buffer.resize(SOCKET_BUFFER_CAPACITY, 0);

            let interface = node_spec.interface.parse::<SocketAddr>().map_err(|_err| {
                InfraError::InvalidSpec {
                    message: format!(
                        "Node {instance_id} - Cannot parse socket address for interface {}",
                        node_spec.interface
                    ),
                }
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
                })
            }
        }

        fn id(&self) -> InstanceId {
            self.base.instance_id
        }

        fn name(&self) -> &str {
            self.base.name.as_str()
        }

        fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
            TcpServerNodeRunner::spawn_with_data(*self)
        }
    }

    impl NodeRunner for TcpServerNodeRunner {
        type Data = TcpServerNodeData;

        fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
            let mut node_runner = Self {
                data,
                statistics: TcpNodeStatistics::default(),
            };

            tokio::spawn(async move { node_runner.run().await })
        }

        async fn run(&mut self) {
            let socket = TcpListener::bind(self.data.interface).await.unwrap();
            // let (mut a,b) = self.socket.accept().await.unwrap();
            // let (reader, writer) = a.split();
            //
            loop {
                tokio::select! {
                    Ok(command) = self.data.base.cmd_rx.recv() => {
                        if command == Command::Quit { break; }
                    }
                    // Ok((stream, addr)) = self.socket.accept() {
                    //
                    //     // run the reader and write loops
                    // }
                }
            }
        }
    }
}

pub mod dis {
    use crate::core::{
        BaseNode, BaseStatistics, InstanceId, NodeConstructor, NodeData, NodeRunner, UntypedNode,
        DEFAULT_AGGREGATE_STATS_INTERVAL_MS, DEFAULT_NODE_CHANNEL_CAPACITY,
        DEFAULT_OUTPUT_STATS_INTERVAL_MS,
    };
    use crate::error::InfraError;
    use crate::runtime::{Command, Event};
    use bytes::{Bytes, BytesMut};
    use dis_rs::enumerations::ProtocolVersion;
    use dis_rs::model::Pdu;
    use serde_derive::{Deserialize, Serialize};
    use std::any::Any;
    use std::time::Duration;
    use tokio::sync::broadcast::{channel, Receiver, Sender};
    use tokio::task::JoinHandle;
    use tracing::trace;

    const SPEC_DIS_RECEIVER_NODE_TYPE: &str = "dis_receiver";
    const SPEC_DIS_SENDER_NODE_TYPE: &str = "dis_sender";

    const DEFAULT_DIS_RECEIVE_VERSIONS: [ProtocolVersion; 2] = [
        ProtocolVersion::IEEE1278_1A1998,
        ProtocolVersion::IEEE1278_12012,
    ];
    const DEFAULT_DIS_SEND_VERSION: ProtocolVersion = ProtocolVersion::IEEE1278_12012;

    const SERIALISE_BUFFER_CAPACITY: usize = 32_768;

    pub fn available_nodes() -> Vec<(&'static str, NodeConstructor)> {
        let dis_nodes_constructor: NodeConstructor = node_from_spec;

        let mut items = Vec::new();
        items.push((SPEC_DIS_RECEIVER_NODE_TYPE, dis_nodes_constructor));
        items.push((SPEC_DIS_SENDER_NODE_TYPE, dis_nodes_constructor));
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
            SPEC_DIS_RECEIVER_NODE_TYPE => {
                let node = DisRxNodeData::new(instance_id, cmd_rx, event_tx, &spec)?.to_dyn();
                Ok(node)
            }
            SPEC_DIS_SENDER_NODE_TYPE => {
                let node = DisTxNodeData::new(instance_id, cmd_rx, event_tx, &spec)?.to_dyn();
                Ok(node)
            }
            unknown_value => Err(InfraError::InvalidSpec {
                message: format!("Unknown node type '{unknown_value}' for module 'dis'"),
            }),
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DisRxNodeSpec {
        name: String,
        exercise_id: Option<u8>,
        allow_dis_versions: Option<Vec<u8>>,
    }

    #[derive(Debug)]
    pub struct DisRxNodeData {
        base: BaseNode,
        exercise_id: Option<u8>,
        allow_dis_versions: Vec<ProtocolVersion>,
        incoming: Option<Receiver<Bytes>>,
        outgoing: Sender<Pdu>,
    }

    pub struct DisRxNodeRunner {
        data: DisRxNodeData,
        statistics: DisStatistics,
    }

    #[derive(Copy, Clone, Debug, Default)]
    pub struct DisStatistics {
        base: BaseStatistics,
    }

    impl NodeData for DisRxNodeData {
        fn new(
            instance_id: InstanceId,
            cmd_rx: Receiver<Command>,
            event_tx: Sender<Event>,
            spec: &toml::Table,
        ) -> Result<Self, InfraError> {
            let node_spec: DisRxNodeSpec =
                toml::from_str(&spec.to_string()).map_err(|err| InfraError::InvalidSpec {
                    message: err.to_string(),
                })?;

            let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

            let allow_dis_versions = node_spec
                .allow_dis_versions
                .clone()
                .map(|versions| {
                    versions
                        .iter()
                        .map(|&version| ProtocolVersion::from(version))
                        .collect()
                })
                .unwrap_or(dis_rs::supported_protocol_versions());

            Ok(Self {
                base: BaseNode {
                    instance_id,
                    name: node_spec.name.clone(),
                    cmd_rx,
                    event_tx,
                },
                exercise_id: node_spec.exercise_id,
                allow_dis_versions,
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
                })
            }
        }

        fn id(&self) -> InstanceId {
            self.base.instance_id
        }

        fn name(&self) -> &str {
            self.base.name.as_str()
        }

        fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
            DisRxNodeRunner::spawn_with_data(*self)
        }
    }

    impl NodeRunner for DisRxNodeRunner {
        type Data = DisRxNodeData;

        fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
            let mut node_runner = Self {
                data,
                statistics: DisStatistics::default(),
            };

            tokio::spawn(async move { node_runner.run().await })
        }

        async fn run(&mut self) {
            loop {
                let mut aggregate_stats_interval = tokio::time::interval(Duration::from_millis(
                    DEFAULT_AGGREGATE_STATS_INTERVAL_MS,
                ));
                let mut output_stats_interval =
                    tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

                tokio::select! {
                    // receiving commands
                    Ok(cmd) = self.data.base.cmd_rx.recv() => {
                        if cmd == Command::Quit { break; }
                    }
                    // receiving from the incoming channel, parse into PDU
                    _ = async {
                        match &mut self.data.incoming {
                            Some(channel) => match channel.recv().await {
                                Ok(packet) => {
                                    let pdus = match dis_rs::parse(&packet) {
                                        Ok(vec) => { vec }
                                        Err(err) => {
                                            trace!("DIS parse error: {err}");
                                            vec![]
                                        }
                                    };
                                    pdus.into_iter()
                                        .filter(|pdu| self.data.allow_dis_versions.contains(&pdu.header.protocol_version))
                                        .filter(|pdu| self.data.exercise_id.is_none() || self.data.exercise_id.is_some_and(|exercise_id| pdu.header.exercise_id == exercise_id ))
                                        .for_each(|pdu| {
                                            let _send_result = self.data.outgoing.send(pdu);
                                            self.statistics.base.incoming_message();
                                        });
                                }
                                Err(_err) => {}
                            },
                            None => {}
                        }
                    } => { }
                    // aggregate statistics for the interval
                    _ = aggregate_stats_interval.tick() => {
                        self.statistics.base.aggregate_interval();
                    }
                    // output current state of the stats
                    _ = output_stats_interval.tick() => {
                    }
                }
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DisTxNodeSpec {
        name: String,
        buffer_size: Option<usize>,
    }

    #[derive(Debug)]
    pub struct DisTxNodeData {
        base: BaseNode,
        buffer: BytesMut,
        incoming: Option<Receiver<Pdu>>,
        outgoing: Sender<Bytes>,
    }

    pub struct DisTxNodeRunner {
        data: DisTxNodeData,
        statistics: DisStatistics,
    }

    impl NodeData for DisTxNodeData {
        fn new(
            instance_id: InstanceId,
            cmd_rx: Receiver<Command>,
            event_tx: Sender<Event>,
            spec: &toml::Table,
        ) -> Result<Self, InfraError> {
            let node_spec: DisTxNodeSpec =
                toml::from_str(&spec.to_string()).map_err(|err| InfraError::InvalidSpec {
                    message: err.to_string(),
                })?;

            let (out_tx, _out_rx) = channel(DEFAULT_NODE_CHANNEL_CAPACITY);

            let mut buffer = BytesMut::with_capacity(SERIALISE_BUFFER_CAPACITY);
            buffer.resize(SERIALISE_BUFFER_CAPACITY, 0);

            Ok(Self {
                base: BaseNode {
                    instance_id,
                    name: node_spec.name.clone(),
                    cmd_rx,
                    event_tx,
                },
                buffer,
                incoming: None,
                outgoing: out_tx,
            })
        }

        fn request_subscription(&self) -> Box<dyn Any> {
            let client = self.outgoing.subscribe();
            Box::new(client)
        }

        fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError> {
            if let Ok(receiver) = receiver.downcast::<Receiver<Pdu>>() {
                self.incoming = Some(*receiver);
                Ok(())
            } else {
                Err(InfraError::SubscribeToChannel {
                    instance_id: self.base.instance_id,
                })
            }
        }

        fn id(&self) -> InstanceId {
            self.base.instance_id
        }

        fn name(&self) -> &str {
            self.base.name.as_str()
        }

        fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
            DisTxNodeRunner::spawn_with_data(*self)
        }
    }

    impl NodeRunner for DisTxNodeRunner {
        type Data = DisTxNodeData;

        fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
            let mut node_runner = Self {
                data,
                statistics: DisStatistics::default(),
            };

            tokio::spawn(async move { node_runner.run().await })
        }

        async fn run(&mut self) {
            loop {
                let mut aggregate_stats_interval = tokio::time::interval(Duration::from_millis(
                    DEFAULT_AGGREGATE_STATS_INTERVAL_MS,
                ));
                let mut output_stats_interval =
                    tokio::time::interval(Duration::from_millis(DEFAULT_OUTPUT_STATS_INTERVAL_MS));

                tokio::select! {
                    // receiving commands
                    Ok(cmd) = self.data.base.cmd_rx.recv() => {
                        if cmd == Command::Quit { break; }
                    }
                    // receiving from the incoming channel, serialise PDU into Bytes
                    _ = async {
                        match &mut self.data.incoming {
                            Some(channel) => match channel.recv().await {
                                Ok(pdu) => {
                                    self.statistics.base.incoming_message();
                                    match pdu.serialize(&mut self.data.buffer) {
                                        Ok(bytes_written) => {
                                            let _send_result = self.data.outgoing
                                            .send(Bytes::copy_from_slice(&self.data.buffer[0..(bytes_written as usize)]))
                                            .inspect(|_bytes_send| self.statistics.base.outgoing_message() )
                                            .inspect_err(|err| {
                                                let _ = self.data.base.event_tx.send(
                                                    Event::NodeError(
                                                        InfraError::RuntimeNode {
                                                            instance_id: self.data.base.instance_id,
                                                            message: err.to_string()
                                                        }
                                                    )
                                                );}
                                            );
                                        }
                                        Err(err) => {
                                            let _ = self.data.base.event_tx.send(
                                                Event::NodeError(
                                                    InfraError::RuntimeNode {
                                                        instance_id: self.data.base.instance_id,
                                                        message: err.to_string()
                                                    }
                                                )
                                            );
                                        }
                                    }
                                }
                                Err(_err) => {}
                            },
                            None => {}
                        }
                    } => { }
                    // aggregate statistics for the interval
                    _ = aggregate_stats_interval.tick() => {
                        self.statistics.base.aggregate_interval();
                    }
                    // output current state of the stats
                    _ = output_stats_interval.tick() => {
                        // TODO
                    }
                }
            }
        }
    }
}
