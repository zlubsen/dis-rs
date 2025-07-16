pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{EntityId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::electromagnetic_emission::model::{ElectromagneticEmission, EmitterSystem};
    use crate::enumerations::{ElectromagneticEmissionStateUpdateIndicator, PduType};
    use crate::model::EventId;
    use bytes::BytesMut;

    #[test]
    fn electromagnetic_emission_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::ElectromagneticEmission);

        let body = ElectromagneticEmission::builder()
            .with_emitting_entity_id(EntityId::new(5, 10, 15))
            .with_event_id(EventId::new(5, 10, 70))
            .with_emitter_system(EmitterSystem::default())
            .with_state_update_indicator(
                ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate,
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
            Err(ref err) => {
                panic!("Parse error: {err}");
            }
        }
    }
}
