use std::time::Duration;

use bytes::{Bytes, BytesMut};

use cdis_assemble::{BitBuffer, CdisError, CdisPdu, SerializeCdisPdu};
use cdis_assemble::codec::{CodecOptions, CodecUpdateMode, DecoderState, EncoderState};
use cdis_assemble::constants::MTU_BYTES;
use cdis_assemble::entity_state::codec::{DecoderStateEntityState};
use dis_rs::model::Pdu;
use dis_rs::{DisError, parse};

use crate::config::{Config, GatewayMode};

// enum StatefulPduUpdateType {
//     FirstOccurrenceFull,
//     WithinHeartbeatPartial,
//     HeartbeatElapsedFull,
// }

// fn is_stateful_pdu_type(pdu: &Pdu) -> bool {
//     match pdu.header.pdu_type {
//         PduType::EntityState |
//         PduType::ElectromagneticEmission |
//         PduType::Transmitter |
//         PduType::Designator |
//         PduType::IFF => { true }
//         _ => { false }
//     }
// }

/// The `Encoder` manages the configuration and state of the encoding of DIS PDUs to C-DIS PDUs.
pub struct Encoder {
    codec_options: CodecOptions,
    cdis_buffer: BitBuffer,
    state: EncoderState,
}

impl Encoder {
    pub fn new(config: &Config) -> Self {
        let codec_options = CodecOptions {
            update_mode: match config.mode.0 {
                CodecUpdateMode::FullUpdate => { CodecUpdateMode::FullUpdate }
                CodecUpdateMode::PartialUpdate => { CodecUpdateMode::PartialUpdate }
            },
            optimize_mode: config.optimization.0,
            use_guise: config.use_guise,
            federation_parameters: config.federation_parameters,
        };

        Self {
            codec_options,
            cdis_buffer: cdis_assemble::create_bit_buffer(),
            state: EncoderState {
                entity_state: Default::default(),
            },
        }
    }

    fn parsing(&self, bytes: Bytes) -> Result<Vec<Pdu>, DisError> {
        parse(&bytes)
    }

    fn writing(&mut self, cdis_pdus: Vec<CdisPdu>) -> Vec<u8> {
        let (total_bits, _cursor) = cdis_pdus.iter()
            .fold((0usize, 0usize),
                  | (total_bits, cursor), pdu| {
                      ( total_bits + pdu.pdu_length(), pdu.serialize(&mut self.cdis_buffer, cursor) )
                  });

        let cdis_wire: Vec<u8> = Vec::from(&self.cdis_buffer.data[0..total_bits.div_ceil(8)]);
        cdis_wire
    }

    // TODO make fallible, result from parse (and encode) function(s)
    pub fn encode_buffer(&mut self, bytes_in: Bytes) -> Vec<u8> {
        let pdus = self.parsing(bytes_in);
        let cdis_pdus = match pdus {
            Ok(pdus) => {
                self.encode_pdus(&pdus)
            }
            Err(err) => {
                println!("{:?}", err);
                Vec::new()
            }
        };
        self.writing(cdis_pdus)
    }

    // TODO make fallible, result from encode function
    pub fn encode_pdus(&mut self, pdus: &[Pdu]) -> Vec<CdisPdu> {
        let cdis_pdus: Vec<CdisPdu> = pdus.iter()
            // TODO
            // switch between partial and full update modes
            // V if full update mode, just encode and send
            // if partial update mode, check if we already have stored a previous snapshot of the PDU type
            // if not, store the PDU, record the time for the update period, send a full update cdis PDU
            // if yes, check if the full_update_period has elapsed;
            //     if yes, store the PDU, record the time, and send a full update cdis PDU
            //     if not, compute a partial update cdis PDU and send it.
            //         Computation happens by comparing optional fields and leaving them out if not changed
            .map(|pdu| {
                CdisPdu::encode(pdu, &self.state, &self.codec_options)
            } )
            .collect();
        cdis_pdus
    }
}

pub struct Decoder {
    mode: GatewayMode,
    dis_buffer: BytesMut,
    state: DecoderState,
    cdis_full_update_period: Duration,
}

impl Decoder {
    pub fn new(mode: GatewayMode, cdis_full_update_period: f32) -> Self {
        let dis_buffer = BytesMut::with_capacity(MTU_BYTES);

        Self {
            mode,
            dis_buffer,
            state: DecoderState { entity_state: DecoderStateEntityState::default() },
            cdis_full_update_period: Duration::from_secs_f32(cdis_full_update_period),
        }
    }

    fn parsing(&self, bytes: Bytes) -> Result<Vec<CdisPdu>, CdisError> {
        cdis_assemble::parse(&bytes)
    }

    fn writing(&mut self, dis_pdus: Vec<Pdu>) -> Vec<u8> {
        // FIXME now we reset the buffer by assigning a new BytesMut; should not cause reallocation of under lying memory, but need to check.
        self.dis_buffer = BytesMut::with_capacity(MTU_BYTES);

        // FIXME number of bytes reported 'serialize' is too large for observed cases (208 actual, reported 252)...
        // number_of_bytes not used in creating the slice to put in the vec.
        let _number_of_bytes: usize = dis_pdus.iter()
            .map(| pdu| {
                pdu.serialize(&mut self.dis_buffer).unwrap() as usize
            } ).sum();

        // TODO perhaps replace Vec with Bytes, but unsure how to assign the latter
        // E.g., Bytes::from_iter(&self.dis_buffer[..].iter()), or Bytes::from(&self.dis_buffer[..])?
        Vec::from(&self.dis_buffer[..])
    }

    // TODO make fallible, result from parse (and decode) function(s)
    pub fn decode_buffer(&mut self, bytes_in: Bytes) -> Vec<u8> {
        let cdis_pdus = self.parsing(bytes_in);
        let pdus = match cdis_pdus {
            Ok(pdus) => {
                self.decode_pdus(&pdus)
            }
            Err(err) => {
                println!("{}", err); // TODO tracing or Result return value
                Vec::new()
            }
        };

        self.writing(pdus)
    }

    // TODO make fallible, result from encode function
    pub fn decode_pdus(&self, pdus: &[CdisPdu]) -> Vec<Pdu> {
        let dis_pdus: Vec<Pdu> = pdus.iter()
            // TODO
            // switch between partial and full update modes
            // in full update mode, decode the cdis PDU and send out the DIS pdu.
            // in partial update mode,
            // when a full update cdis PDU is received, decode to DIS and store the PDU, and record time
            // when a partial update cdis PDU is received
            //     check if we already have a full update stored to fill in the blanks, send out DIS
            //     if no full update is present, discard the cdis PDU
            .map(|cdis_pdu| cdis_pdu.decode(None, true) )
            .collect();
        dis_pdus
    }
}