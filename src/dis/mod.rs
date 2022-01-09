use crate::dis::common::parse_peek_protocol_version;
use crate::dis::common::model::ProtocolVersion;
use crate::dis::errors::DisError;
use crate::dis::v6::parse_multiple_pdu as parse_multiple_pdu_v6;

pub mod errors;
pub mod common;
pub mod v6;
pub mod v7;

pub enum Version {
    V6,
    V7,
    UNSUPPORTED,
}

pub enum Pdu {
    V6Pdu(v6::model::Pdu),
    V7Pdu(v7::model::Pdu),
}

/// Parses the contents of the input, determining the DIS version by itself.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
///
/// Assumes there will only be a single DIS version of PDUs in a buffer (packet).
pub fn parse(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    // TODO we should check version per PDU in the buffer, to support mixed types on a single port?
    let peeked_protocol_version = parse_peek_protocol_version(input)?;
    let version = match peeked_protocol_version {
        ProtocolVersion::Ieee1278_1_1995 => { Version::V6 }
        ProtocolVersion::Ieee1278_1_2012 => { Version::V7 }
        _ => { return Err(DisError::UnsupportedProtocolVersion) }
    };
    parse_as_version(input, version)
}

/// Parses the contents of the input, based on the provided DIS version.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
///
/// Assumes there will only be a single DIS version of PDUs in a buffer (packet).
pub fn parse_as_version(input: &[u8], version: Version) -> Result<Vec<Pdu>, DisError> {
    match version {
        Version::V6 => {
            match parse_v6(input) {
                Ok(mut vec) => { Ok(vec.drain(..).map(|pdu|Pdu::V6Pdu(pdu)).collect()) }
                Err(err) => { Err(err) }
            }
        }
        Version::V7 => {
            match parse_v7(input) {
                Ok(mut vec) => { Ok(vec.drain(..).map(|pdu|Pdu::V7Pdu(pdu)).collect()) }
                Err(err) => { Err(err) }
            }
        }
        Version::UNSUPPORTED => { Err(DisError::UnsupportedProtocolVersion) }
    }
}

/// Parses the contents of the input as DIS version 6.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
///
/// Assumes there will only be a single DIS version of PDUs in a buffer (packet).
pub fn parse_v6(input: &[u8]) -> Result<Vec<v6::model::Pdu>, DisError> {
    parse_multiple_pdu_v6(input)
}

/// Parses the contents of the input as DIS version 7.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
///
/// Assumes there will only be a single DIS version of PDUs in a buffer (packet).
pub fn parse_v7(input: &[u8]) -> Result<Vec<v7::model::Pdu>, DisError> {
    todo!()
}