use std::str::FromStr;
use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, EntityType, Location, Orientation, PduBody, VariableParameter, VectorF32};
use crate::constants::{FOUR_OCTETS, TWELVE_OCTETS, VARIABLE_PARAMETER_RECORD_LENGTH};
use crate::DisError;
use crate::entity_state::builder::EntityStateBuilder;
use crate::enumerations::{ForceId, EntityCapabilities, PduType, EntityMarkingCharacterSet, LandPlatformAppearance, AirPlatformAppearance, SurfacePlatformAppearance, SubsurfacePlatformAppearance, SpacePlatformAppearance, MunitionAppearance, LifeFormsAppearance, EnvironmentalAppearance, CulturalFeatureAppearance, RadioAppearance, ExpendableAppearance, SensorEmitterAppearance, SupplyAppearance, DeadReckoningAlgorithm, EntityKind, PlatformDomain};

const BASE_ENTITY_STATE_BODY_LENGTH : u16 = 132;

// TODO sensible errors for EntityState
#[allow(dead_code)]
pub enum EntityStateValidationError {
    SomeFieldNotOkError,
}

/// 5.3.2 Entity State PDU
///
/// 7.2.2 Entity State PDU
#[derive(Clone, Debug, Default, PartialEq)]
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

impl EntityState {
    pub fn builder() -> EntityStateBuilder {
        EntityStateBuilder::new()
    }

    pub fn into_builder(self) -> EntityStateBuilder {
        EntityStateBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::EntityState(self)
    }
}

impl BodyInfo for EntityState {
    fn body_length(&self) -> u16 {
        BASE_ENTITY_STATE_BODY_LENGTH + (VARIABLE_PARAMETER_RECORD_LENGTH * (self.variable_parameters.len() as u16))
    }

    fn body_type(&self) -> PduType {
        PduType::EntityState
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

/// 6.2.26 Entity Appearance record
#[derive(Copy, Clone, Debug, PartialEq)]
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
    Unspecified([u8;FOUR_OCTETS]),
}

impl Default for EntityAppearance {
    fn default() -> Self {
        Self::Unspecified(0u32.to_be_bytes())
    }
}

impl EntityAppearance {
    pub fn from_bytes(appearance: u32, entity_type: &EntityType) -> Self {
        match (entity_type.kind, entity_type.domain) {
            (EntityKind::Other, _) => EntityAppearance::Unspecified(appearance.to_be_bytes()),
            (EntityKind::Platform, PlatformDomain::Land) => EntityAppearance::LandPlatform(LandPlatformAppearance::from(appearance)),
            (EntityKind::Platform, PlatformDomain::Air) => EntityAppearance::AirPlatform(AirPlatformAppearance::from(appearance)),
            (EntityKind::Platform, PlatformDomain::Surface) => EntityAppearance::SurfacePlatform(SurfacePlatformAppearance::from(appearance)),
            (EntityKind::Platform, PlatformDomain::Subsurface) => EntityAppearance::SubsurfacePlatform(SubsurfacePlatformAppearance::from(appearance)),
            (EntityKind::Platform, PlatformDomain::Space) => EntityAppearance::SpacePlatform(SpacePlatformAppearance::from(appearance)),
            (EntityKind::Munition, _) => EntityAppearance::Munition(MunitionAppearance::from(appearance)),
            (EntityKind::LifeForm, _) => EntityAppearance::LifeForms(LifeFormsAppearance::from(appearance)),
            (EntityKind::Environmental, _) => EntityAppearance::Environmental(EnvironmentalAppearance::from(appearance)),
            (EntityKind::CulturalFeature, _) => EntityAppearance::CulturalFeature(CulturalFeatureAppearance::from(appearance)),
            (EntityKind::Supply, _) => EntityAppearance::Supply(SupplyAppearance::from(appearance)),
            (EntityKind::Radio, _) => EntityAppearance::Radio(RadioAppearance::from(appearance)),
            (EntityKind::Expendable, _) => EntityAppearance::Expendable(ExpendableAppearance::from(appearance)),
            (EntityKind::SensorEmitter, _) => EntityAppearance::SensorEmitter(SensorEmitterAppearance::from(appearance)),
            (_, _) => EntityAppearance::Unspecified(appearance.to_be_bytes())
        }
    }
    pub fn record_length(&self) -> u16 {
        FOUR_OCTETS as u16
    }
}

impl From<&EntityAppearance> for u32 {
    fn from(value: &EntityAppearance) -> Self {
        match value {
            EntityAppearance::LandPlatform(appearance) => u32::from(*appearance),
            EntityAppearance::AirPlatform(appearance) => u32::from(*appearance),
            EntityAppearance::SurfacePlatform(appearance) => u32::from(*appearance),
            EntityAppearance::SubsurfacePlatform(appearance) => u32::from(*appearance),
            EntityAppearance::SpacePlatform(appearance) => u32::from(*appearance),
            EntityAppearance::Munition(appearance) => u32::from(*appearance),
            EntityAppearance::LifeForms(appearance) => u32::from(*appearance),
            EntityAppearance::Environmental(appearance) => u32::from(*appearance),
            EntityAppearance::CulturalFeature(appearance) => u32::from(*appearance),
            EntityAppearance::Supply(appearance) => u32::from(*appearance),
            EntityAppearance::Radio(appearance) => u32::from(*appearance),
            EntityAppearance::Expendable(appearance) => u32::from(*appearance),
            EntityAppearance::SensorEmitter(appearance) => u32::from(*appearance),
            EntityAppearance::Unspecified(appearance) => u32::from_be_bytes(*appearance),
        }
    }
}

/// 6.2.29 Entity Marking record
#[derive(Clone, Debug, PartialEq)]
pub struct EntityMarking {
    pub marking_character_set : EntityMarkingCharacterSet,
    pub marking_string : String, // 11 byte String
}

impl EntityMarking {
    pub fn new(marking: impl Into<String>, character_set: EntityMarkingCharacterSet) -> Self {
        Self {
            marking_character_set: character_set,
            marking_string: marking.into()
        }
    }

    pub fn new_ascii<S: Into<String>>(marking: S) -> Self {
        EntityMarking::new(marking.into(), EntityMarkingCharacterSet::ASCII)
    }

    pub fn with_marking<S: Into<String>>(mut self, marking: S) -> Self {
        self.marking_string = marking.into();
        self
    }

    pub fn record_length(&self) -> u16 {
        TWELVE_OCTETS as u16
    }
}

impl Default for EntityMarking {
    fn default() -> Self {
        Self {
            marking_character_set: EntityMarkingCharacterSet::ASCII,
            marking_string: String::from("Marking"),
        }
    }
}

impl FromStr for EntityMarking {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 11 {
            Ok(Self {
                marking_character_set: EntityMarkingCharacterSet::ASCII,
                marking_string: s.to_string()
            })
        } else {
            Err(DisError::ParseError(format!("String is too long for EntityMarking. Found {}, max 11 allowed.", s.len())))
        }
    }
}

/// Custom defined record to group Dead Reckoning Parameters
#[derive(Clone, Default, Debug, PartialEq)]
pub struct DrParameters {
    pub algorithm : DeadReckoningAlgorithm,
    pub other_parameters : DrOtherParameters,
    pub linear_acceleration : VectorF32,
    pub angular_velocity : VectorF32,
}

impl DrParameters {
    pub fn with_algorithm(mut self, algorithm: DeadReckoningAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    pub fn with_parameters(mut self, parameters: DrOtherParameters) -> Self {
        self.other_parameters = parameters;
        self
    }

    pub fn with_linear_acceleration(mut self, linear_acceleration: VectorF32) -> Self {
        self.linear_acceleration = linear_acceleration;
        self
    }

    pub fn with_angular_velocity(mut self, angular_velocity: VectorF32) -> Self {
        self.angular_velocity = angular_velocity;
        self
    }
}

/// E.8 Use of the Other Parameters field in Dead Reckoning Parameters
#[derive(Clone, Debug, PartialEq)]
pub enum DrOtherParameters {
    None([u8; 15]),
    LocalEulerAngles(DrEulerAngles),
    WorldOrientationQuaternion(DrWorldOrientationQuaternion),
}

impl Default for DrOtherParameters {
    fn default() -> Self {
        Self::None([0u8;15])
    }
}

/// Identical to Table 58—Euler Angles record / 6.2.32 Euler Angles record (which is modeled as `VectorF32`)
#[derive(Clone, Default, Debug, PartialEq)]
pub struct DrEulerAngles {
    pub local_yaw : f32,
    pub local_pitch : f32,
    pub local_roll : f32,
}

impl DrEulerAngles {
    pub fn with_local_yaw(mut self, local_yaw: f32) -> Self {
        self.local_yaw = local_yaw;
        self
    }

    pub fn with_local_pitch(mut self, local_pitch: f32) -> Self {
        self.local_pitch = local_pitch;
        self
    }

    pub fn with_local_roll(mut self, local_roll: f32) -> Self {
        self.local_roll = local_roll;
        self
    }
}

/// Table E.3—World Orientation Quaternion Dead Reckoning Parameters (E.8.2.3 Rotating DRM entities)
#[derive(Clone, Default, Debug, PartialEq)]
pub struct DrWorldOrientationQuaternion {
    pub nil : u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl DrWorldOrientationQuaternion {
    pub fn with_nil(mut self, nil: u16) -> Self {
        self.nil = nil;
        self
    }

    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

}
