use nom::complete::take;
use nom::IResult;
use dis_rs::enumerations::{TransmitterAntennaPatternType, TransmitterCryptoSystem, TransmitterTransmitState};
use dis_rs::transmitter::model::VariableTransmitterParameter;
use crate::{BitBuffer, BodyProperties, CdisBody, CdisInteraction};
use crate::constants::{EIGHT_BITS, FOUR_BITS, SEVENTEEN_BITS, SIXTEEN_BITS, TWENTY_EIGHT_BITS, TWENTY_FOUR_BITS, TWENTY_ONE_BITS};
use crate::parsing::BitInput;
use crate::records::model::{CdisRecord, EntityCoordinateVector, EntityId, EntityType, UnitsDekameters, UnitsMeters, WorldCoordinates};
use crate::types::model::{CdisFloat, VarInt, UVINT16, UVINT8};
use crate::writing::write_value_unsigned;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Transmitter {
    pub units: TransmitterUnits,
    pub full_update_flag: bool,
    pub radio_reference_id: EntityId,
    pub radio_number: UVINT16,
    pub radio_type: Option<EntityType>,
    pub transmit_state: TransmitterTransmitState,
    pub input_source: UVINT8,
    pub antenna_location: Option<WorldCoordinates>,
    pub relative_antenna_location: Option<EntityCoordinateVector>,
    pub antenna_pattern_type: Option<TransmitterAntennaPatternType>,
    pub frequency: Option<TransmitterFrequencyFloat>,
    pub transmit_frequency_bandwidth: Option<TransmitFrequencyBandwidthFloat>,
    pub power: Option<u8>,
    pub modulation_type: Option<ModulationType>,
    pub crypto_system: Option<TransmitterCryptoSystem>,
    pub crypto_key_id: Option<u16>,
    pub modulation_parameters: Vec<u8>,
    pub antenna_pattern: Vec<u8>,
    pub variable_transmitter_parameters: Vec<VariableTransmitterParameter>,
}

impl BodyProperties for Transmitter {
    type FieldsPresent = TransmitterFieldsPresent;
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = EIGHT_BITS;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        (if self.radio_type.is_some() { Self::FieldsPresent::RADIO_TYPE_BIT } else { 0 })
        | (if !self.variable_transmitter_parameters.is_empty() { Self::FieldsPresent::VARIABLE_PARAMETERS_BIT } else { 0 })
        | (if self.antenna_location.is_some() { Self::FieldsPresent::ANTENNA_LOCATION_BIT } else { 0 } )
        | (if self.relative_antenna_location.is_some() { Self::FieldsPresent::RELATIVE_ANTENNA_LOCATION_BIT } else { 0 } )
        | (if self.antenna_pattern_type.is_some() && !self.antenna_pattern.is_empty() { Self::FieldsPresent::ANTENNA_PATTERN_BIT } else { 0 } )
        | (if self.frequency.is_some() && self.transmit_frequency_bandwidth.is_some()
            && self.power.is_some() && self.modulation_type.is_some() {
            Self::FieldsPresent::TRANSMITTER_DETAILS_BIT
        } else { 0 } )
        | (if self.crypto_system.is_some() && self.crypto_key_id.is_some() { Self::FieldsPresent::CRYPTO_DETAILS_BIT } else { 0 } )
        | (if !self.modulation_parameters.is_empty() { Self::FieldsPresent::MODULATION_PARAMETERS_BIT } else { 0 } )
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = 11 + 2 + 3 + 10 + 8 + 20 + 8; // fields present, units, full update flag, transmit state, antenna pattern type, antenna pattern length, power, crypto type, crypto key, modulation parameters length
        const VARIABLE_TRANSMITTER_PARAM_CONST_BIT_SIZE: usize = 48;

        CONST_BIT_SIZE +
            self.radio_reference_id.record_length() +
            self.radio_number.record_length() +
            if let Some(record) = self.radio_type { record.record_length() } else { 0 } +
            self.input_source.record_length() +
            if !self.variable_transmitter_parameters.is_empty() { UVINT8::from(self.variable_transmitter_parameters.len() as u8).record_length() } else { 0 } +
            if let Some(record) = self.antenna_location { record.record_length() } else { 0 } +
            if let Some(record) = self.relative_antenna_location { record.record_length() } else { 0 } +
            if self.frequency.is_some() { TWENTY_EIGHT_BITS } else { 0 } +
            if self.transmit_frequency_bandwidth.is_some() { TWENTY_ONE_BITS } else { 0 } +
            if self.modulation_type.is_some() { SIXTEEN_BITS } else { 0 } +
            (self.modulation_parameters.len() * EIGHT_BITS) +
            (self.antenna_pattern.len() * EIGHT_BITS) +
            self.variable_transmitter_parameters.iter()
                .map(| param | VARIABLE_TRANSMITTER_PARAM_CONST_BIT_SIZE + (param.fields.len() * EIGHT_BITS) )
                .sum::<usize>()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::Transmitter(self)
    }
}

impl CdisInteraction for Transmitter {
    fn originator(&self) -> Option<&EntityId> {
        todo!()
    }

    fn receiver(&self) -> Option<&EntityId> {
        todo!()
    }
}

pub struct TransmitterFieldsPresent;

impl TransmitterFieldsPresent {
    pub const RADIO_TYPE_BIT: u8 = 0x80;
    pub const VARIABLE_PARAMETERS_BIT: u8 = 0x40;
    pub const ANTENNA_LOCATION_BIT: u8 = 0x20;
    pub const RELATIVE_ANTENNA_LOCATION_BIT: u8 = 0x10;
    pub const ANTENNA_PATTERN_BIT: u8 = 0x08;
    pub const TRANSMITTER_DETAILS_BIT: u8 = 0x04;
    pub const CRYPTO_DETAILS_BIT: u8 = 0x02;
    pub const MODULATION_PARAMETERS_BIT: u8 = 0x01;
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TransmitterUnits {
    pub world_location_altitude: UnitsDekameters,
    pub relative_antenna_location: UnitsMeters,
}

impl From<u8> for TransmitterUnits {
    fn from(value: u8) -> Self {
        const WORLD_LOCATION_ALTITUDE_BIT: u8 = 0x02;
        const RELATIVE_ANTENNA_LOCATION_BIT: u8 = 0x01;
        Self {
            world_location_altitude: UnitsDekameters::from((value & WORLD_LOCATION_ALTITUDE_BIT) >> 1),
            relative_antenna_location: UnitsMeters::from(value & RELATIVE_ANTENNA_LOCATION_BIT),
        }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct TransmitterFrequencyFloat {
    mantissa: u32,
    exponent: u8,
}

impl CdisFloat for TransmitterFrequencyFloat {
    type Mantissa = u32;
    type Exponent = u8;
    type InnerFloat = f64;
    const MANTISSA_BITS: usize = TWENTY_FOUR_BITS;
    const EXPONENT_BITS: usize = FOUR_BITS;

    fn new(mantissa: Self::Mantissa, exponent: Self::Exponent) -> Self {
        Self {
            mantissa,
            exponent,
        }
    }

    fn from_float(float: Self::InnerFloat) -> Self {
        let mut mantissa = float;
        let mut exponent = 0usize;
        let max_mantissa = 2f64.powi(Self::MANTISSA_BITS as i32) - 1.0;
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
        self.mantissa as f64 * 10f64.powf(self.exponent as f64)
    }

    fn parse(input: BitInput) -> IResult<BitInput, Self> {
        let (input, mantissa) = take(Self::MANTISSA_BITS)(input)?;
        let (input, exponent) = take(Self::EXPONENT_BITS)(input)?;

        Ok((input, Self {
            mantissa,
            exponent
        }))
    }

    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, Self::MANTISSA_BITS, self.mantissa);
        let cursor = write_value_unsigned(buf, cursor, Self::EXPONENT_BITS, self.exponent);

        cursor
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct TransmitFrequencyBandwidthFloat {
    mantissa: u32,
    exponent: u8,
}

impl CdisFloat for TransmitFrequencyBandwidthFloat {
    type Mantissa = u32;
    type Exponent = u8;
    type InnerFloat = f32;
    const MANTISSA_BITS: usize = SEVENTEEN_BITS;
    const EXPONENT_BITS: usize = FOUR_BITS;

    fn new(mantissa: Self::Mantissa, exponent: Self::Exponent) -> Self {
        Self {
            mantissa,
            exponent,
        }
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
        self.mantissa as f32 * 10f32.powf(self.exponent as f32)
    }

    fn parse(input: BitInput) -> IResult<BitInput, Self> {
        let (input, mantissa) = take(Self::MANTISSA_BITS)(input)?;
        let (input, exponent) = take(Self::EXPONENT_BITS)(input)?;

        Ok((input, Self {
            mantissa,
            exponent
        }))
    }

    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, Self::MANTISSA_BITS, self.mantissa);
        let cursor = write_value_unsigned(buf, cursor, Self::EXPONENT_BITS, self.exponent);

        cursor
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct ModulationType {
    pub spread_spectrum: u8,
    pub major_modulation: u8,
    pub detail: u8,
    pub radio_system: u8,
}