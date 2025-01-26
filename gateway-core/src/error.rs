use crate::core::InstanceId;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum GatewayError {
    #[error("Error starting the gateway's async runtime.")]
    Initialisation,
    #[error(transparent)]
    Specification(SpecificationError),
    #[error(transparent)]
    Creation(CreationError),
    #[error(transparent)]
    Execution(ExecutionError),
}

/// All variants that can go wrong regarding the Specification (reading, parsing, schema, building)
#[derive(Clone, Debug, Error)]
pub enum SpecificationError {
    #[error("File I/O error: {0}")]
    ReadFile(String),
    #[error(transparent)]
    ParseSpecification(toml::de::Error),
    #[error("A specification must contain a non-empty array '{0}', which is missing.")]
    NoNodesSpecified(String),
    #[error("The field '{0}' is not an array.")]
    FieldNotAnArray(String),
    #[error("The field '{0}' is not a string value.")]
    FieldIsNotAString(String),
    #[error("Invalid node spec for index {0}, it is not a valid TOML table.")]
    NodeEntryIsNotATable(InstanceId),
    #[error("The node specification has no field '{0}'.")]
    NodeEntryMissingTypeField(String),
    // channels
    #[error("A specification must contain a non-empty array '{0}', which is missing.")]
    NoChannelsSpecified(String),
    #[error("Channel specification misses field '{0}'.")]
    ChannelEntryMissingField(String),
    #[error("The channel specification field '{field}' references a node with name '{name}', which is not defined.")]
    ChannelEntryUndefinedNodeName { field: String, name: String },
    #[error("Cannot register external input channel, no node named '{0}' is defined.")]
    ExternalInputChannelUndefinedNodeName(String),
    #[error("Cannot register external output channel, no node named '{0}' is defined.")]
    ExternalOutputChannelUndefinedNodeName(String),
    #[error("Unknown node type '{node_type}' for module '{module_name}'")]
    UnknownNodeTypeForModule {
        module_name: String,
        node_type: String,
    },
    #[error("{0}")]
    Module(String), // TODO determine additional fields
}

#[derive(Clone, Debug, Error)]
pub enum CreationError {
    #[error("Node {node_name} could not subscribe to channel, because it has a wrong data type. Expected '{data_type_expected}'.")]
    SubscribeToChannel {
        instance_id: InstanceId,
        node_name: String,
        data_type_expected: String,
    },
    #[error("{0}")]
    ModuleCreationError(String), // TODO determine additional fields; basically forward the error from the module, use Box<dyn Error> ?
}

#[derive(Clone, Debug, Error)]
pub enum ExecutionError {
    // TODO Create variants
    // input channel failure
    // output channel failure
    // command channel (receive) failure
    // event channel (send) failure
    NodeExecution {
        node_id: InstanceId,
        message: String,
    }, // TODO determine additional fields; basically forward the error from the module, use Box<dyn Error> ?
}

#[derive(Clone, Debug, Error)]
pub enum InfraError {
    #[error("Error starting async runtime.")]
    CannotStartRuntime,
    #[error("Specification error: {message}.")]
    InvalidSpec { message: String },
    #[error("Node {instance_id} could not subscribe to channel, wrong data type. Expected '{data_type_expected}'.")]
    SubscribeToChannel {
        instance_id: InstanceId,
        node_name: String,
        data_type_expected: String,
    },
    #[error("Node {instance_id} could not subscribe to channel, wrong data type. Expected '{data_type_expected}'.")]
    SubscribeToExternalChannel {
        instance_id: InstanceId,
        node_name: String,
        data_type_expected: String,
    },
    #[error("Could not create node {instance_id}: {message}.")]
    CreateNode {
        instance_id: InstanceId,
        message: String,
    },
    #[error("Runtime error for node {instance_id}: {message}.")]
    RuntimeNode {
        instance_id: InstanceId,
        message: String,
    },
}
