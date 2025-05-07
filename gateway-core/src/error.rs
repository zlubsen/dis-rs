use crate::core::InstanceId;
use std::error::Error;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error(transparent)]
    Initialisation(#[from] io::Error),
    #[error("A node of type '{0}' is already registered.")]
    Registration(String),
    #[error(transparent)]
    Specification(#[from] SpecificationError),
    #[error(transparent)]
    Creation(#[from] CreationError),
    #[error(transparent)]
    Execution(ExecutionError),
}

/// All variants that can go wrong regarding the Specification (reading, parsing, schema, building)
#[derive(Debug, Error)]
pub enum SpecificationError {
    #[error(transparent)]
    ReadFile(#[from] io::Error),
    #[error(transparent)]
    ParseSpecification(#[from] toml::de::Error),
    #[error("A specification must contain a non-empty array '{0}', which is missing.")]
    NoNodesSpecified(&'static str),
    #[error("The field '{0}' is not an array.")]
    FieldNotAnArray(&'static str),
    #[error("The field '{0}' is not a string value.")]
    FieldIsNotAString(String),
    #[error("Invalid node specification for index {0}, it is not a valid TOML table.")]
    NodeEntryIsNotATable(InstanceId),
    #[error("The node specification has no field '{0}'.")]
    NodeEntryMissingField(&'static str),
    #[error("The specification contains nodes with duplicate names '{0}'.")]
    DuplicateNodeNames(String),
    #[error("A specification must contain a non-empty array '{0}', which is missing.")]
    NoChannelsSpecified(&'static str),
    #[error("Channel specification misses field '{0}'.")]
    ChannelEntryMissingField(String),
    #[error("The channel specification field '{field}' references a node with name '{name}', which is not defined.")]
    ChannelEntryUndefinedNodeName { field: String, name: String },
    #[error("Cannot register external input channel, no node named '{0}' is defined.")]
    ExternalInputChannelUndefinedNodeName(String),
    #[error("Cannot register external output channel, no node named '{0}' is defined.")]
    ExternalOutputChannelUndefinedNodeName(String),
    #[error("Node type '{0}' is not known.")]
    UnknownNodeType(String),
    #[error("Unknown node type '{node_type}' for module '{module_name}'")]
    UnknownNodeTypeForModule {
        module_name: &'static str,
        node_type: String,
    },
    #[error("{0}")]
    Module(Box<dyn NodeError>),
}

pub trait NodeError: Error {}

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("Node {node_name} could not subscribe to channel, because it has a wrong data type. Expected '{data_type_expected}'.")]
    SubscribeToChannel {
        instance_id: InstanceId,
        node_name: String,
        data_type_expected: String,
    },
    #[error(
        "Could not create subscription for external channel to node {0}. Node does not exist."
    )]
    SubscribeToExternalChannelNodeNonexistent(String),
    #[error("Could not create subscription for external channel to node {0}. Incorrect data type for channel.")]
    SubscribeToExternalChannelWrongDataType(String),
    #[error("{0}")]
    CreateNode(Box<dyn NodeError>),
}

#[derive(Clone, Debug, Error)]
pub enum ExecutionError {
    #[error("Node {0} - Input channel receive failure.")]
    InputChannelReceive(InstanceId),
    #[error("Node {0} - Output channel send failure.")]
    OutputChannelSend(InstanceId),
    #[error("Node {0} - Command channel receive failure.")]
    CommandChannelReceive(InstanceId),
    #[error("Node {0} - Event channel send failure, no receivers available.")]
    EventChannelSend(InstanceId),
    #[error("Node {node_id} - {message}")]
    NodeExecution {
        node_id: InstanceId,
        message: String,
    },
}
