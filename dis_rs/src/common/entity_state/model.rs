use buildstructor;
use crate::common::entity_state::builder::EntityStateBuilder;
use crate::common::{Body, Interaction};
use crate::common::model::{EntityId, EntityType, Location, Orientation, VectorF32};
use crate::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, AttachedParts, EntityCapabilities as EntityCapabilitiesV7, EntityMarkingCharacterSet, ForceId, PduType, ProtocolFamily, VariableParameterRecordType};
use crate::v6::entity_state::model::{Appearance as AppearanceV6, DrParameters, EntityCapabilities as EntityCapabilitiesV6};

const BASE_ENTITY_STATE_BODY_LENGTH : usize = 132;
const VARIABLE_PARAMETER_RECORD_LENGTH : usize = 16;

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
    // TODO
    pub entity_appearance_v6: AppearanceV6, // struct
    pub dead_reckoning_parameters : DrParameters, // struct
    pub entity_marking : EntityMarking, // struct
    pub entity_capabilities_v6 : Option<EntityCapabilitiesV6>, // struct
    pub entity_capabilities : Option<EntityCapabilitiesV7>,
    pub variable_parameters: Vec<VariableParameter>,
}

impl Body for EntityState {
    fn body_length(&self) -> usize {
        BASE_ENTITY_STATE_BODY_LENGTH + (VARIABLE_PARAMETER_RECORD_LENGTH * &self.variable_parameters.len())
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

impl Default for EntityMarking {
    fn default() -> Self {
        Self {
            marking_character_set: EntityMarkingCharacterSet::default(),
            marking_string: String::from("default"),
        }
    }
}

pub struct VariableParameter {
    pub parameter_type_designator : VariableParameterRecordType,
    pub changed_attached_indicator: u8,
    pub articulation_attachment_id: u16,
    pub parameter: ParameterVariant,
}

#[derive(Debug, PartialEq)]
pub enum ParameterVariant {
    Attached(AttachedPart),
    Articulated(ArticulatedPart),
    // TODO add following variants
    // Separation
    // Entity Type
    // Entity Association
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttachedPart {
    pub parameter_type: AttachedParts,
    pub attached_part_type: EntityType,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ArticulatedPart {
    pub type_metric : ArticulatedPartsTypeMetric,
    pub type_class : ArticulatedPartsTypeClass,
    pub parameter_value: f32,
}

// TODO refactor builder
#[buildstructor::buildstructor]
impl EntityState {
    pub fn new(entity_id: EntityId, force_id: ForceId, entity_type: EntityType) -> Self {
        Self {
            entity_id,
            force_id,
            entity_type,
            alternative_entity_type: EntityType::default(),
            entity_linear_velocity: VectorF32::default(),
            entity_location: Location::default(),
            entity_orientation: Orientation::default(),
            entity_appearance_v6: AppearanceV6::default(),
            dead_reckoning_parameters: DrParameters::default(),
            entity_marking: EntityMarking::default(), // TODO: EntityMarking::default_for_entity_type(&entity_type), // based on enumerations file
            entity_capabilities_v6: None,
            entity_capabilities: Some(EntityCapabilitiesV7::default()),
            variable_parameters: vec![]
        }
    }

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