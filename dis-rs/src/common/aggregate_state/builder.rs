use crate::aggregate_state::model::{
    AggregateMarking, AggregateState, AggregateType, SilentAggregateSystem, SilentEntitySystem,
};
use crate::enumerations::{AggregateStateAggregateState, AggregateStateFormation, ForceId};
use crate::model::{EntityId, Location, Orientation, VariableDatum, VectorF32};

pub struct AggregateStateBuilder(AggregateState);

impl AggregateStateBuilder {
    pub fn new() -> Self {
        AggregateStateBuilder(AggregateState::default())
    }

    pub fn new_from_body(body: AggregateState) -> Self {
        AggregateStateBuilder(body)
    }

    pub fn build(self) -> AggregateState {
        self.0
    }

    pub fn with_aggregate_id(mut self, aggregate_id: EntityId) -> Self {
        self.0.aggregate_id = aggregate_id;
        self
    }

    pub fn with_force_id(mut self, force_id: ForceId) -> Self {
        self.0.force_id = force_id;
        self
    }

    pub fn with_aggregate_state(mut self, aggregate_state: AggregateStateAggregateState) -> Self {
        self.0.aggregate_state = aggregate_state;
        self
    }

    pub fn with_aggregate_type(mut self, aggregate_type: AggregateType) -> Self {
        self.0.aggregate_type = aggregate_type;
        self
    }

    pub fn with_formation(mut self, formation: AggregateStateFormation) -> Self {
        self.0.formation = formation;
        self
    }

    pub fn with_aggregate_marking(mut self, aggregate_marking: AggregateMarking) -> Self {
        self.0.aggregate_marking = aggregate_marking;
        self
    }

    pub fn with_dimensions(mut self, dimensions: VectorF32) -> Self {
        self.0.dimensions = dimensions;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.0.orientation = orientation;
        self
    }

    pub fn with_center_of_mass(mut self, center_of_mass: Location) -> Self {
        self.0.center_of_mass = center_of_mass;
        self
    }

    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.0.velocity = velocity;
        self
    }

    pub fn with_aggregate(mut self, aggregate: EntityId) -> Self {
        self.0.aggregates.push(aggregate);
        self
    }

    pub fn with_aggregates(mut self, aggregates: Vec<EntityId>) -> Self {
        self.0.aggregates = aggregates;
        self
    }

    pub fn with_entity(mut self, entity: EntityId) -> Self {
        self.0.entities.push(entity);
        self
    }

    pub fn with_entities(mut self, entities: Vec<EntityId>) -> Self {
        self.0.entities = entities;
        self
    }

    pub fn with_silent_aggregate_system(
        mut self,
        silent_aggregate_system: SilentAggregateSystem,
    ) -> Self {
        self.0
            .silent_aggregate_systems
            .push(silent_aggregate_system);
        self
    }

    pub fn with_silent_aggregate_systems(
        mut self,
        silent_aggregate_systems: Vec<SilentAggregateSystem>,
    ) -> Self {
        self.0.silent_aggregate_systems = silent_aggregate_systems;
        self
    }

    pub fn with_silent_entity_system(mut self, silent_entity_system: SilentEntitySystem) -> Self {
        self.0.silent_entity_systems.push(silent_entity_system);
        self
    }

    pub fn with_silent_entity_systems(
        mut self,
        silent_entity_systems: Vec<SilentEntitySystem>,
    ) -> Self {
        self.0.silent_entity_systems = silent_entity_systems;
        self
    }

    pub fn with_variable_datum(mut self, variable_datum: VariableDatum) -> Self {
        self.0.variable_datums.push(variable_datum);
        self
    }

    pub fn with_variable_datums(mut self, variable_datums: Vec<VariableDatum>) -> Self {
        self.0.variable_datums = variable_datums;
        self
    }
}
