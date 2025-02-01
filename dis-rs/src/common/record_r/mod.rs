pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{EntityId, Pdu, PduHeader, RecordSet, RecordSpecification};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{EventType, PduType, RequiredReliabilityService, VariableRecordType};
    use crate::record_r::model::RecordR;
    use bytes::BytesMut;

    #[test]
    fn record_r_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::RecordR);
        let body = RecordR::builder()
            .with_origination_id(EntityId::new(10, 10, 10))
            .with_receiving_id(EntityId::new(20, 20, 20))
            .with_request_id(15)
            .with_required_reliability_service(RequiredReliabilityService::Unacknowledged)
            .with_event_type(EventType::Detect)
            .with_response_serial_number(2132)
            .with_record_specification(
                RecordSpecification::default().with_record_set(
                    RecordSet::default()
                        .with_record_id(VariableRecordType::AirSpeed_240054)
                        .with_record_serial_number(12345)
                        .with_records(vec![vec![0x01, 0x02, 0x03], vec![0x10, 0x20, 0x30]]),
                ),
            )
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
