pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::aggregate_state::model::{AggregateMarking, AggregateState, AggregateType};
    use crate::enumerations::{AggregateStateAggregateState, ForceId, PduType};
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::{DisTimeStamp};
    use crate::model::EntityId;

    #[test]
    fn aggregate_state_internal_consistency() {
        let header = PduHeader::new_v7(1, PduType::AggregateState);

        let body = AggregateState::builder()
            .with_aggregate_id(EntityId::new(10, 10, 10))
            .with_force_id(ForceId::Friendly)
            .with_aggregate_state(AggregateStateAggregateState::Aggregated)
            .with_aggregate_type(AggregateType::try_from("1:2:3:4:5:6:7").unwrap())
            .with_aggregate_marking(AggregateMarking::default().with_marking("Squadron XYZ"))

        // pub formation: AggregateStateFormation,
        // pub aggregate_marking: AggregateMarking,
        // pub dimensions: VectorF32,
        // pub orientation: Orientation,
        // pub center_of_mass: Location,
        // pub velocity: VectorF32,
        // pub aggregates: Vec<EntityId>,
        // pub entities: Vec<EntityId>,
        // pub silent_aggregate_systems: Vec<SilentAggregateSystem>,
        // pub silent_entity_systems: Vec<SilentEntitySystem>,
        // pub variable_datums: Vec<VariableDatum>,

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