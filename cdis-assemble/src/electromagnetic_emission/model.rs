use dis_rs::enumerations::{ElectromagneticEmissionBeamFunction, ElectromagneticEmissionStateUpdateIndicator, EmitterName, EmitterSystemFunction, HighDensityTrackJam};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::constants::{FOUR_BITS, SEVENTEEN_BITS};
use crate::records::model::{CdisRecord, EntityCoordinateVector, EntityId};
use crate::types::model::{CdisFloat, CdisFloatBase, SVINT13, UVINT16, UVINT8, VarInt};

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

    fn fields_present_field(&self) -> Self::FieldsPresentOutput { 0 }

    fn body_length_bits(&self) -> usize {
        const FIXED_FIELDS_BITS: usize = 18;
        FIXED_FIELDS_BITS +
            self.emitting_id.record_length() +
            self.event_id.record_length() +
            UVINT8::from(self.emitter_systems.len() as u8).record_length() +
            self.fundamental_params.iter().map(|param| param.record_length()).sum::<usize>() +
            self.beam_data.iter().map(|beam| beam.record_length()).sum::<usize>() +
            self.site_app_pairs.iter().map(|pair| pair.record_length()).sum::<usize>() +
            self.emitter_systems.iter().map(|system| system.record_length()).sum::<usize>()
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

#[derive(Clone, Default, Debug, PartialEq)]
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
pub struct BeamData {
    pub az_center: SVINT13,
    pub az_sweep: SVINT13,
    pub el_center: SVINT13,
    pub el_sweep: SVINT13,
    pub sweep_sync: u16,
}

impl BeamData {
    fn record_length(&self) -> usize {
        const FIXED_LENGTH_BITS: usize = 10;
        FIXED_LENGTH_BITS +
            self.az_center.record_length() +
            self.az_sweep.record_length() +
            self.el_center.record_length() +
            self.el_sweep.record_length()
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SiteAppPair {
    pub site: UVINT16,
    pub application: UVINT16,
}

impl SiteAppPair {
    fn record_length(&self) -> usize {
        self.site.record_length() +
            self.application.record_length()
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
        FIXED_LENGTH_BITS +
            (if self.name.is_some() { 16 } else { 0 }) +
            (if self.function.is_some() { 8 } else { 0 }) +
            self.number.record_length() +
            (if let Some(location) = self.location_with_respect_to_entity { location.record_length() } else { 0 }) +
            self.emitter_beams.iter().map(|beam| beam.record_length()).sum::<usize>()
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
        FIXED_LENGTH_BITS +
            self.beam_id.record_length() +
            (if let Some(record) = self.jamming_technique_kind { record.record_length() } else { 0 }) +
            (if let Some(record) = self.jamming_technique_category { record.record_length() } else { 0 }) +
            (if let Some(record) = self.jamming_technique_subcategory { record.record_length() } else { 0 }) +
            (if let Some(record) = self.jamming_technique_specific { record.record_length() } else { 0 }) +
            self.track_jam.iter().map(|record| record.record_length()).sum::<usize>()
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
        FIXED_LENGTH_BITS +
            self.entity_id.record_length() +
            (if let Some(record) = self.emitter_number { record.record_length() } else { 0 }) +
            (if let Some(record) = self.beam_number { record.record_length() } else { 0 })
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct FrequencyFloat {
    base: CdisFloatBase,
}

impl CdisFloat for FrequencyFloat {
    const MANTISSA_BITS: usize = SEVENTEEN_BITS;
    const EXPONENT_BITS: usize = FOUR_BITS;

    fn new(mantissa: i32, exponent: i8) -> Self {
        Self {
            base: CdisFloatBase {
                mantissa,
                exponent,
                regular_float: None,
            }
        }
    }

    fn from_f64(regular_float: f64) -> Self {
        Self {
            base: CdisFloatBase {
                mantissa: 0,
                exponent: 0,
                regular_float: Some(regular_float),
            }
        }
    }

    fn mantissa(&self) -> i32 {
        self.base.mantissa
    }

    fn exponent(&self) -> i8 {
        self.base.exponent
    }

    fn regular_float(&self) -> Option<f64> {
        self.base.regular_float
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct PulseWidthFloat {
    base: CdisFloatBase,
}

impl CdisFloat for PulseWidthFloat {
    const MANTISSA_BITS: usize = 14;
    const EXPONENT_BITS: usize = 3;

    fn new(mantissa: i32, exponent: i8) -> Self {
        Self {
            base: CdisFloatBase {
                mantissa,
                exponent,
                regular_float: None,
            }
        }
    }

    fn from_f64(regular_float: f64) -> Self {
        Self {
            base: CdisFloatBase {
                mantissa: 0,
                exponent: 0,
                regular_float: Some(regular_float),
            }
        }
    }

    fn mantissa(&self) -> i32 {
        self.base.mantissa
    }

    fn exponent(&self) -> i8 {
        self.base.exponent
    }

    fn regular_float(&self) -> Option<f64> {
        self.base.regular_float
    }
}