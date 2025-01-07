use crate::error::InfraError;
use crate::runtime::{Command, Event};
use std::any::Any;
use tokio::task::JoinHandle;

pub type InstanceId = u64;
pub type GenericNode = Box<dyn NodeData>;
pub type NodeConstructor = fn(
    u64,
    tokio::sync::broadcast::Receiver<Command>,
    tokio::sync::broadcast::Sender<Event>,
    &toml::Table,
) -> Result<GenericNode, InfraError>;

pub trait NodeData
where
    Self: 'static,
{
    fn request_subscription(&self) -> Box<dyn Any>;
    fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError>;

    fn id(&self) -> u64;
    fn name(&self) -> &str;

    fn to_dyn(self) -> Box<dyn NodeData>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()>;
}

pub trait NodeRunner {
    type Data;

    fn spawn_with_data(data: Self::Data) -> JoinHandle<()>;
    async fn run(&mut self);
}

#[derive(Debug)]
pub(crate) struct BaseNode {
    pub(crate) instance_id: InstanceId,
    pub(crate) name: String,
    pub(crate) cmd_rx: tokio::sync::broadcast::Receiver<Command>,
    pub(crate) event_tx: tokio::sync::broadcast::Sender<Event>,
}

impl BaseNode {
    pub fn new(
        instance_id: u64,
        name: String,
        cmd_rx: tokio::sync::broadcast::Receiver<Command>,
        event_tx: tokio::sync::broadcast::Sender<Event>,
    ) -> Self {
        Self {
            instance_id,
            name,
            cmd_rx,
            event_tx,
        }
    }
}

//// Old stuff to explore on the basic concept

// pub struct NodeOneData {
//     base: BaseNode,
//     field_one: u8,
//     incoming: Option<tokio::sync::broadcast::Receiver<u8>>,
//     outgoing: tokio::sync::broadcast::Sender<u8>,
// }
//
// pub struct NodeOneRunner {
//     data: NodeOneData,
// }
//
// impl NodeOneData {
//     pub fn new(
//         instance_id: u64,
//         name: String,
//         cmd_rx: tokio::sync::broadcast::Receiver<Command>,
//         event_tx: tokio::sync::mpsc::Sender<Event>,
//     ) -> Self {
//         let (out_tx, _out_rx) = tokio::sync::broadcast::channel(100);
//         Self {
//             base: BaseNode {
//                 instance_id,
//                 name,
//                 cmd_rx,
//                 event_tx,
//             },
//             field_one: 1,
//             incoming: None,
//             outgoing: out_tx,
//         }
//     }
// }
//
// impl NodeData for NodeOneData {
//     fn request_subscription(&self) -> Box<dyn Any> {
//         let client = self.outgoing.subscribe();
//         Box::new(client)
//     }
//
//     fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError> {
//         if let Ok(receiver) = receiver.downcast::<tokio::sync::broadcast::Receiver<u8>>() {
//             self.incoming = Some(*receiver);
//             println!("Registered incoming channel for NodeOne");
//             Ok(())
//         } else {
//             println!("Could not downcast");
//             Err(InfraError::SubscribeToChannel {
//                 instance_id: self.base.instance_id,
//             })
//         }
//     }
//
//     fn id(&self) -> u64 {
//         self.base.instance_id
//     }
//
//     fn name(&self) -> &str {
//         self.base.name.as_str()
//     }
//
//     fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
//         NodeOneRunner::spawn_with_data(*self)
//     }
// }
//
// impl NodeRunner for NodeOneRunner {
//     type Data = NodeOneData;
//
//     fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
//         let mut node_runner = Self { data };
//         tokio::spawn(async move { node_runner.run().await })
//     }
//
//     async fn run(&mut self) {
//         println!("Running NodeOne");
//
//         let mut interval = tokio::time::interval(Duration::from_secs(3));
//
//         loop {
//             tokio::select! {
//                 Ok(cmd) = self.data.base.cmd_rx.recv() => {
//                     println!("NodeOne cmd_rx");
//                     if cmd == Command::Quit { break; }
//                     ()
//                 },
//                 Some(value_in) = async {
//                     match &mut self.data.incoming {
//                         Some(channel) => channel.recv().await.ok(),
//                         None => None,
//                     }} => {
//                     println!("NodeOne received: {value_in}");
//                     let value_out = value_in;
//                     self.data.outgoing.send(value_out).expect("Error sending NodeOne output");
//                     ()
//                 },
//                 // Ok(aa) = self.socket.recv_from(&mut self.buf) => {
//                 //     println!("TestUdpNode socket.recv_from");
//                 //     ()
//                 // }
//                 _ = interval.tick() => {
//                     println!("NodeOne tick");
//                     ()
//                 }
//             }
//         }
//     }
// }
//
// pub struct NodeTwoData {
//     base: BaseNode,
//     field_two: u16,
//     incoming: Option<tokio::sync::broadcast::Receiver<u8>>,
//     outgoing: tokio::sync::broadcast::Sender<u16>,
// }
//
// pub struct NodeTwoRunner {
//     data: NodeTwoData,
// }
//
// impl NodeTwoData {
//     pub fn new(
//         instance_id: u64,
//         name: String,
//         cmd_rx: tokio::sync::broadcast::Receiver<Command>,
//         event_tx: tokio::sync::mpsc::Sender<Event>,
//     ) -> Self {
//         let (out_tx, _out_rx) = tokio::sync::broadcast::channel(100);
//         Self {
//             base: BaseNode {
//                 instance_id,
//                 name,
//                 cmd_rx,
//                 event_tx,
//             },
//             field_two: 2,
//             incoming: None,
//             outgoing: out_tx,
//         }
//     }
// }
//
// impl NodeData for NodeTwoData {
//     fn request_subscription(&self) -> Box<dyn Any> {
//         let client = self.outgoing.subscribe();
//         Box::new(client)
//     }
//
//     fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError> {
//         if let Ok(receiver) = receiver.downcast::<tokio::sync::broadcast::Receiver<u8>>() {
//             self.incoming = Some(*receiver);
//             println!("Registered incoming channel for NodeTwo");
//             Ok(())
//         } else {
//             println!("Could not downcast");
//             Err(InfraError::SubscribeToChannel {
//                 instance_id: self.base.instance_id,
//             })
//         }
//     }
//
//     fn id(&self) -> u64 {
//         self.base.instance_id
//     }
//
//     fn name(&self) -> &str {
//         self.base.name.as_str()
//     }
//
//     fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()> {
//         NodeTwoRunner::spawn_with_data(*self)
//     }
// }
//
// impl NodeRunner for NodeTwoRunner {
//     type Data = NodeTwoData;
//
//     fn spawn_with_data(data: Self::Data) -> JoinHandle<()> {
//         let mut node_runner = Self { data };
//         tokio::spawn(async move { node_runner.run().await })
//     }
//
//     async fn run(&mut self) {
//         println!("Running NodeTwo");
//
//         let mut interval = tokio::time::interval(Duration::from_secs(2));
//
//         loop {
//             tokio::select! {
//                 Ok(cmd) = self.data.base.cmd_rx.recv() => {
//                     println!("NodeTwo cmd_rx");
//                     if cmd == Command::Quit { break; }
//                     ()
//                 },
//                 Some(value_in) = async {
//                     match &mut self.data.incoming {
//                         Some(channel) => channel.recv().await.ok(),
//                         None => None,
//                     }} => {
//                     println!("NodeTwo received: {value_in}");
//                     let value_out = value_in as u16 * 2;
//                     self.data.outgoing.send(value_out).expect("NodeTwo: Error sending outgoing: {value_out}");
//                     ()
//                 },
//                 _ = interval.tick() => {
//                     println!("NodeTwo tick");
//                     ()
//                 }
//             }
//         }
//     }
// }
