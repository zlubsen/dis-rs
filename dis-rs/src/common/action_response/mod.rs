pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::action_response::model::ActionResponse;
    use crate::enumerations::{PduType, RequestStatus};
    use crate::common::model::{EntityId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::{Serialize};
    use crate::common::model::{DisTimeStamp};

    #[test]
    fn action_response_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::ActionResponse);

        let body = ActionResponse::builder()
            .with_origination_id(EntityId::new(10,10,10))
            .with_receiving_id(EntityId::new(20,20,20))
            .with_request_id(5)
            .with_request_status(RequestStatus::Complete)
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