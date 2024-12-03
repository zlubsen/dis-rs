use crate::entity_state::model::EntityAppearance;
use crate::entity_state_update::model::EntityStateUpdate;
use crate::model::{EntityId, Location, Orientation, VariableParameter, VectorF32};

pub struct EntityStateUpdateBuilder(EntityStateUpdate);

impl EntityStateUpdateBuilder {
    #[must_use]
    pub fn new() -> Self {
        EntityStateUpdateBuilder(EntityStateUpdate::default())
    }

    #[must_use]
    pub fn new_from_body(body: EntityStateUpdate) -> Self {
        EntityStateUpdateBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> EntityStateUpdate {
        self.0
    }

    #[must_use]
    pub fn with_entity_id(mut self, entity_id: EntityId) -> Self {
        self.0.entity_id = entity_id;
        self
    }

    #[must_use]
    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.0.entity_linear_velocity = velocity;
        self
    }

    #[must_use]
    pub fn with_location(mut self, location: Location) -> Self {
        self.0.entity_location = location;
        self
    }

    #[must_use]
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.0.entity_orientation = orientation;
        self
    }

    #[must_use]
    pub fn with_appearance(mut self, appearance: EntityAppearance) -> Self {
        self.0.entity_appearance = appearance;
        self
    }

    #[must_use]
    pub fn with_variable_parameter(mut self, parameter: VariableParameter) -> Self {
        self.0.variable_parameters.push(parameter);
        self
    }

    #[must_use]
    pub fn with_variable_parameters(mut self, parameters: Vec<VariableParameter>) -> Self {
        self.0.variable_parameters = parameters;
        self
    }
}
