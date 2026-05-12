use crate::Pdu;
use bytes::BytesMut;

#[allow(
    unused,
    reason = "Used by generated code, lints and the compiler don't see the usage."
)]
pub trait Serialize {
    fn serialize(&self, buf: &mut BytesMut) -> u16;
}

impl Serialize for Pdu {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        self.header.serialize(buf);
        self.body.serialize(buf);

        self.pdu_length()
    }
}

#[cfg(test)]
mod tests {
    use crate::common_records::BeamStatus;
    use crate::core::writer::Serialize;
    use crate::entity_info_interaction::extension_records::{EntityAppearance, EntityMarking};
    use crate::enumerations::{
        AirPlatformAppearance, Appearance, AppearanceType, BeamStabilization,
        EntityMarkingCharacterSet, ExtendedAppearanceType, ExtensionRecordTypes, MarkingType,
    };
    use crate::{ExtensionRecord, ExtensionRecordBody};
    use bytes::BytesMut;

    #[test]
    fn test_write_extension_record_entity_appearance() {
        let mut buf = BytesMut::with_capacity(16);

        let er = ExtensionRecord {
            record_type: ExtensionRecordTypes::EntityAppearance,
            record_length: 16,
            body: ExtensionRecordBody::EntityAppearance(EntityAppearance {
                appearance_type: AppearanceType::PlatformAir,
                extended_appearance_type: ExtendedAppearanceType::Other,
                appearance: Appearance::PlatformAir(AirPlatformAppearance {
                    propulsion_killed: true,
                    ..Default::default()
                }),
                extended_appearance: Default::default(),
            }),
        };

        let expected: [u8; 16] = [
            0xEE, 0x07, // record type 2030 - EntityAppearance
            0x10, 0x00, // length 16 bytes fixed length
            0x00, 0x00, // EntityAppearance extension record - padding
            0x02, // Appearance Type (2 - platformair)
            0x00, // Extended Appearance Type (0 - other)
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // fields, 2x 32 bytes
        ];

        let bytes_written = er.serialize(&mut buf);
        assert_eq!(er.record_length, bytes_written);
        assert_eq!(bytes_written, 16);

        assert_eq!(&buf[..], &expected);
    }

    #[test]
    fn test_write_extension_record_entity_marking() {
        let mut buf = BytesMut::with_capacity(16);

        let er = ExtensionRecord {
            record_type: ExtensionRecordTypes::EntityMarking,
            record_length: 16,
            body: ExtensionRecordBody::EntityMarking(EntityMarking {
                marking_type: MarkingType::CallSign,
                character_set: EntityMarkingCharacterSet::UTF8,
                marking: "Viper 01".to_string(),
            }),
        };

        let expected: [u8; 16] = [
            0xF0, 0x07, // ER 1: record type 2032 - Entity Marking
            0x10,
            0x00, // ER 1: length 16 bytes - variable length padded to align with 64 bits
            0x02, // Marking Type: CallSign
            0x04, // CharacterSet: UTF8
            0x09, // String length - 9 bytes including NUL terminator
            0x56, 0x69, 0x70, 0x65, 0x72, 0x20, 0x30, 0x31, // Text: "Viper 01"
            0x00, // String NUL terminator
                  // 0x00, // pad to multiple of 8 bytes (plus 1 byte to 16 total in this case)
        ];

        let bytes_written = er.serialize(&mut buf);
        assert_eq!(er.record_length, bytes_written);
        assert_eq!(bytes_written, 16);

        assert_eq!(&buf[..], &expected);
    }

    // #[test]
    // fn test_parse_extension_record_with_array() {
    //     let buffer: [u8; 24] = [
    //         0x2A, 0x0A, // ER 1: record type 2602 - Logical Object Relationship
    //         0x18, 0x00, // ER 1: length 24 bytes - variable length padded to 64 bits
    //         0x02, // Relationship - PeerPeer - 2
    //         0x03, // Array length - 3
    //         0x01, 0x00, 0x10, 0x00, 0x02, 0x00, // Entity ID 1 - 1:16:2
    //         0x01, 0x00, 0x10, 0x00, 0x04, 0x00, // Entity ID 2 - 1:16:4
    //         0x01, 0x00, 0x10, 0x00, 0x06,
    //         0x00, // Entity ID 3 - 1:16:6
    //         // pad to multiple of 8 bytes (0 in this case, 24 total)
    //     ];
    //
    //     let (_input, er) = crate::parser::extension_record(&buffer).unwrap();
    //
    //     assert_eq!(
    //         er.record_type,
    //         ExtensionRecordTypes::LogicalObjectRelationship
    //     );
    //     assert_eq!(er.record_length, 24);
    //     if let ExtensionRecordBody::LogicalObjectRelationship(relationship) = &er.body {
    //         assert_eq!(
    //             relationship.relationship,
    //             LogicalObjectRelationship::PeerPeer
    //         );
    //         assert_eq!(
    //             relationship.related_logical_object.first().unwrap(),
    //             &EntityIdentifier::new(1, 16, 2)
    //         );
    //         assert_eq!(
    //             relationship.related_logical_object.get(1).unwrap(),
    //             &EntityIdentifier::new(1, 16, 4)
    //         );
    //         assert_eq!(
    //             relationship.related_logical_object.get(2).unwrap(),
    //             &EntityIdentifier::new(1, 16, 6)
    //         );
    //     } else {
    //         panic!("ExtensionRecord is not of type EntityAppearance")
    //     }
    // }
    //
    // #[test]
    // fn test_parse_extension_record_with_external_discriminants() {
    //     let buffer: [u8; 56] = [
    //         0xE7, 0x07, // record type 2023 - Multiple Static Entity with Extended Appearance
    //         0x38, 0x00, // length 56 bytes - variable length padded to 64 bits
    //         0x01, 0x02, 0x99, 0x00, 0x01, 0x02, 0x03, 0x04, // entity type - 1:2:153:1:2:3:4
    //         0x02, // Appearance type 2
    //         0x00, // Extended appearance type
    //         0x01, 0x00, // Array length - 1
    //         // Basic Multiple Entity
    //         0x01, 0x00, 0x02, 0x02, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Extended Appearance
    //         0x00, 0x00, 0x00, 0x00, // Pad to 64-bit boundary
    //     ];
    //
    //     let (_input, er) = crate::parser::extension_record(&buffer).unwrap();
    //
    //     assert_eq!(er.record_type, ExtensionRecordTypes::MultiplePointObject);
    //     assert_eq!(er.record_length, 56);
    //     if let ExtensionRecordBody::MultiplePointObject(multiple) = &er.body {
    //         assert_eq!(multiple.appearance_type, AppearanceType::PlatformAir);
    //         let ent_w_extended_apps = multiple.entity_with_extended_appearance.first().unwrap();
    //         assert_eq!(ent_w_extended_apps.basic_entity.entity_number, 1);
    //         assert_eq!(ent_w_extended_apps.basic_entity.marking_number, 2);
    //         assert_eq!(
    //             ent_w_extended_apps.basic_entity.entity_status,
    //             EntityStatus::from(2)
    //         );
    //         if let Appearance::PlatformAir(air_apps) = ent_w_extended_apps.basic_entity.appearance {
    //             assert!(air_apps.propulsion_killed);
    //         } else {
    //             panic!("Appearance is not of type PlatformAir")
    //         }
    //     } else {
    //         panic!("ExtensionRecord is not of type MultiplePointObject")
    //     }
    // }
    //
    #[test]
    fn test_write_bit_record() {
        let beam_status = BeamStatus {
            deactivated: true,
            beam_stabilization: BeamStabilization::HorizonStabilizedWithGimbalLimit,
        };

        let actual = u8::from(&beam_status);
        assert_eq!(actual, 5u8); // 5 == 0000_0101
    }

    // #[test]
    // fn test_write_fixed_string() {
    //     let expected: [u8; 32] = [
    //         // "Here is some status name"
    //         0x48, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x6F, 0x6D, 0x65, 0x20, 0x73,
    //         0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, // Fixed string - 32 characters
    //     ];
    //
    //     let (_input, fixed_string) = crate::parser::fixed_string_with_length(32)(&buffer).unwrap();
    //
    //     assert_eq!(fixed_string.len(), 24);
    //     assert_eq!(&fixed_string, "Here is some status name");
    // }

    // #[test]
    // fn test_parse_variable_string_single() {
    //     let buffer: [u8; 9] = [
    //         // 0x09, // string length - parsed by fields themselves
    //         0x56, 0x69, 0x70, 0x65, 0x72, 0x20, 0x30, 0x31, // "Viper 01"
    //         0x00, // NUL terminator
    //     ];
    //
    //     let (_input, variable_string) =
    //         crate::parser::variable_string_single_with_length(9)(&buffer).unwrap();
    //
    //     assert_eq!(variable_string.len(), 8);
    //     assert_eq!(&variable_string, "Viper 01");
    // }
    //
    // #[test]
    // fn test_parse_variable_string_multiple() {
    //     let buffer: [u8; 27] = [
    //         0x56, 0x69, 0x70, 0x65, 0x72, 0x20, 0x30, 0x31, // "Viper 01"
    //         0x00, // NUL terminator
    //         0x56, 0x69, 0x70, 0x65, 0x72, 0x20, 0x30, 0x32, // "Viper 02"
    //         0x00, // NUL terminator
    //         0x56, 0x69, 0x70, 0x65, 0x72, 0x20, 0x30, 0x33, // "Viper 03"
    //         0x00, // NUL terminator
    //     ];
    //
    //     let (_input, variable_strings) =
    //         crate::parser::variable_string_multiple_with_length(27)(&buffer).unwrap();
    //
    //     assert_eq!(variable_strings.len(), 3);
    //     assert_eq!(variable_strings.first().unwrap(), "Viper 01");
    //     assert_eq!(variable_strings.get(1).unwrap(), "Viper 02");
    //     assert_eq!(variable_strings.get(2).unwrap(), "Viper 03");
    // }
}
