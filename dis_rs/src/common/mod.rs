pub mod model;
mod builder;
mod parser;

pub mod entity_state;
pub mod fire;
pub mod other;
pub mod symbolic_names;
pub mod errors;
mod writer;

use bytes::BytesMut;
use crate::common::errors::DisError;
use crate::common::model::{Pdu, ProtocolVersion};
use crate::common::parser::parse_multiple_pdu;

#[allow(dead_code)]
pub enum SupportedVersion {
    V6,
    V7,
    UNSUPPORTED,
}

/// Trait for PDUs to implement whether an interaction between one or two
/// entities happens. Used to generically query the originating ``EntityId`` and (optional) receiving ``EntityId`` of
/// the interaction. When a PDU has no interaction, both the originator and receiver are ``None``.
trait Interaction {
    fn originator(&self) -> Option<&model::EntityId>;
    fn receiver(&self) -> Option<&model::EntityId>;
}

/// Trait that implements writing the data structure to a buffer.
/// Return the number of bytes written to the buffer.
pub trait Serialize {
    fn serialize(&self, buf : &mut BytesMut) -> usize;
}

/// Parses the contents of the input, determining the DIS version by itself.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
///
/// Assumes there will only be a single DIS version of PDUs in a buffer (packet).
pub fn parse(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    parse_multiple_pdu(input)
}

/// Parses the contents of the input as DIS version 6.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
///
/// This function will filter out any non-v6 PDUs in a buffer (packet).
pub fn parse_v6(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    let pdus = parse_multiple_pdu(input)?.into_iter()
        .filter(|pdu| pdu.header.protocol_version == ProtocolVersion::Ieee1278_1a_1998)
        .collect();
    Ok(pdus)
}

/// Parses the contents of the input as DIS version 7.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
///
/// This function will filter out any non-v7 PDUs in a buffer (packet).
pub fn parse_v7(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    let pdus = parse_multiple_pdu(input)?.into_iter()
        .filter(|pdu| pdu.header.protocol_version == ProtocolVersion::Ieee1278_1_2012)
        .collect();
    Ok(pdus)
}