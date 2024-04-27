use dis_rs::{
    acknowledge::model::Acknowledge,
    enumerations::PduType,
    model::{EntityId, Pdu, PduHeader, TimeStamp},
};

fn main() {
    println!("A simple user application that uses the dis-rs library");

    let ack = Acknowledge::builder()
        .with_origination_id(EntityId::new(1, 2, 3))
        .build()
        .into_pdu_body();
    let header = PduHeader::new_v7(54, PduType::Acknowledge);
    let pdu = Pdu::finalize_from_parts(header, ack, TimeStamp::new(583));

    println!("PDU: {pdu:?}");
}
