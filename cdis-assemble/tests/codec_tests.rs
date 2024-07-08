use bytes::BytesMut;
use cdis_assemble::{BitBuffer, CdisBody, CdisPdu, SerializeCdisPdu, BodyProperties};
use cdis_assemble::codec::{CodecOptions, DecoderState, EncoderState};
use cdis_assemble::constants::EIGHT_BITS;
use cdis_assemble::entity_state::model::CdisEntityCapabilities;
use cdis_assemble::records::model::{CdisEntityMarking, CdisHeader, CdisProtocolVersion, LinearVelocity, Orientation, UnitsDekameters, WorldCoordinates};
use cdis_assemble::types::model::{SVINT16, SVINT24, UVINT16, UVINT32, UVINT8};
use dis_rs::detonation::model::Detonation;
use dis_rs::entity_state::model::{EntityAppearance, EntityMarking, EntityState};
use dis_rs::enumerations::{AirPlatformAppearance, AirPlatformCapabilities, Country, DeadReckoningAlgorithm, DetonationResult, EntityCapabilities, EntityKind, EntityMarkingCharacterSet, ExplosiveMaterialCategories, FireTypeIndicator, ForceId, MunitionDescriptorFuse, MunitionDescriptorWarhead, PduType, PlatformDomain, ProtocolVersion};
use dis_rs::fire::model::Fire;
use dis_rs::model::{DescriptorRecord, EntityId, EntityType, EventId, Location, MunitionDescriptor, Pdu, PduBody, PduHeader, PduStatus, SimulationAddress, TimeStamp, VectorF32};

#[test]
fn encode_dis_to_cdis_entity_state_full_mode() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();

    let dis_header = PduHeader::new_v7(7, PduType::EntityState);
    let dis_body = EntityState::builder()
        .with_entity_id(EntityId::new(7, 127, 255))
        .with_entity_type(EntityType::default()
            .with_domain(PlatformDomain::Air)
            .with_country(Country::Netherlands_NLD_)
            .with_kind(EntityKind::Platform))
        .with_force_id(ForceId::Friendly8)
        .with_marking(EntityMarking::new("TEST", EntityMarkingCharacterSet::ASCII))
        .build()
        .into_pdu_body();
    let dis_pdu = Pdu::finalize_from_parts(dis_header, dis_body, 1000);

    let (cdis_pdu, _state_result) = CdisPdu::encode(&dis_pdu, &mut encoder_state, &codec_options);

    let mut buf : BitBuffer = BitBuffer::ZERO;
    let written_bits = cdis_pdu.serialize(&mut buf, 0);
    let written_bytes = written_bits.div_ceil(EIGHT_BITS);

    let parsed_cdis_pdus = cdis_assemble::parse(&buf.data[..written_bytes]).unwrap();
    let cdis_pdu = parsed_cdis_pdus.first().unwrap();

    if let CdisBody::EntityState(es) = &cdis_pdu.body {
        assert_eq!(es.entity_id, cdis_assemble::records::model::EntityId::new(UVINT16::from(7), UVINT16::from(127), UVINT16::from(255)));
        assert_eq!(es.entity_type.unwrap().country, u16::from(Country::Netherlands_NLD_));
        assert_eq!(es.entity_marking.as_ref().unwrap().marking.as_str(), "TEST")
    } else {
        assert!(false);
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
        pdu_status: Default::default(),
    };
    let cdis_body = cdis_assemble::entity_state::model::EntityState {
        units: UnitsDekameters::Dekameter,
        full_update_flag: false,
        entity_id: cdis_assemble::records::model::EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
        force_id: Some(UVINT8::from(u8::from(ForceId::Friendly))),
        entity_type: Some(cdis_assemble::records::model::EntityType::new(u8::from(EntityKind::Platform), u8::from(PlatformDomain::Air), u16::from(Country::from(1)), UVINT8::from(1), UVINT8::from(1), UVINT8::from(1), UVINT8::from(1))),
        alternate_entity_type: None,
        entity_linear_velocity: Some(LinearVelocity::new(SVINT16::from(5), SVINT16::from(5),SVINT16::from(-5))),
        entity_location: Some(WorldCoordinates::new(52.0, 5.0, SVINT24::from(1000))),
        entity_orientation: Some(Orientation::new(4, 3, 2)),
        entity_appearance: None,
        dr_algorithm: DeadReckoningAlgorithm::DRM_FPW_ConstantVelocityLowAccelerationLinearMotionEntity,
        dr_params_other: None,
        dr_params_entity_linear_acceleration: None,
        dr_params_entity_angular_velocity: None,
        entity_marking: Some(CdisEntityMarking::new("TEST".to_string())),
        capabilities: Some(CdisEntityCapabilities(UVINT32::from(0xABC00000))),
        variable_parameters: vec![],
    }.into_cdis_body();
    let cdis_pdu = CdisPdu::finalize_from_parts(cdis_header, cdis_body, None::<TimeStamp>);

    let (dis_pdu, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    let mut buf = BytesMut::with_capacity(250);
    dis_pdu.serialize(&mut buf).unwrap();

    let parsed_pdus = dis_rs::parse(&buf).unwrap();
    let parsed_pdu = parsed_pdus.first().unwrap();

    if let PduBody::EntityState(es) = &parsed_pdu.body {
        assert_eq!(parsed_pdu.header.exercise_id, 7);
        assert_eq!(parsed_pdu.header.protocol_version, ProtocolVersion::IEEE1278_12012);
        assert_eq!(es.entity_id, EntityId::new(10, 10, 10));
        assert_eq!(es.force_id, ForceId::Friendly);
        assert_eq!(es.entity_type, EntityType::default()
            .with_kind(EntityKind::Platform)
            .with_domain(PlatformDomain::Air)
            .with_country(Country::from(1))
            .with_category(1)
            .with_subcategory(1)
            .with_specific(1)
            .with_extra(1));
        assert_eq!(es.entity_marking.marking_string.as_str(), "TEST");
    } else {
        assert!(false);
    }
}

#[test]
fn codec_consistency_entity_state_full_mode() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::EntityState).with_pdu_status(PduStatus::default());
    let dis_body = EntityState::builder()
        .with_entity_id(EntityId::new(7, 127, 255))
        .with_entity_type(EntityType::default()
            .with_domain(PlatformDomain::Air)
            .with_country(Country::Netherlands_NLD_)
            .with_kind(EntityKind::Platform))
        .with_force_id(ForceId::Friendly8)
        .with_location(Location::new(0.0, 0.0, 5_000_000.0))
        .with_appearance(EntityAppearance::AirPlatform(AirPlatformAppearance::default()))
        .with_marking(EntityMarking::new("TEST", EntityMarkingCharacterSet::ASCII))
        .with_capabilities(EntityCapabilities::AirPlatformEntityCapabilities(AirPlatformCapabilities::default()))
        .build()
        .into_pdu_body();
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) = CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::EntityState(es) = dis_pdu_in.body { es } else { EntityState::default() };
    let body_out = if let PduBody::EntityState(es) = dis_pdu_out.body { es } else { EntityState::default() };

    assert_eq!(body_in.entity_id, body_out.entity_id);
    assert_eq!(body_in.entity_type, body_out.entity_type);
    assert_eq!(body_in.force_id, body_out.force_id);
    assert_eq!(body_in.entity_appearance, body_out.entity_appearance);
    assert_eq!(body_in.entity_marking, body_out.entity_marking);
    assert_eq!(body_in.entity_capabilities, body_out.entity_capabilities);
    assert_eq!(body_in.entity_location.x_coordinate, body_out.entity_location.x_coordinate.round());
    assert_eq!(body_in.entity_location.y_coordinate, body_out.entity_location.y_coordinate.round());
}

#[test]
fn codec_consistency_fire() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::Fire).with_pdu_status(PduStatus::default().with_fire_type_indicator(FireTypeIndicator::Munition));
    let dis_body = Fire::builder()
        .with_firing_entity_id(EntityId::new(10, 10, 10))
        .with_target_entity_id(EntityId::new(20, 20, 20))
        .with_entity_id(EntityId::new(10, 10, 500))
        .with_event_id(EventId::new(SimulationAddress::new(10, 10), 1))
        .with_location_in_world(Location::new(0.0, 0.0, 20000.0))
        .with_munition_descriptor(
            EntityType::default()
                .with_kind(EntityKind::Munition)
                .with_domain(PlatformDomain::Air),
            MunitionDescriptor::default()
                .with_warhead(MunitionDescriptorWarhead::Dummy)
                .with_fuse(MunitionDescriptorFuse::Dummy_8110)
                .with_quantity(1)
                .with_rate(1))
        .with_range(10000.0)
        .build().into_pdu_body();
    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) = CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Fire(fire) = dis_pdu_in.body { fire } else { Fire::default() };
    let body_out = if let PduBody::Fire(fire) = dis_pdu_out.body { fire } else { Fire::default() };

    assert_eq!(body_in.firing_entity_id, body_out.firing_entity_id);
    assert_eq!(body_in.target_entity_id, body_out.target_entity_id);
    assert_eq!(body_in.entity_id, body_out.entity_id);
    assert_eq!(body_in.event_id, body_out.event_id);
    assert_eq!(body_in.fire_mission_index, body_out.fire_mission_index);
    assert_eq!(body_in.location_in_world.x_coordinate, body_out.location_in_world.x_coordinate.round());
    assert_eq!(body_in.location_in_world.y_coordinate, body_out.location_in_world.y_coordinate.round());
    assert_eq!(body_in.descriptor, body_out.descriptor);
    assert_eq!(body_in.velocity, body_out.velocity);
    assert_eq!(body_in.range, body_out.range);
}

#[test]
fn codec_consistency_detonation() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::Detonation).with_pdu_status(PduStatus::default());
    let dis_body = Detonation::builder()
        .with_source_entity_id(EntityId::new(1, 1, 1))
        .with_target_entity_id(EntityId::new(2, 2, 1))
        .with_exploding_entity_id(EntityId::new(1, 1, 100))
        .with_event_id(EventId::new(SimulationAddress::new(1, 1), 1))
        .with_velocity(VectorF32::new(10.0, 10.0, 10.0))
        .with_world_location(Location::new(0.0, 0.0, 20000.0))
        .with_descriptor(DescriptorRecord::new_explosion(
            EntityType::default()
                .with_kind(EntityKind::Other)
                .with_domain(PlatformDomain::Land),
            ExplosiveMaterialCategories::Alcohol,
            200.0
        ))
        .with_entity_location(VectorF32::new(10.0, 10.0, 0.0))
        .with_detonation_result(DetonationResult::Detonation)
        .build().into_pdu_body();

    let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

    let (cdis_pdu, _state_result) = CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

    let (dis_pdu_out, _state_result) = cdis_pdu.decode(&mut decoder_state, &codec_options);
    assert_eq!(dis_pdu_in.header, dis_pdu_out.header);
    let body_in = if let PduBody::Detonation(detonation) = dis_pdu_in.body { detonation } else { Detonation::default() };
    let body_out = if let PduBody::Detonation(detonation) = dis_pdu_out.body { detonation } else { Detonation::default() };

    assert_eq!(body_in.source_entity_id, body_out.source_entity_id);
    assert_eq!(body_in.target_entity_id, body_out.target_entity_id);
    assert_eq!(body_in.exploding_entity_id, body_out.exploding_entity_id);
    assert_eq!(body_in.event_id, body_out.event_id);
    assert_eq!(body_in.velocity, body_out.velocity);
    assert_eq!(body_in.location_in_world_coordinates.x_coordinate, body_out.location_in_world_coordinates.x_coordinate.round());
    assert_eq!(body_in.location_in_world_coordinates.y_coordinate, body_out.location_in_world_coordinates.y_coordinate.round());
    // FIXME: explosions are not properly encoded/decoded because fields explosive_material and explosive_force are not specified in C-DIS
    assert_eq!(body_in.descriptor, body_out.descriptor);
    assert_eq!(body_in.location_in_entity_coordinates, body_out.location_in_entity_coordinates);
    assert_eq!(body_in.detonation_result, body_out.detonation_result);
    assert_eq!(body_in.variable_parameters, body_out.variable_parameters);
}

#[test]
fn codec_consistency_collision() {
    let mut encoder_state = EncoderState::new();
    let codec_options = CodecOptions::new_full_update();
    let mut decoder_state = DecoderState::new();

    let dis_header = PduHeader::new_v7(7, PduType::Collision).with_pdu_status(PduStatus::default());
    todo!()
}