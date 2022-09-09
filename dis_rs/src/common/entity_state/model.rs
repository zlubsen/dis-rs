use crate::AttachedParts;
use crate::common::entity_state::builder::EntityStateBuilder;
use crate::common::{Body, Interaction};
use crate::common::model::{EntityId, EntityType, Location, Orientation, VectorF32};
use crate::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, DeadReckoningAlgorithm, EntityMarkingCharacterSet, ForceId, VariableParameterRecordType, EntityCapabilities, PduType::EntityState, ProtocolFamily::EntityInformationInteraction};
use crate::v6::entity_state::model::{Appearance as AppearanceV6, EntityCapabilities as EntityCapabilitiesV6};

// TODO sensible errors for EntityState
pub enum EntityStateValidationError {
    SomeFieldNotOkError,
}

pub struct EntityState {
    pub entity_id : EntityId, // struct
    pub force_id : ForceId, // enum
    pub entity_type : EntityType, // struct
    pub alternative_entity_type : EntityType, // struct
    pub entity_linear_velocity : VectorF32, // struct
    pub entity_location : Location, // struct
    pub entity_orientation : Orientation, // struct
    pub entity_appearance_v6: AppearanceV6, // struct
    pub dead_reckoning_parameters : DrParameters, // struct
    pub entity_marking : EntityMarking, // struct
    pub entity_capabilities_v6 : Option<EntityCapabilitiesV6>, // struct
    pub entity_capabilities : Option<EntityCapabilities>,
    pub articulation_parameter : Option<Vec<ArticulationParameter>>, // optional list of records
}

impl Body for EntityState {
    fn body_length(&self) -> usize {
        132 + if let Some(params) = &self.articulation_parameter {
            16 *params.len()
        } else { 0 }
    }

    fn body_type(&self) -> PduType {
        PduType::EntityState
    }

    fn protocol_family(&self) -> ProtocolFamily {
        ProtocolFamily::EntityInformationInteraction
    }
}

pub struct EntityMarking {
    pub marking_character_set : EntityMarkingCharacterSet,
    pub marking_string : String, // 11 byte String
}

pub struct DrParameters {
    pub algorithm : DeadReckoningAlgorithm,
    pub other_parameters : [u8; 15],
    pub linear_acceleration : VectorF32,
    pub angular_velocity : VectorF32,
}

pub struct ArticulationParameter {
    pub parameter_type_designator : VariableParameterRecordType,
    pub parameter_change_indicator : u8,
    pub articulation_attachment_id: u16,
    pub parameter_type_variant : ParameterTypeVariant,
    pub articulation_parameter_value : f32,
}

#[derive(Debug, PartialEq)]
pub enum ParameterTypeVariant {
    Attached(AttachedParts),
    Articulated(ArticulatedParts),
    // TODO add following variants
    // Separation
    // Entity Type
    // Entity Association
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ArticulatedParts {
    pub type_metric : ArticulatedPartsTypeMetric,
    pub type_class : ArticulatedPartsTypeClass,
}

// TODO refactor builder
impl EntityState {
    pub fn builder() -> EntityStateBuilder {
        EntityStateBuilder::new()
    }
}

impl Interaction for EntityState {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}