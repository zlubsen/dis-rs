use bytes::{Bytes, BytesMut};
use tracing::error;

use cdis_assemble::{BitBuffer, CdisError, CdisPdu, Implemented, SerializeCdisPdu, Supported};
use cdis_assemble::codec::{CodecOptions, CodecUpdateMode, DecoderState, EncoderState};
use cdis_assemble::constants::MTU_BYTES;
use dis_rs::model::Pdu;
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
            state: EncoderState::default(),
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
        self.event_tx.try_send(Event::SentCDis(total_bytes)).expect("Event transmit channel failed in Encoder::writing.");
        cdis_wire
    }

    pub fn encode_buffer(&mut self, bytes_in: Bytes) -> Vec<u8> {
        self.event_tx.try_send(Event::ReceivedBytesDis(bytes_in.len())).expect("Event TX channel failed in Encoder::encode_buffer.");
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

    pub fn encode_pdus(&mut self, pdus: &[Pdu]) -> Vec<CdisPdu> {
        let cdis_pdus: Vec<CdisPdu> = pdus.iter()
            .filter(|pdu| if !pdu.header.pdu_type.is_supported() {
                self.event_tx.try_send(Event::RejectedUnsupportedDisPdu(pdu.header.pdu_type, pdu.header.pdu_length as u64))
                    .expect("Event TX channel failed in Encoder::encode_pdus - Reject unsupported PDU.");
                false
            } else { true }) // only process supported PDUs
            .filter(|pdu| if pdu.header.pdu_type.is_implemented() { true
            } else {
                self.event_tx.try_send(Event::UnimplementedEncodedPdu(pdu.header.pdu_type, pdu.header.pdu_length as u64))
                    .expect("Event TX channel failed in Encoder::encode_pdus - Reject unimplemented PDU.");
                false
            }) // only send out implemented C-DIS PDUs
            .inspect(|pdu| self.event_tx.try_send(Event::ReceivedDis(pdu.header.pdu_type, pdu.header.pdu_length as u64))
                .expect("Event TX channel failed in Encoder::encode_pdus - Received PDU.") )
            .map(|pdu| {
                let (pdu, _state_result) = CdisPdu::encode(pdu, &mut self.state, &self.codec_options);
                // FIXME (potential) - processing of state_result is done after encoding of the body; not needed here, unless a combination of header and body is needed for the action
                pdu
            } )
            .inspect(| pdu | self.event_tx.try_send(Event::EncodedPdu(pdu.header.pdu_type, pdu.header.length.div_ceil(8) as u64))
                .expect("Event TX channel failed in Encoder::encode_pdus - Encoded PDU.") )
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
            state: DecoderState::default(),
            event_tx, // TODO emit events during decoding
        }
    }

    fn parsing(&self, bytes: Bytes) -> Result<Vec<CdisPdu>, CdisError> {
        cdis_assemble::parse(&bytes)
    }

    fn writing(&mut self, dis_pdus: Vec<Pdu>) -> Vec<u8> {
        // FIXME now we reset the buffer by assigning a new BytesMut; should not cause reallocation of underlying memory, but need to check.
        self.dis_buffer = BytesMut::with_capacity(MTU_BYTES);

        // number_of_bytes not used in creating the slice to put in the vec.
        let number_of_bytes: usize = dis_pdus.iter()
            .map(| pdu| {
                pdu.serialize(&mut self.dis_buffer).unwrap() as usize
            } ).sum();

        self.event_tx.try_send(Event::SentDis(number_of_bytes)).expect("Event TX channel failed in Decoder::writing.");
        // TODO perhaps replace Vec with Bytes, but unsure how to assign the latter
        // E.g., Bytes::from_iter(&self.dis_buffer[..].iter()), or Bytes::from(&self.dis_buffer[..])?
        Vec::from(&self.dis_buffer[..])
    }

    pub fn decode_buffer(&mut self, bytes_in: Bytes) -> Vec<u8> {
        self.event_tx.try_send(Event::ReceivedBytesCDis(bytes_in.len())).expect("Event TX channel failed in Decoder::decode_buffer.");
        let cdis_pdus = self.parsing(bytes_in);
        let pdus = match cdis_pdus {
            Ok(pdus) => {
                self.decode_pdus(&pdus)
            }
            Err(err) => {
                error!("{}", err); // TODO tracing or Result return value
                Vec::new()
            }
        };

        self.writing(pdus)
    }

    pub fn decode_pdus(&mut self, pdus: &[CdisPdu]) -> Vec<Pdu> {
        let dis_pdus: Vec<Pdu> = pdus.iter()
            // switch between partial and full update modes
            // in full update mode, decode the cdis PDU and send out the DIS pdu.
            // in partial update mode,
            // when a full update cdis PDU is received, decode to DIS and store the PDU, and record time for heartbeat
            // when a partial update cdis PDU is received
            //     check if we already have a full update stored to fill in the blanks, send out DIS
            //     if no full update is present, discard the cdis PDU
            .filter(|cdis| if cdis.header.pdu_type.is_implemented() {
                self.event_tx.try_send(Event::UnimplementedDecodedPdu(cdis.header.pdu_type, cdis.header.length.div_ceil(8) as u64))
                    .expect("Event TX channel failed in Decoder::decode_pdus - Reject unimplemented PDU.");
                // self.event_tx.try_send(Event::RejectedUnsupportedCDisPdu((cdis.header.pdu_type, cdis.header.length.div_ceil(8) as u64))).expect("Event TX channel failed in Decoder::decode_pdus - Reject unsupported.");
                false
            } else { true } )// only process implemented C-DIS PDUs
            .inspect(|cdis_pdu| self.event_tx.try_send(Event::ReceivedCDis(cdis_pdu.header.pdu_type, cdis_pdu.header.length.div_ceil(8) as u64)).expect("Event TX channel failed in Decoder::decode_pdus - Received PDU.") )
            .map(|cdis_pdu| {
                let (pdu, _state_result) = cdis_pdu.decode(&mut self.state, &self.codec_options);
                // FIXME (potential) - processing of state_result is done after decoding of the body; not needed here, unless a combination of header and body is needed for the action
                pdu
            })
            .inspect(|pdu| self.event_tx.try_send(Event::DecodedPdu(pdu.header.pdu_type, pdu.header.pdu_length as u64)).expect("Event TX channel failed in Decoder::decode_pdus - Decoded PDU.") )
            .collect();
        dis_pdus
    }
}