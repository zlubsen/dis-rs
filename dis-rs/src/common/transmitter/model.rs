use crate::common::model::{EntityId, Location};
use crate::enumerations::{VariableRecordType, TransmitterTransmitState, TransmitterInputSource, TransmitterAntennaPatternType, TransmitterCryptoSystem, TransmitterMajorModulation, TransmitterModulationTypeSystem, TransmitterAntennaPatternReferenceSystem};
use crate::{EntityType, length_padded_to_num_bytes, Orientation, PduBody, PduType, VectorF32};
use crate::common::{BodyInfo, Interaction};
use crate::constants::{EIGHT_OCTETS, ZERO_OCTETS};

const BASE_TRANSMITTER_BODY_LENGTH: u16 = 92;
pub const BEAM_ANTENNA_PATTERN_OCTETS: u16 = 40;
pub const BASE_VTP_RECORD_LENGTH: u16 = 6;

pub struct Transmitter {
    pub radio_reference_id: EntityId,
    pub radio_number: u16,
    pub radio_type: EntityType,
    pub transmit_state: TransmitterTransmitState,
    pub input_source: TransmitterInputSource,
    pub antenna_location: Location,
    pub relative_antenna_location: VectorF32,
    pub antenna_pattern_type: TransmitterAntennaPatternType,
    pub frequency: u64,
    pub transmit_frequency_bandwidth: f32,
    pub power: f32,
    pub modulation_type: ModulationType,
    pub crypto_system: TransmitterCryptoSystem,
    pub crypto_key_id: CryptoKeyId,
    pub modulation_parameters: Option<Vec<u8>>,
    pub antenna_pattern: Option<BeamAntennaPattern>,
    pub variable_transmitter_parameters: Vec<VariableTransmitterParameter>,
}

impl Default for Transmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl Transmitter {
    pub fn new() -> Self {
        Self {
            radio_reference_id: Default::default(),
            radio_number: 0,
            radio_type: Default::default(),
            transmit_state: Default::default(),
            input_source: Default::default(),
            antenna_location: Default::default(),
            relative_antenna_location: Default::default(),
            antenna_pattern_type: Default::default(),
            frequency: 0,
            transmit_frequency_bandwidth: 0.0,
            power: 0.0,
            modulation_type: Default::default(),
            crypto_system: Default::default(),
            crypto_key_id: Default::default(),
            modulation_parameters: None,
            antenna_pattern: None,
            variable_transmitter_parameters: vec![],
        }
    }

    pub fn with_radio_reference_id(mut self, radio_reference_id: EntityId) -> Self {
        self.radio_reference_id = radio_reference_id;
        self
    }

    pub fn with_radio_number(mut self, radio_number: u16) -> Self {
        self.radio_number = radio_number;
        self
    }

    pub fn with_radio_type(mut self, radio_type: EntityType) -> Self {
        self.radio_type = radio_type;
        self
    }

    pub fn with_transmit_state(mut self, transmit_state: TransmitterTransmitState) -> Self {
        self.transmit_state = transmit_state;
        self
    }

    pub fn with_input_source(mut self, input_source: TransmitterInputSource) -> Self {
        self.input_source = input_source;
        self
    }

    pub fn with_antenna_location(mut self, antenna_location: Location) -> Self {
        self.antenna_location = antenna_location;
        self
    }

    pub fn with_relative_antenna_location(mut self, relative_antenna_location: VectorF32) -> Self {
        self.relative_antenna_location = relative_antenna_location;
        self
    }

    pub fn with_antenna_pattern_type(mut self, antenna_pattern_type: TransmitterAntennaPatternType) -> Self {
        self.antenna_pattern_type = antenna_pattern_type;
        self
    }

    pub fn with_frequency(mut self, frequency: u64) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn with_transmit_frequency_bandwidth(mut self, transmit_frequency_bandwidth: f32) -> Self {
        self.transmit_frequency_bandwidth = transmit_frequency_bandwidth;
        self
    }
    pub fn with_power(mut self, power: f32) -> Self {
        self.power = power;
        self
    }

    pub fn with_modulation_type(mut self, modulation_type: ModulationType) -> Self {
        self.modulation_type = modulation_type;
        self
    }

    pub fn with_crypto_system(mut self, crypto_system: TransmitterCryptoSystem) -> Self {
        self.crypto_system = crypto_system;
        self
    }

    pub fn with_crypto_key_id(mut self, crypto_key_id: CryptoKeyId) -> Self {
        self.crypto_key_id = crypto_key_id;
        self
    }
    pub fn with_modulation_parameters(mut self, modulation_parameters: Vec<u8>) -> Self {
        self.modulation_parameters = Some(modulation_parameters);
        self
    }

    pub fn with_antenna_pattern(mut self, antenna_pattern: BeamAntennaPattern) -> Self {
        self.antenna_pattern = Some(antenna_pattern);
        self
    }

    pub fn with_variable_transmitter_parameters(mut self, variable_transmitter_parameters: Vec<VariableTransmitterParameter>) -> Self {
        self.variable_transmitter_parameters = variable_transmitter_parameters;
        self
    }

    pub fn with_variable_transmitter_parameter(mut self, variable_transmitter_parameter: VariableTransmitterParameter) -> Self {
        self.variable_transmitter_parameters.push(variable_transmitter_parameter);
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Transmitter(self)
    }
}

impl BodyInfo for Transmitter {
    fn body_length(&self) -> u16 {
        BASE_TRANSMITTER_BODY_LENGTH +
            self.modulation_parameters.as_ref().map_or(ZERO_OCTETS as u16, |params|params.len() as u16) +
            self.antenna_pattern.as_ref().map_or(ZERO_OCTETS as u16, |_| BEAM_ANTENNA_PATTERN_OCTETS) +
            self.variable_transmitter_parameters.iter().map(|vtp|
                length_padded_to_num_bytes(
                    BASE_VTP_RECORD_LENGTH as usize + vtp.fields.len(),
                    EIGHT_OCTETS).record_length_bytes as u16
            ).sum::<u16>()
    }

    fn body_type(&self) -> PduType {
        PduType::Transmitter
    }
}

impl Interaction for Transmitter {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.radio_reference_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

pub struct ModulationType {
    pub spread_spectrum: SpreadSpectrum,
    pub major_modulation: TransmitterMajorModulation,
    pub radio_system: TransmitterModulationTypeSystem
}

impl Default for ModulationType {
    fn default() -> Self {
        Self::new()
    }
}

impl ModulationType {
    pub fn new() -> Self {
        Self {
            spread_spectrum: Default::default(),
            major_modulation: Default::default(),
            radio_system: Default::default(),
        }
    }

    pub fn with_spread_spectrum(mut self, spread_spectrum: SpreadSpectrum) -> Self {
        self.spread_spectrum = spread_spectrum;
        self
    }

    pub fn with_major_modulation(mut self, major_modulation: TransmitterMajorModulation) -> Self {
        self.major_modulation = major_modulation;
        self
    }

    pub fn with_radio_system(mut self, radio_system: TransmitterModulationTypeSystem) -> Self {
        self.radio_system = radio_system;
        self
    }
}

pub struct SpreadSpectrum {
    pub frequency_hopping: bool,
    pub pseudo_noise: bool,
    pub time_hopping: bool
}

impl Default for SpreadSpectrum {
    fn default() -> Self {
        Self::new()
    }
}

impl SpreadSpectrum {
    pub fn new() -> Self {
        Self {
            frequency_hopping: Default::default(),
            pseudo_noise: Default::default(),
            time_hopping: Default::default(),
        }
    }

    pub fn new_with_values(frequency_hopping: bool, pseudo_noise: bool, time_hopping: bool) -> Self {
        Self {
            frequency_hopping,
            pseudo_noise,
            time_hopping,
        }
    }

    pub fn with_frequency_hopping(mut self) -> Self {
        self.frequency_hopping = true;
        self
    }

    pub fn with_pseudo_noise(mut self) -> Self {
        self.pseudo_noise = true;
        self
    }

    pub fn with_time_hopping(mut self) -> Self {
        self.time_hopping = true;
        self
    }
}

pub struct CryptoKeyId {
    pub pseudo_crypto_key: u16,
    pub crypto_mode: CryptoMode,
}

impl Default for CryptoKeyId {
    fn default() -> Self {
        Self {
            pseudo_crypto_key: Default::default(),
            crypto_mode: CryptoMode::Baseband,
        }
    }
}

pub enum CryptoMode {
    Baseband,
    Diphase,
}

impl Default for CryptoMode {
    fn default() -> Self {
        Self::Baseband
    }
}

impl From<bool> for CryptoMode {
    fn from(value: bool) -> Self {
        match value {
            true => { CryptoMode::Diphase }
            false => { CryptoMode::Baseband }
        }
    }
}

pub struct BeamAntennaPattern {
    pub beam_direction: Orientation,
    pub azimuth_beamwidth: f32,
    pub elevation_beamwidth: f32,
    pub reference_system: TransmitterAntennaPatternReferenceSystem,
    pub e_z: f32,
    pub e_x: f32,
    pub phase: f32,
}

impl Default for BeamAntennaPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl BeamAntennaPattern {
    pub fn new() -> Self {
        Self {
            beam_direction: Default::default(),
            azimuth_beamwidth: 0.0,
            elevation_beamwidth: 0.0,
            reference_system: Default::default(),
            e_z: 0.0,
            e_x: 0.0,
            phase: 0.0,
        }
    }

    pub fn with_beam_direction(mut self, beam_direction: Orientation) -> Self {
        self.beam_direction = beam_direction;
        self
    }

    pub fn with_azimuth_beamwidth(mut self, azimuth_beamwidth: f32) -> Self {
        self.azimuth_beamwidth = azimuth_beamwidth;
        self
    }

    pub fn with_elevation_beamwidth(mut self, elevation_beamwidth: f32) -> Self {
        self.elevation_beamwidth = elevation_beamwidth;
        self
    }

    pub fn with_reference_system(mut self, reference_system: TransmitterAntennaPatternReferenceSystem) -> Self {
        self.reference_system = reference_system;
        self
    }

    pub fn with_e_z(mut self, e_z: f32) -> Self {
        self.e_z = e_z;
        self
    }

    pub fn with_e_x(mut self, e_x: f32) -> Self {
        self.e_x = e_x;
        self
    }

    pub fn with_phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }
}

pub struct VariableTransmitterParameter {
    pub record_type: VariableRecordType,
    pub fields: Vec<u8>,
}

impl Default for VariableTransmitterParameter {
    fn default() -> Self {
        Self::new()
    }
}

impl VariableTransmitterParameter {
    pub fn new() -> Self {
        Self {
            record_type: Default::default(),
            fields: Vec::new(),
        }
    }

    pub fn with_record_type(mut self, record_type: VariableRecordType) -> Self {
        self.record_type = record_type;
        self
    }

    pub fn with_fields(mut self, fields: Vec<u8>) -> Self {
        self.fields = fields;
        self
    }
}