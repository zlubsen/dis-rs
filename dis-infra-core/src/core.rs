use std::any::Any;
use tokio::net::UdpSocket;

pub trait Node {
    // const NAME: &'static str; // const UUID: u64;
    // type Input;
    // type Output;

    fn node_name() -> &'static str;
    fn node_instance_id(&self) -> u64;
    async fn run(&self);
    /// Call `subscribe()` to obtain a receiver for this node, outputting the data from the node
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<dyn Any>;
}

pub(crate) struct BaseNode {
    pub(crate) instance_id: u64,
}

pub trait SimpleNode {
    async fn run(&self);
}

pub(crate) struct TestUdpNode {
    base: BaseNode,
    socket: UdpSocket,
}

impl TestUdpNode {

}

impl SimpleNode for TestUdpNode {
    async fn run(&self) {
        todo!()
    }
}