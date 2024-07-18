use dis_rs::enumerations::{ElectromagneticEmissionBeamFunction, ElectromagneticEmissionStateUpdateIndicator, EmitterName, EmitterSystemFunction};
use crate::constants::{FOUR_BITS, SEVENTEEN_BITS};
use crate::records::model::{EntityCoordinateVector, EntityId};
use crate::types::model::{CdisFloat, CdisFloatBase, SVINT13, UVINT16, UVINT8};

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

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FundamentalParameter {
    pub frequency: FrequencyFloat,
    pub frequency_range: FrequencyFloat,
    pub erp: u8,
    pub prf: UVINT16,
    pub pulse_width: PulseWidthFloat,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct BeamData {
    pub az_center: SVINT13,
    pub az_sweep: SVINT13,
    pub el_center: SVINT13,
    pub el_sweep: SVINT13,
    pub sweep_sync: u16,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct SiteAppPair {
    pub site: UVINT16,
    pub application: UVINT16,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct EmitterSystem {
    pub name: EmitterName,
    pub function: EmitterSystemFunction,
    pub number: UVINT8,
    pub location_with_respect_to_entity: EntityCoordinateVector,
    pub emitter_beams: Vec<EmitterBeam>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct EmitterBeam {
    pub beam_id: UVINT8,
    pub beam_parameter_index: u16,
    pub fundamental_params_index: u8,
    pub beam_data_index: u8,
    pub beam_function: ElectromagneticEmissionBeamFunction,
    pub beam_status: bool,
    pub jamming_technique_kind: UVINT8,
    pub jamming_technique_category: UVINT8,
    pub jamming_technique_subcategory: UVINT8,
    pub jamming_technique_specific: UVINT8,
    pub track_jam: Vec<TrackJam>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TrackJam {
    pub site_app_pair_index: u8,
    pub entity_id: UVINT16,
    pub emitter_number: UVINT8,
    pub beam_number: UVINT8,
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