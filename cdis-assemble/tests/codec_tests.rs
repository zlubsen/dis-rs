#![allow(clippy::float_cmp)]

use bytes::BytesMut;
use cdis_assemble::codec::{CodecOptions, CodecStateResult, DecoderState, EncoderState};
use cdis_assemble::constants::EIGHT_BITS;
use cdis_assemble::entity_state::model::CdisEntityCapabilities;
use cdis_assemble::records::model::{
    CdisEntityMarking, CdisHeader, CdisProtocolVersion, LinearVelocity, Orientation,
    UnitsDekameters, WorldCoordinates,
};
use cdis_assemble::types::model::{SVINT16, SVINT24, UVINT16, UVINT32, UVINT8};
use cdis_assemble::{BitBuffer, BodyProperties, CdisBody, CdisPdu, SerializeCdisPdu};
use dis_rs::designator::model::Designator;
use dis_rs::electromagnetic_emission::model::{
    Beam, ElectromagneticEmission, EmitterSystem, FundamentalParameterData, TrackJam,
};
use dis_rs::entity_state::model::{EntityAppearance, EntityMarking, EntityState};
use dis_rs::enumerations::{
    AcknowledgeFlag, ActionId, AirPlatformAppearance, AirPlatformCapabilities, CollisionType,
    Country, DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName, DetonationResult,
    ElectromagneticEmissionBeamFunction, ElectromagneticEmissionStateUpdateIndicator, EmitterName,
    EmitterSystemFunction, EntityCapabilities, EntityKind, EntityMarkingCharacterSet, EventType,
    ExplosiveMaterialCategories, FireTypeIndicator, ForceId, HighDensityTrackJam, IffSystemMode,
    IffSystemName, IffSystemType, MunitionDescriptorFuse, MunitionDescriptorWarhead, PduType,
    PlatformDomain, ProtocolVersion, ReceiverState, RequestStatus, ResponseFlag,
    SignalEncodingClass, SignalEncodingType, SignalTdlType, StopFreezeFrozenBehavior,
    StopFreezeReason, TransmitterAntennaPatternType, TransmitterCryptoSystem,
    TransmitterInputSource, TransmitterMajorModulation, TransmitterModulationTypeSystem,
    TransmitterTransmitState, VariableRecordType,
};
use dis_rs::iff::model::{
    ChangeOptionsRecord, FundamentalOperationalData, Iff, IffLayer2, IffLayer3, IffLayer4,
    IffLayer5, InformationLayers, LayersPresenceApplicability, SystemId,
};
use dis_rs::model::{
    ClockTime, DescriptorRecord, EntityId, EntityType, EventId, FixedDatum, Location,
    MunitionDescriptor, Pdu, PduBody, PduHeader, PduStatus, TimeStamp, VariableDatum, VectorF32,
};
use dis_rs::signal::model::EncodingScheme;
use dis_rs::transmitter::model::{
    BeamAntennaPattern, CryptoKeyId, ModulationType, SpreadSpectrum, Transmitter,
};

#[test]
fn encode_dis_to_cdis_entity_state_full_mode() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();

    let dis_header = PduHeader::new_v7(7, PduType::EntityState);
    let dis_body = EntityState::builder()
        .with_entity_id(EntityId::new(7, 127, 255))
        .with_entity_type(
            EntityType::default()
                .with_domain(PlatformDomain::Air)
                .with_country(Country::Netherlands_NLD_)
                .with_kind(EntityKind::Platform),
        )
        .with_force_id(ForceId::Friendly8)
        .with_marking(EntityMarking::new("TEST", EntityMarkingCharacterSet::ASCII))
        .build()
        .into_pdu_body();
    let dis_pdu = Pdu::finalize_from_parts(dis_header, dis_body, 1000);

    let (cdis_pdu, _state_result) = CdisPdu::encode(&dis_pdu, &mut encoder_state, &codec_options);

    let mut buf: BitBuffer = BitBuffer::ZERO;
    let written_bits = cdis_pdu.serialize(&mut buf, 0);
    let written_bytes = written_bits.div_ceil(EIGHT_BITS);

    let parsed_cdis_pdus = cdis_assemble::parse(&buf.data[..written_bytes]).unwrap();
    let cdis_pdu = parsed_cdis_pdus.first().unwrap();

    if let CdisBody::EntityState(es) = &cdis_pdu.body {
        assert_eq!(
            es.entity_id,
            cdis_assemble::records::model::EntityId::new(
                UVINT16::from(7),
                UVINT16::from(127),
                UVINT16::from(255)
            )
        );
        assert_eq!(
            es.entity_type.unwrap().country,
            u16::from(Country::Netherlands_NLD_)
        );
        assert_eq!(es.entity_marking.as_ref().unwrap().marking.as_str(), "TEST");
    } else {
        panic!();
    }
}

#[test]
fn decode_cdis_to_dis_entity_state_full_mode() {
    let mut decoder_state = DecoderState::new();
    let codec_options = CodecOptions::new_full_update();

    let cdis_header = CdisHeader {
        protocol_version: CdisProtocolVersion::SISO_023_2023,
        exercise_id: UVINT8::from(7),
        pdu_type: PduType::EntityState,
        timestamp: TimeStamp::new(968),
        length: 0,
        pdu_status: PduStatus::default(),
    };
    let cdis_body = cdis_assemble::entity_state::model::EntityState {
        units: UnitsDekameters::Dekameter,
        full_update_flag: false,
        entity_id: cdis_assemble::records::model::EntityId::new(
            UVINT16::from(10),
            UVINT16::from(10),
            UVINT16::from(10),
        ),
        force_id: Some(UVINT8::from(u8::from(ForceId::Friendly))),
        entity_type: Some(cdis_assemble::records::model::EntityType::new(
            u8::from(EntityKind::Platform),
            u8::from(PlatformDomain::Air),
            u16::from(Country::from(1)),
            UVINT8::from(1),
            UVINT8::from(1),
            UVINT8::from(1),
            UVINT8::from(1),
        )),
        alternate_entity_type: None,
        entity_linear_velocity: Some(LinearVelocity::new(
            SVINT16::from(5),
            SVINT16::from(5),
            SVINT16::from(-5),
        )),
        entity_location: Some(WorldCoordinates::new(52.0, 5.0, SVINT24::from(1000))),
        entity_orientation: Some(Orientation::new(4, 3, 2)),
        entity_appearance: None,
        dr_algorithm:
            DeadReckoningAlgorithm::DRM_FPW_ConstantVelocityLowAccelerationLinearMotionEntity,
        dr_params_other: None,
        dr_params_entity_linear_acceleration: None,
        dr_params_entity_angular_velocity: None,
        entity_marking: Some(CdisEntityMarking::new("TEST".to_string())),
        capabilities: Some(CdisEntityCapabilities(UVINT32::from(0xABC0_0000))),
        variable_parameters: vec![],
    }
    .into_cdis_body();
    let cdis_pdu = CdisPdu::finalize_from_parts(cdis_header, cdis_body, None::<TimeStamp>);

    let (dis_pdu, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    let mut buf = BytesMut::with_capacity(250);
    dis_pdu.serialize(&mut buf).unwrap();

    let parsed_pdus = dis_rs::parse(&buf).unwrap();
    let parsed_pdu = parsed_pdus.first().unwrap();

    if let PduBody::EntityState(es) = &parsed_pdu.body {
        assert_eq!(parsed_pdu.header.exercise_id, 7);
        assert_eq!(
            parsed_pdu.header.protocol_version,
            ProtocolVersion::IEEE1278_12012
        );
        assert_eq!(es.entity_id, EntityId::new(10, 10, 10));
        assert_eq!(es.force_id, ForceId::Friendly);
        assert_eq!(
            es.entity_type,
            EntityType::default()
                .with_kind(EntityKind::Platform)
                .with_domain(PlatformDomain::Air)
                .with_country(Country::from(1))
                .with_category(1)
                .with_subcategory(1)
                .with_specific(1)
                .with_extra(1)
        );
        assert_eq!(es.entity_marking.marking_string.as_str(), "TEST");
    } else {
        panic!();
    }
}

#[test]
fn codec_consistency_entity_state_full_mode() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::EntityState).with_pdu_status(PduStatus::default());
    let dis_body = EntityState::builder()
        .with_entity_id(EntityId::new(7, 127, 255))
        .with_entity_type(
            EntityType::default()
                .with_domain(PlatformDomain::Air)
                .with_country(Country::Netherlands_NLD_)
                .with_kind(EntityKind::Platform),
        )
        .with_force_id(ForceId::Friendly8)
        .with_location(Location::new(0.0, 0.0, 5_000_000.0))
        .with_appearance(EntityAppearance::AirPlatform(
            AirPlatformAppearance::default(),
        ))
        .with_marking(EntityMarking::new("TEST", EntityMarkingCharacterSet::ASCII))
        .with_capabilities(EntityCapabilities::AirPlatformEntityCapabilities(
            AirPlatformCapabilities::default(),
        ))
        .build()
        .into_pdu_body();
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::EntityState(es) = dis_pdu_in.body {
        es
    } else {
        EntityState::default()
    };
    let body_out = if let PduBody::EntityState(es) = dis_pdu_out.body {
        es
    } else {
        EntityState::default()
    };

    assert_eq!(body_in.entity_id, body_out.entity_id);
    assert_eq!(body_in.entity_type, body_out.entity_type);
    assert_eq!(body_in.force_id, body_out.force_id);
    assert_eq!(body_in.entity_appearance, body_out.entity_appearance);
    assert_eq!(body_in.entity_marking, body_out.entity_marking);
    assert_eq!(body_in.entity_capabilities, body_out.entity_capabilities);
    assert_eq!(
        body_in.entity_location.x_coordinate,
        body_out.entity_location.x_coordinate.round()
    );
    assert_eq!(
        body_in.entity_location.y_coordinate,
        body_out.entity_location.y_coordinate.round()
    );
}

#[test]
fn codec_consistency_fire() {
    use dis_rs::fire::model::Fire;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::Fire).with_pdu_status(
        PduStatus::default().with_fire_type_indicator(FireTypeIndicator::Munition),
    );
    let dis_body = Fire::builder()
        .with_firing_entity_id(EntityId::new(10, 10, 10))
        .with_target_entity_id(EntityId::new(20, 20, 20))
        .with_entity_id(EntityId::new(10, 10, 500))
        .with_event_id(EventId::new(10, 10, 1))
        .with_location_in_world(Location::new(0.0, 0.0, 20000.0))
        .with_munition_descriptor(
            EntityType::default()
                .with_kind(EntityKind::Munition)
                .with_domain(PlatformDomain::Air),
            MunitionDescriptor::default()
                .with_warhead(MunitionDescriptorWarhead::Dummy)
                .with_fuse(MunitionDescriptorFuse::Dummy_8110)
                .with_quantity(1)
                .with_rate(1),
        )
        .with_range(10000.0)
        .build()
        .into_pdu_body();
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Fire(fire) = dis_pdu_in.body {
        fire
    } else {
        Fire::default()
    };
    let body_out = if let PduBody::Fire(fire) = dis_pdu_out.body {
        fire
    } else {
        Fire::default()
    };

    assert_eq!(body_in.firing_entity_id, body_out.firing_entity_id);
    assert_eq!(body_in.target_entity_id, body_out.target_entity_id);
    assert_eq!(body_in.entity_id, body_out.entity_id);
    assert_eq!(body_in.event_id, body_out.event_id);
    assert_eq!(body_in.fire_mission_index, body_out.fire_mission_index);
    assert_eq!(
        body_in.location_in_world.x_coordinate,
        body_out.location_in_world.x_coordinate.round()
    );
    assert_eq!(
        body_in.location_in_world.y_coordinate,
        body_out.location_in_world.y_coordinate.round()
    );
    assert_eq!(body_in.descriptor, body_out.descriptor);
    assert_eq!(body_in.velocity, body_out.velocity);
    assert_eq!(body_in.range, body_out.range);
}

#[test]
fn codec_consistency_detonation() {
    use dis_rs::detonation::model::Detonation;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::Detonation).with_pdu_status(PduStatus::default());
    let dis_body = Detonation::builder()
        .with_source_entity_id(EntityId::new(1, 1, 1))
        .with_target_entity_id(EntityId::new(2, 2, 1))
        .with_exploding_entity_id(EntityId::new(1, 1, 100))
        .with_event_id(EventId::new(1, 1, 1))
        .with_velocity(VectorF32::new(10.0, 10.0, 10.0))
        .with_world_location(Location::new(0.0, 0.0, 20000.0))
        .with_descriptor(DescriptorRecord::new_explosion(
            EntityType::default()
                .with_kind(EntityKind::Other)
                .with_domain(PlatformDomain::Land),
            ExplosiveMaterialCategories::Alcohol,
            200.0,
        ))
        .with_entity_location(VectorF32::new(10.0, 10.0, 0.0))
        .with_detonation_result(DetonationResult::Detonation)
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Detonation(detonation) = dis_pdu_in.body {
        detonation
    } else {
        Detonation::default()
    };
    let body_out = if let PduBody::Detonation(detonation) = dis_pdu_out.body {
        detonation
    } else {
        Detonation::default()
    };

    assert_eq!(body_in.source_entity_id, body_out.source_entity_id);
    assert_eq!(body_in.target_entity_id, body_out.target_entity_id);
    assert_eq!(body_in.exploding_entity_id, body_out.exploding_entity_id);
    assert_eq!(body_in.event_id, body_out.event_id);
    assert_eq!(body_in.velocity, body_out.velocity);
    assert_eq!(
        body_in.location_in_world_coordinates.x_coordinate,
        body_out.location_in_world_coordinates.x_coordinate.round()
    );
    assert_eq!(
        body_in.location_in_world_coordinates.y_coordinate,
        body_out.location_in_world_coordinates.y_coordinate.round()
    );
    // FIXME: explosions are not properly encoded/decoded because fields explosive_material and explosive_force are not specified in C-DIS - custom impl now, align with future standardization
    assert_eq!(body_in.descriptor, body_out.descriptor);
    assert_eq!(
        body_in.location_in_entity_coordinates,
        body_out.location_in_entity_coordinates
    );
    assert_eq!(body_in.detonation_result, body_out.detonation_result);
    assert_eq!(body_in.variable_parameters, body_out.variable_parameters);
}

#[test]
fn codec_consistency_collision() {
    use dis_rs::collision::model::Collision;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::Collision).with_pdu_status(PduStatus::default());
    let dis_body = Collision::builder()
        .with_issuing_entity_id(EntityId::new(1, 1, 1))
        .with_colliding_entity_id(EntityId::new(2, 2, 1))
        .with_event_id(EventId::new(1, 1, 1))
        .with_collision_type(CollisionType::Inelastic)
        .with_location(VectorF32::new(1.0, 1.0, 1.0))
        .with_velocity(VectorF32::new(10.0, 0.0, 0.0))
        .with_mass(1000.0)
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Collision(collision) = dis_pdu_in.body {
        collision
    } else {
        Collision::default()
    };
    let body_out = if let PduBody::Collision(collision) = dis_pdu_out.body {
        collision
    } else {
        Collision::default()
    };

    assert_eq!(body_in.issuing_entity_id, body_out.issuing_entity_id);
    assert_eq!(body_in.colliding_entity_id, body_out.colliding_entity_id);
    assert_eq!(body_in.event_id, body_out.event_id);
    assert_eq!(body_in.collision_type, body_out.collision_type);
    assert_eq!(body_in.velocity, body_out.velocity);
    assert_eq!(body_in.mass, body_out.mass);
    assert_eq!(body_in.location, body_out.location);
}

#[test]
fn codec_consistency_create_entity() {
    use dis_rs::create_entity::model::CreateEntity;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::CreateEntity).with_pdu_status(PduStatus::default());
    let dis_body = CreateEntity::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_request_id(1)
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::CreateEntity(body) = dis_pdu_in.body {
        body
    } else {
        CreateEntity::default()
    };
    let body_out = if let PduBody::CreateEntity(body) = dis_pdu_out.body {
        body
    } else {
        CreateEntity::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.request_id, body_out.request_id);
}

#[test]
fn codec_consistency_remove_entity() {
    use dis_rs::remove_entity::model::RemoveEntity;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::RemoveEntity).with_pdu_status(PduStatus::default());
    let dis_body = RemoveEntity::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_request_id(1)
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::RemoveEntity(body) = dis_pdu_in.body {
        body
    } else {
        RemoveEntity::default()
    };
    let body_out = if let PduBody::RemoveEntity(body) = dis_pdu_out.body {
        body
    } else {
        RemoveEntity::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.request_id, body_out.request_id);
}

#[test]
fn codec_consistency_start_resume() {
    use dis_rs::start_resume::model::StartResume;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::RemoveEntity).with_pdu_status(PduStatus::default());
    let dis_body = StartResume::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_real_world_time(ClockTime::new(10, 30))
        .with_simulation_time(ClockTime::new(0, 14))
        .with_request_id(1)
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::StartResume(body) = dis_pdu_in.body {
        body
    } else {
        StartResume::default()
    };
    let body_out = if let PduBody::StartResume(body) = dis_pdu_out.body {
        body
    } else {
        StartResume::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.real_world_time, body_out.real_world_time);
    assert_eq!(body_in.simulation_time, body_out.simulation_time);
    assert_eq!(body_in.request_id, body_out.request_id);
}

#[test]
fn codec_consistency_stop_freeze() {
    use dis_rs::stop_freeze::model::StopFreeze;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::RemoveEntity).with_pdu_status(PduStatus::default());
    let dis_body = StopFreeze::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_real_world_time(ClockTime::new(10, 30))
        .with_reason(StopFreezeReason::Termination)
        .with_frozen_behavior(StopFreezeFrozenBehavior {
            run_simulation_clock: false,
            transmit_updates: true,
            process_updates: true,
        })
        .with_request_id(1)
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::StopFreeze(body) = dis_pdu_in.body {
        body
    } else {
        StopFreeze::default()
    };
    let body_out = if let PduBody::StopFreeze(body) = dis_pdu_out.body {
        body
    } else {
        StopFreeze::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.real_world_time, body_out.real_world_time);
    assert_eq!(body_in.reason, body_out.reason);
    assert_eq!(body_in.frozen_behavior, body_out.frozen_behavior);
    assert_eq!(body_in.request_id, body_out.request_id);
}

#[test]
fn codec_consistency_acknowledge() {
    use dis_rs::acknowledge::model::Acknowledge;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::Acknowledge).with_pdu_status(PduStatus::default());
    let dis_body = Acknowledge::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_acknowledge_flag(AcknowledgeFlag::StartResume)
        .with_response_flag(ResponseFlag::AbleToComply)
        .with_request_id(1)
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Acknowledge(body) = dis_pdu_in.body {
        body
    } else {
        Acknowledge::default()
    };
    let body_out = if let PduBody::Acknowledge(body) = dis_pdu_out.body {
        body
    } else {
        Acknowledge::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.acknowledge_flag, body_out.acknowledge_flag);
    assert_eq!(body_in.response_flag, body_out.response_flag);
    assert_eq!(body_in.request_id, body_out.request_id);
}

#[test]
fn codec_consistency_action_request() {
    use dis_rs::action_request::model::ActionRequest;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::ActionRequest).with_pdu_status(PduStatus::default());
    let dis_body = ActionRequest::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_request_id(1)
        .with_action_id(ActionId::JoinExercise)
        .with_fixed_datums(vec![FixedDatum::new(
            VariableRecordType::AngleOfAttack_610026,
            10,
        )])
        .with_variable_datums(vec![VariableDatum::new(
            VariableRecordType::VehicleMass_26000,
            vec![0x01, 0x02, 0x03],
        )])
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::ActionRequest(body) = dis_pdu_in.body {
        body
    } else {
        ActionRequest::default()
    };
    let body_out = if let PduBody::ActionRequest(body) = dis_pdu_out.body {
        body
    } else {
        ActionRequest::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.request_id, body_out.request_id);
    assert_eq!(body_in.action_id, body_out.action_id);
    assert_eq!(body_in.fixed_datum_records, body_out.fixed_datum_records);
    assert_eq!(
        body_in.variable_datum_records,
        body_out.variable_datum_records
    );
}

#[test]
fn codec_consistency_action_response() {
    use dis_rs::action_response::model::ActionResponse;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::ActionResponse).with_pdu_status(PduStatus::default());
    let dis_body = ActionResponse::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_request_id(1)
        .with_request_status(RequestStatus::Pending)
        .with_fixed_datums(vec![FixedDatum::new(
            VariableRecordType::AngleOfAttack_610026,
            10,
        )])
        .with_variable_datums(vec![VariableDatum::new(
            VariableRecordType::VehicleMass_26000,
            vec![0x01, 0x02, 0x03],
        )])
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::ActionResponse(body) = dis_pdu_in.body {
        body
    } else {
        ActionResponse::default()
    };
    let body_out = if let PduBody::ActionResponse(body) = dis_pdu_out.body {
        body
    } else {
        ActionResponse::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.request_id, body_out.request_id);
    assert_eq!(body_in.request_status, body_out.request_status);
    assert_eq!(body_in.fixed_datum_records, body_out.fixed_datum_records);
    assert_eq!(
        body_in.variable_datum_records,
        body_out.variable_datum_records
    );
}

#[test]
fn codec_consistency_data_query() {
    use dis_rs::data_query::model::DataQuery;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::DataQuery).with_pdu_status(PduStatus::default());
    let dis_body = DataQuery::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_request_id(1)
        .with_time_interval(2000)
        .with_fixed_datums(vec![VariableRecordType::AngleOfAttack_610026])
        .with_variable_datums(vec![VariableRecordType::VehicleMass_26000])
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::DataQuery(body) = dis_pdu_in.body {
        body
    } else {
        DataQuery::default()
    };
    let body_out = if let PduBody::DataQuery(body) = dis_pdu_out.body {
        body
    } else {
        DataQuery::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.request_id, body_out.request_id);
    assert_eq!(body_in.time_interval, body_out.time_interval);
    assert_eq!(body_in.fixed_datum_records, body_out.fixed_datum_records);
    assert_eq!(
        body_in.variable_datum_records,
        body_out.variable_datum_records
    );
}

#[test]
fn codec_consistency_set_data() {
    use dis_rs::set_data::model::SetData;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::SetData).with_pdu_status(PduStatus::default());
    let dis_body = SetData::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_request_id(1)
        .with_fixed_datums(vec![FixedDatum::new(
            VariableRecordType::AngleOfAttack_610026,
            10,
        )])
        .with_variable_datums(vec![VariableDatum::new(
            VariableRecordType::VehicleMass_26000,
            vec![0x01, 0x02, 0x03],
        )])
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::SetData(body) = dis_pdu_in.body {
        body
    } else {
        SetData::default()
    };
    let body_out = if let PduBody::SetData(body) = dis_pdu_out.body {
        body
    } else {
        SetData::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.request_id, body_out.request_id);
    assert_eq!(body_in.fixed_datum_records, body_out.fixed_datum_records);
    assert_eq!(
        body_in.variable_datum_records,
        body_out.variable_datum_records
    );
}

#[test]
fn codec_consistency_data() {
    use dis_rs::data::model::Data;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::Data).with_pdu_status(PduStatus::default());
    let dis_body = Data::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_request_id(1)
        .with_fixed_datums(vec![FixedDatum::new(
            VariableRecordType::AngleOfAttack_610026,
            10,
        )])
        .with_variable_datums(vec![VariableDatum::new(
            VariableRecordType::VehicleMass_26000,
            vec![0x01, 0x02, 0x03],
        )])
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Data(body) = dis_pdu_in.body {
        body
    } else {
        Data::default()
    };
    let body_out = if let PduBody::Data(body) = dis_pdu_out.body {
        body
    } else {
        Data::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.request_id, body_out.request_id);
    assert_eq!(body_in.fixed_datum_records, body_out.fixed_datum_records);
    assert_eq!(
        body_in.variable_datum_records,
        body_out.variable_datum_records
    );
}

#[test]
fn codec_consistency_event_report() {
    use dis_rs::event_report::model::EventReport;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header =
        PduHeader::new_v7(7, PduType::EventReport).with_pdu_status(PduStatus::default());
    let dis_body = EventReport::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_event_type(EventType::RanOutOfFuel)
        .with_fixed_datums(vec![FixedDatum::new(
            VariableRecordType::AngleOfAttack_610026,
            10,
        )])
        .with_variable_datums(vec![VariableDatum::new(
            VariableRecordType::VehicleMass_26000,
            vec![0x01, 0x02, 0x03],
        )])
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::EventReport(body) = dis_pdu_in.body {
        body
    } else {
        EventReport::default()
    };
    let body_out = if let PduBody::EventReport(body) = dis_pdu_out.body {
        body
    } else {
        EventReport::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(body_in.event_type, body_out.event_type);
    assert_eq!(body_in.fixed_datum_records, body_out.fixed_datum_records);
    assert_eq!(
        body_in.variable_datum_records,
        body_out.variable_datum_records
    );
}

#[test]
fn codec_consistency_comment() {
    use dis_rs::comment::model::Comment;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::Comment).with_pdu_status(PduStatus::default());
    let dis_body = Comment::builder()
        .with_origination_id(EntityId::new(1, 1, 1))
        .with_receiving_id(EntityId::new(2, 2, 2))
        .with_variable_datums(vec![VariableDatum::new(
            VariableRecordType::VehicleMass_26000,
            vec![0x01, 0x02, 0x03],
        )])
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    if let CdisBody::Comment(comment) = &cdis_pdu.body {
        assert!(comment.datum_specification.fixed_datum_records.is_empty());
    }

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Comment(body) = dis_pdu_in.body {
        body
    } else {
        Comment::default()
    };
    let body_out = if let PduBody::Comment(body) = dis_pdu_out.body {
        body
    } else {
        Comment::default()
    };

    assert_eq!(body_in.originating_id, body_out.originating_id);
    assert_eq!(body_in.receiving_id, body_out.receiving_id);
    assert_eq!(
        body_in.variable_datum_records,
        body_out.variable_datum_records
    );
}

#[test]
fn codec_consistency_electromagnetic_emission_full_mode() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_body = ElectromagneticEmission::builder()
        .with_emitting_entity_id(EntityId::new(1, 1, 1))
        .with_event_id(EventId::new(1, 1, 100))
        .with_state_update_indicator(ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate)
        .with_emitter_system(
            EmitterSystem::default()
                .with_number(20)
                .with_name(EmitterName::ANFPS16_5505)
                .with_function(EmitterSystemFunction::SearchAcquisition_102)
                .with_location(VectorF32::new(1.0, 2.0, 3.0))
                .with_beam(
                    Beam::default()
                        .with_number(1)
                        .with_beam_function(ElectromagneticEmissionBeamFunction::Acquisition)
                        .with_high_density_track_jam(HighDensityTrackJam::NotSelected)
                        .with_parameter_index(1)
                        .with_parameter_data(FundamentalParameterData::default())
                        .with_track_jam(TrackJam::default().with_entity_id(EntityId::new(1, 2, 3))),
                ),
        )
        .build()
        .into_pdu_body();
    let dis_header = PduHeader::new_v7(7, PduType::ElectromagneticEmission)
        .with_pdu_status(PduStatus::default());
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::ElectromagneticEmission(ee) = dis_pdu_in.body {
        ee
    } else {
        ElectromagneticEmission::default()
    };
    let body_out = if let PduBody::ElectromagneticEmission(ee) = dis_pdu_out.body {
        ee
    } else {
        ElectromagneticEmission::default()
    };

    assert_eq!(body_in.emitting_entity_id, body_out.emitting_entity_id);
    assert_eq!(body_in.emitter_systems, body_out.emitter_systems);
    assert_eq!(body_in.event_id, body_out.event_id);
    assert_eq!(
        body_in.state_update_indicator,
        body_out.state_update_indicator
    );
}

/// FIXME This test actually does little more than check a full update PDU conversion because there is no state.
#[test]
fn codec_consistency_electromagnetic_emission_partial_mode() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_partial_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::ElectromagneticEmission)
        .with_pdu_status(PduStatus::default());
    let dis_body = ElectromagneticEmission::builder()
        .with_emitting_entity_id(EntityId::new(1, 1, 1))
        .with_event_id(EventId::new(1, 1, 100))
        .with_state_update_indicator(ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate)
        .with_emitter_system(
            EmitterSystem::default()
                .with_number(20)
                .with_name(EmitterName::ANFPS16_5505)
                .with_function(EmitterSystemFunction::SearchAcquisition_102)
                .with_location(VectorF32::new(1.0, 2.0, 3.0))
                .with_beam(
                    Beam::default()
                        .with_number(1)
                        .with_beam_function(ElectromagneticEmissionBeamFunction::Acquisition)
                        .with_high_density_track_jam(HighDensityTrackJam::NotSelected)
                        .with_parameter_index(1)
                        .with_parameter_data(FundamentalParameterData::default())
                        .with_track_jam(TrackJam::default().with_entity_id(EntityId::new(1, 2, 3))),
                ),
        )
        .build()
        .into_pdu_body();
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    assert_eq!(dis_pdu_in.pdu_length(), dis_pdu_out.pdu_length());
    let body_in = if let PduBody::ElectromagneticEmission(ee) = dis_pdu_in.body {
        ee
    } else {
        ElectromagneticEmission::default()
    };
    let body_out = if let PduBody::ElectromagneticEmission(ee) = dis_pdu_out.body {
        ee
    } else {
        ElectromagneticEmission::default()
    };

    assert_eq!(body_in.emitting_entity_id, body_out.emitting_entity_id);
    assert_eq!(body_in.emitter_systems, body_out.emitter_systems);
    assert_eq!(body_in.event_id, body_out.event_id);
    assert_eq!(
        body_in.state_update_indicator,
        body_out.state_update_indicator
    );
}

#[test]
fn codec_consistency_designator_full_mode() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_body = Designator::builder()
        .with_designating_entity_id(EntityId::new(10, 10, 10))
        .with_system_name(DesignatorSystemName::ANAAS38BNiteHawk)
        .with_designated_entity_id(EntityId::new(20, 20, 20))
        .with_code(DesignatorCode::Other)
        .with_power(12.0)
        .with_wavelength(15.0)
        .with_spot_wrt_designated_entity(VectorF32::new(10.0, 10.0, 0.0))
        .with_spot_location(Location::new(0.0, 0.0, 5_000_000.0))
        .with_dead_reckoning_algorithm(
            DeadReckoningAlgorithm::DRM_FPW_ConstantVelocityLowAccelerationLinearMotionEntity,
        )
        .with_linear_acceleration(VectorF32::new(0.0, 0.0, 0.0))
        .build()
        .into_pdu_body();
    let dis_header =
        PduHeader::new_v7(7, PduType::Designator).with_pdu_status(PduStatus::default());
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Designator(designator) = dis_pdu_in.body {
        designator
    } else {
        Designator::default()
    };
    let body_out = if let PduBody::Designator(designator) = dis_pdu_out.body {
        designator
    } else {
        Designator::default()
    };

    assert_eq!(
        body_in.designating_entity_id,
        body_out.designating_entity_id
    );
    assert_eq!(body_in.designated_entity_id, body_out.designated_entity_id);
    assert_eq!(body_in.system_name, body_out.system_name);
    assert_eq!(body_in.power, body_out.power);
    assert_eq!(body_in.wavelength, body_out.wavelength);
    assert_eq!(
        body_in.spot_location.x_coordinate,
        body_out.spot_location.x_coordinate.round()
    );
    assert_eq!(
        body_in.spot_location.y_coordinate,
        body_out.spot_location.y_coordinate.round()
    );
    assert_eq!(
        body_in.spot_location.z_coordinate,
        body_out.spot_location.z_coordinate.round()
    );
    assert_eq!(body_in.linear_acceleration, body_out.linear_acceleration);
}

#[test]
fn codec_consistency_signal() {
    use dis_rs::signal::model::Signal;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::Signal).with_pdu_status(PduStatus::default());
    let dis_body = Signal::builder()
        .with_radio_reference_id(EntityId::new(1, 1, 1))
        .with_radio_number(20)
        .with_encoding_scheme(EncodingScheme::EncodedAudio {
            encoding_class: SignalEncodingClass::EncodedAudio,
            encoding_type: SignalEncodingType::_8bitMulaw_ITUTG_711_1,
        })
        .with_tdl_type(SignalTdlType::Other_0)
        .with_sample_rate(480)
        .with_samples(150)
        .with_data(vec![0x01, 0x02, 0x03])
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);

    assert_eq!(state_result, CodecStateResult::StateUnaffected);

    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Signal(body) = dis_pdu_in.body {
        body
    } else {
        Signal::default()
    };
    let body_out = if let PduBody::Signal(body) = dis_pdu_out.body {
        body
    } else {
        Signal::default()
    };

    assert_eq!(body_in, body_out);
}

#[test]
fn codec_consistency_receiver() {
    use dis_rs::receiver::model::Receiver;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::Receiver).with_pdu_status(PduStatus::default());
    let dis_body = Receiver::builder()
        .with_radio_reference_id(EntityId::new(1, 1, 1))
        .with_radio_number(20)
        .with_receiver_state(ReceiverState::OnAndReceiving)
        .with_received_power(-90.0)
        .with_transmitter_radio_reference_id(EntityId::new(2, 2, 2))
        .with_transmitter_radio_number(10)
        .build()
        .into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);

    assert_eq!(state_result, CodecStateResult::StateUnaffected);

    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Receiver(body) = dis_pdu_in.body {
        body
    } else {
        Receiver::default()
    };
    let body_out = if let PduBody::Receiver(body) = dis_pdu_out.body {
        body
    } else {
        Receiver::default()
    };

    assert_eq!(body_in, body_out);
}

#[test]
fn codec_consistency_transmitter_full_mode() {
    use std::str::FromStr;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_body = Transmitter::builder()
        .with_radio_reference_id(EntityId::new(10, 10, 10))
        .with_radio_number(1)
        .with_radio_type(EntityType::from_str("1:2:3:4:5:6:7").unwrap())
        .with_transmit_state(TransmitterTransmitState::OnAndTransmitting)
        .with_input_source(TransmitterInputSource::Pilot)
        .with_antenna_location(Location::new(0.0, 0.0, 5_000_000.0))
        .with_relative_antenna_location(VectorF32::new(1.0, 1.0, 0.0))
        .with_antenna_pattern_type(TransmitterAntennaPatternType::Beam)
        .with_frequency(1_234_000_000)
        .with_transmit_frequency_bandwidth(20.0)
        .with_power(12.0)
        .with_modulation_type(
            ModulationType::default()
                .with_spread_spectrum(SpreadSpectrum::default())
                .with_major_modulation(TransmitterMajorModulation::NoStatement)
                .with_radio_system(TransmitterModulationTypeSystem::JTIDSMIDS),
        )
        .with_crypto_system(TransmitterCryptoSystem::NoEncryptionDevice)
        .with_crypto_key_id(CryptoKeyId::default())
        .with_modulation_parameters(vec![])
        .with_antenna_pattern(BeamAntennaPattern::default().with_e_x(20.0).with_e_z(10.0))
        .build()
        .into_pdu_body();
    let dis_header =
        PduHeader::new_v7(7, PduType::Transmitter).with_pdu_status(PduStatus::default());
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Transmitter(designator) = dis_pdu_in.body {
        designator
    } else {
        Transmitter::default()
    };
    let body_out = if let PduBody::Transmitter(designator) = dis_pdu_out.body {
        designator
    } else {
        Transmitter::default()
    };

    assert_eq!(body_in.radio_reference_id, body_out.radio_reference_id);
    assert_eq!(body_in.transmit_state, body_out.transmit_state);
    assert_eq!(body_in.modulation_type, body_out.modulation_type);
    assert_eq!(body_in.antenna_pattern, body_out.antenna_pattern);
    assert_eq!(
        body_in.relative_antenna_location,
        body_out.relative_antenna_location
    );
}

/// FIXME This test actually does little more than check a full update PDU conversion because there is no state.
#[test]
fn codec_consistency_transmitter_partial_mode() {
    use std::str::FromStr;

    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_partial_update();
    let mut decoder_state = DecoderState::new();

    let dis_body = Transmitter::builder()
        .with_radio_reference_id(EntityId::new(10, 10, 10))
        .with_radio_number(1)
        .with_radio_type(EntityType::from_str("1:2:3:4:5:6:7").unwrap())
        .with_transmit_state(TransmitterTransmitState::OnAndTransmitting)
        .with_input_source(TransmitterInputSource::Pilot)
        .with_antenna_location(Location::new(0.0, 0.0, 5_000_000.0))
        .with_relative_antenna_location(VectorF32::new(1.0, 1.0, 0.0))
        .with_antenna_pattern_type(TransmitterAntennaPatternType::Beam)
        .with_frequency(1_234_000_000)
        .with_transmit_frequency_bandwidth(20.0)
        .with_power(12.0)
        .with_modulation_type(
            ModulationType::default()
                .with_spread_spectrum(SpreadSpectrum::default())
                .with_major_modulation(TransmitterMajorModulation::NoStatement)
                .with_radio_system(TransmitterModulationTypeSystem::JTIDSMIDS),
        )
        .with_crypto_system(TransmitterCryptoSystem::KGV135A)
        .with_crypto_key_id(CryptoKeyId::default())
        .with_modulation_parameters(vec![])
        .with_antenna_pattern(BeamAntennaPattern::default().with_e_x(20.0).with_e_z(10.0))
        .build()
        .into_pdu_body();
    let dis_header =
        PduHeader::new_v7(7, PduType::Transmitter).with_pdu_status(PduStatus::default());
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Transmitter(designator) = dis_pdu_in.body {
        designator
    } else {
        Transmitter::default()
    };
    let body_out = if let PduBody::Transmitter(designator) = dis_pdu_out.body {
        designator
    } else {
        Transmitter::default()
    };

    assert_eq!(body_in.radio_reference_id, body_out.radio_reference_id);
    assert_eq!(body_in.transmit_state, body_out.transmit_state);
    assert_eq!(body_in.modulation_type, body_out.modulation_type);
    assert_eq!(body_in.antenna_pattern, body_out.antenna_pattern);
    assert_eq!(body_in.power, body_out.power);
    assert_eq!(body_in.crypto_system, body_out.crypto_system);
    assert_eq!(
        body_in.relative_antenna_location,
        body_out.relative_antenna_location
    );
}

#[test]
fn codec_consistency_iff_full_mode() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_body = Iff::builder()
        .with_emitting_entity_id(EntityId::new(10, 10, 10))
        .with_event_id(EventId::new(10, 10, 1))
        .with_relative_antenna_location(VectorF32::new(1.0, 2.0, 3.0))
        .with_system_id(
            SystemId::builder()
                .with_system_name(
                    IffSystemName::GenericMarkXIIACombinedInterrogatorTransponder_CIT_,
                )
                .with_system_mode(IffSystemMode::Normal)
                .with_system_type(IffSystemType::MarkXIICombinedInterrogatorTransponder_CIT_)
                .with_change_options(ChangeOptionsRecord::default())
                .build(),
        )
        .with_system_designator(255)
        .with_system_specific_data(0)
        .with_fundamental_operational_data(
            FundamentalOperationalData::builder()
                .with_information_layers(
                    InformationLayers::builder()
                        .with_layer_2(LayersPresenceApplicability::PresentApplicable)
                        .with_layer_3(LayersPresenceApplicability::PresentApplicable)
                        .with_layer_4(LayersPresenceApplicability::PresentApplicable)
                        .with_layer_5(LayersPresenceApplicability::PresentApplicable)
                        .build(),
                )
                .build(),
        )
        .with_layer_2(IffLayer2::default().finalize_layer_header_length())
        .with_layer_3(IffLayer3::default().finalize_layer_header_length())
        .with_layer_4(IffLayer4::default().finalize_layer_header_length())
        .with_layer_5(IffLayer5::default().finalize_layer_header_length())
        .build()
        .into_pdu_body();
    let dis_header = PduHeader::new_v7(7, PduType::IFF).with_pdu_status(PduStatus::default());
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) =
        CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::IFF(iff) = dis_pdu_in.body {
        iff
    } else {
        Iff::default()
    };
    let body_out = if let PduBody::IFF(iff) = dis_pdu_out.body {
        iff
    } else {
        Iff::default()
    };
    assert_eq!(body_in, body_out);

    // assert_eq!(body_in.radio_reference_id, body_out.radio_reference_id);
    // assert_eq!(body_in.transmit_state, body_out.transmit_state);
    // assert_eq!(body_in.modulation_type, body_out.modulation_type);
    // assert_eq!(body_in.antenna_pattern, body_out.antenna_pattern);
    // assert_eq!(body_in.relative_antenna_location, body_out.relative_antenna_location);
}
