use bytes::BytesMut;
use cdis_assemble::{BitBuffer, CdisBody, CdisPdu, SerializeCdisPdu, BodyProperties};
use cdis_assemble::codec::{CodecOptions, DecoderState, EncoderState};
use cdis_assemble::entity_state::model::CdisEntityCapabilities;
use cdis_assemble::records::model::{CdisEntityMarking, CdisHeader, CdisProtocolVersion, LinearVelocity, Orientation, Units, WorldCoordinates};
use cdis_assemble::types::model::{SVINT16, SVINT24, UVINT16, UVINT32, UVINT8};
use dis_rs::entity_state::model::{EntityMarking, EntityState};
use dis_rs::enumerations::{Country, DeadReckoningAlgorithm, EntityKind, EntityMarkingCharacterSet, ForceId, PduType, PlatformDomain, ProtocolVersion};
use dis_rs::model::{EntityId, EntityType, Pdu, PduBody, PduHeader, TimeStamp};

#[test]
fn dis_to_cdis_entity_state() {
    let encoder_state = EncoderState::new();
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

    let (cdis_pdu, _state_result) = CdisPdu::encode(&dis_pdu, &encoder_state, &codec_options);

    let mut buf : BitBuffer = BitBuffer::ZERO;
    let written_bits = cdis_pdu.serialize(&mut buf, 0);

    let written_bytes = written_bits.div_ceil(8);
    // FIXME parsing fails because what is encoded using CodecOptions and what is serialized / calculated size does not match.
    // Even if an optional field is Some(), it could be left out of the serialization (and vice versa?)
    // FIXME Could also be due to some fields that can be left out even without a full update - dr params other, capabilities etc.
    let parsed_cdis_pdus = cdis_assemble::parse(&buf.data[..written_bytes]).unwrap()
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
fn cdis_to_dis_entity_state() {
    let decoder_state = DecoderState::new();
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
        units: Units::Dekameter,
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

    let (dis_pdu, _state_result) = cdis_pdu.decode(&decoder_state, &codec_options);
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