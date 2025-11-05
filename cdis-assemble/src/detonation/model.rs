use crate::constants::{EIGHT_BITS, FIFTEEN_BITS, FOUR_BITS, SIXTEEN_BITS, THREE_BITS, TWO_BITS};
use crate::parsing::{take_signed, BitInput};
use crate::records::model::{
    CdisRecord, CdisVariableParameter, EntityCoordinateVector, EntityId, EntityType,
    LinearVelocity, UnitsDekameters, UnitsMeters, WorldCoordinates,
};
use crate::types::model::{CdisFloat, VarInt, UVINT16, UVINT8};
use crate::writing::{write_value_signed, write_value_unsigned};
use crate::{BitBuffer, BodyProperties, CdisBody, CdisInteraction};
use nom::bits::complete::take;
use nom::IResult;

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
    pub descriptor_explosive_material: Option<UVINT16>,
    pub descriptor_explosive_force: Option<ExplosiveForceFloat>,
    pub location_in_entity_coordinates: EntityCoordinateVector,
    pub detonation_results: UVINT8,
    pub variable_parameters: Vec<CdisVariableParameter>,
}

impl BodyProperties for Detonation {
    type FieldsPresent = DetonationFieldsPresent;
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = FOUR_BITS;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        (if self.descriptor_warhead.is_some() && self.descriptor_fuze.is_some() {
            Self::FieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT
        } else {
            0
        }) | (if self.descriptor_quantity.is_some() && self.descriptor_rate.is_some() {
            Self::FieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT
        } else {
            0
        }) | (if self.descriptor_explosive_material.is_some()
            && self.descriptor_explosive_force.is_some()
        {
            Self::FieldsPresent::DESCRIPTOR_EXPLOSIVE_BIT
        } else {
            0
        }) | (if !self.variable_parameters.is_empty() {
            Self::FieldsPresent::VARIABLE_PARAMETERS_BIT
        } else {
            0
        })
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = TWO_BITS; // Units flags
        Self::FIELDS_PRESENT_LENGTH
            + CONST_BIT_SIZE
            + self.source_entity_id.record_length()
            + self.target_entity_id.record_length()
            + self.exploding_entity_id.record_length()
            + self.event_id.record_length()
            + self.entity_linear_velocity.record_length()
            + self.location_in_world_coordinates.record_length()
            + self.descriptor_entity_type.record_length()
            + (if self.descriptor_warhead.is_some() {
                SIXTEEN_BITS
            } else {
                0
            })
            + (if self.descriptor_fuze.is_some() {
                SIXTEEN_BITS
            } else {
                0
            })
            + (if self.descriptor_quantity.is_some() {
                EIGHT_BITS
            } else {
                0
            })
            + (if self.descriptor_rate.is_some() {
                EIGHT_BITS
            } else {
                0
            })
            + (if let Some(record) = self.descriptor_explosive_material {
                record.record_length()
            } else {
                0
            })
            + (if let Some(record) = self.descriptor_explosive_force {
                record.record_length()
            } else {
                0
            })
            + self.location_in_entity_coordinates.record_length()
            + self.detonation_results.record_length()
            + (if self.variable_parameters.is_empty() {
                0
            } else {
                EIGHT_BITS
                    + self
                        .variable_parameters
                        .iter()
                        .map(crate::records::model::CdisRecord::record_length)
                        .sum::<usize>()
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
    pub const DESCRIPTOR_WARHEAD_FUZE_BIT: u8 = 0x08;
    pub const DESCRIPTOR_QUANTITY_RATE_BIT: u8 = 0x04;
    pub const DESCRIPTOR_EXPLOSIVE_BIT: u8 = 0x02;
    pub const VARIABLE_PARAMETERS_BIT: u8 = 0x01;
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct DetonationUnits {
    pub world_location_altitude: UnitsDekameters,
    pub location_entity_coordinates: UnitsMeters,
}

impl From<u8> for DetonationUnits {
    fn from(value: u8) -> Self {
        const WORLD_LOCATION_ALTITUDE_BIT: u8 = 0x02;
        const LOCATION_IN_ENTITY_COORDINATES_BIT: u8 = 0x01;
        Self {
            world_location_altitude: UnitsDekameters::from(
                (value & WORLD_LOCATION_ALTITUDE_BIT) >> 1,
            ),
            location_entity_coordinates: UnitsMeters::from(
                value & LOCATION_IN_ENTITY_COORDINATES_BIT,
            ),
        }
    }
}

/// Custom encoding of the explosive force for the Explosion Descriptor record
/// TODO not part of the v1.0 standard - need to align the actual encoding once standardized.
#[derive(Copy, Clone, Default, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct ExplosiveForceFloat {
    mantissa: u16,
    exponent: i8,
}

impl ExplosiveForceFloat {
    #[must_use]
    pub fn record_length(&self) -> usize {
        Self::MANTISSA_BITS + Self::EXPONENT_BITS
    }
}

impl CdisFloat for ExplosiveForceFloat {
    type Mantissa = u16;
    type Exponent = i8;
    type InnerFloat = f32;
    const MANTISSA_BITS: usize = FIFTEEN_BITS;
    const EXPONENT_BITS: usize = THREE_BITS;

    fn new(mantissa: Self::Mantissa, exponent: Self::Exponent) -> Self {
        Self { mantissa, exponent }
    }

    fn from_float(float: Self::InnerFloat) -> Self {
        let mut mantissa = float;
        let mut exponent = 0usize;
        let max_mantissa = 2f32.powi(Self::MANTISSA_BITS as i32) - 1.0;
        while (mantissa > max_mantissa) & (exponent <= Self::EXPONENT_BITS) {
            mantissa /= 10.0;
            exponent += 1;
        }

        Self {
            mantissa: mantissa as Self::Mantissa,
            exponent: exponent as Self::Exponent,
        }
    }

    fn to_float(&self) -> Self::InnerFloat {
        f32::from(self.mantissa) * 10f32.powf(f32::from(self.exponent))
    }

    fn parse(input: BitInput) -> IResult<BitInput, Self> {
        let (input, mantissa) = take(Self::MANTISSA_BITS)(input)?;
        let (input, exponent) = take_signed(Self::EXPONENT_BITS)(input)?;
        let exponent = exponent as i8;

        Ok((input, Self { mantissa, exponent }))
    }

    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, Self::MANTISSA_BITS, self.mantissa);
        let cursor = write_value_signed(buf, cursor, Self::EXPONENT_BITS, self.exponent);

        cursor
    }
}
