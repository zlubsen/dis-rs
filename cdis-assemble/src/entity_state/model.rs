use dis_rs::enumerations::{DeadReckoningAlgorithm};
use crate::{BodyProperties, CdisBody};
use crate::records::model::{AngularVelocity, CdisEntityMarking, CdisRecord, CdisVariableParameter, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, Units, WorldCoordinates};
use crate::types::model::{VarInt, UVINT32, UVINT8};

#[derive(Clone, Debug, PartialEq)]
pub struct EntityState {
    pub units: Units,
    pub full_update_flag: bool,
    pub entity_id: EntityId,
    pub force_id: Option<UVINT8>,
    pub entity_type: Option<EntityType>,
    pub alternate_entity_type: Option<EntityType>,
    pub entity_linear_velocity: Option<LinearVelocity>,
    pub entity_location: Option<WorldCoordinates>,
    pub entity_orientation: Option<Orientation>,
    pub entity_appearance: Option<CdisEntityAppearance>,
    pub dr_algorithm: DeadReckoningAlgorithm,
    pub dr_params_other: Option<u128>,
    pub dr_params_entity_linear_acceleration: Option<LinearAcceleration>,
    pub dr_params_entity_angular_velocity: Option<AngularVelocity>,
    pub entity_marking: Option<CdisEntityMarking>,
    pub capabilities: Option<CdisEntityCapabilities>,
    pub variable_parameters: Vec<CdisVariableParameter>
}

impl BodyProperties for EntityState {
    type FieldsPresent = EntityStateFieldsPresent;
    type FieldsPresentOutput = u16;
    const FIELDS_PRESENT_LENGTH: usize = 13;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        0
            | if self.force_id.is_some() { Self::FieldsPresent::FORCE_ID_BIT } else { 0 }
            | if !self.variable_parameters.is_empty() { Self::FieldsPresent::VP_BIT } else { 0 }
            | if self.entity_type.is_some() { Self::FieldsPresent::ENTITY_TYPE_BIT } else { 0 }
            | if self.alternate_entity_type.is_some() { Self::FieldsPresent::ALT_ENTITY_TYPE_BIT } else { 0 }
            | if self.entity_linear_velocity.is_some() { Self::FieldsPresent::LINEAR_VELOCITY_BIT } else { 0 }
            | if self.entity_location.is_some() { Self::FieldsPresent::ENTITY_LOCATION_BIT } else { 0 }
            | if self.entity_orientation.is_some() { Self::FieldsPresent::ENTITY_ORIENTATION_BIT } else { 0 }
            | if self.entity_appearance.is_some() { Self::FieldsPresent::ENTITY_APPEARANCE_BIT } else { 0 }
            | if self.dr_params_other.is_some() { Self::FieldsPresent::DR_OTHER_BIT } else { 0 }
            | if self.dr_params_entity_linear_acceleration.is_some() { Self::FieldsPresent::DR_LINEAR_ACCELERATION_BIT } else { 0 }
            | if self.dr_params_entity_angular_velocity.is_some() { Self::FieldsPresent::DR_ANGULAR_VELOCITY_BIT } else { 0 }
            | if self.entity_marking.is_some() { Self::FieldsPresent::MARKING_BIT } else { 0 }
            | if self.capabilities.is_some() { Self::FieldsPresent::CAPABILITIES_BIT } else { 0 }
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = 6;
        Self::FIELDS_PRESENT_LENGTH + CONST_BIT_SIZE
            + self.entity_id.record_length()
            + if let Some(force_id) = self.force_id { force_id.bit_size() } else { 0 }
            + if !self.variable_parameters.is_empty() { UVINT8::from(self.variable_parameters.len() as u8).bit_size() } else { 0 }
            + if let Some(record) = self.entity_type { record.record_length() } else { 0 }
            + if let Some(record) = self.alternate_entity_type { record.record_length() } else { 0 }
            + if let Some(record) = self.entity_linear_velocity { record.record_length() } else { 0 }
            + if let Some(record) = self.entity_location { record.record_length() } else { 0 }
            + if let Some(record) = self.entity_orientation { record.record_length() } else { 0 }
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::EntityState(self)
    }
}

/// The Entity Appearance field is not explicitly modeled because the interpretation of the on-wire value
/// depends on the EntityType, which is not yet known when that field is not
/// present in the received C-DIS PDU.
/// This struct wraps the type in the wire format.
#[derive(Clone, Debug, PartialEq)]
pub struct CdisEntityAppearance(pub u32);

/// The Capabilities field is not explicitly modeled because the interpretation of the on-wire value
/// depends on the EntityType, which is not yet known when that field is not
/// present in the received C-DIS PDU.
/// This struct wraps the type in the wire format.
#[derive(Clone, Debug, PartialEq)]
pub struct CdisEntityCapabilities(pub UVINT32);

/// The DR Parameters Other field is not explicitly modeled because the interpretation of the on-wire value
/// depends on the DR Algorithm.
/// This struct wraps the type in the wire format.
#[derive(Clone, Debug, PartialEq)]
pub struct CdisDRParametersOther(pub Vec<u8>);

impl From<u128> for CdisDRParametersOther {
    fn from(value: u128) -> Self {
        const FIRST_BYTES_IS_EMPTY: usize = 0;
        let mut bytes = value.to_be_bytes().to_vec();
        let _ = bytes.remove(FIRST_BYTES_IS_EMPTY);
        Self(bytes)
    }
}

pub struct EntityStateFieldsPresent;

impl EntityStateFieldsPresent {
    pub const FORCE_ID_BIT: u16 = 12; // 0x1000;
    pub const VP_BIT: u16 = 11; // 0x0800;
    pub const ENTITY_TYPE_BIT: u16 = 10; // 0x0400;
    pub const ALT_ENTITY_TYPE_BIT: u16 = 9; // 0x0200;
    pub const LINEAR_VELOCITY_BIT: u16 = 8; // 0x0100;
    pub const ENTITY_LOCATION_BIT: u16 = 7; // 0x0080;
    pub const ENTITY_ORIENTATION_BIT: u16 = 6; // 0x0040;
    pub const ENTITY_APPEARANCE_BIT: u16 = 5; // 0x0020;
    pub const DR_OTHER_BIT: u16 = 4; // 0x0010;
    pub const DR_LINEAR_ACCELERATION_BIT: u16 = 3; // 0x0008;
    pub const DR_ANGULAR_VELOCITY_BIT: u16 = 2; // 0x0004;
    pub const MARKING_BIT: u16 = 1; // 0x0002;
    pub const CAPABILITIES_BIT: u16 = 0; // 0x0001;
}
