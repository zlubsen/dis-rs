pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{PduType, RequiredReliabilityService, VariableRecordType};
    use crate::common::model::{EntityId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::{DisTimeStamp};
    use crate::model::{RecordSet, RecordSpecification};
    use crate::set_record_r::model::SetRecordR;

    #[test]
    fn set_record_r_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::RecordR);
        let body = SetRecordR::builder()
            .with_origination_id(EntityId::new(10,10,10))
            .with_receiving_id(EntityId::new(20,20,20))
            .with_request_id(15)
            .with_required_reliability_service(RequiredReliabilityService::Acknowledged)
            .with_record_specification(RecordSpecification::default()
                .with_record_set(
                    RecordSet::default().with_record_id(VariableRecordType::AirSpeed_240054)
                        .with_record_serial_number(12345)
                        .with_records(vec![vec![0x01, 0x02, 0x03], vec![0x10, 0x20, 0x30]])))
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