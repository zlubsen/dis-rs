pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{PduType, RequiredReliabilityService, VariableRecordType};
    use crate::model::{EntityId, FixedDatum, VariableDatum};
    use crate::set_data_r::model::SetDataR;
    use bytes::BytesMut;

    #[test]
    fn set_data_r_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::SetDataR);

        let body = SetDataR::builder()
            .with_origination_id(EntityId::new(10, 10, 10))
            .with_receiving_id(EntityId::new(20, 20, 20))
            .with_required_reliability_service(RequiredReliabilityService::Unacknowledged)
            .with_request_id(5)
            .with_fixed_datums(vec![FixedDatum::new(
                VariableRecordType::_7_62mm_Quantity_24010,
                100,
            )])
            .with_variable_datums(vec![VariableDatum::new(
                VariableRecordType::Country_11130,
                vec![0x1, 0x2, 0x3],
            )])
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
