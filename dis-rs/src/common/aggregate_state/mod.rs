pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::aggregate_state::model::{
        AggregateMarking, AggregateState, AggregateType, SilentAggregateSystem, SilentEntitySystem,
    };
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::entity_state::model::EntityAppearance;
    use crate::enumerations::{
        AggregateStateAggregateState, AggregateStateFormation, AirPlatformAppearance,
        CoupledExtensionIndicator, ForceId, LvcIndicator, PduType, VariableRecordType,
    };
    use crate::model::{EntityId, EntityType, Location, Orientation, VariableDatum, VectorF32};
    use crate::v7::model::PduStatus;
    use bytes::BytesMut;
    use std::str::FromStr;

    #[test]
    fn aggregate_state_internal_consistency() {
        let header = PduHeader::new_v7(1, PduType::AggregateState).with_pdu_status(
            PduStatus::default()
                .with_lvc_indicator(LvcIndicator::NoStatement)
                .with_coupled_extension_indicator(CoupledExtensionIndicator::NotCoupled),
        );

        let body = AggregateState::builder()
            .with_aggregate_id(EntityId::new(10, 10, 10))
            .with_force_id(ForceId::Friendly)
            .with_aggregate_state(AggregateStateAggregateState::Aggregated)
            .with_aggregate_type(AggregateType::try_from("1:2:3:4:5:6:7").unwrap())
            .with_aggregate_marking(AggregateMarking::default().with_marking("Squadron XYZ"))
            .with_formation(AggregateStateFormation::Vee)
            .with_dimensions(VectorF32::new(100.0, 100.0, 100.0))
            .with_orientation(Orientation::new(0.0, 0.0, 0.0))
            .with_center_of_mass(Location::new(1.0, 2.0, 3.0))
            .with_velocity(VectorF32::new(20.0, 20.0, 0.0))
            .with_aggregate(EntityId::new(20, 20, 20))
            .with_entity(EntityId::new(30, 30, 30))
            .with_silent_aggregate_system(
                SilentAggregateSystem::default()
                    .with_aggregate_type(AggregateType::from_str("1:2:3:4:5:6:7").unwrap()),
            )
            .with_silent_entity_system(
                SilentEntitySystem::default()
                    .with_entity_type(EntityType::from_str("1:2:10:10:10:10:10").unwrap())
                    .with_number_of_entities(1)
                    .with_appearance(EntityAppearance::AirPlatform(
                        AirPlatformAppearance::default(),
                    )),
            )
            .with_variable_datum(VariableDatum::new(
                VariableRecordType::AirSpeed_240054,
                vec![250u8],
            ))
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
