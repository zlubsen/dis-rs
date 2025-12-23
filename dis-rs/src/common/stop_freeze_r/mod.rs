pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{EntityId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{
        PduType, RequiredReliabilityService, StopFreezeFrozenBehavior, StopFreezeReason,
    };
    use crate::model::ClockTime;
    use crate::stop_freeze_r::model::StopFreezeR;
    use crate::BodyRaw;
    use bytes::BytesMut;

    #[test]
    fn stop_freeze_r_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::StopFreezeR);

        let body = StopFreezeR::builder()
            .with_origination_id(EntityId::new(10, 10, 10))
            .with_receiving_id(EntityId::new(20, 20, 20))
            .with_request_id(5)
            .with_real_world_time(ClockTime::new(12, 32))
            .with_reason(StopFreezeReason::Termination)
            .with_frozen_behavior(StopFreezeFrozenBehavior::default())
            .with_required_reliability_service(RequiredReliabilityService::Acknowledged)
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
