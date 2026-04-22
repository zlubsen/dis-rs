pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::BodyRaw;
    use crate::common::model::{EntityId, Pdu, PduHeader, TimeUnits, Timestamp};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{
        PduType, RecordQueryREventType, RequiredReliabilityService, VariableRecordType,
    };
    use crate::record_query_r::model::{RecordQueryR, RecordQuerySpecification};
    use bytes::BytesMut;

    #[test]
    fn record_query_r_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::RecordR);
        let body = RecordQueryR::builder()
            .with_origination_id(EntityId::new(10, 10, 10))
            .with_receiving_id(EntityId::new(20, 20, 20))
            .with_request_id(15)
            .with_required_reliability_service(RequiredReliabilityService::Unacknowledged)
            .with_event_type(RecordQueryREventType::InternalEntityStateData)
            .with_time(Timestamp::Absolute(TimeUnits::new(35_791_394).unwrap()))
            .with_record_query_specification(RecordQuerySpecification::default().with_record_ids(
                vec![
                    VariableRecordType::_7_62mmM62_Quantity_24005,
                    VariableRecordType::_7_62mmM80_Quantity_24008,
                ],
            ))
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(
            header,
            body,
            Timestamp::Absolute(TimeUnits::new(35_791_394).unwrap()),
        );
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
            Err(ref err) => {
                panic!("{err}")
            }
        }
    }
}
