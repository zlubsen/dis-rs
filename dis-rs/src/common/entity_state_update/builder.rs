use crate::entity_state::model::EntityAppearance;
use crate::entity_state_update::model::EntityStateUpdate;
use crate::model::{EntityId, Location, Orientation, VariableParameter, VectorF32};

pub struct EntityStateUpdateBuilder(EntityStateUpdate);

impl EntityStateUpdateBuilder {
    pub fn new() -> Self {
        EntityStateUpdateBuilder(EntityStateUpdate::default())
    }

    pub fn new_from_body(body: EntityStateUpdate) -> Self {
        EntityStateUpdateBuilder(body)
    }

    pub fn build(self) -> EntityStateUpdate {
        self.0
    }

    pub fn with_entity_id(mut self, entity_id: EntityId) -> Self {
        self.0.entity_id = entity_id;
        self
    }

    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.0.entity_linear_velocity = velocity;
        self
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.0.entity_location = location;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.0.entity_orientation = orientation;
        self
    }

    pub fn with_appearance(mut self, appearance: EntityAppearance) -> Self {
        self.0.entity_appearance = appearance;
        self
    }

    pub fn with_variable_parameter(mut self, parameter: VariableParameter) -> Self {
        self.0.variable_parameters.push(parameter);
        self
    }

    pub fn with_variable_parameters(mut self, parameters: Vec<VariableParameter>) -> Self {
        self.0.variable_parameters = parameters;
        self
    }
}
