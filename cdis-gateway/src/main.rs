use std::io::Read;
use bytes::BytesMut;
use cdis_assemble::entity_state::model::EntityState;
use cdis_assemble::{BitBuffer, CdisPdu, Codec, create_bit_buffer, SerializeCdisPdu};
use dis_rs::entity_state::model::EntityMarking;
use dis_rs::enumerations::{Country, EntityKind, EntityMarkingCharacterSet, PduType, PlatformDomain};
use dis_rs::model::{EntityId, EntityType, Location, Pdu, PduHeader};

fn main() {
    let mut write_buf: BitBuffer = create_bit_buffer();
    let mut read_buf = BytesMut::with_capacity(1400);

    let dis_entity_state_body = dis_rs::entity_state::model::EntityState::builder()
        .with_entity_id(EntityId::new(8, 8, 8))
        .with_entity_type(EntityType::default()
            .with_kind(EntityKind::Platform)
            .with_domain(PlatformDomain::Air)
            .with_country(Country::Netherlands_NLD_))
        .with_location(Location::new(35000.0, 10000.0, 30000.0))
        .with_marking(EntityMarking::new("My1stPlane", EntityMarkingCharacterSet::ASCII))
        .build().into_pdu_body();
    let header = PduHeader::new_v7(1, PduType::EntityState);
    let dis_entity_state = Pdu::finalize_from_parts(header, dis_entity_state_body, 500);
    // TODO encode entire PDU...
    let cdis_entity_state = CdisPdu::encode(&dis_entity_state);

    let cursor = cdis_entity_state.serialize(&mut write_buf, 0);

    let cdis_wire: Vec<u8> = write_buf.data[0..cursor].chunks_exact(8).map(|ch| { ch[0] } ).collect();
    println!("{}", cdis_wire.len());
    dbg!(cdis_wire.clone());

    let parsed_cdis = cdis_assemble::parse(cdis_wire.as_slice()).unwrap();
    dbg!(parsed_cdis);

    let decoded_es = cdis_entity_state.decode();
    dbg!(decoded_es);
}
