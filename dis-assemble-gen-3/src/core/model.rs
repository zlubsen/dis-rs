use crate::common_records::PDUHeader;
use crate::core::errors::DisError;
use crate::core::parser::parse_multiple_pdu;
use crate::enumerations::{DISPDUType, DISProtocolFamily, DISProtocolVersion};
use crate::{PduBody, PDU_HEADER_LEN_BYTES};
use bytes::BytesMut;
#[cfg(feature = "serde")]
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

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
#[cfg_attr(feature = "serde", derive(SerdeSerialize, SerdeDeserialize))]
pub struct Pdu {
    pub header: PDUHeader,
    pub body: PduBody,
}

impl Pdu {
    #[must_use]
    pub fn finalize_from_parts(header: PDUHeader, body: PduBody, time_stamp: i64) -> Self {
        Self {
            header: header
                .with_pdu_type(body.body_type())
                .with_timestamp(time_stamp)
                .with_length(u16::from(PDU_HEADER_LEN_BYTES) + body.body_length()),
            body,
        }
    }

    #[must_use]
    pub fn pdu_length(&self) -> u16 {
        u16::from(PDU_HEADER_LEN_BYTES) + self.body.body_length()
    }

    #[must_use]
    pub fn protocol_family(&self) -> DISProtocolFamily {
        self.body.protocol_family()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(SerdeSerialize, SerdeDeserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub struct ExtensionRecord {
    pub record_type: crate::enumerations::ExtensionRecordTypes,
    pub record_length: u16,
    pub body: crate::ExtensionRecordBody,
}

impl ExtensionRecord {
    #[must_use]
    pub fn record_length(&self) -> u16 {
        // fields record_type and record_length are counted as part of the body
        self.body.record_length()
    }

    #[must_use]
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

pub trait Serialize {
    fn serialize(&self, buf: &mut BytesMut) -> u16;
}
