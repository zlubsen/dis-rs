pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{IsPartOfNature, IsPartOfPosition, PduType, StationName};
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::model::DisTimeStamp;
    use crate::is_part_of::model::{IsPartOf, NamedLocationId, Relationship};
    use crate::model::{EntityId, VectorF32};

    #[test]
    fn is_part_of_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::IsPartOf);

        let body = IsPartOf::builder()
            .with_originating_simulation_id(EntityId::new(10,10,10))
            .with_receiving_entity_id(EntityId::new(20, 20, 20))
            .with_relationship(Relationship::default()
                .with_position(IsPartOfPosition::OnTopOf)
                .with_nature(IsPartOfNature::EmitterMountedOnHost))
            .with_part_location(VectorF32::new(1.0, 2.0, 3.0))
            .with_named_location_id(NamedLocationId::default()
                .with_station_name(StationName::OnStationRangeAndBearing)
                .with_station_number(44))
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf).unwrap();

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