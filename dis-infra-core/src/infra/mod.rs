use bytes::{Bytes, BytesMut};
use futures::stream::FuturesUnordered;
use tokio::select;
use tokio::sync::broadcast::{Sender, Receiver};
use crate::core::{BaseNode, Node};

pub(crate) struct UdpNode {
    base_node: BaseNode,
    buffer: BytesMut,
    pub(crate) socket: tokio::net::UdpSocket,
    pub(crate) outgoing: Sender<Bytes>,
    pub(crate) incoming: Vec<Receiver<Bytes>>,
}

impl UdpNode {
    pub fn new_broadcast_socket(instance_id: u64) -> Self {
        let (sender, _receiver) = tokio::sync::broadcast::channel(10);
        let mut buffer = BytesMut::with_capacity(32684);
        buffer.resize(32684, 0);
        Self {
            base_node: BaseNode {
                instance_id,
            },
            socket: tokio::net::UdpSocket::bind("0.0.0.0:8080"),
            buffer,
            outgoing: sender,
            incoming: vec![],
        }
    }
}

impl Node for UdpNode {
    // type Output = Bytes;
    // type Input = Bytes;
    // const NAME: &'static str = "UDP Socket";

    fn node_name() -> &'static str {
        "UDP Socket"
    }

    fn node_instance_id(&self) -> u64 {
        self.base_node.instance_id
    }

    async fn run(&mut self) {
        let incoming_futures = FuturesUnordered::from_iter(&self.incoming);
        let incoming = incoming_futures.collect::<Vec<Bytes>>().await;
        // self.incoming.get(0).unwrap().rec
        loop {
            select! {
                _ = self.socket.recv(&mut self.buffer) => {
                    ()
                }
                _ = incoming => {
                    ()
                }
                _ = self.socket.recv(&mut self.buffer) => {
                    ()
                }
            }
        }
    }

    fn subscribe(&self) -> Receiver<dyn std::any::Any> {
        self.outgoing.subscribe()
    }
}