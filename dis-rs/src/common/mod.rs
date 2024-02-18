pub mod model;
pub(crate) mod parser;

pub mod acknowledge;
pub mod acknowledge_r;
pub mod action_request;
pub mod action_request_r;
pub mod action_response;
pub mod action_response_r;
pub mod attribute;
pub mod collision;
pub mod collision_elastic;
pub mod comment;
pub mod comment_r;
pub mod data;
pub mod data_r;
pub mod data_query;
pub mod data_query_r;
pub mod designator;
pub mod detonation;
pub mod electromagnetic_emission;
pub mod entity_state;
pub mod entity_state_update;
pub mod event_report;
pub mod event_report_r;
pub mod fire;
pub mod iff;
pub mod create_entity;
pub mod create_entity_r;
pub mod record_query_r;
pub mod record_r;
pub mod remove_entity;
pub mod remove_entity_r;
pub mod repair_response;
pub mod repair_complete;
pub mod resupply_cancel;
pub mod resupply_offer;
pub mod resupply_received;
pub mod service_request;
pub mod set_data;
pub mod set_data_r;
pub mod set_record_r;
pub mod start_resume;
pub mod start_resume_r;
pub mod stop_freeze;
pub mod stop_freeze_r;
pub mod other;
pub mod signal;
pub mod transmitter;
pub mod receiver;

pub mod errors;
mod writer;

use bytes::BytesMut;
use crate::common::errors::DisError;
use crate::common::model::{Pdu};
use crate::common::parser::parse_multiple_pdu;
use crate::enumerations::{PduType, ProtocolVersion};

#[allow(dead_code)]
pub enum SupportedVersion {
    V6,
    V7,
    Unsupported,
}

impl From<ProtocolVersion> for SupportedVersion {
    fn from(version: ProtocolVersion) -> Self {
        match version {
            ProtocolVersion::IEEE1278_1A1998 => SupportedVersion::V6,
            ProtocolVersion::IEEE1278_12012 => SupportedVersion::V7,
            _ => SupportedVersion::Unsupported,
        }
    }
}

/// Trait for PduBody-s to query basic information, typically used in the header
trait BodyInfo {
    fn body_length(&self) -> u16;
    fn body_type(&self) -> PduType;
}

/// Trait for PDUs to implement whether an interaction between one or two
/// entities happens. Used to generically query the originating ``EntityId`` and (optional) receiving ``EntityId`` of
/// the interaction. When a PDU has no interaction, both the originator and receiver are ``None``.
trait Interaction {
    fn originator(&self) -> Option<&model::EntityId>;
    fn receiver(&self) -> Option<&model::EntityId>;
}

/// Trait that implements writing a PduBody to a buffer
/// based on the protocol version of the PDU.
/// Returns the number of bytes written to the buffer.
pub trait SerializePdu {
    fn serialize_pdu(&self, version: SupportedVersion, buf : &mut BytesMut) -> u16;
}

/// Trait that implements writing data structures to a buffer.
/// This serialize must be independent of protocol version differences for the data structure.
/// Returns the number of bytes written to the buffer.
pub trait Serialize {
    fn serialize(&self, buf : &mut BytesMut) -> u16;
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
        .filter(|pdu| pdu.header.protocol_version == ProtocolVersion::IEEE1278_1A1998)
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
        .filter(|pdu| pdu.header.protocol_version == ProtocolVersion::IEEE1278_12012)
        .collect();
    Ok(pdus)
}