use crate::common::model::{
    length_padded_to_num, EntityId, EntityType, Location, Orientation, PduBody, VectorF32,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::{EIGHT_OCTETS, ZERO_OCTETS};
use crate::enumerations::{
    PduType, TransmitterAntennaPatternReferenceSystem, TransmitterAntennaPatternType,
    TransmitterCryptoSystem, TransmitterDetailAmplitudeAngleModulation,
    TransmitterDetailAmplitudeModulation, TransmitterDetailAngleModulation,
    TransmitterDetailCarrierPhaseShiftModulation, TransmitterDetailCombinationModulation,
    TransmitterDetailPulseModulation, TransmitterDetailSATCOMModulation,
    TransmitterDetailUnmodulatedModulation, TransmitterInputSource, TransmitterMajorModulation,
    TransmitterModulationTypeSystem, TransmitterTransmitState, VariableRecordType,
};
use crate::transmitter::builder::TransmitterBuilder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const BASE_TRANSMITTER_BODY_LENGTH: u16 = 92;
pub const BEAM_ANTENNA_PATTERN_OCTETS: u16 = 40;
pub const BASE_VTP_RECORD_LENGTH: u16 = 6;

/// 5.8.3 Transmitter PDU
///
/// 7.7.2 Transmitter PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[must_use]
    pub fn builder() -> TransmitterBuilder {
        TransmitterBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> TransmitterBuilder {
        TransmitterBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Transmitter(self)
    }
}

impl BodyInfo for Transmitter {
    fn body_length(&self) -> u16 {
        BASE_TRANSMITTER_BODY_LENGTH
            + self
                .modulation_parameters
                .as_ref()
                .map_or(ZERO_OCTETS as u16, |params| params.len() as u16)
            + self
                .antenna_pattern
                .as_ref()
                .map_or(ZERO_OCTETS as u16, |_| BEAM_ANTENNA_PATTERN_OCTETS)
            + self
                .variable_transmitter_parameters
                .iter()
                .map(|vtp| {
                    length_padded_to_num(
                        BASE_VTP_RECORD_LENGTH as usize + vtp.fields.len(),
                        EIGHT_OCTETS,
                    )
                    .record_length as u16
                })
                .sum::<u16>()
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModulationType {
    pub spread_spectrum: SpreadSpectrum,
    pub major_modulation: TransmitterMajorModulation,
    pub radio_system: TransmitterModulationTypeSystem,
}

impl Default for ModulationType {
    fn default() -> Self {
        Self::new()
    }
}

impl ModulationType {
    #[must_use]
    pub fn new() -> Self {
        Self {
            spread_spectrum: SpreadSpectrum::default(),
            major_modulation: TransmitterMajorModulation::default(),
            radio_system: TransmitterModulationTypeSystem::default(),
        }
    }

    #[must_use]
    pub fn with_spread_spectrum(mut self, spread_spectrum: SpreadSpectrum) -> Self {
        self.spread_spectrum = spread_spectrum;
        self
    }

    #[must_use]
    pub fn with_major_modulation(mut self, major_modulation: TransmitterMajorModulation) -> Self {
        self.major_modulation = major_modulation;
        self
    }

    #[must_use]
    pub fn with_radio_system(mut self, radio_system: TransmitterModulationTypeSystem) -> Self {
        self.radio_system = radio_system;
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpreadSpectrum {
    pub frequency_hopping: bool,
    pub pseudo_noise: bool,
    pub time_hopping: bool,
}

impl Default for SpreadSpectrum {
    fn default() -> Self {
        Self::new()
    }
}

impl SpreadSpectrum {
    #[must_use]
    pub fn new() -> Self {
        Self {
            frequency_hopping: Default::default(),
            pseudo_noise: Default::default(),
            time_hopping: Default::default(),
        }
    }

    #[must_use]
    pub fn new_with_values(
        frequency_hopping: bool,
        pseudo_noise: bool,
        time_hopping: bool,
    ) -> Self {
        Self {
            frequency_hopping,
            pseudo_noise,
            time_hopping,
        }
    }

    #[must_use]
    pub fn with_frequency_hopping(mut self) -> Self {
        self.frequency_hopping = true;
        self
    }

    #[must_use]
    pub fn with_pseudo_noise(mut self) -> Self {
        self.pseudo_noise = true;
        self
    }

    #[must_use]
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
        } else {
            spectrum
        };
        let spectrum = if value.pseudo_noise {
            spectrum | BIT_1
        } else {
            spectrum
        };
        let spectrum = if value.time_hopping {
            spectrum | BIT_2
        } else {
            spectrum
        };
        #[allow(clippy::let_and_return)]
        spectrum
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
            CryptoMode::Baseband => 0u16,
            CryptoMode::Diphase => 1u16,
        };
        (value.pseudo_crypto_key << 1) + crypto_mode
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        if value {
            CryptoMode::Diphase
        } else {
            CryptoMode::Baseband
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            beam_direction: Orientation::default(),
            azimuth_beamwidth: 0.0,
            elevation_beamwidth: 0.0,
            reference_system: TransmitterAntennaPatternReferenceSystem::default(),
            e_z: 0.0,
            e_x: 0.0,
            phase: 0.0,
        }
    }

    #[must_use]
    pub fn with_beam_direction(mut self, beam_direction: Orientation) -> Self {
        self.beam_direction = beam_direction;
        self
    }

    #[must_use]
    pub fn with_azimuth_beamwidth(mut self, azimuth_beamwidth: f32) -> Self {
        self.azimuth_beamwidth = azimuth_beamwidth;
        self
    }

    #[must_use]
    pub fn with_elevation_beamwidth(mut self, elevation_beamwidth: f32) -> Self {
        self.elevation_beamwidth = elevation_beamwidth;
        self
    }

    #[must_use]
    pub fn with_reference_system(
        mut self,
        reference_system: TransmitterAntennaPatternReferenceSystem,
    ) -> Self {
        self.reference_system = reference_system;
        self
    }

    #[must_use]
    pub fn with_e_z(mut self, e_z: f32) -> Self {
        self.e_z = e_z;
        self
    }

    #[must_use]
    pub fn with_e_x(mut self, e_x: f32) -> Self {
        self.e_x = e_x;
        self
    }

    #[must_use]
    pub fn with_phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            record_type: VariableRecordType::default(),
            fields: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_record_type(mut self, record_type: VariableRecordType) -> Self {
        self.record_type = record_type;
        self
    }

    #[must_use]
    pub fn with_fields(mut self, fields: Vec<u8>) -> Self {
        self.fields = fields;
        self
    }
}

impl TransmitterMajorModulation {
    #[must_use]
    pub fn new_from_bytes_with_detail(major_modulation: u16, detail: u16) -> Self {
        let major_modulation = TransmitterMajorModulation::from(major_modulation);
        match major_modulation {
            TransmitterMajorModulation::NoStatement => TransmitterMajorModulation::NoStatement,
            TransmitterMajorModulation::Amplitude(_) => TransmitterMajorModulation::Amplitude(
                TransmitterDetailAmplitudeModulation::from(detail),
            ),
            TransmitterMajorModulation::AmplitudeAndAngle(_) => {
                TransmitterMajorModulation::AmplitudeAndAngle(
                    TransmitterDetailAmplitudeAngleModulation::from(detail),
                )
            }
            TransmitterMajorModulation::Angle(_) => {
                TransmitterMajorModulation::Angle(TransmitterDetailAngleModulation::from(detail))
            }
            TransmitterMajorModulation::Combination(_) => TransmitterMajorModulation::Combination(
                TransmitterDetailCombinationModulation::from(detail),
            ),
            TransmitterMajorModulation::Pulse(_) => {
                TransmitterMajorModulation::Pulse(TransmitterDetailPulseModulation::from(detail))
            }
            TransmitterMajorModulation::Unmodulated(_) => TransmitterMajorModulation::Unmodulated(
                TransmitterDetailUnmodulatedModulation::from(detail),
            ),
            TransmitterMajorModulation::CarrierPhaseShiftModulation_CPSM_(_) => {
                TransmitterMajorModulation::CarrierPhaseShiftModulation_CPSM_(
                    TransmitterDetailCarrierPhaseShiftModulation::from(detail),
                )
            }
            TransmitterMajorModulation::SATCOM(_) => {
                TransmitterMajorModulation::SATCOM(TransmitterDetailSATCOMModulation::from(detail))
            }
            TransmitterMajorModulation::Unspecified(_) => {
                TransmitterMajorModulation::Unspecified(detail)
            }
        }
    }

    #[must_use]
    pub fn to_bytes_with_detail(&self) -> (u16, u16) {
        match self {
            TransmitterMajorModulation::NoStatement => (0, 0),
            TransmitterMajorModulation::Amplitude(detail) => (1, (*detail).into()),
            TransmitterMajorModulation::AmplitudeAndAngle(detail) => (2, (*detail).into()),
            TransmitterMajorModulation::Angle(detail) => (3, (*detail).into()),
            TransmitterMajorModulation::Combination(detail) => (4, (*detail).into()),
            TransmitterMajorModulation::Pulse(detail) => (5, (*detail).into()),
            TransmitterMajorModulation::Unmodulated(detail) => (6, (*detail).into()),
            TransmitterMajorModulation::CarrierPhaseShiftModulation_CPSM_(detail) => {
                (7, (*detail).into())
            }
            TransmitterMajorModulation::SATCOM(detail) => (8, (*detail).into()),
            TransmitterMajorModulation::Unspecified(detail) => (9, *detail),
        }
    }
}
