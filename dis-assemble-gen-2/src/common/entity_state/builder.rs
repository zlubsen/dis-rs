use crate::entity_state::model::{DrParameters, EntityAppearance, EntityMarking, EntityState};
use crate::enumerations::{EntityCapabilities, ForceId};
use crate::model::{EntityId, EntityType, Location, Orientation, VariableParameter, VectorF32};

pub struct EntityStateBuilder(EntityState);

impl Default for EntityStateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityStateBuilder {
    #[must_use]
    pub fn new() -> Self {
        EntityStateBuilder(EntityState::default())
    }

    #[must_use]
    pub fn new_from_body(body: EntityState) -> Self {
        EntityStateBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> EntityState {
        self.0
    }

    #[must_use]
    pub fn with_entity_id(mut self, entity_id: EntityId) -> Self {
        self.0.entity_id = entity_id;
        self
    }

    #[must_use]
    pub fn with_entity_type(mut self, entity_type: EntityType) -> Self {
        self.0.entity_type = entity_type;
        self
    }

    #[must_use]
    pub fn with_force_id(mut self, force_id: ForceId) -> Self {
        self.0.force_id = force_id;
        self
    }

    #[must_use]
    pub fn with_alternative_entity_type(mut self, entity_type: EntityType) -> Self {
        self.0.alternative_entity_type = entity_type;
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
    pub fn with_dead_reckoning_parameters(mut self, parameters: DrParameters) -> Self {
        self.0.dead_reckoning_parameters = parameters;
        self
    }

    #[must_use]
    pub fn with_marking(mut self, marking: EntityMarking) -> Self {
        self.0.entity_marking = marking;
        self
    }

    #[must_use]
    pub fn with_capabilities(mut self, capabilities: EntityCapabilities) -> Self {
        self.0.entity_capabilities = capabilities;
        self
    }

    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)]
    pub fn with_capabilities_flags(
        mut self,
        ammunition_supply: bool,
        fuel_supply: bool,
        recovery: bool,
        repair: bool,
    ) -> Self {
        use crate::v6::entity_state::model::EntityCapabilities as CapabilitiesV6;
        let v6_capabilities = CapabilitiesV6 {
            ammunition_supply,
            fuel_supply,
            recovery,
            repair,
        };
        self.0.entity_capabilities = EntityCapabilities::from(v6_capabilities);
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
