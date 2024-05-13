use std::time::Instant;

use bytes::{Bytes, BytesMut};
use tracing::error;

use cdis_assemble::{BitBuffer, CdisBody, CdisError, CdisInteraction, CdisPdu, SerializeCdisPdu};
use cdis_assemble::codec::{CodecOptions, CodecStateResult, CodecUpdateMode, DecoderState, EncoderState};
use cdis_assemble::constants::MTU_BYTES;
use cdis_assemble::entity_state::codec::{DecoderStateEntityState};
use dis_rs::model::{EntityId, Location, Pdu, PduBody};
use dis_rs::{DisError, parse};

use crate::config::Config;
use crate::Event;

/// The `Encoder` manages the configuration and state of the encoding of DIS PDUs to C-DIS PDUs.
pub struct Encoder {
    codec_options: CodecOptions,
    cdis_buffer: BitBuffer,
    state: EncoderState,
    event_tx: tokio::sync::mpsc::Sender<Event>,
}

impl Encoder {
    pub fn new(config: &Config, event_tx: tokio::sync::mpsc::Sender<Event>) -> Self {
        let codec_options = CodecOptions {
            update_mode: match config.mode.0 {
                CodecUpdateMode::FullUpdate => { CodecUpdateMode::FullUpdate }
                CodecUpdateMode::PartialUpdate => { CodecUpdateMode::PartialUpdate }
            },
            optimize_mode: config.optimization.0,
            use_guise: config.use_guise,
            federation_parameters: config.federation_parameters,
            hbt_cdis_full_update_mplier: config.hbt_cdis_full_update_mplier,
        };

        Self {
            codec_options,
            cdis_buffer: cdis_assemble::create_bit_buffer(),
            state: EncoderState {
                entity_state: Default::default(),
            },
            event_tx,
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

        let total_bytes = total_bits.div_ceil(8);
        let cdis_wire: Vec<u8> = Vec::from(&self.cdis_buffer.data[0..total_bytes]);
        self.event_tx.blocking_send(Event::SentCDis(total_bytes)).expect("Event TX channel failed in Encoder::writing.");
        cdis_wire
    }

    // TODO make fallible, result from parse (and encode) function(s)
    pub fn encode_buffer(&mut self, bytes_in: Bytes) -> Vec<u8> {
        self.event_tx.blocking_send(Event::ReceivedBytesDis(bytes_in.len())).expect("Event TX channel failed in Encoder::encode_buffer.");
        let pdus = self.parsing(bytes_in);
        let cdis_pdus = match pdus {
            Ok(pdus) => {
                self.encode_pdus(&pdus)
            }
            Err(err) => {
                error!("{:?}", err);
                Vec::new()
            }
        };
        self.writing(cdis_pdus)
    }

    // TODO make fallible, result from encode function
    pub fn encode_pdus(&mut self, pdus: &[Pdu]) -> Vec<CdisPdu> {
        let cdis_pdus: Vec<CdisPdu> = pdus.iter()
            .map(|pdu| {
                self.event_tx.blocking_send(Event::ReceivedDis(pdu.header.pdu_type, )).expect("Event TX channel failed in Encoder::encode_pdus - Received bytes.");
                let (pdu, state_result) = CdisPdu::encode(pdu, &mut self.state, &self.codec_options);
                match state_result {
                    CodecStateResult::StateUnaffected => {}
                    CodecStateResult::StateUpdateEntityState => {
                        self.state.entity_state.entry(dis_rs::model::EntityId::from(
                            pdu.originator().expect("EntityState PDU always should have an originating EntityId")))
                            .and_modify(|e| e.last_send = Instant::now() )
                            .or_default();
                    }
                }
                self.event_tx.blocking_send(Event::EncodedPdu(pdu.header.pdu_type)).expect("Event TX channel failed in Encoder::encode_pdus - Encoded PDU.");
                pdu
            } )
            .filter(|pdu| if let CdisBody::Unsupported(_) = pdu.body {
                self.event_tx.blocking_send(Event::RejectedUnsupportedDisPdu(pdu.header.pdu_type)).expect("Event TX channel failed in Encoder::encode_pdus - Reject unsupported.");
                false
            } else { true }) // only process supported PDUs
            .collect();
        cdis_pdus
    }
}

pub struct Decoder {
    codec_options: CodecOptions,
    dis_buffer: BytesMut,
    state: DecoderState,
    event_tx: tokio::sync::mpsc::Sender<Event>,
}

impl Decoder {
    pub fn new(config: &Config, event_tx: tokio::sync::mpsc::Sender<Event>,) -> Self {
        let codec_options = CodecOptions {
            update_mode: match config.mode.0 {
                CodecUpdateMode::FullUpdate => { CodecUpdateMode::FullUpdate }
                CodecUpdateMode::PartialUpdate => { CodecUpdateMode::PartialUpdate }
            },
            optimize_mode: config.optimization.0,
            use_guise: config.use_guise,
            federation_parameters: config.federation_parameters,
            hbt_cdis_full_update_mplier: config.hbt_cdis_full_update_mplier,
        };
        let dis_buffer = BytesMut::with_capacity(MTU_BYTES);

        Self {
            codec_options,
            dis_buffer,
            state: DecoderState { entity_state: Default::default() },
            __event_tx, // TODO emit events during decoding
        }
    }

    fn parsing(&self, bytes: Bytes) -> Result<Vec<CdisPdu>, CdisError> {
        cdis_assemble::parse(&bytes)
    }

    fn writing(&mut self, dis_pdus: Vec<Pdu>) -> Vec<u8> {
        // FIXME now we reset the buffer by assigning a new BytesMut; should not cause reallocation of underlying memory, but need to check.
        self.dis_buffer = BytesMut::with_capacity(MTU_BYTES);

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
    pub fn decode_pdus(&mut self, pdus: &[CdisPdu]) -> Vec<Pdu> {
        let dis_pdus: Vec<Pdu> = pdus.iter()
            // TODO
            // switch between partial and full update modes
            // in full update mode, decode the cdis PDU and send out the DIS pdu.
            // in partial update mode,
            // when a full update cdis PDU is received, decode to DIS and store the PDU, and record time
            // when a partial update cdis PDU is received
            //     check if we already have a full update stored to fill in the blanks, send out DIS
            //     if no full update is present, discard the cdis PDU
            .map(|cdis_pdu| {
                let (pdu, state_result) = cdis_pdu.decode(&mut self.state, &self.codec_options);
                match state_result {
                    CodecStateResult::StateUnaffected => {}
                    CodecStateResult::StateUpdateEntityState => {
                        self.state.entity_state.entry(EntityId::from(
                            cdis_pdu.originator().expect("EntityState PDU always should have an originating EntityId")))
                            .and_modify(|mut entry| {
                                if let PduBody::EntityState(es) = &pdu.body {
                                    *entry = DecoderStateEntityState::new(&es)
                                }
                            } )
                            // TODO only insert full updates
                            .or_insert(DecoderStateEntityState::default());
                    }
                }
                pdu
            })
            .collect();
        dis_pdus
    }
}