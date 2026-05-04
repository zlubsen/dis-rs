pub mod errors;
mod parser;
pub mod writer;

use crate::common_records::PDUHeader;
use crate::core::errors::DisError;
use crate::core::parser::parse_multiple_pdu;
use crate::enumerations::{DISPDUType, DISProtocolFamily, DISProtocolVersion};
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
    #[must_use]
    pub fn finalize_from_parts(header: PDUHeader, body: PduBody, time_stamp: i64) -> Self {
        Self {
            header: header
                .with_pdu_type(body.body_type())
                .with_timestamp(time_stamp)
                .with_length(PDU_HEADER_LEN_BYTES + body.body_length()),
            body,
        }
    }

    #[must_use]
    pub fn pdu_length(&self) -> u16 {
        PDU_HEADER_LEN_BYTES + self.body.body_length()
    }

    #[must_use]
    pub fn protocol_family(&self) -> DISProtocolFamily {
        self.body.protocol_family()
    }
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
    #[must_use]
    pub fn record_length(&self) -> u16 {
        4 + self.body.record_length()
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

#[cfg(test)]
mod tests {
    use crate::common_records::PDUStatus;
    use crate::enumerations::{DISPDUType, DISProtocolVersion};

    #[test]
    fn test_parse_pdu_header() {
        let buffer: [u8; 16] = [
            0x08, // Protocol Version: 8
            0x08, // Compatibility Version: 8
            0x01, // Exercise ID
            0x01, // PDU Type: Entity State
            0x00, // PDU Status
            0x10, // HDR length: 16 bytes
            0x10, 0x00, // PDU length: also 16 bytes
            0x00, 0x40, 0x20, 0x46, 0x48, 0x47, 0x06,
            0x00, // Timestamp: 01-01-2026 00:00:00 GMT = 1_767_225_600_000_000
        ];
        let (_, header) = crate::common_records::parser::pdu_header(&buffer).unwrap();

        assert_eq!(header.protocol_version, DISProtocolVersion::IEEE1278_1202X);
        assert_eq!(
            header.compatibility_version,
            DISProtocolVersion::IEEE1278_1202X
        );
        assert_eq!(header.exercise_identifier, 1);
        assert_eq!(header.pdu_type, DISPDUType::EntityState);
        assert_eq!(header.pdu_status, PDUStatus::default());
        assert_eq!(header.pdu_header_length, 16);
        assert_eq!(header.pdu_length, 16);
        assert_eq!(header.timestamp, 1_767_225_600_000_000);
    }
}
