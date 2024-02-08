use crate::common::{BodyInfo, Interaction};
use crate::common::detonation::builder::DetonationBuilder;
use crate::enumerations::{DetonationResult, PduType};
use crate::common::model::{DescriptorRecord, EntityId, EventId, Location, PduBody, VectorF32, VariableParameter};
use crate::constants::VARIABLE_PARAMETER_RECORD_LENGTH;

const BASE_DETONATION_BODY_LENGTH : u16 = 104;

#[derive(Clone, Debug, Default, PartialEq)]
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

impl Detonation {
    pub fn builder() -> DetonationBuilder {
        DetonationBuilder::new()
    }

    pub fn into_builder(self) -> DetonationBuilder {
        DetonationBuilder::new_from_body(self)
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
