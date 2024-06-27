use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::constants::{EIGHT_BITS, SIXTEEN_BITS};
use crate::records::model::{CdisRecord, CdisVariableParameter, EntityCoordinateVector, EntityId, EntityType, LinearVelocity, Units, WorldCoordinates};
use crate::types::model::{UVINT8, VarInt};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Detonation {
    pub units: DetonationUnits,
    pub source_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub exploding_entity_id: EntityId,
    pub event_id: EntityId,
    pub entity_linear_velocity: LinearVelocity,
    pub location_in_world_coordinates: WorldCoordinates,
    pub descriptor_entity_type: EntityType,
    pub descriptor_warhead: Option<u16>,
    pub descriptor_fuze: Option<u16>,
    pub descriptor_quantity: Option<u8>,
    pub descriptor_rate: Option<u8>,
    pub location_in_entity_coordinates: EntityCoordinateVector,
    pub detonation_results: UVINT8,
    pub variable_parameters: Vec<CdisVariableParameter>,
}

impl BodyProperties for Detonation {
    type FieldsPresent = DetonationFieldsPresent;
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 3;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        (if self.descriptor_warhead.is_some() && self.descriptor_fuze.is_some() { Self::FieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT } else { 0 })
            | (if self.descriptor_quantity.is_some() && self.descriptor_rate.is_some() { Self::FieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT } else { 0 })
            | (if !self.variable_parameters.is_empty() { Self::FieldsPresent::VARIABLE_PARAMETERS_BIT } else { 0 })
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = 2; // Units flags
        Self::FIELDS_PRESENT_LENGTH + CONST_BIT_SIZE
            + self.source_entity_id.record_length()
            + self.target_entity_id.record_length()
            + self.exploding_entity_id.record_length()
            + self.event_id.record_length()
            + self.entity_linear_velocity.record_length()
            + self.location_in_world_coordinates.record_length()
            + self.descriptor_entity_type.record_length()
            + (if self.descriptor_warhead.is_some() { SIXTEEN_BITS } else { 0 })
            + (if self.descriptor_fuze.is_some() { SIXTEEN_BITS } else { 0 })
            + (if self.descriptor_quantity.is_some() { EIGHT_BITS } else { 0 })
            + (if self.descriptor_rate.is_some() { EIGHT_BITS } else { 0 })
            + self.location_in_entity_coordinates.record_length()
            + self.detonation_results.record_length()
            + (if self.variable_parameters.is_empty() { 0 } else {
                EIGHT_BITS + self.variable_parameters.iter().map(|vp| vp.record_length() ).sum()
            })
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::Detonation(self)
    }
}

impl CdisInteraction for Detonation {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.source_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.target_entity_id)
    }
}

pub struct DetonationFieldsPresent;

impl DetonationFieldsPresent {
    pub const DESCRIPTOR_WARHEAD_FUZE_BIT: u8 = 0x04;
    pub const DESCRIPTOR_QUANTITY_RATE_BIT: u8 = 0x02;
    pub const VARIABLE_PARAMETERS_BIT: u8 = 0x01;
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct DetonationUnits {
    pub world_location_altitude: Units,
    pub location_entity_coordinates: UnitsMeters,
}

impl From<u8> for DetonationUnits {
    fn from(value: u8) -> Self {
        pub const WORLD_LOCATION_ALTITUDE_BIT: u8 = 0x02;
        pub const LOCATION_IN_ENTITY_COORDINATES_BIT: u8 = 0x01;
        Self {
            world_location_altitude: Units::from((value & WORLD_LOCATION_ALTITUDE_BIT) >> 1),
            location_entity_coordinates: UnitsMeters::from(value & LOCATION_IN_ENTITY_COORDINATES_BIT),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum UnitsMeters {
    Centimeter,
    #[default]
    Meter,
}

impl From<u8> for UnitsMeters {
    fn from(value: u8) -> Self {
        match value {
            0 => UnitsMeters::Centimeter,
            _ => UnitsMeters::Meter,
        }
    }
}

impl From<UnitsMeters> for u8 {
    fn from(value: UnitsMeters) -> Self {
        match value {
            UnitsMeters::Centimeter => 0,
            UnitsMeters::Meter => 1,
        }
    }
}
