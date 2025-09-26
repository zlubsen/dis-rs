pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::collision::model::Collision;
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{EntityId, EventId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{CollisionType, PduType};
    use bytes::BytesMut;

    #[test]
    fn collision_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::Collision);

        let body = Collision::builder()
            .with_issuing_entity_id(EntityId::new(10, 10, 10))
            .with_colliding_entity_id(EntityId::new(20, 20, 20))
            .with_collision_type(CollisionType::Elastic)
            .with_event_id(EventId::new(10, 10, 43))
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
