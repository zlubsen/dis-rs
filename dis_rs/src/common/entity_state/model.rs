use crate::AttachedParts;
use crate::common::entity_state::builder::EntityStateBuilder;
use crate::common::{Body, Interaction};
use crate::common::model::{EntityId, EntityType, Location, Orientation, VectorF32};
use crate::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, EntityCapabilities, EntityMarkingCharacterSet, ForceId, PduType, ProtocolFamily, VariableParameterRecordType};
use crate::v6::entity_state::model::{Appearance as AppearanceV6, DrParameters, EntityCapabilities as EntityCapabilitiesV6};

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
    pub entity_capabilities : Option<EntityCapabilities>,
    // FIXME factor out v6 and v7 variants, and distinction between articulated and attached parts > will be merged in the Variable Param
    // pub articulation_parameters: Option<Vec<ArticulationParameter>>, // optional list of records
    pub variable_parameters: Vec<VariableParameter>,
}

impl Body for EntityState {
    fn body_length(&self) -> usize {
        132 + (16 * &self.variable_parameters.len())
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