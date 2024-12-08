use crate::common::detonation::model::Detonation;
use crate::common::model::{
    DescriptorRecord, EntityId, EntityType, EventId, Location, MunitionDescriptor,
    VariableParameter, VectorF32,
};
use crate::enumerations::{DetonationResult, ExplosiveMaterialCategories};

pub struct DetonationBuilder(Detonation);

impl Default for DetonationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DetonationBuilder {
    #[must_use]
    pub fn new() -> Self {
        DetonationBuilder(Detonation::default())
    }

    #[must_use]
    pub fn new_from_body(body: Detonation) -> Self {
        DetonationBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> Detonation {
        self.0
    }

    #[must_use]
    pub fn with_source_entity_id(mut self, source_entity_id: EntityId) -> Self {
        self.0.source_entity_id = source_entity_id;
        self
    }

    #[must_use]
    pub fn with_target_entity_id(mut self, target_entity_id: EntityId) -> Self {
        self.0.target_entity_id = target_entity_id;
        self
    }

    #[must_use]
    pub fn with_exploding_entity_id(mut self, exploding_entity_id: EntityId) -> Self {
        self.0.exploding_entity_id = exploding_entity_id;
        self
    }

    #[must_use]
    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.0.event_id = event_id;
        self
    }

    #[must_use]
    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.0.velocity = velocity;
        self
    }

    #[must_use]
    pub fn with_world_location(mut self, location: Location) -> Self {
        self.0.location_in_world_coordinates = location;
        self
    }

    #[must_use]
    pub fn with_descriptor(mut self, descriptor: DescriptorRecord) -> Self {
        self.0.descriptor = descriptor;
        self
    }

    #[must_use]
    pub fn with_munition_descriptor(
        mut self,
        entity_type: EntityType,
        munition: MunitionDescriptor,
    ) -> Self {
        self.0.descriptor = DescriptorRecord::new_munition(entity_type, munition);
        self
    }

    #[must_use]
    pub fn with_expendable_descriptor(mut self, entity_type: EntityType) -> Self {
        self.0.descriptor = DescriptorRecord::Expendable { entity_type };
        self
    }

    #[must_use]
    pub fn with_explosion_descriptor(
        mut self,
        entity_type: EntityType,
        explosive_material: ExplosiveMaterialCategories,
        explosive_force: f32,
    ) -> Self {
        self.0.descriptor =
            DescriptorRecord::new_explosion(entity_type, explosive_material, explosive_force);
        self
    }

    #[must_use]
    pub fn with_entity_location(mut self, location: VectorF32) -> Self {
        self.0.location_in_entity_coordinates = location;
        self
    }

    #[must_use]
    pub fn with_detonation_result(mut self, detonation_result: DetonationResult) -> Self {
        self.0.detonation_result = detonation_result;
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
