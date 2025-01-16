use crate::error::InfraError;
use crate::modules::{dis, network, util};
use crate::runtime::{Command, Event};
use serde_derive::{Deserialize, Serialize};
use std::any::Any;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;
use toml::Value;
use tracing::{error, trace};

pub type InstanceId = u64;
pub type UntypedNode = Box<dyn NodeData>;
pub type NodeConstructor = fn(
    InstanceId,
    Receiver<Command>,
    Sender<Event>,
    &str,
    &toml::Table,
) -> Result<UntypedNode, InfraError>;

pub(crate) const SPEC_NODE_ARRAY: &str = "nodes";
pub(crate) const SPEC_NODE_TYPE_FIELD: &str = "type";
pub(crate) const SPEC_CHANNEL_ARRAY: &str = "channels";
pub(crate) const SPEC_CHANNEL_FROM_FIELD: &str = "from";
pub(crate) const SPEC_CHANNEL_TO_FIELD: &str = "to";
pub(crate) const SPEC_EXTERNALS_TABLE: &str = "externals";
pub(crate) const SPEC_EXTERNALS_INCOMING_FIELD: &str = "incoming";
pub(crate) const SPEC_EXTERNALS_OUTGOING_FIELD: &str = "outgoing";

pub const DEFAULT_NODE_CHANNEL_CAPACITY: usize = 50;
pub const DEFAULT_AGGREGATE_STATS_INTERVAL_MS: u64 = 1000;
pub const DEFAULT_OUTPUT_STATS_INTERVAL_MS: u64 = 1000;

/// Trait that defines the basic operations that a Node should have when defining the node.
pub trait NodeData
where
    Self: 'static,
{
    /// Create a new `NodeData` struct from the `spec`, hooking up the
    /// required coordination channels and setting the issued `instance_id`.
    ///
    /// Returns an `InfraError::InvalidSpec` when the node cannot be constructed from the provided spec.
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<Self, InfraError>
    where
        Self: Sized;

    /// Get a subscription to a Node's outgoing channel, with the specific (data) type made opaque.
    fn request_subscription(&self) -> Box<dyn Any>;
    /// Register an opaque subscription to a Node's data, to the incoming channel for this Node (`self`).
    ///
    /// The registration of the subscription will fail when the concrete data type of the channel does not match,
    /// returning an `InfraError::SubscribeToChannel` error.
    fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError>;
    /// Obtain a Sender part of a channel that can be used to send data from outside the runtime to this node.
    fn request_external_sender(&mut self) -> Result<Box<dyn Any>, InfraError>;

    fn id(&self) -> InstanceId;
    fn name(&self) -> &str;

    /// Convert the concrete node into a dynamic, opaque Node type.
    fn to_dyn(self) -> Box<dyn NodeData>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    /// This method must convert the NodeData to the associated NodeRunner by calling
    /// the `spawn_with_data()` method on the concrete Runner type to spawn the Node on the `InfraRuntime`.
    fn spawn_into_runner(self: Box<Self>) -> Result<JoinHandle<()>, InfraError>;
}

/// Trait that defines a node at runtime. It connects the associated `NodeData` struct (as associated type `Data`) to the runtime logic.
pub trait NodeRunner {
    type Data;
    type Incoming: Clone;
    type Outgoing: Clone;

    fn id(&self) -> InstanceId;
    fn name(&self) -> &str;

    /// Spawns the actual node, given the associated `NodeData` type.
    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, InfraError>;

    /// The async method executed when spawned.
    #[allow(async_fn_in_trait)]
    async fn run(
        &mut self,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        incoming: Option<Receiver<Self::Incoming>>,
        outgoing: Sender<Self::Outgoing>,
    );

    /// Send out an event over the event channel (convenience / shorthand function)
    fn emit_event(event_tx: &Sender<Event>, event: Event) {
        if let Err(err) = event_tx.send(event) {
            // error!("Node '{}' - {}", self.base().name, err);
            // TODO make a method again to log the name and such
            error!("{}", err);
        }
    }

    /// Shorthand function to receive messages from the optional incoming channel of the node.
    #[warn(async_fn_in_trait)]
    fn receive_incoming(
        node_id: InstanceId,
        channel_opt: &mut Option<Receiver<Self::Incoming>>,
    ) -> impl std::future::Future<Output = Option<Self::Incoming>> + Send
    where
        <Self as NodeRunner>::Incoming: std::marker::Send,
    {
        async move {
            match channel_opt {
                None => None,
                Some(ref mut channel) => match channel.recv().await {
                    Ok(message) => Some(message),
                    Err(err) => {
                        error!("Node {node_id}: {err}");
                        None
                    }
                },
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseNodeSpec {
    pub(crate) name: String,
}

#[derive(Debug)]
pub struct BaseNode {
    pub instance_id: InstanceId,
    pub name: String,
    pub cmd_rx: Receiver<Command>,
    pub event_tx: Sender<Event>,
}

impl BaseNode {
    pub fn new(
        instance_id: InstanceId,
        name: String,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
    ) -> Self {
        Self {
            instance_id,
            name,
            cmd_rx,
            event_tx,
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct BaseStatistics {
    total: BaseStatisticsItems,
    running_interval: BaseStatisticsItems,
    latest_interval: BaseStatisticsItems,
}

impl BaseStatistics {
    pub fn incoming_message(&mut self) {
        self.total.messages_in += 1;
        self.running_interval.messages_in += 1;
    }

    pub fn outgoing_message(&mut self) {
        self.total.messages_out += 1;
        self.running_interval.messages_out += 1;
    }

    pub fn aggregate_interval(&mut self) {
        self.latest_interval = self.running_interval;
        self.running_interval = Default::default();
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct BaseStatisticsItems {
    messages_in: u64,
    messages_out: u64,
}

/// Returns an inventory of all builtin Node types and their constructors,
/// as a tuple consisting of the node type attribute and function pointer.
pub(crate) fn builtin_nodes() -> Vec<(&'static str, NodeConstructor)> {
    let mut items = Vec::new();
    let mod_util = util::available_nodes();
    let mod_network = network::available_nodes();
    let mod_dis = dis::available_nodes();
    items.extend(mod_util);
    items.extend(mod_network);
    items.extend(mod_dis);
    items
}

/// Convenience function to retrieve a NodeConstructor from the provided lookup,
/// given the provided key (`node_type`).
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
                message: format!("Node type is of an invalid data type ('{}')", invalid_value),
            }),
        }
    }
}

pub(crate) fn construct_nodes_from_spec(
    constructor_lookup: &Vec<(&'static str, NodeConstructor)>,
    command_tx: Sender<Command>,
    event_tx: Sender<Event>,
    contents: &toml::Table,
) -> Result<Vec<UntypedNode>, InfraError> {
    if !contents.contains_key(SPEC_NODE_ARRAY) {
        return Err(InfraError::InvalidSpec {
            message: format!(
                "A spec file must contain a non-empty list of '{}', which is missing.",
                SPEC_NODE_ARRAY
            ),
        });
    }
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
                        Ok(node) => {
                            trace!("Created node '{}' with id {} ", node.name(), node.id());
                            Ok(node)
                        }
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
            message: format!(
                "The '{}' field in the spec file is not an array.",
                SPEC_NODE_ARRAY
            ),
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
    if !spec.contains_key(SPEC_CHANNEL_ARRAY) {
        return Err(InfraError::InvalidSpec {
            message: format!(
                "A spec file must contain a non-empty array of '{}', which is missing.",
                SPEC_CHANNEL_ARRAY
            ),
        });
    }
    if let Value::Array(array) = &spec[SPEC_CHANNEL_ARRAY] {
        for channel in array {
            if let Value::Table(spec) = channel {
                register_channel_from_spec(spec, nodes)?
            }
        }
    } else {
        return Err(InfraError::InvalidSpec {
            message: format!(
                "A spec file must contain a non-empty array of '{}', which is not an array.",
                SPEC_CHANNEL_ARRAY
            ),
        });
    };
    Ok(())
}

/// Reads a 'channel' part of a spec (in TOML), checks
pub(crate) fn register_channel_from_spec(
    spec: &toml::Table,
    nodes: &mut Vec<UntypedNode>,
) -> Result<(), InfraError> {
    let from = get_channel_name_from_spec(spec, SPEC_CHANNEL_FROM_FIELD)?;
    let to = get_channel_name_from_spec(spec, SPEC_CHANNEL_TO_FIELD)?;

    let from_id = channel_name_to_instance_id(nodes, from, SPEC_CHANNEL_FROM_FIELD)?;
    let to_id = channel_name_to_instance_id(nodes, to, SPEC_CHANNEL_TO_FIELD)?;

    let from_node = nodes
        .get(from_id as usize)
        .expect("Node with id is present.");
    let sub = from_node.request_subscription();
    let to_node = nodes
        .get_mut(to_id as usize)
        .expect("Node with id is present.");
    to_node.register_subscription(sub)?;

    Ok(())
}

/// Retrieve the node name from the spec for the provided field.
fn get_channel_name_from_spec<'a>(
    spec: &'a toml::Table,
    field: &str,
) -> Result<&'a str, InfraError> {
    Ok(spec
        .get(field)
        .ok_or(InfraError::InvalidSpec {
            message: format!("Channel spec misses field '{}'.", field),
        })?
        .as_str()
        .ok_or(InfraError::InvalidSpec {
            message: format!("Channel spec field '{}' is not a string value.", field),
        })?)
}

/// Get the instance_id of the node based on the name for a given channel spec field.
fn channel_name_to_instance_id(
    nodes: &mut Vec<UntypedNode>,
    name: &str,
    field: &str,
) -> Result<InstanceId, InfraError> {
    match find_node_id_from_name(nodes, name) {
        None => { Err(InfraError::InvalidSpec {
            message:
            format!("Invalid channel spec (field '{field}'), no correct node with name '{name}' is defined.")
        })}
        Some(id) => { Ok(id) }
    }
}

/// Find the InstanceId for a node with the given name in the list of nodes.
fn find_node_id_from_name(nodes: &mut Vec<UntypedNode>, name: &str) -> Option<InstanceId> {
    nodes
        .iter()
        .find(|node| node.name() == name)
        .map(|node| node.id())
}

/// Register external incoming and outgoing channels to nodes as defined in the spec.
///
/// When the `incoming` or `outgoing` fields are not defined in the spec, no respective channel is returned.
pub(crate) fn register_external_channels(
    spec: &toml::Table,
    nodes: &mut Vec<UntypedNode>,
) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>), InfraError> {
    if !spec.contains_key(SPEC_EXTERNALS_TABLE) {
        return Ok((None, None));
    }

    if let Value::Table(externals) = &spec[SPEC_EXTERNALS_TABLE] {
        let incoming = if let Some(Value::String(node_name)) =
            externals.get(SPEC_EXTERNALS_INCOMING_FIELD)
        {
            // get the name, get the id, get the node, connect the channel
            let node_id = match find_node_id_from_name(nodes, node_name) {
                Some(id) => id,
                None => {
                    return Err(InfraError::InvalidSpec {
                        message: format!(
                            "Cannot register external input channel: no node '{node_name}' is defined."
                        ),
                    });
                }
            };

            let node = nodes
                .get_mut(node_id as usize)
                .expect("Node with id is present.");
            let incoming_tx = node.request_external_sender()?;
            Some(incoming_tx)
        } else {
            None
        };

        let outgoing = if let Some(Value::String(node_name)) =
            externals.get(SPEC_EXTERNALS_OUTGOING_FIELD)
        {
            let node_id = match find_node_id_from_name(nodes, node_name) {
                Some(id) => id,
                None => {
                    return Err(InfraError::InvalidSpec {
                        message: format!(
                            "Cannot register external output channel: no node '{node_name}' is defined."
                        ),
                    });
                }
            };
            let node = nodes
                .get_mut(node_id as usize)
                .expect("Node with id is present.");
            let outgoing = node.request_subscription();
            Some(outgoing)
        } else {
            None
        };
        Ok((incoming, outgoing))
    } else {
        Ok((None, None))
    }
}
