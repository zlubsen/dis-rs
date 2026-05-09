use dis_assemble_gen_3::common_records::EntityIdentifier;
use dis_assemble_gen_3::enumerations::{
    Appearance, Capabilities, Country, DeadReckoningAlgorithm, EntityKind,
    EntityMarkingCharacterSet, ExtensionRecordTypes, ForceID, LogicalObjectRelationship,
    MarkingType, PlatformDomain,
};
use dis_assemble_gen_3::{ExtensionRecordBody, PduBody};

#[test]
fn test_parse_pdu_es_no_extension_records() {
    let buffer: [u8; 72] = [
        0x08, // Protocol Version: 8
        0x08, // Compatibility Version: 8
        0x01, // Exercise ID
        0x01, // PDU Type: Entity State
        0x00, // PDU Status
        0x10, // HDR length: 16 bytes
        0x48, 0x00, // PDU length: 72 bytes
        0x00, 0x40, 0x20, 0x46, 0x48, 0x47, 0x06,
        0x00, // Timestamp: 01-01-2026 00:00:00 GMT = 1_767_225_600_000_000
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, // entity id - 1:2:3
        0x01, // force id - 1 (blue
        0x01, // DRA - 1
        0x01, 0x02, 0x99, 0x00, 0x01, 0x02, 0x03, 0x04, // entity type - 1:2:153:1:2:3:4
        0xcc, 0xbe, 0xa4, 0xde, 0x1a, 0xc4, 0x50, 0x41, // entity location x (8 bytes field)
        0x5d, 0xf0, 0x3c, 0x13, 0xfa, 0xc9, 0x50, 0x41, // entity location y (8 bytes field)
        0x78, 0x16, 0x7a, 0x9e, 0x16, 0x79, 0x35, 0x41, // entity location z (8 bytes field)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // entity orientation (12 bytes field)
        0x01, // entity status (flaming)
        0x00, // placement attributes
        0x00, 0x00, // num extension records (0)
    ];
    let parsed = dis_assemble_gen_3::parse(&buffer).unwrap();
    let pdu = parsed.first().unwrap();

    if let PduBody::EntityState(es) = &pdu.body {
        assert_eq!(es.entity_id.entity_number, 3);
        assert_eq!(es.entity_id.simulation_address.application_number, 2);
        assert_eq!(es.entity_id.simulation_address.site_number, 1);
        assert_eq!(es.force_id, ForceID::Friendly);
        assert_eq!(es.dead_reckoning_algorithm, DeadReckoningAlgorithm::from(1));
        // entity type
        assert_eq!(es.entity_type.entity_kind, EntityKind::Platform);
        assert_eq!(
            PlatformDomain::from(es.entity_type.domain),
            PlatformDomain::Air
        );
        assert_eq!(es.entity_type.country, Country::Netherlands_NLD);
        assert_eq!(es.entity_type.category, 1);
        assert_eq!(es.entity_type.subcategory, 2);
        assert_eq!(es.entity_type.specific, 3);
        assert_eq!(es.entity_type.extra, 4);

        assert_eq!(es.entity_location.x_coordinate, 4_395_115.478_805_255f64);
        assert_eq!(es.entity_location.y_coordinate, 4_401_128.300_594_416f64);
        assert_eq!(es.entity_location.z_coordinate, 1_407_254.619_050_411_5f64);
    }
}

#[test]
fn test_parse_pdu_es_with_extension_records() {
    let buffer: [u8; 104] = [
        0x08, // Protocol Version: 8
        0x08, // Compatibility Version: 8
        0x01, // Exercise ID
        0x01, // PDU Type: Entity State
        0x00, // PDU Status
        0x10, // HDR length: 16 bytes
        0x90, 0x00, // PDU length: 144 bytes
        0x00, 0x40, 0x20, 0x46, 0x48, 0x47, 0x06,
        0x00, // Timestamp: 01-01-2026 00:00:00 GMT = 1_767_225_600_000_000
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, // entity id - 1:2:3
        0x01, // force id - 1 (blue
        0x01, // DRA - 1
        0x01, 0x02, 0x99, 0x00, 0x01, 0x02, 0x03, 0x04, // entity type - 1:2:153:1:2:3:4
        0xcc, 0xbe, 0xa4, 0xde, 0x1a, 0xc4, 0x50, 0x41, // entity location x (8 bytes field)
        0x5d, 0xf0, 0x3c, 0x13, 0xfa, 0xc9, 0x50, 0x41, // entity location y (8 bytes field)
        0x78, 0x16, 0x7a, 0x9e, 0x16, 0x79, 0x35, 0x41, // entity location z (8 bytes field)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // entity orientation (12 bytes field)
        0x01, // entity status (flaming)
        0x00, // placement attributes
        0x02, 0x00, // num extension records (2)
        0xEE, 0x07, // ER 1: record type 2030 - EntityAppearance
        0x10, 0x00, // ER 1: length 16 bytes fixed length
        0x00, 0x00, // ER 1: EntityAppearance extension record - padding
        0x02, // ER 1: Appearance Type (2 - platformair)
        0x00, // ER 1: Extended Appearance Type (0 - other)
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // ER 1: fields, 2x 32 bytes
        0xEF, 0x07, // ER 2: record type 2031 - Capabilities
        0x10, 0x00, // ER 2: length 16 bytes fixed length
        0x00, 0x00, // ER 2: Capabilities extension record - padding
        0x02, // ER 2: Type Air Platform Capabilities
        0x00, // ER 2: 1 byte padding
        0x10, 0x00, 0x00, 0x00, // AdaptiveRecord 4 bytes - bit 4 = adsb
        0x00, 0x00, 0x00, 0x00, // ER 2: 4 byte padding
    ];
    let parsed = dis_assemble_gen_3::parse(&buffer).unwrap();
    let pdu = parsed.first().unwrap();

    if let PduBody::EntityState(es) = &pdu.body {
        let first = es.extension_records.first().unwrap();
        assert_eq!(first.record_type, ExtensionRecordTypes::EntityAppearance);
        assert_eq!(first.record_length, 16);
        if let ExtensionRecordBody::EntityAppearance(apps) = &first.body {
            if let Appearance::PlatformAir(air_apps) = apps.appearance {
                assert!(air_apps.propulsion_killed);
            } else {
                panic!("Appearance is not of type PlatformAir")
            }
        } else {
            panic!("ExtensionRecord is not of type EntityAppearance")
        }

        let second = es.extension_records.get(1).unwrap();
        assert_eq!(second.record_type, ExtensionRecordTypes::EntityCapabilities);
        assert_eq!(second.record_length, 16);
        if let ExtensionRecordBody::EntityCapabilities(capes) = &second.body {
            if let Capabilities::AirPlatformEntityCapabilities(air_capes) = capes.capabilities {
                assert!(air_capes.adsb);
            } else {
                panic!("Capabilities is not of type AirPlatformEntityCapabilities")
            }
        } else {
            panic!("ExtensionRecord is not of type EntityCapabilities")
        }
    }
}

#[test]
fn test_variable_extension_record_marking() {
    let buffer: [u8; 88] = [
        0x08, // Protocol Version: 8
        0x08, // Compatibility Version: 8
        0x01, // Exercise ID
        0x01, // PDU Type: Entity State
        0x00, // PDU Status
        0x10, // HDR length: 16 bytes
        0x90, 0x00, // PDU length: 144 bytes
        0x00, 0x40, 0x20, 0x46, 0x48, 0x47, 0x06,
        0x00, // Timestamp: 01-01-2026 00:00:00 GMT = 1_767_225_600_000_000
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, // entity id - 1:2:3
        0x01, // force id - 1 (blue
        0x01, // DRA - 1
        0x01, 0x02, 0x99, 0x00, 0x01, 0x02, 0x03, 0x04, // entity type - 1:2:153:1:2:3:4
        0xcc, 0xbe, 0xa4, 0xde, 0x1a, 0xc4, 0x50, 0x41, // entity location x (8 bytes field)
        0x5d, 0xf0, 0x3c, 0x13, 0xfa, 0xc9, 0x50, 0x41, // entity location y (8 bytes field)
        0x78, 0x16, 0x7a, 0x9e, 0x16, 0x79, 0x35, 0x41, // entity location z (8 bytes field)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // entity orientation (12 bytes field)
        0x01, // entity status (flaming)
        0x00, // placement attributes
        0x01, 0x00, // num extension records (1)
        0xF0, 0x07, // ER 1: record type 2032 - Entity Marking
        0x10, 0x00, // ER 1: length 16 bytes - variable length padded to align with 64 bits
        0x02, // Marking Type: CallSign
        0x04, // CharacterSet: UTF8
        0x08, // String length - 8 bytes including NUL terminator
        0x56, 0x69, 0x70, 0x65, 0x72, 0x30, 0x31, // Text: "Viper01" (note big endian?)
        0x00, // String NUL terminator
        0x00, // pad to multiple of 8 bytes (16 in this case)
    ];
    let parsed = dis_assemble_gen_3::parse(&buffer).unwrap();
    let pdu = parsed.first().unwrap();

    if let PduBody::EntityState(es) = &pdu.body {
        let first = es.extension_records.first().unwrap();
        assert_eq!(first.record_type, ExtensionRecordTypes::EntityMarking);
        assert_eq!(first.record_length, 16);
        if let ExtensionRecordBody::EntityMarking(marking) = &first.body {
            assert_eq!(marking.marking_type, MarkingType::CallSign);
            assert_eq!(marking.character_set, EntityMarkingCharacterSet::UTF8);
            assert_eq!(&marking.marking, "Viper01");
        } else {
            panic!("ExtensionRecord is not of type EntityAppearance")
        }
    }
}

#[test]
fn test_extension_record_array() {
    let buffer: [u8; 96] = [
        0x08, // Protocol Version: 8
        0x08, // Compatibility Version: 8
        0x01, // Exercise ID
        0x01, // PDU Type: Entity State
        0x00, // PDU Status
        0x10, // HDR length: 16 bytes
        0x90, 0x00, // PDU length: 144 bytes
        0x00, 0x40, 0x20, 0x46, 0x48, 0x47, 0x06,
        0x00, // Timestamp: 01-01-2026 00:00:00 GMT = 1_767_225_600_000_000
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, // entity id - 1:2:3
        0x01, // force id - 1 (blue
        0x01, // DRA - 1
        0x01, 0x02, 0x99, 0x00, 0x01, 0x02, 0x03, 0x04, // entity type - 1:2:153:1:2:3:4
        0xcc, 0xbe, 0xa4, 0xde, 0x1a, 0xc4, 0x50, 0x41, // entity location x (8 bytes field)
        0x5d, 0xf0, 0x3c, 0x13, 0xfa, 0xc9, 0x50, 0x41, // entity location y (8 bytes field)
        0x78, 0x16, 0x7a, 0x9e, 0x16, 0x79, 0x35, 0x41, // entity location z (8 bytes field)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, // entity orientation (12 bytes field)
        0x01, // entity status (flaming)
        0x00, // placement attributes
        0x01, 0x00, // num extension records (1)
        0x2A, 0x0A, // ER 1: record type 2602 - Logical Object Relationship
        0x18, 0x00, // ER 1: length 24 bytes - variable length padded to 64 bits
        0x02, // Relationship - PeerPeer - 2
        0x03, // Array length - 3
        0x01, 0x00, 0x10, 0x00, 0x02, 0x00, // Entity ID 1 - 1:16:2
        0x01, 0x00, 0x10, 0x00, 0x04, 0x00, // Entity ID 2 - 1:16:4
        0x01, 0x00, 0x10, 0x00, 0x06,
        0x00, // Entity ID 3 - 1:16:6
              // pad to multiple of 8 bytes (0 in this case)
    ];
    let parsed = dis_assemble_gen_3::parse(&buffer).unwrap();
    let pdu = parsed.first().unwrap();

    if let PduBody::EntityState(es) = &pdu.body {
        let first = es.extension_records.first().unwrap();
        assert_eq!(
            first.record_type,
            ExtensionRecordTypes::LogicalObjectRelationship
        );
        assert_eq!(first.record_length, 24);
        if let ExtensionRecordBody::LogicalObjectRelationship(relationship) = &first.body {
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
}
