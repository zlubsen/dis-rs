use crate::common::{BodyInfo, Interaction};
use crate::common::model::{BeamData, EntityId, EventId, PduBody, VectorF32};
use crate::electromagnetic_emission::builder::ElectromagneticEmissionBuilder;
use crate::enumerations::{BeamStatusBeamState, ElectromagneticEmissionBeamFunction, ElectromagneticEmissionStateUpdateIndicator, EmitterName, EmitterSystemFunction, HighDensityTrackJam, PduType};

const EMISSION_BASE_BODY_LENGTH : u16 = 16;
const EMITTER_SYSTEM_BASE_LENGTH : u16 = 20;
const BEAM_BASE_LENGTH : u16 = 52;
const TRACK_JAM_BASE_LENGTH : u16 = 8;

/// 5.7.3 Electromagnetic Emission (EE) PDU
///
/// 7.6.2 Electromagnetic Emission (EE) PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElectromagneticEmission {
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_update_indicator: ElectromagneticEmissionStateUpdateIndicator,
    pub emitter_systems: Vec<EmitterSystem>,
}

impl ElectromagneticEmission {
    pub fn builder() -> ElectromagneticEmissionBuilder {
        ElectromagneticEmissionBuilder::new()
    }

    pub fn into_builder(self) -> ElectromagneticEmissionBuilder {
        ElectromagneticEmissionBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ElectromagneticEmission(self)
    }
}

impl BodyInfo for ElectromagneticEmission {
    fn body_length(&self) -> u16 {
        EMISSION_BASE_BODY_LENGTH
            + self.emitter_systems.iter()
            .map(|system| system.system_data_length_bytes())
            .sum::<u16>()
    }

    fn body_type(&self) -> PduType {
        PduType::ElectromagneticEmission
    }
}

impl Interaction for ElectromagneticEmission {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.emitting_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        // just selects the first track or jam
        if let Some(emitter) = self.emitter_systems.first() {
            if let Some(beam) = emitter.beams.first() {
                if let Some(tracks) = beam.track_jam_data.first() {
                    Some(&tracks.entity_id)
                } else { None }
            } else { None }
        } else { None }
    }
}

/// 6.2.23 Emitter System record
#[derive(Clone, Debug, PartialEq)]
pub struct EmitterSystem {
    pub name: EmitterName,
    pub function: EmitterSystemFunction,
    pub number: u8,
    pub location: VectorF32,
    pub beams: Vec<Beam>,
}

impl Default for EmitterSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl EmitterSystem {
    pub fn new() -> Self {
        Self {
            name: EmitterName::default(),
            function: EmitterSystemFunction::default(),
            number: 0,
            location: Default::default(),
            beams: vec![]
        }
    }

    pub fn with_name(mut self, name: EmitterName) -> Self {
        self.name = name;
        self
    }

    pub fn with_function(mut self, function: EmitterSystemFunction) -> Self {
        self.function = function;
        self
    }

    pub fn with_number(mut self, number: u8) -> Self {
        self.number = number;
        self
    }

    pub fn with_location(mut self, location: VectorF32) -> Self {
        self.location = location;
        self
    }

    pub fn with_beams(mut self, beams: &mut Vec<Beam>) -> Self {
        self.beams.append(beams);
        self
    }

    pub fn with_beam(mut self, beam: Beam) -> Self {
        self.beams.push(beam);
        self
    }

    pub fn system_data_length_bytes(&self) -> u16 {
        EMITTER_SYSTEM_BASE_LENGTH
            + self.beams.iter()
            .map(|beam| beam.beam_data_length_bytes() )
            .sum::<u16>()
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Beam {
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

impl Beam {
    pub fn new() -> Self {
        Self {
            number: 0,
            parameter_index: 0,
            parameter_data: FundamentalParameterData::default(),
            beam_data: BeamData::default(),
            beam_function: ElectromagneticEmissionBeamFunction::default(),
            high_density_track_jam: HighDensityTrackJam::default(),
            beam_status: BeamStatusBeamState::default(),
            jamming_technique: JammingTechnique::default(),
            track_jam_data: vec![]
        }
    }

    pub fn with_number(mut self, number: u8) -> Self {
        self.number = number;
        self
    }

    pub fn with_parameter_index(mut self, parameter_index: u16) -> Self {
        self.parameter_index = parameter_index;
        self
    }

    pub fn with_parameter_data(mut self, parameter_data: FundamentalParameterData) -> Self {
        self.parameter_data = parameter_data;
        self
    }

    pub fn with_beam_data(mut self, beam_data: BeamData) -> Self {
        self.beam_data = beam_data;
        self
    }

    pub fn with_beam_function(mut self, beam_function: ElectromagneticEmissionBeamFunction) -> Self {
        self.beam_function = beam_function;
        self
    }

    pub fn with_high_density_track_jam(mut self, high_density_track_jam: HighDensityTrackJam) -> Self {
        self.high_density_track_jam = high_density_track_jam;
        self
    }

    pub fn with_beam_status(mut self, beam_status: BeamStatusBeamState) -> Self {
        self.beam_status = beam_status;
        self
    }

    pub fn with_jamming_technique(mut self, jamming_technique: JammingTechnique) -> Self {
        self.jamming_technique = jamming_technique;
        self
    }

    pub fn with_track_jams(mut self, track_jam_data: &mut Vec<TrackJam>) -> Self {
        self.track_jam_data.append(track_jam_data);
        self
    }

    pub fn with_track_jam(mut self, track_jam_data: TrackJam) -> Self {
        self.track_jam_data.push(track_jam_data);
        self
    }

    pub fn beam_data_length_bytes(&self) -> u16 {
        BEAM_BASE_LENGTH + (TRACK_JAM_BASE_LENGTH * self.track_jam_data.len() as u16)
    }
}

/// 6.2.22 EE Fundamental Parameter Data record
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct FundamentalParameterData {
    pub frequency: f32,
    pub frequency_range: f32,
    pub effective_power: f32,
    pub pulse_repetition_frequency: f32,
    pub pulse_width: f32,
}

impl FundamentalParameterData {
    pub fn new() -> Self {
        Self {
            frequency: 0.0,
            frequency_range: 0.0,
            effective_power: 0.0,
            pulse_repetition_frequency: 0.0,
            pulse_width: 0.0
        }
    }

    pub fn with_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn with_frequency_range(mut self, frequency_range: f32) -> Self {
        self.frequency_range = frequency_range;
        self
    }

    pub fn with_effective_power(mut self, effective_power: f32) -> Self {
        self.effective_power = effective_power;
        self
    }

    pub fn with_pulse_repetition_frequency(mut self, pulse_repetition_frequency: f32) -> Self {
        self.pulse_repetition_frequency = pulse_repetition_frequency;
        self
    }

    pub fn with_pulse_width(mut self, pulse_width: f32) -> Self {
        self.pulse_width = pulse_width;
        self
    }
}

/// 6.2.49 Jamming Technique record
#[derive(Clone, Default, Debug, PartialEq)]
pub struct JammingTechnique {
    pub kind: u8,
    pub category: u8,
    pub subcategory: u8,
    pub specific: u8,
}

impl JammingTechnique {
    pub fn new() -> Self {
        Self {
            kind: 0,
            category: 0,
            subcategory: 0,
            specific: 0
        }
    }

    pub fn with_kind(mut self, kind: u8) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_category(mut self, category: u8) -> Self {
        self.category = category;
        self
    }

    pub fn with_subcategory(mut self, subcategory: u8) -> Self {
        self.subcategory = subcategory;
        self
    }

    pub fn with_specific(mut self, specific: u8) -> Self {
        self.specific = specific;
        self
    }
}

/// 6.2.90 Track/Jam Data record
#[derive(Clone, Default, Debug, PartialEq)]
pub struct TrackJam {
    pub entity_id: EntityId,
    pub emitter: u8,
    pub beam: u8,
}

impl TrackJam {
    pub fn new() -> Self {
        Self {
            entity_id: EntityId::default(),
            emitter: 0,
            beam: 0
        }
    }

    pub fn with_entity_id(mut self, entity_id: EntityId) -> Self {
        self.entity_id = entity_id;
        self
    }

    pub fn with_emitter(mut self, emitter: u8) -> Self {
        self.emitter = emitter;
        self
    }

    pub fn with_beam(mut self, beam: u8) -> Self {
        self.beam = beam;
        self
    }
}