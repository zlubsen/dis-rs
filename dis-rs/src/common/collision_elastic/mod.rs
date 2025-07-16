pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::collision_elastic::model::CollisionElastic;
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{EventId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::PduType;
    use bytes::BytesMut;

    #[test]
    fn collision_elastic_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::CollisionElastic);

        let body = CollisionElastic::builder()
            .with_mass(88.4f32)
            .with_coefficient_of_restitution(6.43)
            .with_event_id(EventId::new(11, 11, 11))
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
