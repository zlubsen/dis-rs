use bytes::{BufMut, BytesMut};
use dis_rs::entity_state::model::{DrOtherParameters, EntityAppearance};
use dis_rs::enumerations::{DeadReckoningAlgorithm};
use dis_rs::Serialize;
use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::constants::{HUNDRED_TWENTY_BITS, THIRTY_TWO_BITS};
use crate::records::model::{AngularVelocity, CdisEntityMarking, CdisRecord, CdisVariableParameter, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, Units, WorldCoordinates};
use crate::types::model::{VarInt, UVINT32, UVINT8};

#[derive(Clone, Default, Debug, PartialEq)]
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
    pub dr_params_other: Option<CdisDRParametersOther>,
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
        (if self.force_id.is_some() { Self::FieldsPresent::FORCE_ID_BIT } else { 0 })
            | (if !self.variable_parameters.is_empty() { Self::FieldsPresent::VP_BIT } else { 0 })
            | (if self.entity_type.is_some() { Self::FieldsPresent::ENTITY_TYPE_BIT } else { 0 })
            | (if self.alternate_entity_type.is_some() { Self::FieldsPresent::ALT_ENTITY_TYPE_BIT } else { 0 })
            | (if self.entity_linear_velocity.is_some() { Self::FieldsPresent::LINEAR_VELOCITY_BIT } else { 0 })
            | (if self.entity_location.is_some() { Self::FieldsPresent::ENTITY_LOCATION_BIT } else { 0 })
            | (if self.entity_orientation.is_some() { Self::FieldsPresent::ENTITY_ORIENTATION_BIT } else { 0 })
            | (if self.entity_appearance.is_some() { Self::FieldsPresent::ENTITY_APPEARANCE_BIT } else { 0 })
            | (if self.dr_params_other.is_some() { Self::FieldsPresent::DR_OTHER_BIT } else { 0 })
            | (if self.dr_params_entity_linear_acceleration.is_some() { Self::FieldsPresent::DR_LINEAR_ACCELERATION_BIT } else { 0 })
            | (if self.dr_params_entity_angular_velocity.is_some() { Self::FieldsPresent::DR_ANGULAR_VELOCITY_BIT } else { 0 })
            | (if self.entity_marking.is_some() { Self::FieldsPresent::MARKING_BIT } else { 0 })
            | (if self.capabilities.is_some() { Self::FieldsPresent::CAPABILITIES_BIT } else { 0 })
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = 5; // Full_update_flag + DR_algorithm
        Self::FIELDS_PRESENT_LENGTH + CONST_BIT_SIZE
            + self.units.record_length()
            + self.entity_id.record_length()
            + (if let Some(force_id) = &self.force_id { force_id.record_length() } else { 0 })
            + (if self.variable_parameters.is_empty() { 0 } else { UVINT8::from(self.variable_parameters.len() as u8).record_length() })
            + (if let Some(record) = &self.entity_type { record.record_length() } else { 0 })
            + (if let Some(record) = &self.alternate_entity_type { record.record_length() } else { 0 })
            + (if let Some(record) = &self.entity_linear_velocity { record.record_length() } else { 0 })
            + (if let Some(record) = &self.entity_location { record.record_length() } else { 0 })
            + (if let Some(record) = &self.entity_orientation { record.record_length() } else { 0 })
            + (if self.entity_appearance.is_some() { THIRTY_TWO_BITS } else { 0 } )
            + (if self.dr_params_other.is_some() { HUNDRED_TWENTY_BITS } else { 0 } )
            + (if let Some(record) = &self.dr_params_entity_linear_acceleration { record.record_length() } else { 0 } )
            + (if let Some(record) = &self.dr_params_entity_angular_velocity { record.record_length() } else { 0 } )
            + (if let Some(record) = &self.entity_marking { record.record_length() } else { 0 } )
            + (if let Some(record) = &self.capabilities { record.0.record_length() } else { 0 } )
            + self.variable_parameters.iter().map(|vp| vp.record_length() ).sum::<usize>()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::EntityState(self)
    }
}

impl CdisInteraction for EntityState {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}


/// The Entity Appearance field is not explicitly modeled because the interpretation of the on-wire value
/// depends on the EntityType, which is not yet known when that field is not
/// present in the received C-DIS PDU.
/// This struct wraps the type in the wire format.
#[derive(Clone, Debug, PartialEq)]
pub struct CdisEntityAppearance(pub u32);

impl From<&EntityAppearance> for CdisEntityAppearance{
    fn from(value: &EntityAppearance) -> Self {
        CdisEntityAppearance(value.into())
    }
}

/// The Capabilities field is not explicitly modeled because the interpretation of the on-wire value
/// depends on the EntityType, which is not yet known when that field is not
/// present in the received C-DIS PDU.
/// This struct wraps the type in the wire format.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CdisEntityCapabilities(pub UVINT32);

/// The DR Parameters Other field is not explicitly modeled because the interpretation of the on-wire value
/// depends on the DR Algorithm.
/// This struct wraps the type in the wire format.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CdisDRParametersOther(pub u128);

impl CdisDRParametersOther {
    pub fn decode(&self, algorithm: DeadReckoningAlgorithm) -> DrOtherParameters {
        let other: [u8; 15] = self.0.to_be_bytes()[1..16]
            .as_ref()
            .try_into()
            .unwrap_or(Default::default());
        let dr_params_other = match dis_rs::parse_dr_other_parameters(&other, algorithm) {
            Ok((_input, params)) => { params }
            Err(_) => { DrOtherParameters::default() } // when something goes wrong in parsing (although we parse exactly 15 bytes of input), just return zeroes (default).
        };

        dr_params_other
    }
}

impl From<u128> for CdisDRParametersOther {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl From<&DrOtherParameters> for CdisDRParametersOther {
    fn from(value: &DrOtherParameters) -> Self {
        const SIXTEEN_BYTES: usize = std::mem::size_of::<u128>();
        let mut buf = BytesMut::with_capacity(SIXTEEN_BYTES);
        buf.put_u8(0);
        value.serialize(&mut buf);
        let int_bytes: [u8; SIXTEEN_BYTES] = buf[..SIXTEEN_BYTES].try_into().unwrap_or([0; SIXTEEN_BYTES]);
        Self(u128::from_be_bytes(int_bytes))
    }
}

pub struct EntityStateFieldsPresent;

impl EntityStateFieldsPresent {
    pub const FORCE_ID_BIT: u16 = 0x1000;
    pub const VP_BIT: u16 = 0x0800;
    pub const ENTITY_TYPE_BIT: u16 = 0x0400;
    pub const ALT_ENTITY_TYPE_BIT: u16 = 0x0200;
    pub const LINEAR_VELOCITY_BIT: u16 = 0x0100;
    pub const ENTITY_LOCATION_BIT: u16 = 0x0080;
    pub const ENTITY_ORIENTATION_BIT: u16 = 0x0040;
    pub const ENTITY_APPEARANCE_BIT: u16 = 0x0020;
    pub const DR_OTHER_BIT: u16 = 0x0010;
    pub const DR_LINEAR_ACCELERATION_BIT: u16 = 0x0008;
    pub const DR_ANGULAR_VELOCITY_BIT: u16 = 0x0004;
    pub const MARKING_BIT: u16 = 0x0002;
    pub const CAPABILITIES_BIT: u16 = 0x0001;
}
