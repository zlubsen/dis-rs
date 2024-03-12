pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::action_request_r::model::ActionRequestR;
    use crate::enumerations::{ActionId, PduType, RequiredReliabilityService, VariableRecordType};
    use crate::common::model::{EntityId, FixedDatum, Pdu, PduHeader, VariableDatum};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::{DisTimeStamp};

    #[test]
    fn action_request_r_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::ActionRequestR);

        let body = ActionRequestR::builder()
            .with_origination_id(EntityId::new(10,10,10))
            .with_receiving_id(EntityId::new(20,20,20))
            .with_required_reliability_service(RequiredReliabilityService::Acknowledged)
            .with_request_id(5)
            .with_action_id(ActionId::Dismount)
            .with_fixed_datums(vec![FixedDatum::new(VariableRecordType::Azimuth_52340, 45)])
            .with_variable_datums(vec![VariableDatum::new(VariableRecordType::Azimuth_52340, vec![1,2,3,4,5,6,7])])
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