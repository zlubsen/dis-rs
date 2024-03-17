pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{PduType, RecordQueryREventType, RequiredReliabilityService, VariableRecordType};
    use crate::common::model::{EntityId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::model::DisTimeStamp;
    use crate::model::TimeStamp;
    use crate::record_query_r::model::{RecordQueryR, RecordQuerySpecification};

    #[test]
    fn record_query_r_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::RecordR);
        let body = RecordQueryR::builder()
            .with_origination_id(EntityId::new(10,10,10))
            .with_receiving_id(EntityId::new(20,20,20))
            .with_request_id(15)
            .with_required_reliability_service(RequiredReliabilityService::Unacknowledged)
            .with_event_type(RecordQueryREventType::InternalEntityStateData)
            .with_time(TimeStamp::new(123456))
            .with_record_query_specification(RecordQuerySpecification::default()
                .with_record_ids(vec![VariableRecordType::_7_62mmM62_quantity_24005, VariableRecordType::_7_62mmM80_quantity_24008]))
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
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