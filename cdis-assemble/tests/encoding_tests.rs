use cdis_assemble::{BitBuffer, CdisPdu, Codec, SerializeCdisPdu};
use dis_rs::entity_state::model::{EntityMarking, EntityState};
use dis_rs::enumerations::{Country, EntityKind, EntityMarkingCharacterSet, ForceId, PduType, PlatformDomain};
use dis_rs::model::{EntityId, EntityType, Pdu, PduHeader};

#[test]
fn dis_to_cdis() {
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

    let bytes = _cursor.div_ceil(8);
    println!("buf data {:?}", &buf.data[..bytes]);

    let vec = Vec::from(&buf.data[.._cursor]);

    let parsed_cdis = cdis_assemble::parse(&buf.data[..bytes]).unwrap();
// TODO panics due to attempting to parse several pdu's but not enough input > handle error

    println!("{:?}", parsed_cdis);
}