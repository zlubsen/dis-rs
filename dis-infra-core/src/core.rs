use crate::error::InfraError;
use crate::infra::{dis, network};
use crate::runtime::{Command, Event};
use std::any::Any;
use tokio::task::JoinHandle;
use toml::Value;

pub type InstanceId = u64;
pub type UntypedNode = Box<dyn NodeData>;
pub type NodeConstructor = fn(
    InstanceId,
    tokio::sync::broadcast::Receiver<Command>,
    tokio::sync::broadcast::Sender<Event>,
    &str,
    &toml::Table,
) -> Result<UntypedNode, InfraError>;

pub(crate) const SPEC_NODE_ARRAY: &str = "nodes";
pub(crate) const SPEC_NODE_TYPE_FIELD: &str = "type";
pub(crate) const SPEC_CHANNEL_ARRAY: &str = "channels";
pub(crate) const SPEC_CHANNEL_FROM_FIELD: &str = "from";
pub(crate) const SPEC_CHANNEL_TO_FIELD: &str = "to";

pub const DEFAULT_NODE_CHANNEL_CAPACITY: usize = 50;

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
pub struct BaseNode {
    pub instance_id: InstanceId,
    pub name: String,
    pub cmd_rx: tokio::sync::broadcast::Receiver<Command>,
    pub event_tx: tokio::sync::broadcast::Sender<Event>,
}

impl BaseNode {
    pub fn new(
        instance_id: InstanceId,
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

/// Returns an inventory of all builtin Node types and their constructors,
/// as a tuple consisting of the node type attribute and function pointer.
pub(crate) fn builtin_nodes() -> Vec<(&'static str, NodeConstructor)> {
    let mut items = Vec::new();
    let mod_network = network::available_nodes();
    let mod_dis = dis::available_nodes();
    items.extend(mod_network);
    items.extend(mod_dis);
    items
}

/// Convenience function to retrieve a NodeConstructor from the provided lookup,
/// given the provided key (node_type).
///
/// Return an `InfraError::InvalidSpec` when the key is not found in the lookup.
pub(crate) fn lookup_node_constructor(
    lookup: &Vec<(&'static str, NodeConstructor)>,
    key: &str,
) -> Result<NodeConstructor, InfraError> {
    lookup
        .iter()
        .find(|&&tup| tup.0 == key)
        .ok_or(InfraError::InvalidSpec {
            message: format!("Node type '{key}' is not known."),
        })
        .map(|tup| tup.1)
}

/// Check if the provided TOML Table spec contains a field named 'type' and is of data type String.
///
/// Returns the value of the 'type' field when present and valid, otherwise an `InfraError::InvalidSpec` error.
pub(crate) fn check_node_spec_type_value(spec: &toml::Table) -> Result<String, InfraError> {
    if !spec.contains_key(SPEC_NODE_TYPE_FIELD) {
        Err(InfraError::InvalidSpec {
            message: format!(
                "Node specification does not contain the '{}' field of the node.",
                SPEC_NODE_TYPE_FIELD
            ),
        })
    } else {
        match &spec[SPEC_NODE_TYPE_FIELD] {
            Value::String(value) => Ok(value.clone()),
            invalid_value => Err(InfraError::InvalidSpec {
                message: format!(
                    "Node type is of an invalid data type ('{}')",
                    invalid_value.to_string()
                ),
            }),
        }
    }
}

pub(crate) fn construct_nodes_from_spec(
    constructor_lookup: &Vec<(&'static str, NodeConstructor)>,
    command_tx: tokio::sync::broadcast::Sender<Command>,
    event_tx: tokio::sync::broadcast::Sender<Event>,
    contents: &toml::Table,
) -> Result<Vec<UntypedNode>, InfraError> {
    if let Value::Array(array) = &contents[SPEC_NODE_ARRAY] {
        let nodes: Vec<Result<UntypedNode, InfraError>> = array
            .iter()
            .enumerate()
            .map(|(id, node)| {
                if let Value::Table(spec) = node {
                    let node_type_value = check_node_spec_type_value(spec)?;

                    let factory = lookup_node_constructor(constructor_lookup, &node_type_value);
                    let factory = factory?;
                    match factory(
                        id as InstanceId,
                        command_tx.subscribe(),
                        event_tx.clone(),
                        &node_type_value,
                        spec,
                    ) {
                        Ok(node) => Ok(node),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(InfraError::InvalidSpec {
                        message: format!(
                            "Invalid node spec for index {id}, it is not a valid TOML table."
                        ),
                    })
                }
            })
            .collect();
        let nodes: Result<Vec<UntypedNode>, InfraError> = nodes.into_iter().collect();
        nodes
    } else {
        Err(InfraError::InvalidSpec {
            message: "A spec file must contain a non-empty list of 'nodes', which is missing."
                .to_string(),
        })
    }
}

/// Reads the provided spec file for channel definitions,
/// and constructs the specified channels between nodes.
///
/// Returns `InfraError::InvalidSpec` errors when the specification is incorrect.
/// Plain `Result::Ok` when no errors are encountered.
pub(crate) fn register_channels_for_nodes(
    spec: &toml::Table,
    nodes: &mut Vec<UntypedNode>,
) -> Result<(), InfraError> {
    if let Value::Array(array) = &spec[SPEC_CHANNEL_ARRAY] {
        for channel in array {
            if let Value::Table(spec) = channel {
                if let Err(err) = register_channel_from_spec(spec, nodes) {
                    return Err(err);
                }
            }
        }
    } else {
        return Err(InfraError::InvalidSpec {
            message: "A spec file must contain a non-empty list of 'channels', which is missing."
                .to_string(),
        });
    };
    Ok(())
}

/// Reads a 'channel' part of a spec (in TOML), checks
pub(crate) fn register_channel_from_spec(
    spec: &toml::Table,
    nodes: &mut Vec<UntypedNode>,
) -> Result<(), InfraError> {
    let from = spec
        .get(SPEC_CHANNEL_FROM_FIELD)
        .ok_or(InfraError::InvalidSpec {
            message: format!("Channel spec misses field '{}'.", SPEC_CHANNEL_FROM_FIELD),
        })?
        .as_str()
        .ok_or(InfraError::InvalidSpec {
            message: format!(
                "Channel spec field '{}' is not a string value.",
                SPEC_CHANNEL_FROM_FIELD
            ),
        })?;
    let to = spec
        .get("to")
        .ok_or(InfraError::InvalidSpec {
            message: format!("Channel spec misses field '{}'.", SPEC_CHANNEL_TO_FIELD),
        })?
        .as_str()
        .ok_or(InfraError::InvalidSpec {
            message: format!(
                "Channel spec field '{}' is not a string value.",
                SPEC_CHANNEL_TO_FIELD
            ),
        })?;

    let from_id = nodes
        .iter()
        .find(|node| node.name() == from)
        .ok_or(InfraError::InvalidSpec {
            message: format!(
                "Invalid channel spec, no correct ({}) node with name '{from}' is defined.",
                SPEC_CHANNEL_FROM_FIELD
            ),
        })?
        .id();
    let to_id = nodes
        .iter()
        .find(|node| node.name() == to)
        .ok_or(InfraError::InvalidSpec {
            message: format!(
                "Invalid channel spec, no correct ({}) node with name '{to}' is defined.",
                SPEC_CHANNEL_TO_FIELD
            ),
        })?
        .id();

    let from_node = nodes.get(from_id as usize).unwrap();
    let sub = from_node.request_subscription();
    let to_node = nodes.get_mut(to_id as usize).unwrap();
    to_node.register_subscription(sub)?;

    Ok(())
}

//// Old stuff to explore on the basic concept

// }
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
