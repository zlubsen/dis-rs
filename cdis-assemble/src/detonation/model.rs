use crate::constants::{EIGHT_BITS, FIFTEEN_BITS, THREE_BITS, TWO_BITS};
use crate::parsing::{BitInput, take_signed};
use crate::records::model::{
    CdisRecord, CdisVariableParameter, DescriptorQuantityRate, DescriptorWarheadFuse,
    EntityCoordinateVector, EntityId, EntityType, LinearVelocity, UnitsDekameters, UnitsMeters,
    WorldCoordinates,
};
use crate::types::model::{CdisFloat, UVINT8, VarInt};
use crate::writing::write_integer_bits;
use crate::{BitBuffer, BodyProperties, CdisBody, CdisInteraction};
use bitflags::bitflags;
use nom::IResult;
use nom::bits::complete::take;

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
    pub descriptor_warhead_fuse: Option<DescriptorWarheadFuse>,
    pub descriptor_quantity_rate: Option<DescriptorQuantityRate>,
    // NOTE The C-DIS standard (SISO-STD-023-2023: [13.4]) does not provide information about how to
    //      include the specific fields of the Explosion Descriptor (namely: Explosive Material and
    //      Explosive Force) in the Detonation PDU, despite listing the Explosion Descriptor as a
    //      valid type for the Descriptor field. The Explosion Descriptor is only described in the
    //      DIS standard (IEEE 1278.1-2012: [6.2.19.3]).
    // pub descriptor_explosive_material: Option<UVINT16>,
    // pub descriptor_explosive_force: Option<ExplosiveForceFloat>,
    pub location_in_entity_coordinates: EntityCoordinateVector,
    pub detonation_results: UVINT8,
    pub variable_parameters: Vec<CdisVariableParameter>,
}

impl BodyProperties for Detonation {
    type FieldsPresentFlags = DetonationFieldsPresentFlags;
    type FieldsPresentField = u8;
    const FIELDS_PRESENT_FLAGS_LENGTH: usize = THREE_BITS;

    fn fields_present_field(&self) -> Self::FieldsPresentField {
        let mut flags = Self::FieldsPresentFlags::empty();

        if self.descriptor_warhead_fuse.is_some() {
            flags.insert(Self::FieldsPresentFlags::DESCRIPTOR_WARHEAD_FUSE);
        }
        if self.descriptor_quantity_rate.is_some() {
            flags.insert(Self::FieldsPresentFlags::DESCRIPTOR_QUANTITY_RATE);
        }
        if !self.variable_parameters.is_empty() {
            flags.insert(Self::FieldsPresentFlags::VARIABLE_PARAMETERS);
        }

        flags.bits()
    }

    fn body_length_bits(&self) -> usize {
        // 1 bit = Units
        let mut length = Self::FIELDS_PRESENT_FLAGS_LENGTH + TWO_BITS;

        length += self.source_entity_id.record_length()
            + self.target_entity_id.record_length()
            + self.exploding_entity_id.record_length()
            + self.event_id.record_length()
            + self.entity_linear_velocity.record_length()
            + self.location_in_world_coordinates.record_length()
            + self.descriptor_entity_type.record_length();

        if let Some(descriptor_warhead_fuse) = self.descriptor_warhead_fuse {
            length += descriptor_warhead_fuse.record_length();
        }

        if let Some(descriptor_quantity_rate) = self.descriptor_quantity_rate {
            length += descriptor_quantity_rate.record_length();
        }

        length += self.location_in_entity_coordinates.record_length()
            + self.detonation_results.record_length();

        if !self.variable_parameters.is_empty() {
            // 8 bits = Number Of Variable Parameter Records
            length += EIGHT_BITS
                + self
                    .variable_parameters
                    .iter()
                    .map(CdisRecord::record_length)
                    .sum::<usize>();
        }

        length
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

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DetonationFieldsPresentFlags: <Detonation as BodyProperties>::FieldsPresentField {
        const DESCRIPTOR_WARHEAD_FUSE = 0x04;
        const DESCRIPTOR_QUANTITY_RATE = 0x02;
        const VARIABLE_PARAMETERS = 0x01;
    }
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

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_sign_loss)]
    fn from_float(float: Self::InnerFloat) -> Self {
        let max_mantissa = ((1usize << Self::MANTISSA_BITS) - 1) as Self::Mantissa;
        let min_exponent = -(1isize << (Self::EXPONENT_BITS - 1)) as Self::Exponent;
        let max_exponent = ((1isize << (Self::EXPONENT_BITS - 1)) - 1) as Self::Exponent;

        let mut x = float.abs();
        let mut exponent: Self::Exponent = 0;
        let mut overflow = false;

        if x != 0.0 {
            while (x * 10.0).round() <= Self::InnerFloat::from(max_mantissa)
                && exponent > min_exponent
            {
                x *= 10.0;
                exponent -= 1;
            }
        }

        while x.round() > Self::InnerFloat::from(max_mantissa) {
            if exponent < max_exponent {
                x /= 10.0;
                exponent += 1;
            } else {
                overflow = true;
                break;
            }
        }

        let mantissa = if !overflow {
            x.round() as Self::Mantissa
        } else {
            max_mantissa
        };
        if mantissa == 0 {
            exponent = 0;
        }

        Self { mantissa, exponent }
    }

    fn to_float(&self) -> Self::InnerFloat {
        Self::InnerFloat::from(self.mantissa) * 10f32.powi(i32::from(self.exponent))
    }

    fn parse(input: BitInput) -> IResult<BitInput, Self> {
        let (input, mantissa) = take(Self::MANTISSA_BITS)(input)?;
        let (input, exponent) = take_signed::<i16>(Self::EXPONENT_BITS)(input)?;

        Ok((input, Self::new(mantissa, exponent as Self::Exponent)))
    }

    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, Self::MANTISSA_BITS, self.mantissa);
        let cursor = write_integer_bits(buf, cursor, Self::EXPONENT_BITS, self.exponent);

        cursor
    }
}
