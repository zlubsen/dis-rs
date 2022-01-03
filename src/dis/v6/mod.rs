use crate::dis::errors::DisError;
use crate::dis::v6::model::{Pdu, PduHeader};

pub mod model;
pub mod parser;
pub mod builder;
pub mod dr;

pub mod other;
pub mod entity_state;

pub fn parse_pdu(input: &[u8]) -> Result<Pdu, DisError> {
    // TODO
    Err(DisError::MalformedPdu)
}

pub fn parse_header(input: &[u8]) -> Result<PduHeader, DisError> {
    // TODO
    Err(DisError::MalformedPdu)
}