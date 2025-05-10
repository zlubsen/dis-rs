use crate::error::{CreationError, GatewayError, SpecificationError};
use crate::modules::{dis, network, util};
use crate::runtime::{Command, Event};
use serde_derive::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::task::JoinHandle;
use toml::Value;
use tracing::{error, trace};

pub type InstanceId = u64;
pub type UntypedNode = Box<dyn NodeData>;
/// Alias for a tuple connecting a node type specification value to the function that creates the node.
pub type NodeConstructorPointer = (&'static str, NodeConstructor);
/// Signature alias of functions that create a node from a specification.
pub type NodeConstructor = fn(
    InstanceId,
    Receiver<Command>,
    Sender<Event>,
    &str,
    &toml::Table,
) -> Result<UntypedNode, SpecificationError>;

pub(crate) const SPEC_NODE_ARRAY: &str = "nodes";
pub(crate) const SPEC_NODE_TYPE_FIELD: &str = "type";
pub(crate) const SPEC_NODE_NAME_FIELD: &str = "name";
pub(crate) const SPEC_CHANNEL_ARRAY: &str = "channels";
pub(crate) const SPEC_CHANNEL_FROM_FIELD: &str = "from";
pub(crate) const SPEC_CHANNEL_TO_FIELD: &str = "to";
// FIXME remove method or create default input/output spec definition
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
    /// Returns a `SpecificationError` when the node cannot be constructed from the provided spec.
    fn new(
        instance_id: InstanceId,
        cmd_rx: Receiver<Command>,
        event_tx: Sender<Event>,
        spec: &toml::Table,
    ) -> Result<Self, SpecificationError>
    where
        Self: Sized;

    /// Get a subscription to a Node's outgoing channel, with the specific (data) type made opaque.
    fn request_subscription(&self) -> Box<dyn Any>;
    /// Register an opaque subscription to a Node's data, to the incoming channel for this Node (`self`).
    ///
    /// The registration of the subscription will fail when the concrete data type of the channel does not match,
    /// returning an `CreationError::SubscribeToChannel` error.
    fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), CreationError>;
    /// Obtain a Sender part of a channel that can be used to send data from outside the runtime to this node.
    fn request_external_input_sender(&mut self) -> Result<Box<dyn Any>, CreationError>;
    /// Obtain a clone of the Sender part of the channel that can be used to subscribe to outgoing data from this node, to link external receivers.
    fn request_external_output_sender(&self) -> Box<dyn Any>;

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
    /// the `spawn_with_data()` method on the concrete Runner type to spawn the Node on the runtime.
    fn spawn_into_runner(self: Box<Self>) -> Result<JoinHandle<()>, CreationError>;
}

/// Macro to generate the more trivial methods, implementation wise, of the NodeData trait for a node.
///
/// It is to be provided:
/// - The type of the incoming data channel,
/// - The field of `self` that is the incoming data channel, including `self`. E.g. `self.incoming`.
/// - The field of `self` that is the outgoing data channel, including `self`. E.g. `self.outgoing`.
/// - The field of `self` that holds the node InstanceId, including `self`. E.g. `self.base.node_id`.
/// - The field of `self` that holds the node name, including `self`. E.g. `self.base.name`.
/// - The type of the `NodeRunner`, which is to be spawned with this `NodeData`. E.g. `MyNodeRunner`.
#[macro_export]
macro_rules! node_data_impl {
    ($in_data_type:ty,
    self$(.$in_chan_field:ident)*,
    self$(.$out_chan_field:ident)*,
    self$(.$node_id_field:ident)*,
    self$(.$node_name_field:ident)*,
    $runner_type:ty) => {
        fn request_subscription(&self) -> Box<dyn Any> {
            let client = self$(.$out_chan_field)*.subscribe();
            Box::new(client)
        }

        fn register_subscription(&mut self, receiver: Box<dyn Any>) -> Result<(), CreationError> {
            if let Ok(receiver) = receiver.downcast::<Receiver<$in_data_type>>() {
                self$(.$in_chan_field)* = Some(*receiver);
                Ok(())
            } else {
                Err(CreationError::SubscribeToChannel {
                    instance_id: self$(.$node_id_field)*,
                    node_name: self$(.$node_name_field)*.clone(),
                    data_type_expected: std::any::type_name::<$in_data_type>().to_string(),
                })
            }
        }

        fn request_external_input_sender(&mut self) -> Result<Box<dyn Any>, CreationError> {
            let (incoming_tx, incoming_rx) =
                channel::<$in_data_type>(DEFAULT_NODE_CHANNEL_CAPACITY);
            self.register_subscription(Box::new(incoming_rx))?;
            Ok(Box::new(incoming_tx))
        }

        fn request_external_output_sender(&self) -> Box<dyn Any> {
            let sender = self$(.$out_chan_field)*.clone();
            Box::new(sender)
        }

        fn id(&self) -> InstanceId {
            self$(.$node_id_field)*
        }

        fn name(&self) -> &str {
            self$(.$node_name_field)*.as_str()
        }

        fn spawn_into_runner(self: Box<Self>) -> Result<JoinHandle<()>, CreationError> {
            <$runner_type>::spawn_with_data(*self)
        }
    };
}

/// Trait that defines a node at runtime. It connects the associated `NodeData` struct (as associated type `Data`) to the runtime logic.
pub trait NodeRunner {
    type Data;
    type Incoming: Clone;
    type Outgoing: Clone;

    fn id(&self) -> InstanceId;
    fn name(&self) -> &str;

    /// Spawns the actual node, given the associated `NodeData` type.
    fn spawn_with_data(data: Self::Data) -> Result<JoinHandle<()>, CreationError>;

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
                        error!("Node {node_id} incoming channel: {err}");
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

#[derive(Copy, Clone, Default, Debug, Serialize)]
pub struct BaseStatistics {
    node_id: InstanceId,
    total: BaseStatisticsItems,
    #[serde(skip)]
    running_interval: BaseStatisticsItems,
    latest_interval: BaseStatisticsItems,
}

impl BaseStatistics {
    pub fn new(node_id: InstanceId) -> Self {
        Self {
            node_id,
            ..Default::default()
        }
    }
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

#[derive(Copy, Clone, Debug, Default, Serialize)]
struct BaseStatisticsItems {
    messages_in: u64,
    messages_out: u64,
}

/// Returns an inventory of all builtin Node types and their constructors,
/// as a tuple consisting of the node type attribute and function pointer.
pub(crate) fn builtin_nodes() -> Vec<NodeConstructorPointer> {
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
/// Returns a `SpecificationError` when the key is not found in the lookup.
pub(crate) fn lookup_node_constructor(
    lookup: &Vec<NodeConstructorPointer>,
    key: &str,
) -> Result<NodeConstructor, SpecificationError> {
    lookup
        .iter()
        .find(|&&tup| tup.0 == key)
        .ok_or(SpecificationError::UnknownNodeType(key.to_string()))
        .map(|tup| tup.1)
}

/// Check if the provided TOML Table spec contains a field named 'type' and is of data type String.
///
/// Returns the value of the 'type' field when present and valid, otherwise an `SpecificationError` error.
pub(crate) fn check_node_spec_type_value(
    instance_id: InstanceId,
    spec: &toml::Table,
) -> Result<String, SpecificationError> {
    check_node_spec_has_string_entry(instance_id, spec, SPEC_NODE_TYPE_FIELD)
}

/// Check if the provided TOML Table spec contains a field named 'name' and is of data type String.
///
/// Returns the value of the 'name' field when present and valid, otherwise an `SpecificationError` error.
pub(crate) fn check_node_spec_name_value(
    instance_id: InstanceId,
    spec: &toml::Table,
) -> Result<String, SpecificationError> {
    check_node_spec_has_string_entry(instance_id, spec, SPEC_NODE_NAME_FIELD)
}

/// Check if the provided TOML Table spec contains a field named by parameter `field_name` and is of data type String.
///
/// Returns the value of the 'name' field when present and valid, otherwise an `SpecificationError` error.
fn check_node_spec_has_string_entry(
    instance_id: InstanceId,
    spec: &toml::Table,
    field_name: &'static str,
) -> Result<String, SpecificationError> {
    if !spec.contains_key(field_name) {
        Err(SpecificationError::NodeEntryMissingField(field_name))
    } else {
        match &spec[field_name] {
            Value::String(value) => Ok(value.clone()),
            _invalid_value => Err(SpecificationError::NodeEntryIsNotATable(instance_id)),
        }
    }
}

pub(crate) fn check_spec_for_duplicate_node_names(
    contents: &toml::Table,
) -> Result<(), SpecificationError> {
    if !contents.contains_key(SPEC_NODE_ARRAY) {
        return Err(SpecificationError::NoNodesSpecified(SPEC_NODE_ARRAY));
    }
    if let Value::Array(array) = &contents[SPEC_NODE_ARRAY] {
        let mut set = HashSet::new();
        let nodes: Vec<String> = array
            .iter()
            .enumerate()
            .map(|(id, node)| {
                if let Value::Table(spec) = node {
                    check_node_spec_name_value(id as InstanceId, spec)
                } else {
                    Err(SpecificationError::NodeEntryIsNotATable(id as InstanceId))
                }
            })
            .filter(|name| name.is_ok())
            .map(|name| name.unwrap())
            .collect();

        for name in nodes.iter() {
            match set.insert(name) {
                true => {}
                false => {
                    return Err(SpecificationError::DuplicateNodeNames(name.to_string()));
                }
            }
        }

        Ok(())
    } else {
        Err(SpecificationError::FieldNotAnArray(SPEC_NODE_ARRAY))
    }
}

pub(crate) fn construct_nodes_from_spec(
    constructor_lookup: &Vec<(&'static str, NodeConstructor)>,
    command_tx: Sender<Command>,
    event_tx: Sender<Event>,
    contents: &toml::Table,
) -> Result<Vec<UntypedNode>, SpecificationError> {
    if !contents.contains_key(SPEC_NODE_ARRAY) {
        return Err(SpecificationError::NoNodesSpecified(SPEC_NODE_ARRAY));
    }
    if let Value::Array(array) = &contents[SPEC_NODE_ARRAY] {
        let nodes: Vec<Result<UntypedNode, SpecificationError>> = array
            .iter()
            .enumerate()
            .map(|(id, node)| {
                if let Value::Table(spec) = node {
                    let node_type_value = check_node_spec_type_value(id as InstanceId, spec)?;

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
                    Err(SpecificationError::NodeEntryIsNotATable(id as InstanceId))
                }
            })
            .collect();
        let nodes: Result<Vec<UntypedNode>, SpecificationError> = nodes.into_iter().collect();
        nodes
    } else {
        Err(SpecificationError::FieldNotAnArray(SPEC_NODE_ARRAY))
    }
}

/// Reads the provided spec file for channel definitions,
/// and constructs the specified channels between nodes.
///
/// Returns `SpecificationError` errors when the specification is incorrect.
/// Plain `Result::Ok` when no errors are encountered.
pub(crate) fn register_channels_for_nodes(
    spec: &toml::Table,
    nodes: &mut Vec<UntypedNode>,
) -> Result<(), GatewayError> {
    if !spec.contains_key(SPEC_CHANNEL_ARRAY) {
        return Err(GatewayError::from(SpecificationError::NoChannelsSpecified(
            SPEC_CHANNEL_ARRAY,
        )));
    }
    if let Value::Array(array) = &spec[SPEC_CHANNEL_ARRAY] {
        for channel in array {
            if let Value::Table(spec) = channel {
                register_channel_from_spec(spec, nodes)?
            }
        }
    } else {
        return Err(GatewayError::from(SpecificationError::FieldNotAnArray(
            SPEC_CHANNEL_ARRAY,
        )));
    };
    Ok(())
}

/// Reads a 'channel' part of a spec (in TOML), checks
pub(crate) fn register_channel_from_spec(
    spec: &toml::Table,
    nodes: &mut Vec<UntypedNode>,
) -> Result<(), GatewayError> {
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
) -> Result<&'a str, SpecificationError> {
    spec.get(field)
        .ok_or(SpecificationError::ChannelEntryMissingField(
            field.to_string(),
        ))?
        .as_str()
        .ok_or(SpecificationError::FieldIsNotAString(field.to_string()))
}

/// Get the instance_id of the node based on the name for a given channel spec field.
fn channel_name_to_instance_id(
    nodes: &mut Vec<UntypedNode>,
    name: &str,
    field: &str,
) -> Result<InstanceId, SpecificationError> {
    match find_node_id_from_name(nodes, name) {
        None => Err(SpecificationError::ChannelEntryUndefinedNodeName {
            field: field.to_string(),
            name: name.to_string(),
        }),
        Some(id) => Ok(id),
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
#[allow(clippy::type_complexity)]
pub(crate) fn register_external_channels(
    spec: &toml::Table,
    nodes: &mut Vec<UntypedNode>,
) -> Result<(Option<Box<dyn Any>>, Option<Box<dyn Any>>), GatewayError> {
    if !spec.contains_key(SPEC_EXTERNALS_TABLE) {
        return Ok((None, None));
    }

    if let Value::Table(externals) = &spec[SPEC_EXTERNALS_TABLE] {
        let incoming =
            if let Some(Value::String(node_name)) = externals.get(SPEC_EXTERNALS_INCOMING_FIELD) {
                // get the name, get the id, get the node, connect the channel
                let node_id = match find_node_id_from_name(nodes, node_name) {
                    Some(id) => id,
                    None => {
                        return Err(GatewayError::from(
                            SpecificationError::ExternalInputChannelUndefinedNodeName(
                                node_name.to_string(),
                            ),
                        ));
                    }
                };

                let node = nodes
                    .get_mut(node_id as usize)
                    .expect("Node with id is present.");
                let incoming_tx = node.request_external_input_sender()?;
                Some(incoming_tx)
            } else {
                None
            };

        let outgoing =
            if let Some(Value::String(node_name)) = externals.get(SPEC_EXTERNALS_OUTGOING_FIELD) {
                let node_id = match find_node_id_from_name(nodes, node_name) {
                    Some(id) => id,
                    None => {
                        return Err(GatewayError::from(
                            SpecificationError::ExternalOutputChannelUndefinedNodeName(
                                node_name.to_string(),
                            ),
                        ));
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

struct ReceiverFuture<'a, T> {
    receiver: &'a Vec<Receiver<T>>,
}

impl<'a, T> ReceiverFuture<'a, T> {
    pub fn new(receiver: &'a Vec<Receiver<T>>) -> Self {
        Self { receiver }
    }
}

impl<T> Future for ReceiverFuture<'_, T> {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(index) = self
            .receiver
            .iter()
            .enumerate()
            .find(|(_, recv)| !recv.is_empty())
        {
            Poll::Ready(index.0)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
