pub mod parser;
pub mod model;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::collision::model::Collision;
    use crate::enumerations::{PduType, CollisionType};
    use crate::common::model::{EntityId, EventId, Pdu, PduHeader, SimulationAddress};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::{DisTimeStamp};

    #[test]
    fn collision_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::Acknowledge);

        let body = Collision::builder()
            .with_issuing_entity_id(EntityId::new(10, 10, 10))
            .with_colliding_entity_id(EntityId::new(20, 20, 20))
            .with_collision_type(CollisionType::Elastic)
            .with_event_id(EventId::new(SimulationAddress::new(10, 10), 43))
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