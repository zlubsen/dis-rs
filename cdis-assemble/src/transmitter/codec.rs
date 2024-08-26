use std::time::Instant;
use dis_rs::enumerations::TransmitterCryptoSystem;
use dis_rs::model::PduBody;
use crate::{BodyProperties, CdisBody};
use crate::codec::{Codec, CodecOptions, CodecStateResult, CodecUpdateMode, DecoderState, EncoderState};
use crate::records::codec::{encode_entity_coordinate_vector, encode_world_coordinates};
use crate::records::model::{EntityId, EntityType};
use crate::transmitter::model::{ModulationType, TransmitFrequencyBandwidthFloat, Transmitter, TransmitterFrequencyFloat, TransmitterUnits};
use crate::types::model::{CdisFloat, UVINT16, UVINT8};

type Counterpart = dis_rs::transmitter::model::Transmitter;

pub(crate) fn encode_transmitter_body_and_update_state(dis_body: &Counterpart,
                                                        state: &mut EncoderState,
                                                        options: &CodecOptions) -> (CdisBody, CodecStateResult) {
    let state_for_id = state.transmitter.get(&dis_body.radio_reference_id);

    let (cdis_body, state_result) = Transmitter::encode(dis_body, state_for_id, options);

    if state_result == CodecStateResult::StateUpdateTransmitter {
        state.transmitter.entry(dis_body.radio_reference_id)
            .and_modify(|tr| { tr.heartbeat = Instant::now() })
            .or_default();
    }

    (cdis_body.into_cdis_body(), state_result)
}

pub(crate) fn decode_transmitter_body_and_update_state(cdis_body: &Transmitter,
                                                        state: &mut DecoderState,
                                                        options: &CodecOptions) -> (PduBody, CodecStateResult) {
    let state_for_id = state.transmitter.get(&dis_rs::model::EntityId::from(&cdis_body.radio_reference_id));
    let (dis_body, state_result) = cdis_body.decode(state_for_id, options);

    if state_result == CodecStateResult::StateUpdateTransmitter {
        state.transmitter.entry(dis_rs::model::EntityId::from(&cdis_body.radio_reference_id))
            .and_modify(|tr| {
                tr.heartbeat = Instant::now();
                // ...
            })
            .or_insert(DecoderStateTransmitter::new(&dis_body));
    }

    (dis_body.into_pdu_body(), state_result)
}

impl Transmitter {
    pub fn encode(item: &Counterpart, state: Option<&EncoderStateTransmitter>, options: &CodecOptions) -> (Self, CodecStateResult) {
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
            && state.is_some_and(|state|
            !evaluate_timeout_for_transmitter(&state.heartbeat, options) ) {
            (TransmitterUnits::default(), false, None, None, None, None, None, None, None, CodecStateResult::StateUnaffected)
        } else {
            let (antenna_location, antenna_location_units) = encode_world_coordinates(&item.antenna_location);
            // include when not zeroed
            let (relative_antenna_location, relative_antenna_location_units) = encode_entity_coordinate_vector(&item.relative_antenna_location);
            let units = TransmitterUnits {
                world_location_altitude: antenna_location_units,
                relative_antenna_location: relative_antenna_location_units,
            };
            (units, true,
             Some(EntityType::encode(&item.radio_type)),
             Some(antenna_location),
             Some(relative_antenna_location),
             Some(TransmitterFrequencyFloat::from_float(item.frequency as f64)),
             Some(TransmitFrequencyBandwidthFloat::from_float(item.transmit_frequency_bandwidth)),
             Some(item.power as u8),
             Some(ModulationType::from(&item.modulation_type)),
             CodecStateResult::StateUpdateTransmitter)
        };

        // include when crypto fields are not zeroed
        let (crypto_system, crypto_key_id) = if item.crypto_system != TransmitterCryptoSystem::NoEncryptionDevice {
            (Some(item.crypto_system), Some(item.crypto_key_id))
        } else { (None, None) };

        // include when modulation parameters are present
        let modulation_parameters= if let Some(params) = &item.modulation_parameters {
            params.clone()
        } else { Default::default() };

        // include when antenna_pattern is not zeroed
        let (antenna_pattern_type, antenna_pattern) = if let Some(pattern) = &item.antenna_pattern {
            let mut vec = Vec::new();
            vec.extend(pattern.e_x.to_be_bytes());
            // vec.extend(pattern.e_z) TODO
            (Some(item.antenna_pattern_type), vec)
        } else {
            (None, vec![])
        };

        let input_source: u8 = item.input_source.into();

        (Self {
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
        }, state_result)
    }

    pub fn decode(&self, state: Option<&DecoderStateTransmitter>, options: &CodecOptions) -> (Counterpart, CodecStateResult) {
        todo!()
    }
}

#[derive(Debug)]
pub struct EncoderStateTransmitter {
    pub heartbeat: Instant,
}

impl Default for EncoderStateTransmitter {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now()
        }
    }
}

#[derive(Debug)]
pub struct DecoderStateTransmitter {
    pub heartbeat: Instant,
}

impl DecoderStateTransmitter {
    pub fn new(pdu: &Counterpart) -> Self {
        Self {
            heartbeat: Instant::now(),
        }
    }
}

impl Default for DecoderStateTransmitter {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
        }
    }
}

fn evaluate_timeout_for_transmitter(last_heartbeat: &Instant, options: &CodecOptions) -> bool {
    let elapsed = last_heartbeat.elapsed().as_secs_f32();
    elapsed > (options.federation_parameters.HBT_PDU_TRANSMITTER * options.hbt_cdis_full_update_mplier)
}