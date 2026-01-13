use crate::codec::{
    Codec, CodecOptions, CodecStateResult, CodecUpdateMode, DecoderState, EncoderState,
};
use crate::records::codec::{
    decode_entity_coordinate_vector, decode_world_coordinates, encode_entity_coordinate_vector,
    encode_world_coordinates,
};
use crate::records::model::{BeamAntennaPattern, EntityId, EntityType};
use crate::transmitter::model::{
    ModulationType, TransmitFrequencyBandwidthFloat, Transmitter, TransmitterFrequencyFloat,
    TransmitterUnits,
};
use crate::types::model::{CdisFloat, UVINT16, UVINT8};
use crate::{BodyProperties, CdisBody};
use dis_rs::enumerations::{TransmitterCryptoSystem, TransmitterInputSource};
use dis_rs::model::{
    EntityType as DisEntityType, Location as DisLocation, Location, Orientation, PduBody, VectorF32,
};
use dis_rs::BodyRaw;
use std::time::Instant;

type Counterpart = dis_rs::transmitter::model::Transmitter;

pub(crate) fn encode_transmitter_body_and_update_state(
    dis_body: &Counterpart,
    state: &mut EncoderState,
    options: &CodecOptions,
) -> (CdisBody, CodecStateResult) {
    let state_for_id = state
        .transmitter
        .get(&(dis_body.radio_reference_id, dis_body.radio_number));

    let (cdis_body, state_result) = Transmitter::encode(dis_body, state_for_id, options);

    if state_result == CodecStateResult::StateUpdateTransmitter {
        state
            .transmitter
            .entry((dis_body.radio_reference_id, dis_body.radio_number))
            .and_modify(|tr| tr.heartbeat = Instant::now())
            .or_default();
    }

    (cdis_body.into_cdis_body(), state_result)
}

pub(crate) fn decode_transmitter_body_and_update_state(
    cdis_body: &Transmitter,
    state: &mut DecoderState,
    options: &CodecOptions,
) -> (PduBody, CodecStateResult) {
    let state_for_id = state.transmitter.get(&(
        dis_rs::model::EntityId::from(&cdis_body.radio_reference_id),
        cdis_body.radio_number.value,
    ));
    let (dis_body, state_result) = cdis_body.decode(state_for_id, options);

    if state_result == CodecStateResult::StateUpdateTransmitter {
        state
            .transmitter
            .entry((
                dis_rs::model::EntityId::from(&cdis_body.radio_reference_id),
                cdis_body.radio_number.value,
            ))
            .and_modify(|tr| {
                tr.heartbeat = Instant::now();
                tr.radio_type = dis_body.radio_type;
                tr.antenna_location = dis_body.antenna_location;
                tr.frequency = dis_body.frequency;
                tr.transmit_frequency_bandwidth = dis_body.transmit_frequency_bandwidth;
                tr.power = dis_body.power;
                tr.modulation_type = dis_body.modulation_type;
            })
            .or_insert(DecoderStateTransmitter::new(&dis_body));
    }

    (dis_body.into_pdu_body(), state_result)
}

impl Transmitter {
    #[must_use]
    pub fn encode(
        item: &Counterpart,
        state: Option<&EncoderStateTransmitter>,
        options: &CodecOptions,
    ) -> (Self, CodecStateResult) {
        let (
            units,
            full_update_flag,
            radio_type,
            antenna_location,
            relative_antenna_location,
            frequency,
            transmit_frequency_bandwidth,
            power,
            modulation_type,
            state_result,
        ) = if options.update_mode == CodecUpdateMode::PartialUpdate
            && state
                .is_some_and(|state| !evaluate_timeout_for_transmitter(&state.heartbeat, options))
        {
            (
                TransmitterUnits::default(),
                false,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                CodecStateResult::StateUnaffected,
            )
        } else {
            let (antenna_location, antenna_location_units) =
                encode_world_coordinates(&item.antenna_location);
            // include when not zeroed
            let (relative_antenna_location, relative_antenna_location_units) =
                encode_entity_coordinate_vector(&item.relative_antenna_location);
            let units = TransmitterUnits {
                world_location_altitude: antenna_location_units,
                relative_antenna_location: relative_antenna_location_units,
            };
            (
                units,
                true,
                Some(EntityType::encode(&item.radio_type)),
                Some(antenna_location),
                Some(relative_antenna_location),
                Some(TransmitterFrequencyFloat::from_float(item.frequency as f64)),
                Some(TransmitFrequencyBandwidthFloat::from_float(
                    item.transmit_frequency_bandwidth,
                )),
                Some(item.power as u8),
                Some(ModulationType::from(&item.modulation_type)),
                CodecStateResult::StateUpdateTransmitter,
            )
        };

        // include when crypto fields are not zeroed
        let (crypto_system, crypto_key_id) =
            if item.crypto_system != TransmitterCryptoSystem::NoEncryptionDevice {
                (Some(item.crypto_system), Some(item.crypto_key_id))
            } else {
                (None, None)
            };

        // include when modulation parameters are present
        let modulation_parameters = if let Some(params) = &item.modulation_parameters {
            params.clone()
        } else {
            Vec::default()
        };

        // include when antenna_pattern is not zeroed
        let (antenna_pattern_type, antenna_pattern) = if let Some(pattern) = &item.antenna_pattern {
            (
                Some(item.antenna_pattern_type),
                Some(BeamAntennaPattern::encode(pattern)),
            )
        } else {
            (None, None)
        };

        let input_source: u8 = item.input_source.into();

        (
            Self {
                units,
                full_update_flag,
                radio_reference_id: EntityId::encode(&item.radio_reference_id),
                radio_number: UVINT16::from(item.radio_number),
                radio_type,
                transmit_state: item.transmit_state,
                input_source: UVINT8::from(input_source),
                antenna_location,
                relative_antenna_location,
                antenna_pattern_type,
                frequency,
                transmit_frequency_bandwidth,
                power,
                modulation_type,
                crypto_system,
                crypto_key_id,
                modulation_parameters,
                antenna_pattern,
                variable_transmitter_parameters: item.variable_transmitter_parameters.clone(),
            },
            state_result,
        )
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn decode(
        &self,
        state: Option<&DecoderStateTransmitter>,
        options: &CodecOptions,
    ) -> (Counterpart, CodecStateResult) {
        let (
            radio_type,
            antenna_location,
            relative_antenna_location,
            frequency,
            transmit_frequency_bandwidth,
            power,
            modulation_type,
            state_result,
        ) = match options.update_mode {
            CodecUpdateMode::FullUpdate => (
                self.radio_type.map(|rt| rt.decode()).unwrap_or_default(),
                decode_world_coordinates(
                    &self.antenna_location.unwrap_or_default(),
                    self.units.world_location_altitude,
                ),
                decode_entity_coordinate_vector(
                    &self.relative_antenna_location.unwrap_or_default(),
                    self.units.relative_antenna_location,
                ),
                self.frequency
                    .map(|freq| freq.to_float())
                    .unwrap_or_default() as u64,
                self.transmit_frequency_bandwidth
                    .map(|tfb| tfb.to_float())
                    .unwrap_or_default(),
                self.power.map(f32::from).unwrap_or_default(),
                dis_rs::transmitter::model::ModulationType::from(
                    &self.modulation_type.unwrap_or_default(),
                ),
                CodecStateResult::StateUnaffected,
            ),
            CodecUpdateMode::PartialUpdate => {
                if self.full_update_flag {
                    (
                        self.radio_type.map(|rt| rt.decode()).unwrap_or_default(),
                        decode_world_coordinates(
                            &self.antenna_location.unwrap_or_default(),
                            self.units.world_location_altitude,
                        ),
                        decode_entity_coordinate_vector(
                            &self.relative_antenna_location.unwrap_or_default(),
                            self.units.relative_antenna_location,
                        ),
                        self.frequency
                            .map(|freq| freq.to_float())
                            .unwrap_or_default() as u64,
                        self.transmit_frequency_bandwidth
                            .map(|tfb| tfb.to_float())
                            .unwrap_or_default(),
                        self.power.map(f32::from).unwrap_or_default(),
                        dis_rs::transmitter::model::ModulationType::from(
                            &self.modulation_type.unwrap_or_default(),
                        ),
                        CodecStateResult::StateUpdateTransmitter,
                    )
                } else {
                    (
                        self.radio_type.map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.radio_type
                                } else {
                                    DisEntityType::default()
                                }
                            },
                            |rt| rt.decode(),
                        ),
                        self.antenna_location.map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.antenna_location
                                } else {
                                    Location::default()
                                }
                            },
                            |loc| {
                                decode_world_coordinates(&loc, self.units.world_location_altitude)
                            },
                        ),
                        self.relative_antenna_location
                            .map_or_else(VectorF32::default, |ral| {
                                decode_entity_coordinate_vector(
                                    &ral,
                                    self.units.relative_antenna_location,
                                )
                            }),
                        self.frequency.map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.frequency
                                } else {
                                    Default::default()
                                }
                            },
                            |freq| freq.to_float() as u64,
                        ),
                        self.transmit_frequency_bandwidth.map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.transmit_frequency_bandwidth
                                } else {
                                    Default::default()
                                }
                            },
                            |tfb| tfb.to_float(),
                        ),
                        self.power.map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.power
                                } else {
                                    Default::default()
                                }
                            },
                            f32::from,
                        ),
                        self.modulation_type.map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.modulation_type
                                } else {
                                    dis_rs::transmitter::model::ModulationType::default()
                                }
                            },
                            |record| dis_rs::transmitter::model::ModulationType::from(&record),
                        ),
                        CodecStateResult::StateUnaffected,
                    )
                }
            }
        };

        (
            Counterpart::builder()
                .with_radio_reference_id(self.radio_reference_id.decode())
                .with_radio_number(self.radio_number.value)
                .with_radio_type(radio_type)
                .with_transmit_state(self.transmit_state)
                .with_input_source(TransmitterInputSource::from(self.input_source.value))
                .with_antenna_location(antenna_location)
                .with_relative_antenna_location(relative_antenna_location)
                .with_antenna_pattern_type(self.antenna_pattern_type.unwrap_or_default()) // zeroed when None?
                .with_frequency(frequency)
                .with_transmit_frequency_bandwidth(transmit_frequency_bandwidth)
                .with_power(power)
                .with_modulation_type(modulation_type)
                .with_crypto_system(self.crypto_system.unwrap_or_default()) // zeroed when None?
                .with_crypto_key_id(self.crypto_key_id.unwrap_or_default()) // zeroed when None?
                .with_modulation_parameters(self.modulation_parameters.clone())
                .with_antenna_pattern(
                    self.antenna_pattern
                        .map(|pattern| pattern.decode())
                        .unwrap_or_default(),
                ) // zeroed when None?
                .with_variable_transmitter_parameters(self.variable_transmitter_parameters.clone())
                .build(),
            state_result,
        )
    }
}

#[derive(Debug)]
pub struct EncoderStateTransmitter {
    pub heartbeat: Instant,
}

impl Default for EncoderStateTransmitter {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub struct DecoderStateTransmitter {
    pub heartbeat: Instant,
    pub radio_type: DisEntityType,
    pub antenna_location: DisLocation,
    pub frequency: u64,
    pub transmit_frequency_bandwidth: f32,
    pub power: f32,
    pub modulation_type: dis_rs::transmitter::model::ModulationType,
}

impl DecoderStateTransmitter {
    #[must_use]
    pub fn new(pdu: &Counterpart) -> Self {
        Self {
            heartbeat: Instant::now(),
            radio_type: pdu.radio_type,
            antenna_location: pdu.antenna_location,
            frequency: pdu.frequency,
            transmit_frequency_bandwidth: pdu.transmit_frequency_bandwidth,
            power: pdu.power,
            modulation_type: pdu.modulation_type,
        }
    }
}

impl Default for DecoderStateTransmitter {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
            radio_type: DisEntityType::default(),
            antenna_location: DisLocation::default(),
            frequency: 0,
            transmit_frequency_bandwidth: 0.0,
            power: 0.0,
            modulation_type: dis_rs::transmitter::model::ModulationType::default(),
        }
    }
}

fn evaluate_timeout_for_transmitter(last_heartbeat: &Instant, options: &CodecOptions) -> bool {
    let elapsed = last_heartbeat.elapsed().as_secs_f32();
    elapsed
        > (options.federation_parameters.HBT_PDU_TRANSMITTER * options.hbt_cdis_full_update_mplier)
}

impl Codec for BeamAntennaPattern {
    type Counterpart = dis_rs::transmitter::model::BeamAntennaPattern;
    const SCALING: f32 = 4095f32 / std::f32::consts::PI; // (2^12 - 1) = 4095

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            beam_direction_psi: (item.beam_direction.psi * Self::SCALING) as i16,
            beam_direction_theta: (item.beam_direction.theta * Self::SCALING) as i16,
            beam_direction_phi: (item.beam_direction.phi * Self::SCALING) as i16,
            az_beamwidth: (item.azimuth_beamwidth * Self::SCALING) as i16,
            el_beamwidth: (item.elevation_beamwidth * Self::SCALING) as i16,
            reference_system: item.reference_system,
            e_z: item.e_z as i16, // TODO is this encoding correct?)
            e_x: item.e_x as i16, // TODO is this encoding correct?)
            phase: (item.phase * Self::SCALING) as i16,
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_beam_direction(Orientation::new(
                f32::from(self.beam_direction_psi) / Self::SCALING,
                f32::from(self.beam_direction_theta) / Self::SCALING,
                f32::from(self.beam_direction_phi) / Self::SCALING,
            ))
            .with_azimuth_beamwidth(f32::from(self.az_beamwidth) / Self::SCALING)
            .with_elevation_beamwidth(f32::from(self.el_beamwidth) / Self::SCALING)
            .with_reference_system(self.reference_system)
            .with_e_z(f32::from(self.e_z))
            .with_e_x(f32::from(self.e_x))
            .with_phase(f32::from(self.phase) / Self::SCALING)
    }
}
