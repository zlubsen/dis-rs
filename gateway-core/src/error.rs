use crate::core::InstanceId;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
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
