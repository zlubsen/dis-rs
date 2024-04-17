use cdis_assemble::{BitBuffer, CdisBody, CdisPdu, Codec, SerializeCdisPdu};
use cdis_assemble::types::model::UVINT16;
use dis_rs::entity_state::model::{EntityMarking, EntityState};
use dis_rs::enumerations::{Country, EntityKind, EntityMarkingCharacterSet, ForceId, PduType, PlatformDomain};
use dis_rs::model::{EntityId, EntityType, Pdu, PduHeader};

#[test]
fn dis_to_cdis_entity_state() {
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

    let cdis_pdu = CdisPdu::encode(&dis_pdu);

    let mut buf : BitBuffer = BitBuffer::ZERO;
    let _cursor = cdis_pdu.serialize(&mut buf, 0);

    let written_bytes = _cursor.div_ceil(8);

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
fn cdis_to_dis_entity_state() {
    assert!(false);
}