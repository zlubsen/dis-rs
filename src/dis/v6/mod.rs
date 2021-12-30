use crate::dis::errors::DisError;
use crate::dis::v6::model::Pdu;

pub mod model;
pub mod parser;
pub mod builder;
pub mod dr;

pub mod entity_state;

pub fn parse_pdu(input: &[u8]) -> Result<Pdu, DisError> {
    // TODO
    Err(DisError::MalformedPdu)
}