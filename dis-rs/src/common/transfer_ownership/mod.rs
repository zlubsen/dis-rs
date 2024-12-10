pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{PduType, RequiredReliabilityService, TransferControlTransferType};
    use crate::model::{EntityId, RecordSet, RecordSpecification};
    use crate::transfer_ownership::model::TransferOwnership;
    use bytes::BytesMut;

    #[test]
    fn transfer_ownership_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::TransferOwnership);

        let body = TransferOwnership::builder()
            .with_originating_id(EntityId::new(10, 10, 10))
            .with_receiving_id(EntityId::new(20, 20, 20))
            .with_request_id(123)
            .with_required_reliability_service(RequiredReliabilityService::Unacknowledged)
            .with_transfer_type(TransferControlTransferType::PushTransferEntity_1)
            .with_transfer_entity_id(EntityId::new(30, 30, 30))
            .with_record_specification(
                RecordSpecification::default().with_record_set(RecordSet::default()),
            )
            .build()
            .into_pdu_body();
        let original_pdu =
            Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf).unwrap();

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
