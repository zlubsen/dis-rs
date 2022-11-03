use crate::common::model::{EntityId, EventId};
use crate::enumerations::{BeamStatusBeamState, EmitterName, EmitterSystemFunction, ElectromagneticEmissionBeamFunction, ElectromagneticEmissionStateUpdateIndicator, HighDensityTrackJam};
use crate::VectorF32;

pub struct ElectromagneticEmission {
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_update_indicator: ElectromagneticEmissionStateUpdateIndicator,
    pub emitter_systems: Vec<EmitterSystem>,
}

pub struct EmitterSystem {
    pub system_data_length: u8,
    pub name: EmitterName,
    pub function: EmitterSystemFunction,
    pub number: u8,
    pub location: VectorF32,
    pub beams: Vec<Beam>,
}

pub struct Beam {
    pub data_length: u8,
    pub number: u8,
    pub parameter_index: u16,
    pub parameter_data: FundamentalParameterData,
    pub beam_data: BeamData,
    pub beam_function: ElectromagneticEmissionBeamFunction,
    pub high_density_track_jam: HighDensityTrackJam,
    pub beam_status: BeamStatusBeamState,
    pub jamming_technique: JammingTechnique,
    pub track_jam_data: Vec<TrackJam>,
}

pub struct FundamentalParameterData {
    pub frequency: f32,
    pub frequency_range: f32,
    pub effective_power: f32,
    pub pulse_repetition_frequency: f32,
    pub pulse_width: f32,
}

pub struct BeamData {
    pub azimuth_center: f32,
    pub azimuth_sweep: f32,
    pub elevation_center: f32,
    pub elevation_sweep: f32,
    pub sweep_sync: f32,
}

pub struct JammingTechnique {
    // TODO uid 284 - new format to extract/generate...
    pub kind: u8,
    pub category: u8,
    pub subcategory: u8,
    pub specific: u8,
}

pub struct TrackJam {
    pub site: u16,
    pub application: u16,
    pub entity: u16,
    pub emitter: u8,
    pub beam: u8,
}