use crate::error::InfraError;
use crate::infra::{dis, network, util};
use crate::runtime::{Command, Event};
use std::any::Any;
use tokio::task::JoinHandle;
use toml::Value;
use tracing::trace;

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
pub const DEFAULT_AGGREGATE_STATS_INTERVAL_MS: u64 = 1000;
pub const DEFAULT_OUTPUT_STATS_INTERVAL_MS: u64 = 1000;

/// Trait that defines the basic operations that a Node should have when defining the node.
pub trait NodeData
where
    Self: 'static,
{
    /// Get a subscription to a Node's outgoing channel, with the specific (data) type made opaque.
    fn request_subscription(&self) -> Box<dyn Any>;
    /// Register an opaque subscription to a Node's data, to the incoming channel for this Node (`self`).
    ///
    /// The registration of the subscription will fail when the concrete data type of the channel does not match,
    /// returning an `InfraError::SubscribeToChannel` error.
    fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), InfraError>;

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
    fn spawn_into_runner(self: Box<Self>) -> JoinHandle<()>;
}

/// Trait that defines a node at runtime. It connects the associated `NodeData` struct (as associated type `Data`) to the runtime logic.
pub trait NodeRunner {
    type Data;

    /// Spawns the actual node, given the associated `NodeData` type.
    fn spawn_with_data(data: Self::Data) -> JoinHandle<()>;
    /// The async method executed when spawned.
    #[allow(async_fn_in_trait)]
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
        .get(SPEC_CHANNEL_TO_FIELD)
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
