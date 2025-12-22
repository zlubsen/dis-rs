use crate::common::{BodyInfo, Interaction};
use crate::constants::{EIGHT_OCTETS, FOUR_OCTETS, ONE_OCTET, TWENTY_OCTETS};
use crate::enumerations::{
    APAStatus, PduType, UAAcousticEmitterSystemFunction, UAAcousticSystemName,
    UAActiveEmissionParameterIndex, UAAdditionalPassiveActivityParameterIndex,
    UAPassiveParameterIndex, UAPropulsionPlantConfiguration, UAScanPattern,
    UAStateChangeUpdateIndicator,
};
use crate::model::{EntityId, EventId, PduBody, VectorF32};
use crate::underwater_acoustic::builder::UnderwaterAcousticBuilder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const BASE_UA_BODY_LENGTH: u16 = 20;

/// 5.7.5 Underwater Acoustic (UA) PDU
///
/// 7.6.4 Underwater Acoustic (UA) PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnderwaterAcoustic {
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_change_update_indicator: UAStateChangeUpdateIndicator,
    pub passive_parameter_index: UAPassiveParameterIndex,
    pub propulsion_plant_configuration: PropulsionPlantConfiguration,
    pub shafts: Vec<Shaft>,
    pub apas: Vec<APA>,
    pub emitter_systems: Vec<UAEmitterSystem>,
}

impl UnderwaterAcoustic {
    #[must_use]
    pub fn builder() -> UnderwaterAcousticBuilder {
        UnderwaterAcousticBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> UnderwaterAcousticBuilder {
        UnderwaterAcousticBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::UnderwaterAcoustic(self)
    }
}

impl BodyInfo for UnderwaterAcoustic {
    fn body_length(&self) -> u16 {
        BASE_UA_BODY_LENGTH
            + self.shafts.iter().map(Shaft::record_length).sum::<u16>()
            + self.apas.iter().map(APA::record_length).sum::<u16>()
            + self
                .emitter_systems
                .iter()
                .map(UAEmitterSystem::record_length)
                .sum::<u16>()
    }

    fn body_type(&self) -> PduType {
        PduType::UnderwaterAcoustic
    }
}

impl Interaction for UnderwaterAcoustic {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.emitting_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

impl From<UnderwaterAcoustic> for PduBody {
    #[inline]
    fn from(value: UnderwaterAcoustic) -> Self {
        value.into_pdu_body()
    }
}

/// Implementation of UID 149
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PropulsionPlantConfiguration {
    pub configuration: UAPropulsionPlantConfiguration,
    pub hull_mounted_masker: bool,
}

impl PropulsionPlantConfiguration {
    #[must_use]
    pub fn with_configuration(mut self, configuration: UAPropulsionPlantConfiguration) -> Self {
        self.configuration = configuration;
        self
    }

    #[must_use]
    pub fn with_hull_mounted_masker(mut self, masker_on: bool) -> Self {
        self.hull_mounted_masker = masker_on;
        self
    }

    #[must_use]
    pub fn record_length(&self) -> u16 {
        ONE_OCTET as u16
    }
}

/// 7.6.4 Underwater Acoustic (UA) PDU
///
/// Table 164—UA PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Shaft {
    pub current_rpm: i16,
    pub ordered_rpm: i16,
    pub rpm_rate_of_change: i32,
}

impl Shaft {
    #[must_use]
    pub fn with_current_rpm(mut self, rpm: i16) -> Self {
        self.current_rpm = rpm;
        self
    }

    #[must_use]
    pub fn with_ordered_rpm(mut self, rpm: i16) -> Self {
        self.ordered_rpm = rpm;
        self
    }

    #[must_use]
    pub fn with_rpm_rate_of_change(mut self, rate_of_change: i32) -> Self {
        self.rpm_rate_of_change = rate_of_change;
        self
    }

    #[must_use]
    pub fn record_length(&self) -> u16 {
        EIGHT_OCTETS as u16
    }
}

/// 7.6.4 Underwater Acoustic (UA) PDU
///
/// Table 163—APA Parameter Index record
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct APA {
    pub parameter: UAAdditionalPassiveActivityParameterIndex,
    pub status: APAStatus,
    pub value: i16,
}

impl APA {
    #[must_use]
    pub fn with_parameter(mut self, parameter: UAAdditionalPassiveActivityParameterIndex) -> Self {
        self.parameter = parameter;
        self
    }

    #[must_use]
    pub fn with_status(mut self, status: APAStatus) -> Self {
        self.status = status;
        self
    }
    #[must_use]
    pub fn with_value(mut self, value: i16) -> Self {
        self.value = value;
        self
    }

    #[must_use]
    pub fn record_length(&self) -> u16 {
        FOUR_OCTETS as u16
    }
}

/// Figure 50 — General form of emitter systems in the UA PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UAEmitterSystem {
    pub acoustic_emitter_system: AcousticEmitterSystem,
    pub location: VectorF32,
    pub beams: Vec<UABeam>,
}

impl UAEmitterSystem {
    #[must_use]
    pub fn with_acoustic_emitter_system(
        mut self,
        acoustic_emitter_system: AcousticEmitterSystem,
    ) -> Self {
        self.acoustic_emitter_system = acoustic_emitter_system;
        self
    }

    #[must_use]
    pub fn with_location(mut self, location: VectorF32) -> Self {
        self.location = location;
        self
    }

    #[must_use]
    pub fn with_beam(mut self, beam: UABeam) -> Self {
        self.beams.push(beam);
        self
    }

    #[must_use]
    pub fn with_beams(mut self, beams: Vec<UABeam>) -> Self {
        self.beams = beams;
        self
    }

    #[must_use]
    pub fn record_length(&self) -> u16 {
        TWENTY_OCTETS as u16 + self.beams.iter().map(UABeam::record_length).sum::<u16>()
    }
}

/// 6.2.2 Acoustic Emitter System record
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AcousticEmitterSystem {
    pub acoustic_system_name: UAAcousticSystemName,
    pub function: UAAcousticEmitterSystemFunction,
    pub acoustic_id_number: u8,
}

impl AcousticEmitterSystem {
    #[must_use]
    pub fn with_acoustic_system_name(mut self, acoustic_system_name: UAAcousticSystemName) -> Self {
        self.acoustic_system_name = acoustic_system_name;
        self
    }

    #[must_use]
    pub fn with_function(mut self, function: UAAcousticEmitterSystemFunction) -> Self {
        self.function = function;
        self
    }

    #[must_use]
    pub fn with_acoustic_id_number(mut self, acoustic_id_number: u8) -> Self {
        self.acoustic_id_number = acoustic_id_number;
        self
    }

    #[must_use]
    pub fn record_length(&self) -> u16 {
        FOUR_OCTETS as u16
    }
}

/// Custom record for an UA Beam
///
/// 7.6.4 Underwater Acoustic (UA) PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UABeam {
    pub beam_data_length: u8,
    pub beam_id_number: u8,
    pub fundamental_parameters: UAFundamentalParameterData,
}

impl UABeam {
    #[must_use]
    pub fn with_beam_data_length(mut self, beam_data_length: u8) -> Self {
        self.beam_data_length = beam_data_length;
        self
    }

    #[must_use]
    pub fn with_beam_id_number(mut self, beam_id_number: u8) -> Self {
        self.beam_id_number = beam_id_number;
        self
    }

    #[must_use]
    pub fn with_fundamental_parameters(
        mut self,
        fundamental_parameters: UAFundamentalParameterData,
    ) -> Self {
        self.fundamental_parameters = fundamental_parameters;
        self
    }

    #[must_use]
    pub fn record_length(&self) -> u16 {
        FOUR_OCTETS as u16 + self.fundamental_parameters.record_length()
    }
}

/// 6.2.91 UA Fundamental Parameter Data record
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UAFundamentalParameterData {
    pub active_emission_parameter_index: UAActiveEmissionParameterIndex,
    pub scan_pattern: UAScanPattern,
    pub beam_center_azimuth: f32,
    pub azimuthal_beamwidth: f32,
    pub beam_center_depression_elevation: f32,
    pub depression_elevation_beamwidth: f32,
}

impl UAFundamentalParameterData {
    #[must_use]
    pub fn with_active_emission_parameter_index(
        mut self,
        active_emission_parameter_index: UAActiveEmissionParameterIndex,
    ) -> Self {
        self.active_emission_parameter_index = active_emission_parameter_index;
        self
    }

    #[must_use]
    pub fn with_scan_pattern(mut self, scan_pattern: UAScanPattern) -> Self {
        self.scan_pattern = scan_pattern;
        self
    }

    #[must_use]
    pub fn with_beam_center_azimuth(mut self, beam_center_azimuth: f32) -> Self {
        self.beam_center_azimuth = beam_center_azimuth;
        self
    }

    #[must_use]
    pub fn with_azimuthal_beamwidth(mut self, azimuthal_beamwidth: f32) -> Self {
        self.azimuthal_beamwidth = azimuthal_beamwidth;
        self
    }

    #[must_use]
    pub fn with_beam_center_depression_elevation(
        mut self,
        beam_center_depression_elevation: f32,
    ) -> Self {
        self.beam_center_depression_elevation = beam_center_depression_elevation;
        self
    }

    #[must_use]
    pub fn with_depression_elevation_beamwidth(
        mut self,
        depression_elevation_beamwidth: f32,
    ) -> Self {
        self.depression_elevation_beamwidth = depression_elevation_beamwidth;
        self
    }
    #[must_use]
    pub fn record_length(&self) -> u16 {
        TWENTY_OCTETS as u16
    }
}
