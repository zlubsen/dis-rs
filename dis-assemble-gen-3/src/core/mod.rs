mod errors;
mod parser;
pub(crate) mod writer;

use crate::common_records::{PDUHeader, PDUStatus};
use crate::core::errors::DisError;
use crate::core::parser::parse_multiple_pdu;
use crate::enumerations::{DISPDUType, DISProtocolVersion};
use crate::{PduBody, PDU_HEADER_LEN_BYTES};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub enum SupportedVersion {
    V8,
    Unsupported,
}

impl From<DISProtocolVersion> for SupportedVersion {
    fn from(version: DISProtocolVersion) -> Self {
        match version {
            DISProtocolVersion::IEEE1278_1202X => SupportedVersion::V8,
            _ => SupportedVersion::Unsupported,
        }
    }
}

/// Returns a `Vec` of all `ProtocolVersion`s supported by the crate.
#[must_use]
pub fn supported_protocol_versions() -> Vec<DISProtocolVersion> {
    vec![DISProtocolVersion::IEEE1278_1202X]
}

pub trait BodyRaw {
    type Builder;

    #[must_use]
    fn builder() -> Self::Builder;

    #[must_use]
    fn into_builder(self) -> Self::Builder;

    #[must_use]
    fn into_pdu_body(self) -> crate::PduBody;

    #[must_use]
    fn body_length(&self) -> u16;

    #[must_use]
    fn body_type(&self) -> DISPDUType;
}

impl<T: BodyRaw> From<T> for crate::PduBody {
    #[inline]
    fn from(value: T) -> Self {
        value.into_pdu_body()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pdu {
    pub header: PDUHeader,
    pub body: PduBody,
}

impl Pdu {
    pub fn finalize_from_parts(header: PDUHeader, body: PduBody, time_stamp: i64) -> Self {
        Self {
            header: header
                .with_pdu_type(body.body_type())
                .with_timestamp(time_stamp)
                .with_length(body.body_length()),
            body,
        }
    }

    #[must_use]
    pub fn pdu_length(&self) -> u16 {
        PDU_HEADER_LEN_BYTES + self.body.body_length()
    }
}

impl PDUHeader {
    #[must_use]
    pub fn new(
        protocol_version: DISProtocolVersion,
        exercise_identifier: u8,
        pdu_type: DISPDUType,
    ) -> Self {
        // TODO set the right values for fields
        Self {
            protocol_version,
            compatibility_version: Default::default(),
            pdu_type,
            pdu_length: 0u16,
            pdu_status: PDUStatus::default(),
            exercise_identifier,
            pdu_header_length: 0,
            timestamp: 0,
        }
    }

    #[must_use]
    pub fn with_pdu_type(mut self, pdu_type: DISPDUType) -> Self {
        self.pdu_type = pdu_type;
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn with_timestamp(mut self, timestamp: impl Into<i64>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    #[must_use]
    pub fn with_length(mut self, body_length: u16) -> Self {
        self.pdu_length = PDU_HEADER_LEN_BYTES + body_length;
        self
    }

    // #[must_use]
    // pub fn with_pdu_status(mut self, pdu_status: PduStatus) -> Self {
    //     self.pdu_status = Some(pdu_status);
    //     self
    // }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub struct ExtensionRecord {
    pub record_type: crate::enumerations::ExtensionRecordTypes,
    pub record_length: u16,
    pub body: crate::ExtensionRecordBody,
}

impl ExtensionRecord {
    // TODO match the field value with actual data length (variable fields), when manually constructing an ER.
    pub fn record_length(&self) -> u16 {
        self.body.record_length() as u16
    }

    pub fn record_type(&self) -> crate::enumerations::ExtensionRecordTypes {
        self.record_type
    }
}

/// Parses the contents of the input, determining the DIS version by itself.
/// This function tries to parse as many PDUs as there are in the buffer,
/// expecting there are only complete PDUs present in the input.
///
/// Expects there will only be a single DIS version of PDUs in a buffer (packet).
///
/// # Errors
/// Returns a `DisError` when parsing fails
pub fn parse(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    parse_multiple_pdu(input)
}
