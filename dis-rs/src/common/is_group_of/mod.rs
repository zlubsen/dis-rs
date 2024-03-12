pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{AirPlatformAppearance, IsGroupOfGroupedEntityCategory, PduType};
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::{DisTimeStamp};
    use crate::entity_state::model::EntityAppearance;
    use crate::is_group_of::model::{GEDRecord7, GEDRecord8, GroupEntityDescription, GroupReferencePoint, IsGroupOf};
    use crate::model::EntityId;

    #[test]
    fn is_group_of_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::IsGroupOf);

        let body = IsGroupOf::builder()
            .with_group_id(EntityId::new(1, 10, 20))
            .with_group_reference_point(GroupReferencePoint::default()
                .with_latitude(1.0)
                .with_longitude(1.0))
            .with_grouped_entity_category(IsGroupOfGroupedEntityCategory::EnhancedFixedWingAircraft)
            .with_description(GroupEntityDescription::EnhancedFixedWingAircraft(
                GEDRecord8 {
                    basic_fixed_wing_aircraft: GEDRecord7 {
                        entity_id: 30,
                        location: Default::default(),
                        appearance: EntityAppearance::AirPlatform(AirPlatformAppearance::default()),
                        orientation: Default::default(),
                        fuel_status: 0,
                        movement_horizontal_deviation: 0,
                        movement_vertical_deviation: 0,
                        movement_speed: 0,
                    },
                    supplemental_fuel_status: 1,
                    air_maintenance_status: 2,
                    primary_ammunition: 3,
                    secondary_ammunition: 4,
                }
            ))

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