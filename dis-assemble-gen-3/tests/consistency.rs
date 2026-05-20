use bytes::BytesMut;
use dis_assemble_gen_3::acoustic::acoustic::Acoustic;
use dis_assemble_gen_3::acoustic::AcousticAttributes;
use dis_assemble_gen_3::common_records::{
    EntityIdentifier, EntityType, EulerAngles, PDUHeader, WorldCoordinates,
};
use dis_assemble_gen_3::communications::transmitter::Transmitter;
use dis_assemble_gen_3::communications::{HAVEQUICKNetID, NetID, TransmitterAttributes};
use dis_assemble_gen_3::core::model::{parse, BodyRaw, Pdu};
use dis_assemble_gen_3::core::model::{ExtensionRecord, Serialize};
use dis_assemble_gen_3::entity_info_interaction::extension_records::EntityMarking;
use dis_assemble_gen_3::entity_info_interaction::{EntityStatus, PlacementAttributes};
use dis_assemble_gen_3::enumerations::{
    AppearanceDamage, DeadReckoningAlgorithm, EntityClampingType, EntityMarkingCharacterSet,
    EventReportEventType, ExtensionRecordTypes, ForceID, LocationDefinition, MarkingType,
    NETIDRecordFrequencyTable, NETIDRecordMode, NetIDType, OriginOfTheAcousticEmitter,
    UAPropulsionPlantConfiguration, UAPropulsionPlantConfigurationConfiguration,
};
use dis_assemble_gen_3::siman::event_report::EventReport;
use dis_assemble_gen_3::warfare::entity_damage_status::EntityDamageStatus;
use dis_assemble_gen_3::ExtensionRecordBody;

const TIMESTAMP_01_01_2026: i64 = 1_767_225_600_000_000;

#[test]
fn test_consistency_entity_state() {
    use std::str::FromStr;

    let er_body = ExtensionRecordBody::EntityMarking(EntityMarking {
        marking_type: MarkingType::CallSign,
        character_set: EntityMarkingCharacterSet::UTF8,
        marking: "Viper 01".to_string(),
    });

    let header = PDUHeader::default();
    let body = dis_assemble_gen_3::entity_info_interaction::entity_state::EntityState::builder()
        .with_entity_id(EntityIdentifier::new(10, 50, 1))
        .with_entity_type(EntityType::from_str("1:2:153:50:4:4:0").unwrap())
        .with_entity_location(WorldCoordinates {
            x_coordinate: 4_395_115.478_805_255,
            y_coordinate: 4_401_128.300_594_416,
            z_coordinate: 1_407_254.619_050_411_5,
        })
        .with_entity_orientation(EulerAngles {
            psi: 0.0,
            theta: 0.0,
            phi: 0.0,
        })
        .with_force_id(ForceID::Friendly)
        .with_dead_reckoning_algorithm(DeadReckoningAlgorithm::DRM_FVWHighSpeedOrManeuveringEntity)
        .with_entity_status(EntityStatus {
            deactivated: false,
            frozen: false,
            power_plant_on: true,
            mobility_killed: false,
            damage_health: AppearanceDamage::NoDamage,
            smoke_emanating: false,
            flaming: false,
        })
        .with_placement_attributes(PlacementAttributes {
            location_definition:
                LocationDefinition::ArealObjectLocation_centerOfCentroidOrBoundingVolume,
            entity_clamping_type: EntityClampingType::ConformalClamp,
            clamp_to_ground: true,
            clamp_to_water: false,
            clamp_to_structures: false,
            clamp_to_foliage: false,
        })
        .with_extension_record(ExtensionRecord {
            record_type: ExtensionRecordTypes::EntityMarking,
            record_length: er_body.record_length(),
            body: er_body,
        })
        .build()
        .into_pdu_body();
    let pdu_in = Pdu::finalize_from_parts(header, body, TIMESTAMP_01_01_2026);

    let mut buf = BytesMut::new();

    let _bytes_written = pdu_in.serialize(&mut buf);

    let pdu_out = parse(&buf).unwrap();
    let pdu_out = pdu_out.first().unwrap();

    assert_eq!(&pdu_in, pdu_out);
}

#[test]
fn test_consistency_entity_damage_status() {
    let header = PDUHeader::default();
    let body = EntityDamageStatus::builder()
        .with_damaged_entity_id(EntityIdentifier::new(1, 2, 3))
        .build()
        .into_pdu_body();
    let pdu_in = Pdu::finalize_from_parts(header, body, TIMESTAMP_01_01_2026);

    let mut buf = BytesMut::new();

    let _bytes_written = pdu_in.serialize(&mut buf);

    let pdu_out = parse(&buf).unwrap();
    let pdu_out = pdu_out.first().unwrap();

    assert_eq!(&pdu_in, pdu_out);
}

#[test]
fn test_consistency_event_report() {
    let header = PDUHeader::default();
    let body = EventReport::builder()
        .with_originating_id(EntityIdentifier::new(1, 2, 3))
        .with_receiving_id(EntityIdentifier::new(4, 5, 6))
        .with_event_type(EventReportEventType::FireDisabled)
        .build()
        .into_pdu_body();
    let pdu_in = Pdu::finalize_from_parts(header, body, TIMESTAMP_01_01_2026);

    let mut buf = BytesMut::new();

    let _bytes_written = pdu_in.serialize(&mut buf);

    let pdu_out = parse(&buf).unwrap();
    let pdu_out = pdu_out.first().unwrap();

    assert_eq!(&pdu_in, pdu_out);
}

#[test]
fn test_consistency_acoustic() {
    let header = PDUHeader::default();
    let body = Acoustic::builder()
        .with_emitting_entity_id(EntityIdentifier::new(1, 2, 3))
        .with_attributes(AcousticAttributes {
            permanently_associated: false,
            location_provided: true,
            origin_of_the_acoustic_emitter: OriginOfTheAcousticEmitter::Underwater,
        })
        .with_passive_parameter_index(88)
        .with_propulsion_plant_configuration(UAPropulsionPlantConfiguration {
            configuration: UAPropulsionPlantConfigurationConfiguration::Battery,
            hull_mounted_masker_on: true,
        })
        .build()
        .into_pdu_body();
    let pdu_in = Pdu::finalize_from_parts(header, body, TIMESTAMP_01_01_2026);

    let mut buf = BytesMut::new();

    let _bytes_written = pdu_in.serialize(&mut buf);

    let pdu_out = parse(&buf).unwrap();
    let pdu_out = pdu_out.first().unwrap();

    assert_eq!(&pdu_in, pdu_out);
}

#[test]
fn test_consistency_transmitter() {
    let header = PDUHeader::default();
    let body = Transmitter::builder()
        .with_entity_id(EntityIdentifier::new(10, 20, 30))
        .with_attributes(TransmitterAttributes {
            lossless_propagation: true,
            video: true,
            ..TransmitterAttributes::default()
        })
        .with_frequency(1000)
        .with_power(56.78f32)
        .with_node_number(4)
        .with_net_id_type(NetIDType::HAVEQUICK)
        .with_net_id(NetID::HAVEQUICKNetID(HAVEQUICKNetID {
            net_number: 3,
            frequency_table: NETIDRecordFrequencyTable::HQIOperations,
            mode: NETIDRecordMode::AHAVEQUICKIOrHAVEQUICKIICOMBAT,
        }))
        .build()
        .into_pdu_body();
    let pdu_in = Pdu::finalize_from_parts(header, body, TIMESTAMP_01_01_2026);

    let mut buf = BytesMut::new();

    let _bytes_written = pdu_in.serialize(&mut buf);

    let pdu_out = parse(&buf).unwrap();
    let pdu_out = pdu_out.first().unwrap();

    assert_eq!(&pdu_in, pdu_out);
}

// Other
// CommunicationsNode
// Signal
// LogicalNetworkState
// ElectromagneticEmission
// Collision
// Attribute
// LogicalObjectState
// AggregateState
// TransferOwnership
// IFF
// IFFInteractive
// InformationOperationsAction
// InformationOperationsReport
// Laser
// ServiceRequest
// ResupplyOffer
// ResupplyReceived
// ResupplyCancel
// RepairComplete
// RepairResponse
// CreateEntity
// RemoveEntity
// StartResume
// StopFreeze
// Acknowledge
// ActionRequest
// ActionResponse
// DataQuery
// SetData
// Data
// Comment
// ApplicationControl
// EnvironmentalProcess
// GriddedData
// Weather
// Fire
// Detonation
// DirectedEnergyFire
