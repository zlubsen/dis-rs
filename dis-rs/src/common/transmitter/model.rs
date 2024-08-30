use crate::common::model::{EntityId, Location, EntityType, length_padded_to_num, Orientation, PduBody, VectorF32};
use crate::enumerations::{PduType, VariableRecordType, TransmitterTransmitState, TransmitterInputSource, TransmitterAntennaPatternType, TransmitterCryptoSystem, TransmitterMajorModulation, TransmitterModulationTypeSystem, TransmitterAntennaPatternReferenceSystem};
use crate::common::{BodyInfo, Interaction};
use crate::constants::{EIGHT_OCTETS, ZERO_OCTETS};
use crate::transmitter::builder::TransmitterBuilder;

const BASE_TRANSMITTER_BODY_LENGTH: u16 = 92;
pub const BEAM_ANTENNA_PATTERN_OCTETS: u16 = 40;
pub const BASE_VTP_RECORD_LENGTH: u16 = 6;

#[derive(Clone, Debug, Default, PartialEq)]
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

impl Transmitter {
    pub fn builder() -> TransmitterBuilder {
        TransmitterBuilder::new()
    }

    pub fn into_builder(self) -> TransmitterBuilder {
        TransmitterBuilder::new_from_body(self)
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
                length_padded_to_num(
                    BASE_VTP_RECORD_LENGTH as usize + vtp.fields.len(),
                    EIGHT_OCTETS).record_length as u16
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<u16> for SpreadSpectrum {
    fn from(spread_spectrum_values: u16) -> Self {
        let frequency_hopping = ((spread_spectrum_values >> 15) & 0x0001) != 0;
        let pseudo_noise = ((spread_spectrum_values >> 14) & 0x0001) != 0;
        let time_hopping = ((spread_spectrum_values >> 13) & 0x0001) != 0;

        SpreadSpectrum::new_with_values(frequency_hopping, pseudo_noise, time_hopping)
    }
}

impl From<&SpreadSpectrum> for u16 {
    fn from(value: &SpreadSpectrum) -> Self {
        const BIT_0: u16 = 0x8000;
        const BIT_1: u16 = 0x4000;
        const BIT_2: u16 = 0x2000;

        let spectrum = 0u16;
        let spectrum = if value.frequency_hopping {
            spectrum | BIT_0
        } else { spectrum };
        let spectrum = if value.pseudo_noise {
            spectrum | BIT_1
        } else { spectrum };
        let spectrum = if value.time_hopping {
            spectrum | BIT_2
        } else { spectrum };
        spectrum
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<u16> for CryptoKeyId {
    fn from(value: u16) -> Self {
        let pseudo_crypto_key = value >> 1;
        let crypto_mode = (value & 0x0001) != 0;
        let crypto_mode = CryptoMode::from(crypto_mode);

        Self {
            pseudo_crypto_key,
            crypto_mode,
        }
    }
}

impl From<CryptoKeyId> for u16 {
    fn from(value: CryptoKeyId) -> Self {
        let crypto_mode = match value.crypto_mode {
            CryptoMode::Baseband => { 0u16 }
            CryptoMode::Diphase => { 1u16 }
        };
        (value.pseudo_crypto_key << 1) + crypto_mode
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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
