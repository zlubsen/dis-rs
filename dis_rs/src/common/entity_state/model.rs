use buildstructor;
use crate::common::{BodyInfo, Interaction};
use crate::common::entity_state::builder::EntityStateBuilder;
use crate::common::model::{EntityId, EntityType, Location, Orientation, VectorF32};
use crate::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, AttachedParts, EntityCapabilities, EntityMarkingCharacterSet, ForceId, PduType, VariableParameterRecordType};
use crate::enumerations::{LandPlatformAppearance, AirPlatformAppearance, SurfacePlatformAppearance, SubsurfacePlatformAppearance, SpacePlatformAppearance, MunitionAppearance, LifeFormsAppearance, EnvironmentalAppearance, CulturalFeatureAppearance, SupplyAppearance, RadioAppearance, ExpendableAppearance, SensorEmitterAppearance};
use crate::v6::entity_state::model::{DrParameters};

const BASE_ENTITY_STATE_BODY_LENGTH : u16 = 132;
const VARIABLE_PARAMETER_RECORD_LENGTH : u16 = 16;

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
    pub entity_appearance: EntityAppearance, // enum
    pub dead_reckoning_parameters : DrParameters, // struct
    pub entity_marking : EntityMarking, // struct
    pub entity_capabilities : EntityCapabilities, // enum
    pub variable_parameters: Vec<VariableParameter>,
}

impl BodyInfo for EntityState {
    fn body_length(&self) -> u16 {
        BASE_ENTITY_STATE_BODY_LENGTH + (VARIABLE_PARAMETER_RECORD_LENGTH * (*&self.variable_parameters.len() as u16))
    }

    fn body_type(&self) -> PduType {
        PduType::EntityState
    }
}

pub enum EntityAppearance {
    LandPlatform(LandPlatformAppearance),
    AirPlatform(AirPlatformAppearance),
    SurfacePlatform(SurfacePlatformAppearance),
    SubsurfacePlatform(SubsurfacePlatformAppearance),
    SpacePlatform(SpacePlatformAppearance),
    Munition(MunitionAppearance),
    LifeForms(LifeFormsAppearance),
    Environmental(EnvironmentalAppearance),
    CulturalFeature(CulturalFeatureAppearance),
    Supply(SupplyAppearance),
    Radio(RadioAppearance),
    Expendable(ExpendableAppearance),
    SensorEmitter(SensorEmitterAppearance),
    Unspecified(u32),
}

impl Default for EntityAppearance {
    fn default() -> Self {
        Self::Unspecified(0u32)
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
            entity_appearance: EntityAppearance::default(),
            dead_reckoning_parameters: DrParameters::default(),
            entity_marking: EntityMarking::default(), // TODO: EntityMarking::default_for_entity_type(&entity_type), // based on enumerations file
            entity_capabilities: EntityCapabilities::default(),
            variable_parameters: vec![]
        }
    }

    // #[builder]
    // fn macro_new(entity_id: EntityId, force_id: ForceId, entity_type: EntityType,
    //     alternative_entity_type : Option<EntityType>, entity_linear_velocity: Option<VectorF32>, entity_location: Option<Location>, entity_orientation: ) -> Self {
    //     Self {
    //         entity_id,
    //         force_id,
    //         entity_type,
    //         alternative_entity_type: alternative_entity_type.unwrap_or(Default::default()),
    //         entity_linear_velocity: Default::default(),
    //         entity_location: Default::default(),
    //         entity_orientation: Default::default(),
    //         entity_appearance_v6: Default::default(),
    //         dead_reckoning_parameters: Default::default(),
    //         entity_marking: Default::default(),
    //         entity_capabilities_v6: None,
    //         entity_capabilities: None,
    //         variable_parameters: vec![]
    //     }
    // }

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