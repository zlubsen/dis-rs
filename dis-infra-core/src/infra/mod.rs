const COMMAND_CHANNEL_CAPACITY: usize = 50;
const EVENT_CHANNEL_CAPACITY: usize = 50;
const NODE_CHANNEL_CAPACITY: usize = 50;
const SOCKET_BUFFER_CAPACITY: usize = 32_768;

pub mod network {
    use crate::core::{BaseNode, NodeData, NodeRunner};
    use crate::infra::{NODE_CHANNEL_CAPACITY, SOCKET_BUFFER_CAPACITY};
    use crate::runtime::{Command, Event};
    use bytes::{Bytes, BytesMut};
    use std::any::Any;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::time::Duration;
    use serde_derive::Deserialize;
    use tokio::net::UdpSocket;
    use tokio::select;
    use tokio::sync::broadcast::{Receiver, Sender};
    use tokio::task::JoinHandle;
    use tracing::error;

    #[derive(Debug, Deserialize)]
    pub struct UdpEndPointSpec {
        pub uri: String,
        pub interface: String,
        pub mode: Option<String>,
        pub ttl: Option<u16>,
        pub block_own_socket: Option<bool>,
    }

    pub struct UdpNodeData {
        base: BaseNode,
        buffer: BytesMut,
        mode: UdpMode,
        interface: SocketAddr,
        address: SocketAddr,
        ttl: u16,
        block_own_socket: bool,
        incoming: Option<Receiver<Bytes>>,
        outgoing: Sender<Bytes>,
    }

    pub struct UdpNodeRunner {
        data: UdpNodeData,
        socket: UdpSocket,
    }

    #[allow(clippy::enum_variant_names)]
    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    pub(crate) enum UdpMode {
        #[default]
        UniCast,
        BroadCast,
        MultiCast,
    }

    impl UdpNodeData {
        pub fn new(
            instance_id: u64,
            cmd_rx: Receiver<Command>,
            event_tx: tokio::sync::mpsc::Sender<Event>,
        ) -> Self {
            let (out_tx, _out_rx) = tokio::sync::broadcast::channel(NODE_CHANNEL_CAPACITY);

            let mut buffer = BytesMut::with_capacity(SOCKET_BUFFER_CAPACITY);
            buffer.resize(SOCKET_BUFFER_CAPACITY, 0);

            Self {
                base: BaseNode {
                    instance_id,
                    cmd_rx,
                    event_tx,
                },
                buffer,
                mode: Default::default(),
                interface: "127.0.0.1:3000".parse().unwrap(),
                address: "127.0.0.1:3001".parse().unwrap(),
                ttl: 0,
                block_own_socket: false,
                incoming: None,
                outgoing: out_tx,
            }
        }
    }

    impl NodeData for UdpNodeData {
        fn request_subscription(&self) -> Box<dyn Any> {
            let client = self.outgoing.subscribe();
            Box::new(client)
        }

        fn register_subscription(&mut self, receiver: Box<dyn Any>) {
            if let Ok(receiver) = receiver.downcast::<Receiver<Bytes>>() {
                self.incoming = Some(*receiver);
                // Ok(())
            } else {
                // Err(InfraError::SubscribeToChannelError)
            }
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
            };

            tokio::spawn(async move { node_runner.run().await })
        }

        async fn run(&mut self) {
            const ONE_SECOND: u64 = 1;
            const DEFAULT_OWN_ADDRESS: SocketAddr =
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

            let mut collect_stats_interval = tokio::time::interval(Duration::from_secs(ONE_SECOND));

            let socket = UdpSocket::bind(self.data.address)
                .await
                .unwrap();

            let default_own_socketaddr = DEFAULT_OWN_ADDRESS;
            let local_address = socket.local_addr().unwrap_or(default_own_socketaddr);

            loop {
                select! {
                    // receiving commands
                    Ok(cmd) = self.data.base.cmd_rx.recv() => {
                        if cmd == Command::Quit { break; }
                    },
                    // receiving from the incoming channel
                    Some(incoming_data) = async {
                        match &mut self.data.incoming {
                            Some(channel) => channel.recv().await.ok(),
                            None => None,
                        }} => {
                            match socket.send_to(&incoming_data, "127.0.0.1:3000").await {
                                Ok(bytes_send) => { } // TODO (stats) did sent bytes over socket
                                Err(_) => { }  // TODO (event) failed to send bytes over socket
                        }
                    },
                    // receiving from the socket
                    Ok((bytes_received, from_address)) = socket.recv_from(&mut self.data.buffer) => {
                        if self.data.block_own_host && (local_address == from_address) {
                            // Action::BlockedPacket
                        } else {
                            self.
                            Bytes::copy_from_slice(&self.data.buffer[..bytes_received];
                            // Action::ReceivedPacket(Bytes::copy_from_slice(&self.data.buffer[..bytes_received]), from_address)
                        }
                        // Err(err) => { Action::Error(err) }
                        //     if let Ok(num_receivers) = self.data.outgoing.send(self.data.buffer[..bytes_received]) {
                        //         // TODO sending bytes to next nodes succeeded
                        //     } else {
                        //         // TODO sending bytes to next nodes failed
                        //     };
                        // }
                    }
                    _ = collect_stats_interval.tick() => {
                        // TODO collect/aggregate statistics each given time interval
                        ()
                    }
                }
            }
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
                    endpoint.address, err);
                }
            }
            (true, UdpMode::BroadCast) => {
                if let Err(err) = socket.set_broadcast(true) {
                    error!(
                    "Failed to set SO_BROADCAST for endpoint address {} - {}.",
                    endpoint.interface, err);
                }
                if let Err(err) = socket.bind(&endpoint.interface.into()) {
                    error!(
                    "Failed to bind to IPv4 address {:?} - {}",
                    endpoint.address, err);
                }
                if let Err(err) = socket.set_ttl(1) {
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
