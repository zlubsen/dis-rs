use dis_rs::acknowledge::model::Acknowledge;
use dis_rs::enumerations::{PduType};
use dis_rs::model::{EntityId, Pdu, PduHeader, TimeStamp};
use dis_rs::other::model::Other;


fn main() {
    println!("A simple user application that uses the dis-rs library.");

    let other = Other::builder().w

    let ack = Acknowledge::builder()
        .with_origination_id(EntityId::new(1,2,3))
        .build()
        .into_pdu_body();
    let header = PduHeader::new_v7(54, PduType::Acknowledge);
    let _pdu = Pdu::finalize_from_parts(header, ack, TimeStamp::new(583));
}
