use std::any::Any;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use std::time::Duration;
use bytes::{Bytes, BytesMut};
use tokio::net::UdpSocket;
use tokio::task::JoinHandle;
use crate::runtime::{Command, Event};

pub type InstanceId = u64;

pub(crate) struct BaseNode {
    pub(crate) instance_id: InstanceId,
    pub(crate) cmd_rx: tokio::sync::broadcast::Receiver<Command>,
    pub(crate) event_tx: tokio::sync::mpsc::Sender<Event>,
}

// pub trait NodeTrait {
//     async fn init(instance_id: u64, cmd_rx: Receiver<u8>, event_tx: Sender<u8>) -> (JoinHandle<()>, impl Fn() -> u8);
//     async fn run(&mut self);
//     fn subscribe(&self) -> u8;
// }
//
// pub(crate) struct TestUdpNode {
//     base: BaseNode,
//     socket: UdpSocket,
//     buf: BytesMut,
//     incoming: Option<tokio::sync::mpsc::Receiver<u8>>,
//     outgoing: tokio::sync::broadcast::Sender<u8>,
// }
//
// impl TestUdpNode {
//     pub async fn new(instance_id: u64, cmd_rx: Receiver<Command>, event_tx: Sender<Event>) -> Self {
//         let mut buf = BytesMut::with_capacity(32768);
//         buf.resize(32768, 0);
//
//         let socket = UdpSocket::bind("0.0.0.0:3000").await.unwrap();
//
//         let (out_tx, _out_rx) = tokio::sync::broadcast::channel(100);
//         Self {
//             base: BaseNode {
//                 instance_id,
//                 cmd_rx,
//                 event_tx
//             },
//             socket,
//             buf,
//             incoming: None,
//             outgoing: out_tx,
//         }
//     }
// }
//
// impl NodeTrait for TestUdpNode {
//     async fn init(instance_id: u64, cmd_rx: Receiver<u8>, event_tx: Sender<u8>) -> (JoinHandle<()>, impl Fn() -> u8) {
//         println!("Initialising TestUdpNode");
//         let mut node = TestUdpNode::new(instance_id, cmd_rx, event_tx).await;
//
//         let callback = || node.subscribe();
//         let handle = tokio::spawn( async move { node.run().await } );
//
//         (handle, callback)
//     }
//
//     async fn run(&mut self) {
//         println!("Running TestUdpNode");
//         let mut interval = tokio::time::interval(Duration::from_secs(3));
//
//         loop {
//             tokio::select! {
//                 Ok(cmd) = self.base.cmd_rx.recv() => {
//                     println!("TestUdpNode cmd_rx");
//                     if cmd == Command::Quit { break; }
//                     ()
//                 },
//                 Ok(aa) = self.socket.recv_from(&mut self.buf) => {
//                     println!("TestUdpNode socket.recv_from");
//                     ()
//                 }
//                 _ = interval.tick() => {
//                     println!("TestUdpNode tick");
//                     ()
//                 }
//             }
//         }
//     }
//
//     fn subscribe(&self) -> u8 {
//         println!("TestUdpNode subscribe");
//         1
//     }
// }
//
// pub(crate) struct TestFilterNode {
//     base: BaseNode,
//     incoming: Option<tokio::sync::mpsc::Receiver<u8>>,
//     outgoing: tokio::sync::broadcast::Sender<u8>,
// }
//
// impl TestFilterNode {
//     pub fn new(instance_id: u64, cmd_rx: Receiver<u8>, event_tx: Sender<u8>) -> Self {
//         let (out_tx, _out_rx) = tokio::sync::broadcast::channel(100);
//         Self {
//             base: BaseNode {
//                 instance_id,
//                 cmd_rx,
//                 event_tx
//             },
//             incoming: None,
//             outgoing: out_tx,
//
//         }
//     }
// }
//
// impl NodeTrait for TestFilterNode {
//     async fn init(instance_id: u64, cmd_rx: Receiver<u8>, event_tx: Sender<u8>) -> (JoinHandle<()>, impl Fn() -> u8) {
//         println!("Initialising TestUdpNode");
//         let mut node = TestFilterNode::new(instance_id, cmd_rx, event_tx);
//
//         let callback = || node.subscribe();
//         let handle = tokio::spawn( async move { node.run().await } );
//
//         (handle, callback)
//     }
//
//     async fn run(&mut self) {
//         println!("Running TestUdpNode");
//
//         let mut interval = tokio::time::interval(Duration::from_secs(2));
//
//         loop {
//             tokio::select! {
//                 Ok(cmd) = self.base.cmd_rx.recv() => {
//                     println!("TestFilterNode cmd_rx");
//                     if cmd == 8 { break; }
//                     ()
//                 },
//
//                 _ = interval.tick() => {
//                     println!("TestFilterNode tick");
//                     ()
//                 }
//             }
//         }
//     }
//
//     fn subscribe(&self) -> u8 {
//         println!("TestFilterNode subscribe");
//         2
//     }
// }

////

pub enum InfraError {
    SubscribeToChannelError { instance_id: InstanceId },
}

pub trait NodeData {
    fn request_subscription(&self) -> Box<dyn Any>;
    fn register_subscription(&mut self, receiver: Box<dyn Any>);

    fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()>;
}

pub trait NodeRunner {
    type Data;

    fn spawn_with_data(data: Self::Data) -> JoinHandle<()>;
    async fn run(&mut self);
}

pub struct NodeOneData {
    base: BaseNode,
    field_one: u8,
    incoming: Option<tokio::sync::broadcast::Receiver<u8>>,
    outgoing: tokio::sync::broadcast::Sender<u8>,
}

pub struct NodeOneRunner {
    data: NodeOneData,
}

impl NodeOneData {
    pub fn new(instance_id: u64, cmd_rx: tokio::sync::broadcast::Receiver<Command>, event_tx: tokio::sync::mpsc::Sender<Event>) -> Self {
        let (out_tx, _out_rx) = tokio::sync::broadcast::channel(100);
        Self {
            base: BaseNode {
                instance_id,
                cmd_rx,
                event_tx,
            },
            field_one: 1,
            incoming: None,
            outgoing: out_tx,
        }
    }
}

impl NodeData for NodeOneData {
    fn request_subscription(&self) -> Box<dyn Any> {
        let client = self.outgoing.subscribe();
        Box::new(client)
    }

    fn register_subscription(&mut self, receiver: Box<dyn Any>) {
        if let Ok(receiver) = receiver.downcast::<tokio::sync::broadcast::Receiver<u8>>() {
            self.incoming = Some(*receiver);
            println!("Registered incoming channel for NodeOne");
        } else {
            println!("Could not downcast");
        }
    }

    fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
        NodeOneRunner::spawn_with_data(*self)
    }
}

impl NodeRunner for NodeOneRunner {
    type Data = NodeOneData;

    fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
        let mut node_runner = Self {
            data
        };
        tokio::spawn(async move { node_runner.run().await })
    }

    async fn run(&mut self) {
        println!("Running NodeOne");

        let mut interval = tokio::time::interval(Duration::from_secs(3));

        loop {
            tokio::select! {
                Ok(cmd) = self.data.base.cmd_rx.recv() => {
                    println!("NodeOne cmd_rx");
                    if cmd == Command::Quit { break; }
                    ()
                },
                Some(value_in) = async {
                    match &mut self.data.incoming {
                        Some(channel) => channel.recv().await.ok(),
                        None => None,
                    }} => {
                    println!("NodeOne received: {value_in}");
                    let value_out = value_in;
                    self.data.outgoing.send(value_out).expect("Error sending NodeOne output");
                    ()
                },
                // Ok(aa) = self.socket.recv_from(&mut self.buf) => {
                //     println!("TestUdpNode socket.recv_from");
                //     ()
                // }
                _ = interval.tick() => {
                    println!("NodeOne tick");
                    ()
                }
            }
        }
    }
}

pub struct NodeTwoData {
    base: BaseNode,
    field_two: u16,
    incoming: Option<tokio::sync::broadcast::Receiver<u8>>,
    outgoing: tokio::sync::broadcast::Sender<u16>,
}

pub struct NodeTwoRunner {
    data: NodeTwoData,
}

impl NodeTwoData {
    pub fn new(instance_id: u64, cmd_rx: tokio::sync::broadcast::Receiver<Command>, event_tx: tokio::sync::mpsc::Sender<Event>) -> Self {
        let (out_tx, _out_rx) = tokio::sync::broadcast::channel(100);
        Self {
            base: BaseNode {
                instance_id,
                cmd_rx,
                event_tx,
            },
            field_two: 2,
            incoming: None,
            outgoing: out_tx,
        }
    }
}

impl NodeData for NodeTwoData {
    fn request_subscription(&self) -> Box<dyn Any> {
        let client = self.outgoing.subscribe();
        Box::new(client)
    }

    fn register_subscription(&mut self, receiver: Box<dyn Any>) {
        if let Ok(receiver) = receiver.downcast::<tokio::sync::broadcast::Receiver<u8>>() {
            self.incoming = Some(*receiver);
            println!("Registered incoming channel for NodeTwo");
        } else {
            println!("Could not downcast");
        }
    }

    fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
        NodeTwoRunner::spawn_with_data(*self)
    }
}

impl NodeRunner for NodeTwoRunner {
    type Data = NodeTwoData;

    fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
        let mut node_runner = Self {
            data
        };
        tokio::spawn(async move { node_runner.run().await })
    }

    async fn run(&mut self) {
        println!("Running NodeTwo");

        let mut interval = tokio::time::interval(Duration::from_secs(2));

        loop {
            tokio::select! {
                Ok(cmd) = self.data.base.cmd_rx.recv() => {
                    println!("NodeTwo cmd_rx");
                    if cmd == Command::Quit { break; }
                    ()
                },
                Some(value_in) = async {
                    match &mut self.data.incoming {
                        Some(channel) => channel.recv().await.ok(),
                        None => None,
                    }} => {
                    println!("NodeTwo received: {value_in}");
                    let value_out = value_in as u16 * 2;
                    self.data.outgoing.send(value_out).expect("NodeTwo: Error sending outgoing: {value_out}");
                    ()
                },
                _ = interval.tick() => {
                    println!("NodeTwo tick");
                    ()
                }
            }
        }
    }
}
