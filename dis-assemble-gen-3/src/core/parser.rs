use crate::common_records::parser::pdu_header;
use crate::common_records::PDUHeader;
use crate::core::errors::DisError;
use crate::core::model::Pdu;
use crate::enumerations::DISProtocolVersion;
use crate::parser::pdu_body;
use crate::PDU_HEADER_LEN_BYTES;
use alloc::vec::Vec;
use nom::bytes::complete::take;
use nom::combinator::peek;
use nom::error::ErrorKind::Eof;
use nom::multi::many1;
use nom::number::complete::be_u8;
use nom::Err;
use nom::IResult;
use nom::Parser;

pub(crate) fn parse_multiple_pdu(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    match many1(pdu).parse(input) {
        Ok((_, pdus)) => Ok(pdus),
        Err(err) => Err(DisError::ParseError(err.to_string())),
    }
}

#[allow(dead_code)]
pub(crate) fn parse_pdu(input: &[u8]) -> Result<Pdu, DisError> {
    match pdu(input) {
        Ok((_, pdu)) => Ok(pdu),
        Err(err) => Err(DisError::ParseError(err.to_string())),
    }
}

#[allow(dead_code)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn parse_multiple_header(input: &[u8]) -> Result<Vec<PDUHeader>, DisError> {
    match many1(pdu_header_skip_body).parse(input) {
        Ok((_, headers)) => Ok(headers),
        Err(parse_error) => {
            if let Err::Error(ref error) = parse_error
                && error.code == Eof
            {
                return Err(DisError::InsufficientHeaderLength(input.len() as u16));
            }
            Err(DisError::ParseError(parse_error.to_string()))
        }
    }
}

/// Parse the input for a PDU header, and skip the rest of the pdu body in the input
#[allow(dead_code)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn parse_header(input: &[u8]) -> Result<PDUHeader, DisError> {
    match pdu_header(input) {
        Ok((input, header)) => {
            let skipped = skip_body(header.pdu_length)(input); // Discard the body
            if let Err(Err::Error(error)) = skipped {
                return if error.code == Eof {
                    Err(DisError::InsufficientPduLength(
                        header.pdu_length - u16::from(PDU_HEADER_LEN_BYTES),
                        input.len() as u16,
                    ))
                } else {
                    Err(DisError::ParseError(
                        "ParseError while parsing a pdu header and skipping body.".to_string(),
                    ))
                };
            }
            Ok(header)
        }
        Err(parse_error) => {
            if let Err::Error(ref error) = parse_error
                && error.code == Eof
            {
                return Err(DisError::InsufficientHeaderLength(input.len() as u16));
            }
            Err(DisError::ParseError(parse_error.to_string()))
        }
    }
}

fn pdu(input: &[u8]) -> IResult<&[u8], Pdu> {
    // parse the header
    let (input, header) = pdu_header(input)?;

    // parse the body based on the type
    // and produce the final pdu combined with the header
    let (input, body) = pdu_body(&header)(input)?;

    Ok((input, Pdu { header, body }))
}

#[allow(dead_code)]
fn pdu_header_skip_body(input: &[u8]) -> IResult<&[u8], PDUHeader> {
    let (input, header) = pdu_header(input)?;
    let (input, _) = skip_body(header.pdu_length)(input)?;
    Ok((input, header))
}

#[allow(dead_code)]
pub(crate) fn parse_peek_protocol_version(input: &[u8]) -> Result<DISProtocolVersion, DisError> {
    let parse_result = peek_protocol_version(input);
    match parse_result {
        Ok((_, protocol_version)) => Ok(protocol_version),
        Err(err) => Err(DisError::ParseError(err.to_string())),
    }
}

/// Function tries to peek the protocol version field of the DIS header
#[allow(dead_code)]
fn peek_protocol_version(input: &[u8]) -> IResult<&[u8], DISProtocolVersion> {
    let (input, protocol_version) = peek(be_u8).parse(input)?;
    let protocol_version = DISProtocolVersion::from(protocol_version);
    Ok((input, protocol_version))
}

/// Skip the bytes of a PDU's body, by calculating the total length minus the length of a header.
/// The function will skip zero bytes when the total length provided is less than the length of a header (12 bytes).
#[allow(dead_code)]
pub(crate) fn skip_body(total_bytes: u16) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    let bytes_to_skip = total_bytes.saturating_sub(u16::from(PDU_HEADER_LEN_BYTES));
    move |input| take(bytes_to_skip)(input)
}

#[cfg(test)]
mod tests {
    use crate::common_records::PDUStatus;
    use crate::common_records::{BeamStatus, EntityIdentifier};
    use crate::entity_info_interaction::EntityStatus;
    use crate::enumerations::{
        Appearance, AppearanceType, BeamStabilization, EntityMarkingCharacterSet,
        ExtensionRecordTypes, LogicalObjectRelationship, MarkingType,
    };
    use crate::enumerations::{DISPDUType, DISProtocolVersion};
    use crate::ExtensionRecordBody;

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

    #[test]
    fn test_parse_extension_record_entity_appearance() {
        let buffer: [u8; 16] = [
            0xEE, 0x07, // record type 2030 - EntityAppearance
            0x10, 0x00, // length 16 bytes fixed length
            0x00, 0x00, // EntityAppearance extension record - padding
            0x02, // Appearance Type (2 - platformair)
            0x00, // Extended Appearance Type (0 - other)
            0x02, 0x00, 0x00, 0x00, // appearance field
            0x00, 0x00, 0x00, 0x00, // extended appearance field
        ];

        let (_input, er) = crate::parser::extension_record(&buffer).unwrap();

        assert_eq!(er.record_type, ExtensionRecordTypes::EntityAppearance);
        assert_eq!(er.record_length, 16);
        if let ExtensionRecordBody::EntityAppearance(apps) = &er.body {
            if let Appearance::PlatformAir(air_apps) = apps.appearance {
                assert!(air_apps.propulsion_killed);
            } else {
                panic!("Appearance is not of type PlatformAir")
            }
        } else {
            panic!("ExtensionRecord is not of type EntityAppearance")
        }
    }

    #[test]
    fn test_parse_extension_record_entity_marking() {
        let buffer: [u8; 16] = [
            0xF0, 0x07, // ER 1: record type 2032 - Entity Marking
            0x10,
            0x00, // ER 1: length 16 bytes - variable length padded to align with 64 bits
            0x02, // Marking Type: CallSign
            0x04, // CharacterSet: UTF8
            0x08, // String length - 8 bytes including NUL terminator
            0x56, 0x69, 0x70, 0x65, 0x72, 0x30, 0x31, // Text: "Viper01"
            0x00, // String NUL terminator
            0x00, // pad to multiple of 8 bytes (plus 1 byte to 16 total in this case)
        ];

        let (_input, er) = crate::parser::extension_record(&buffer).unwrap();

        assert_eq!(er.record_type, ExtensionRecordTypes::EntityMarking);
        assert_eq!(er.record_length, 16);
        if let ExtensionRecordBody::EntityMarking(marking) = &er.body {
            assert_eq!(marking.marking_type, MarkingType::CallSign);
            assert_eq!(marking.character_set, EntityMarkingCharacterSet::UTF8);
            assert_eq!(&marking.marking, "Viper01");
        } else {
            panic!("ExtensionRecord is not of type EntityAppearance")
        }
    }

    #[test]
    fn test_parse_extension_record_with_array() {
        #[rustfmt::skip]
        let buffer: [u8; 24] = [
            0x2A, 0x0A, // ER 1: record type 2602 - Logical Object Relationship
            0x18, 0x00, // ER 1: length 24 bytes - variable length padded to 64 bits
            0x02, // Relationship - PeerPeer - 2
            0x03, // Array length - 3
            0x01, 0x00, 0x10, 0x00, 0x02, 0x00, // Entity ID 1 - 1:16:2
            0x01, 0x00, 0x10, 0x00, 0x04, 0x00, // Entity ID 2 - 1:16:4
            0x01, 0x00, 0x10, 0x00, 0x06, 0x00, // Entity ID 3 - 1:16:6
            // pad to multiple of 8 bytes (0 in this case, 24 total)
        ];

        let (_input, er) = crate::parser::extension_record(&buffer).unwrap();

        assert_eq!(
            er.record_type,
            ExtensionRecordTypes::LogicalObjectRelationship
        );
        assert_eq!(er.record_length, 24);
        if let ExtensionRecordBody::LogicalObjectRelationship(relationship) = &er.body {
            assert_eq!(
                relationship.relationship,
                LogicalObjectRelationship::PeerPeer
            );
            assert_eq!(
                relationship.related_logical_object.first().unwrap(),
                &EntityIdentifier::new(1, 16, 2)
            );
            assert_eq!(
                relationship.related_logical_object.get(1).unwrap(),
                &EntityIdentifier::new(1, 16, 4)
            );
            assert_eq!(
                relationship.related_logical_object.get(2).unwrap(),
                &EntityIdentifier::new(1, 16, 6)
            );
        } else {
            panic!("ExtensionRecord is not of type EntityAppearance")
        }
    }

    #[test]
    fn test_parse_extension_record_with_external_discriminants() {
        #[rustfmt::skip]
        let buffer: [u8; 56] = [
            0xE7, 0x07, // record type 2023 - Multiple Static Entity with Extended Appearance
            0x38, 0x00, // length 56 bytes - variable length padded to 64 bits
            0x01, 0x02, 0x99, 0x00, 0x01, 0x02, 0x03, 0x04, // entity type - 1:2:153:1:2:3:4
            0x02, // Appearance type 2
            0x00, // Extended appearance type
            0x01, 0x00, // Array length - 1
            // Basic Multiple Entity
            0x01, 0x00, // entity number
            0x02, // marking number
            0x02, // entity status
            0x02, 0x00, 0x00, 0x00, // appearance
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // Extended Appearance
            0x00, 0x00, 0x00, 0x00, // Pad to 64-bit boundary
        ];

        let (_input, er) = crate::parser::extension_record(&buffer).unwrap();

        assert_eq!(er.record_type, ExtensionRecordTypes::MultiplePointObject);
        assert_eq!(er.record_length, 56);
        if let ExtensionRecordBody::MultiplePointObject(multiple) = &er.body {
            assert_eq!(multiple.appearance_type, AppearanceType::PlatformAir);
            let ent_w_extended_apps = multiple.entity_with_extended_appearance.first().unwrap();
            assert_eq!(ent_w_extended_apps.basic_entity.entity_number, 1);
            assert_eq!(ent_w_extended_apps.basic_entity.marking_number, 2);
            assert_eq!(
                ent_w_extended_apps.basic_entity.entity_status,
                EntityStatus::from(2)
            );
            if let Appearance::PlatformAir(air_apps) = ent_w_extended_apps.basic_entity.appearance {
                assert!(air_apps.propulsion_killed);
            } else {
                panic!("Appearance is not of type PlatformAir")
            }
        } else {
            panic!("ExtensionRecord is not of type MultiplePointObject")
        }
    }

    #[test]
    fn test_parse_bit_record() {
        let beam_status = BeamStatus::from(5); // 0000_0101

        assert!(beam_status.deactivated);
        assert_eq!(
            beam_status.beam_stabilization,
            BeamStabilization::HorizonStabilizedWithGimbalLimit
        );
    }

    #[test]
    fn test_parse_fixed_string() {
        #[rustfmt::skip]
        let buffer: [u8; 32] = [
            // "Here is some status name"
            0x48, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20,
            0x73, 0x6F, 0x6D, 0x65, 0x20, 0x73, 0x74, 0x61,
            0x74, 0x75, 0x73, 0x20, 0x6E, 0x61, 0x6D, 0x65,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Fixed string - 32 characters/bytes
        ];

        let (_input, fixed_string) = crate::parser::fixed_string_with_length(32)(&buffer).unwrap();

        assert_eq!(fixed_string.len(), 24);
        assert_eq!(&fixed_string, "Here is some status name");
    }

    #[test]
    fn test_parse_variable_string_single() {
        #[rustfmt::skip]
        let buffer: [u8; 9] = [
            // 0x09, // string length - parsed earlier in the records
            0x56, 0x69, 0x70, 0x65, 0x72, 0x20, 0x30, 0x31, // "Viper 01"
            0x00, // NUL terminator
        ];

        let (_input, variable_string) =
            crate::parser::variable_string_single_with_length(9)(&buffer).unwrap();

        assert_eq!(variable_string.len(), 8);
        assert_eq!(&variable_string, "Viper 01");
    }

    #[test]
    fn test_parse_variable_string_multiple() {
        #[rustfmt::skip]
        let buffer: [u8; 27] = [
            0x56, 0x69, 0x70, 0x65, 0x72, 0x20, 0x30, 0x31, // "Viper 01"
            0x00, // NUL terminator
            0x56, 0x69, 0x70, 0x65, 0x72, 0x20, 0x30, 0x32, // "Viper 02"
            0x00, // NUL terminator
            0x56, 0x69, 0x70, 0x65, 0x72, 0x20, 0x30, 0x33, // "Viper 03"
            0x00, // NUL terminator
        ];

        let (_input, variable_strings) =
            crate::parser::variable_string_multiple_with_length(27)(&buffer).unwrap();

        assert_eq!(variable_strings.len(), 3);
        assert_eq!(variable_strings.first().unwrap(), "Viper 01");
        assert_eq!(variable_strings.get(1).unwrap(), "Viper 02");
        assert_eq!(variable_strings.get(2).unwrap(), "Viper 03");
    }
}
