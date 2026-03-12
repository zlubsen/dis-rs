pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::action_response::model::ActionResponse;
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{EntityId, FixedDatum, Pdu, PduHeader, VariableDatum};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{PduType, RequestStatus, VariableRecordType};
    use crate::BodyRaw;
    use bytes::BytesMut;

    #[test]
    fn action_response_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::ActionResponse);

        let body = ActionResponse::builder()
            .with_origination_id(EntityId::new(10, 10, 10))
            .with_receiving_id(EntityId::new(20, 20, 20))
            .with_request_id(5)
            .with_request_status(RequestStatus::Complete)
            .with_fixed_datums(vec![FixedDatum::new(VariableRecordType::Azimuth_52340, 45)])
            .with_variable_datums(vec![VariableDatum::new(
                VariableRecordType::Azimuth_52340,
                vec![1, 2, 3, 4, 5, 6, 7],
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
            Err(ref err) => {
                panic!("Parse error: {err}");
            }
        }
    }
}
