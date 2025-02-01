pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{EntityId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{PduType, RepairResponseRepairResult};
    use crate::repair_response::model::RepairResponse;
    use bytes::BytesMut;

    #[test]
    fn repair_response_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::RepairResponse);

        let body = RepairResponse::builder()
            .with_receiving_id(EntityId::new(1, 1, 2))
            .with_repairing_id(EntityId::new(9, 1, 1))
            .with_repair_result(RepairResponseRepairResult::RepairEnded)
            .build()
            .into_pdu_body();
        let original_pdu =
            Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;
        let original_length = original_pdu.pdu_length();

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        let serialized_length = original_pdu.serialize(&mut buf).unwrap();

        assert_eq!(original_length, serialized_length);

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
