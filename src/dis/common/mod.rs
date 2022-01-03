use model::ProtocolVersion;
use crate::dis::errors::DisError;

pub mod model;
pub mod builder;
pub mod parser;

pub fn peek_version(input: &[u8]) -> Result<ProtocolVersion, DisError> {
    // TODO
    Err(DisError::MalformedPdu)
}