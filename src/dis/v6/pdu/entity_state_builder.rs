use std::error::Error;
use std::mem::size_of;
use crate::dis::v6::pdu::*;
use crate::dis::v6::pdu::entity_state::*;
use crate::dis::v6::pdu::PduType::EntityState;

pub struct EntityStateBuilder {
    header : Option<PduHeader>,
    entity_id : Option<EntityId>,
    force_id : Option<ForceId>,
    entity_type : Option<EntityType>,
    alternative_entity_type : Option<EntityType>,
    entity_linear_velocity : Option<VectorF32>,
    entity_location : Option<Location>,
    entity_orientation : Option<Orientation>,
    entity_appearance : Option<Appearance>,
    dead_reckoning_parameters : Option<DrParameters>,
    entity_marking : Option<EntityMarking>,
    entity_capabilities : Option<EntityCapabilities>,
    articulation_parameter : Vec<ArticulationParameter>,
}

pub enum EntityStateValidationError {
    SomeFieldNotOkError,
}

impl EntityStateBuilder {
    pub(crate) fn new() -> EntityStateBuilder {
        EntityStateBuilder {
            header: None,
            entity_id: None,
            force_id: None,
            entity_type: None,
            alternative_entity_type: None,
            entity_linear_velocity: None,
            entity_location: None,
            entity_orientation: None,
            entity_appearance: None,
            dead_reckoning_parameters: None,
            entity_marking: None,
            entity_capabilities: None,
            articulation_parameter: vec![]
        }
    }

    fn validate(&self) -> Result<bool, EntityStateValidationError> {
        // TODO "Check if all fields are filled in properly; required are set, valid values..., based on the standard"

        return if self.header.is_some() &&
            self.entity_id.is_some() &&
            self.force_id.is_some() &&
            self.entity_type.is_some() &&
            self.alternative_entity_type.is_some() &&
            self.entity_linear_velocity.is_some() &&
            self.entity_location.is_some() &&
            self.entity_orientation.is_some() &&
            self.entity_appearance.is_some() &&
            self.dead_reckoning_parameters.is_some() &&
            self.entity_marking.is_some() &&
            self.entity_capabilities.is_some() {
            Ok(true)
        } else { Err(EntityStateValidationError::SomeFieldNotOkError )}
    }

    pub fn build(self) -> EntityState {
        self.validate();

        EntityState {
            header: self.header.expect("Value expected, but not found."),
            entity_id: self.entity_id.expect("Value expected, but not found."),
            force_id: self.force_id.expect("Value expected, but not found."),
            articulated_parts_no: self.articulation_parameter.expect("").len() as u8,
            entity_type: self.entity_type.expect("Value expected, but not found."),
            alternative_entity_type: self.alternative_entity_type.expect("Value expected, but not found."),
            entity_linear_velocity: self.entity_linear_velocity.expect("Value expected, but not found."),
            entity_location: self.entity_location.expect("Value expected, but not found."),
            entity_orientation: self.entity_orientation.expect("Value expected, but not found."),
            entity_appearance: self.entity_appearance.expect("Value expected, but not found."),
            dead_reckoning_parameters: self.dead_reckoning_parameters.expect("Value expected, but not found."),
            entity_marking: self.entity_marking.expect("Value expected, but not found."),
            entity_capabilities: self.entity_capabilities.expect("Value expected, but not found."),
            articulation_parameter: if self.articulation_parameter.is_empty() { Some(self.articulation_parameter) } else { None },
        }
    }
}