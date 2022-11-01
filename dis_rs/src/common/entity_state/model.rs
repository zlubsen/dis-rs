use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, EntityType, Location, Orientation, VectorF32};
use crate::constants::VARIABLE_PARAMETER_RECORD_LENGTH;
use crate::enumerations::*;
use crate::PduBody;

const BASE_ENTITY_STATE_BODY_LENGTH : u16 = 132;

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

    pub fn as_pdu_body(self) -> PduBody {
        PduBody::EntityState(self)
    }
}

impl BodyInfo for EntityState {
    fn body_length(&self) -> u16 {
        BASE_ENTITY_STATE_BODY_LENGTH + (VARIABLE_PARAMETER_RECORD_LENGTH * (*&self.variable_parameters.len() as u16))
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
            marking_string: String::from("default"),
        }
    }
}

#[derive(Debug)]
pub enum VariableParameter {
    Articulated(ArticulatedPart),
    Attached(AttachedPart),
    Separation(SeparationParameter),
    EntityType(EntityTypeParameter),
    EntityAssociation(EntityAssociationParameter),
    Unspecified(u8, [u8;15]),
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ArticulatedPart {
    pub change_indicator: ChangeIndicator,
    pub attachment_id: u16,
    pub type_metric: ArticulatedPartsTypeMetric,
    pub type_class: ArticulatedPartsTypeClass,
    pub parameter_value: f32,
}

impl ArticulatedPart {
    pub fn with_change_indicator(mut self, change_indicator: ChangeIndicator) -> Self {
        self.change_indicator = change_indicator;
        self
    }

    pub fn with_attachment_id(mut self, attachment_id: u16) -> Self {
        self.attachment_id = attachment_id;
        self
    }

    pub fn with_type_metric(mut self, type_metric: ArticulatedPartsTypeMetric) -> Self {
        self.type_metric = type_metric;
        self
    }

    pub fn with_type_class(mut self, type_class: ArticulatedPartsTypeClass) -> Self {
        self.type_class = type_class;
        self
    }

    pub fn with_parameter_value(mut self, parameter_value: f32) -> Self {
        self.parameter_value = parameter_value;
        self
    }

    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::Articulated(self)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct AttachedPart {
    pub detached_indicator: AttachedPartDetachedIndicator,
    pub attachment_id: u16,
    pub parameter_type: AttachedParts,
    pub attached_part_type: EntityType,
}

impl AttachedPart {
    pub fn with_detached_indicator(mut self, detached_indicator: AttachedPartDetachedIndicator) -> Self {
        self.detached_indicator = detached_indicator;
        self
    }

    pub fn with_attachment_id(mut self, attachment_id: u16) -> Self {
        self.attachment_id = attachment_id;
        self
    }

    pub fn with_parameter_type(mut self, parameter_type: AttachedParts) -> Self {
        self.parameter_type = parameter_type;
        self
    }

    pub fn with_attached_part_type(mut self, attached_part_type: EntityType) -> Self {
        self.attached_part_type = attached_part_type;
        self
    }

    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::Attached(self)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SeparationParameter {
    pub reason: SeparationReasonForSeparation,
    pub pre_entity_indicator: SeparationPreEntityIndicator,
    pub parent_entity_id: EntityId,
    pub station_name: StationName,
    pub station_number: u16,
}

impl SeparationParameter {
    pub fn with_reason(mut self, reason: SeparationReasonForSeparation) -> Self {
        self.reason = reason;
        self
    }

    pub fn with_pre_entity_indicator(mut self, pre_entity_indicator: SeparationPreEntityIndicator) -> Self {
        self.pre_entity_indicator = pre_entity_indicator;
        self
    }

    pub fn with_parent_entity_id(mut self, parent_entity_id: EntityId) -> Self {
        self.parent_entity_id = parent_entity_id;
        self
    }

    pub fn with_station_name(mut self, station_name: StationName) -> Self {
        self.station_name = station_name;
        self
    }

    pub fn with_station_number(mut self, station_number: u16) -> Self {
        self.station_number = station_number;
        self
    }

    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::Separation(self)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct EntityTypeParameter {
    pub change_indicator: ChangeIndicator,
    pub entity_type: EntityType,
}

impl EntityTypeParameter {
    pub fn with_change_indicator(mut self, change_indicator: ChangeIndicator) -> Self {
        self.change_indicator = change_indicator;
        self
    }

    pub fn with_entity_type(mut self, entity_type: EntityType) -> Self {
        self.entity_type = entity_type;
        self
    }

    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::EntityType(self)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityAssociationParameter {
    pub change_indicator: ChangeIndicator,
    pub association_status: EntityAssociationAssociationStatus,
    pub association_type: EntityAssociationPhysicalAssociationType,
    pub entity_id: EntityId,
    pub own_station_location: StationName,
    pub physical_connection_type: EntityAssociationPhysicalConnectionType,
    pub group_member_type: EntityAssociationGroupMemberType,
    pub group_number: u16,
}

impl EntityAssociationParameter {
    pub fn with_change_indicator(mut self, change_indicator: ChangeIndicator) -> Self {
        self.change_indicator = change_indicator;
        self
    }

    pub fn with_association_status(mut self, association_status: EntityAssociationAssociationStatus) -> Self {
        self.association_status = association_status;
        self
    }

    pub fn with_association_type(mut self, association_type: EntityAssociationPhysicalAssociationType) -> Self {
        self.association_type = association_type;
        self
    }

    pub fn with_entity_id(mut self, entity_id: EntityId) -> Self {
        self.entity_id = entity_id;
        self
    }

    pub fn with_own_station_location(mut self, own_station_location: StationName) -> Self {
        self.own_station_location = own_station_location;
        self
    }

    pub fn with_physical_connection_type(mut self, physical_connection_type: EntityAssociationPhysicalConnectionType) -> Self {
        self.physical_connection_type = physical_connection_type;
        self
    }

    pub fn with_group_member_type(mut self, group_member_type: EntityAssociationGroupMemberType) -> Self {
        self.group_member_type = group_member_type;
        self
    }

    pub fn with_group_number(mut self, group_number: u16) -> Self {
        self.group_number = group_number;
        self
    }

    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::EntityAssociation(self)
    }
}

pub struct DrParameters {
    pub algorithm : DeadReckoningAlgorithm,
    pub other_parameters : DrOtherParameters,
    pub linear_acceleration : VectorF32,
    pub angular_velocity : VectorF32,
}

impl Default for DrParameters {
    fn default() -> Self {
        Self {
            algorithm: DeadReckoningAlgorithm::default(),
            other_parameters: DrOtherParameters::default(),
            linear_acceleration: VectorF32::default(),
            angular_velocity: VectorF32::default(),
        }
    }
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

#[derive(Default)]
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

#[derive(Default)]
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
