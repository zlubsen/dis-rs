use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, EntityType, Location, Orientation, PduBody, VariableParameter, VectorF32};
use crate::constants::VARIABLE_PARAMETER_RECORD_LENGTH;
use crate::enumerations::{ForceId, EntityCapabilities, PduType, EntityMarkingCharacterSet, LandPlatformAppearance, AirPlatformAppearance, SurfacePlatformAppearance, SubsurfacePlatformAppearance, SpacePlatformAppearance, MunitionAppearance, LifeFormsAppearance, EnvironmentalAppearance, CulturalFeatureAppearance, RadioAppearance, ExpendableAppearance, SensorEmitterAppearance, SupplyAppearance, DeadReckoningAlgorithm};

const BASE_ENTITY_STATE_BODY_LENGTH : u16 = 132;

// TODO sensible errors for EntityState
#[allow(dead_code)]
pub enum EntityStateValidationError {
    SomeFieldNotOkError,
}

#[derive(Debug, PartialEq)]
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
            entity_marking: EntityMarking::default(),
            entity_capabilities: EntityCapabilities::default(),
            variable_parameters: vec![]
        }
    }

    pub fn with_alternative_entity_type(mut self, entity_type: EntityType) -> Self {
        self.alternative_entity_type = entity_type;
        self
    }

    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.entity_linear_velocity = velocity;
        self
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.entity_location = location;
        self
    }

    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.entity_orientation = orientation;
        self
    }

    pub fn with_appearance(mut self, appearance: EntityAppearance) -> Self {
        self.entity_appearance = appearance;
        self
    }

    pub fn with_dead_reckoning_parameters(mut self, parameters: DrParameters) -> Self {
        self.dead_reckoning_parameters = parameters;
        self
    }

    pub fn with_marking(mut self, marking: EntityMarking) -> Self {
        self.entity_marking = marking;
        self
    }

    pub fn with_capabilities(mut self, capabilities: EntityCapabilities) -> Self {
        self.entity_capabilities = capabilities;
        self
    }

    pub fn with_capabilities_flags(mut self,
                                   ammunition_supply : bool,
                                   fuel_supply : bool,
                                   recovery : bool,
                                   repair : bool) -> Self {
        use crate::v6::entity_state::model::EntityCapabilities as CapabilitiesV6;
        let v6_capabilities = CapabilitiesV6 {
            ammunition_supply,
            fuel_supply,
            recovery,
            repair,
        };
        self.entity_capabilities = EntityCapabilities::from(v6_capabilities);
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

#[derive(Debug, PartialEq)]
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
    Unspecified([u8;4]),
}

impl Default for EntityAppearance {
    fn default() -> Self {
        Self::Unspecified(0u32.to_be_bytes())
    }
}

#[derive(Debug, PartialEq)]
pub struct EntityMarking {
    pub marking_character_set : EntityMarkingCharacterSet,
    pub marking_string : String, // 11 byte String
}

impl EntityMarking {
    pub fn new(marking: String, character_set: EntityMarkingCharacterSet) -> Self {
        Self {
            marking_character_set: character_set,
            marking_string: marking
        }
    }

    pub fn with_marking(mut self, marking: String) -> Self {
        self.marking_string = marking;
        self
    }
}

impl Default for EntityMarking {
    fn default() -> Self {
        Self {
            marking_character_set: EntityMarkingCharacterSet::default(),
            marking_string: String::from("Default"),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Default, Debug, PartialEq)]
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

#[derive(Default, Debug, PartialEq)]
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
