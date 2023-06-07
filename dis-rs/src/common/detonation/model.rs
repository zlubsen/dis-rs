use crate::common::{BodyInfo, Interaction};
use crate::{DescriptorRecord, DetonationResult, EntityId, EntityType, EventId, ExplosiveMaterialCategories, Location, MunitionDescriptor, PduBody, PduType, VectorF32};
use crate::common::model::VariableParameter;
use crate::constants::VARIABLE_PARAMETER_RECORD_LENGTH;

const BASE_DETONATION_BODY_LENGTH : u16 = 104;

pub struct Detonation {
    pub source_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub exploding_entity_id: EntityId,
    pub event_id: EventId,
    pub velocity: VectorF32,
    pub location_in_world_coordinates: Location,
    pub descriptor: DescriptorRecord,
    pub location_in_entity_coordinates: VectorF32,
    pub detonation_result: DetonationResult,
    pub variable_parameters: Vec<VariableParameter>,
}

impl Default for Detonation {
    fn default() -> Self {
        Self::new()
    }
}

impl Detonation {
    pub fn new() -> Self {
        Self {
            source_entity_id: Default::default(),
            target_entity_id: Default::default(),
            exploding_entity_id: Default::default(),
            event_id: Default::default(),
            velocity: Default::default(),
            location_in_world_coordinates: Default::default(),
            descriptor: Default::default(),
            location_in_entity_coordinates: Default::default(),
            detonation_result: Default::default(),
            variable_parameters: vec![]
        }
    }

    pub fn with_source_entity_id(mut self, source_entity_id: EntityId) -> Self {
        self.source_entity_id = source_entity_id;
        self
    }

    pub fn with_target_entity_id(mut self, target_entity_id: EntityId) -> Self {
        self.target_entity_id = target_entity_id;
        self
    }

    pub fn with_exploding_entity_id(mut self, exploding_entity_id: EntityId) -> Self {
        self.exploding_entity_id = exploding_entity_id;
        self
    }

    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.event_id = event_id;
        self
    }

    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_world_location(mut self, location: Location) -> Self {
        self.location_in_world_coordinates = location;
        self
    }

    pub fn with_descriptor(mut self, descriptor: DescriptorRecord) -> Self {
        self.descriptor = descriptor;
        self
    }

    pub fn with_munition_descriptor(mut self, entity_type: EntityType, munition: MunitionDescriptor) -> Self {
        self.descriptor = DescriptorRecord::new_munition(entity_type, munition);
        self
    }

    pub fn with_expendable_descriptor(mut self, entity_type: EntityType) -> Self {
        self.descriptor = DescriptorRecord::Expendable { entity_type };
        self
    }

    pub fn with_explosion_descriptor(mut self,
                                     entity_type: EntityType,
                                     explosive_material: ExplosiveMaterialCategories,
                                     explosive_force: f32) -> Self {
        self.descriptor = DescriptorRecord::new_explosion(entity_type, explosive_material, explosive_force);
        self
    }

    pub fn with_entity_location(mut self, location: VectorF32) -> Self {
        self.location_in_entity_coordinates = location;
        self
    }

    pub fn with_detonation_result(mut self, detonation_result: DetonationResult) -> Self {
        self.detonation_result = detonation_result;
        self
    }

    pub fn with_variable_parameter(mut self, parameter: VariableParameter) -> Self {
        self.variable_parameters.push(parameter);
        self
    }

    pub fn with_variable_parameters(mut self, parameters: Vec<VariableParameter>) -> Self {
        self.variable_parameters = parameters;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Detonation(self)
    }
}

impl BodyInfo for Detonation {
    fn body_length(&self) -> u16 {
        BASE_DETONATION_BODY_LENGTH + (VARIABLE_PARAMETER_RECORD_LENGTH * (self.variable_parameters.len() as u16))
    }

    fn body_type(&self) -> PduType {
        PduType::Detonation
    }
}

impl Interaction for Detonation {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.source_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.target_entity_id)
    }
}