pub mod parser;
pub mod model;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{PduType, RepairResponseRepairResult};
    use crate::common::model::{EntityId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::DisTimeStamp;
    use crate::repair_response::model::RepairResponse;

    #[test]
    fn repair_response_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::RepairResponse);

        let body = RepairResponse::builder()
            .with_receiving_id(EntityId::new(1, 1, 2))
            .with_repairing_id(EntityId::new(9, 1, 1))
            .with_repair_result(RepairResponseRepairResult::repairended)
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf);

        let parsed = parse_pdu(&buf);
        match parsed {
            Ok(ref pdu) => {
                assert_eq!(&original_pdu, pdu);
            }
            Err(ref _err) => {
                println!("{_err}");
                assert!(false);
            }
        }
    }
}