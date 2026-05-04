use bytes::BytesMut;
use dis_assemble_gen_3::common_records::PDUHeader;
use dis_assemble_gen_3::{core::writer::Serialize, parse};
use dis_assemble_gen_3::{BodyRaw, Pdu};

const TIMESTAMP_01_01_2026: i64 = 1_767_225_600_000_000;

#[test]
fn test_consistency_entity_state() {
    let header = PDUHeader::default();
    let body = dis_assemble_gen_3::entity_info_interaction::entity_state::EntityState::builder()
        // TODO build the PDU
        .build()
        .into_pdu_body();
    let pdu_in = Pdu::finalize_from_parts(header, body, TIMESTAMP_01_01_2026);

    let mut buf = BytesMut::new();

    let bytes_written = pdu_in.serialize(&mut buf);

    let pdu_out = parse(&buf).unwrap();
    let pdu_out = pdu_out.first().unwrap();

    assert_eq!(&pdu_in, pdu_out);
}
