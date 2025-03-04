use crate::constants::{FOURTEEN_BITS, THREE_BITS};
use crate::parsing::{take_signed, BitInput};
use crate::records::model::{
    BeamData, CdisRecord, EntityCoordinateVector, EntityId, FrequencyFloat,
};
use crate::types::model::{CdisFloat, VarInt, UVINT16, UVINT8};
use crate::writing::{write_value_signed, write_value_unsigned};
use crate::{BitBuffer, BodyProperties, CdisBody, CdisInteraction};
use dis_rs::enumerations::{
    ElectromagneticEmissionBeamFunction, ElectromagneticEmissionStateUpdateIndicator, EmitterName,
    EmitterSystemFunction, HighDensityTrackJam,
};
use nom::bits::complete::take;
use nom::IResult;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct ElectromagneticEmission {
    pub full_update_flag: bool,
    pub emitting_id: EntityId,
    pub event_id: EntityId,
    pub state_update_indicator: ElectromagneticEmissionStateUpdateIndicator,
    pub fundamental_params: Vec<FundamentalParameter>,
    pub beam_data: Vec<BeamData>,
    pub site_app_pairs: Vec<SiteAppPair>,
    pub emitter_systems: Vec<EmitterSystem>,
}

impl BodyProperties for ElectromagneticEmission {
    type FieldsPresent = ();
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 0;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        0
    }

    fn body_length_bits(&self) -> usize {
        const FIXED_FIELDS_BITS: usize = 18;
        FIXED_FIELDS_BITS
            + self.emitting_id.record_length()
            + self.event_id.record_length()
            + UVINT8::from(self.emitter_systems.len() as u8).record_length()
            + self
                .fundamental_params
                .iter()
                .map(FundamentalParameter::record_length)
                .sum::<usize>()
            + self
                .beam_data
                .iter()
                .map(crate::records::model::CdisRecord::record_length)
                .sum::<usize>()
            + self
                .site_app_pairs
                .iter()
                .map(SiteAppPair::record_length)
                .sum::<usize>()
            + self
                .emitter_systems
                .iter()
                .map(EmitterSystem::record_length)
                .sum::<usize>()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::ElectromagneticEmission(self)
    }
}

impl CdisInteraction for ElectromagneticEmission {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.emitting_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

#[derive(Clone, Default, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct FundamentalParameter {
    pub frequency: FrequencyFloat,
    pub frequency_range: FrequencyFloat,
    pub erp: u8,
    pub prf: UVINT16,
    pub pulse_width: PulseWidthFloat,
}

impl FundamentalParameter {
    fn record_length(&self) -> usize {
        const FIXED_LENGTH_BITS: usize = 67;
        FIXED_LENGTH_BITS + self.prf.record_length()
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SiteAppPair {
    pub site: UVINT16,
    pub application: UVINT16,
}

impl SiteAppPair {
    fn record_length(&self) -> usize {
        self.site.record_length() + self.application.record_length()
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct EmitterSystem {
    pub name: Option<EmitterName>,
    pub function: Option<EmitterSystemFunction>,
    pub number: UVINT8,
    pub location_with_respect_to_entity: Option<EntityCoordinateVector>,
    pub emitter_beams: Vec<EmitterBeam>,
}

impl EmitterSystem {
    fn record_length(&self) -> usize {
        const FIXED_LENGTH_BITS: usize = 7;
        FIXED_LENGTH_BITS
            + (if self.name.is_some() { 16 } else { 0 })
            + (if self.function.is_some() { 8 } else { 0 })
            + self.number.record_length()
            + (if let Some(location) = self.location_with_respect_to_entity {
                location.record_length()
            } else {
                0
            })
            + self
                .emitter_beams
                .iter()
                .map(EmitterBeam::record_length)
                .sum::<usize>()
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct EmitterBeam {
    pub beam_id: UVINT8,
    pub beam_parameter_index: u16,
    pub fundamental_params_index: Option<u8>,
    pub beam_data_index: Option<u8>,
    pub beam_function: ElectromagneticEmissionBeamFunction,
    pub high_density_track_jam: HighDensityTrackJam,
    pub beam_status: bool,
    pub jamming_technique_kind: Option<UVINT8>,
    pub jamming_technique_category: Option<UVINT8>,
    pub jamming_technique_subcategory: Option<UVINT8>,
    pub jamming_technique_specific: Option<UVINT8>,
    pub track_jam: Vec<TrackJam>,
}

impl EmitterBeam {
    fn record_length(&self) -> usize {
        const FIXED_LENGTH_BITS: usize = 4 + 16 + 21;
        FIXED_LENGTH_BITS
            + self.beam_id.record_length()
            + (if let Some(record) = self.jamming_technique_kind {
                record.record_length()
            } else {
                0
            })
            + (if let Some(record) = self.jamming_technique_category {
                record.record_length()
            } else {
                0
            })
            + (if let Some(record) = self.jamming_technique_subcategory {
                record.record_length()
            } else {
                0
            })
            + (if let Some(record) = self.jamming_technique_specific {
                record.record_length()
            } else {
                0
            })
            + self
                .track_jam
                .iter()
                .map(TrackJam::record_length)
                .sum::<usize>()
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TrackJam {
    pub site_app_pair_index: u8,
    pub entity_id: UVINT16,
    pub emitter_number: Option<UVINT8>,
    pub beam_number: Option<UVINT8>,
}

impl TrackJam {
    fn record_length(&self) -> usize {
        const FIXED_LENGTH_BITS: usize = 6;
        FIXED_LENGTH_BITS
            + self.entity_id.record_length()
            + (if let Some(record) = self.emitter_number {
                record.record_length()
            } else {
                0
            })
            + (if let Some(record) = self.beam_number {
                record.record_length()
            } else {
                0
            })
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct PulseWidthFloat {
    pub mantissa: u16,
    pub exponent: i8,
}

impl CdisFloat for PulseWidthFloat {
    type Mantissa = u16;
    type Exponent = i8;
    type InnerFloat = f32;
    const MANTISSA_BITS: usize = FOURTEEN_BITS;
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

        let exponent = exponent as Self::Exponent;
        Ok((input, Self { mantissa, exponent }))
    }

    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, Self::MANTISSA_BITS, self.mantissa);
        let cursor = write_value_signed(buf, cursor, Self::EXPONENT_BITS, self.exponent);

        cursor
    }
}
