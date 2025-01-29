pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::entity_state_update::model::EntityStateUpdate;
    use crate::enumerations::PduType;
    use crate::model::{EntityId, Location, Orientation, VectorF32};
    use bytes::BytesMut;

    #[test]
    fn entity_state_update_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::EntityStateUpdate);

        let body = EntityStateUpdate::builder()
            .with_entity_id(EntityId::new(500, 900, 14))
            .with_velocity(VectorF32::new(10.0, 10.0, 10.0))
            .with_location(Location::new(4.0, 5.0, 6.0))
            .with_orientation(Orientation::new(0.5, 0.5, 0.5))
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
