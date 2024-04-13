use bytes::{Bytes, BytesMut};
use cdis_assemble::{BitBuffer, CdisError, CdisPdu, Codec, SerializeCdisPdu};
use dis_rs::model::Pdu;
use dis_rs::{DisError, parse};
use crate::config::GatewayMode;

pub struct Encoder {
    mode: GatewayMode,
    cdis_buffer: BitBuffer,
    // hold a bytes buffer to convert the bitbuffer to bytes?
    // hold a buffer/map of received PDUs to look up which fields can be left out
}

impl Encoder {
    pub fn new(mode: GatewayMode) -> Self {
        Self {
            mode,
            cdis_buffer: cdis_assemble::create_bit_buffer()
        }
    }

    fn parsing(&self, bytes: Bytes) -> Result<Vec<Pdu>, DisError> {
        parse(&bytes)
    }

    fn writing(&mut self, cdis_pdus: Vec<CdisPdu>) -> Bytes {
        let cursor = cdis_pdus.iter()
            .fold(0usize,
                  | cursor, pdu| pdu.serialize(&mut self.cdis_buffer, cursor) );
        // `cursor` contains the amount of bits written
        let cdis_wire: Vec<u8> = self.cdis_buffer.data[0..cursor].chunks_exact(8).map(|ch| { ch[0] } ).collect();
        Bytes::from(cdis_wire)
    }

    // TODO make fallible, result from parse (and encode) function(s)
    pub fn encode_buffer(&mut self, bytes_in: Bytes) -> Bytes {
        let pdus = self.parsing(bytes_in);
        let cdis_pdus = match pdus {
            Ok(pdus) => {
                self.encode_pdus(&pdus)
            }
            Err(err) => {
                println!("{}", err);
                Vec::new()
            }
        };
        self.writing(cdis_pdus)
    }

    // TODO make fallible, result from encode function
    pub fn encode_pdus(&self, pdus: &Vec<Pdu>) -> Vec<CdisPdu> {
        let cdis_pdus: Vec<CdisPdu> = pdus.iter()
            .map(|pdu| CdisPdu::encode(pdu) )
            .collect();
        cdis_pdus
    }
}

pub struct Decoder {
    mode: GatewayMode,
    dis_buffer: BytesMut,
}

impl Decoder {
    pub fn new(mode: GatewayMode) -> Self {
        Self {
            mode,
        }
    }

    fn parsing(&self, bytes: Bytes) -> Result<Vec<CdisPdu>, CdisError> {
        cdis_assemble::parse(&bytes)
    }

    fn writing(&mut self, dis_pdus: Vec<Pdu>) -> Bytes {
        let cursor: usize = dis_pdus.iter()
            .map(| pdu| { pdu.serialize(&mut self.dis_buffer).unwrap() as usize } ).sum();

        let dis_wire: Vec<u8> = self.cdis_buffer.data[0..cursor].chunks_exact(8).map(|ch| { ch[0] } ).collect();
        Bytes::from(cdis_wire)
    }

    // TODO make fallible, result from parse (and encode) function(s)
    pub fn encode_buffer(&mut self, bytes_in: Bytes) -> Bytes {
        let pdus = self.parsing(bytes_in);
        let cdis_pdus = match pdus {
            Ok(pdus) => {
                self.encode_pdus(&pdus)
            }
            Err(err) => {
                println!("{}", err);
                Vec::new()
            }
        };
        self.writing(cdis_pdus)
    }

    // TODO make fallible, result from encode function
    pub fn encode_pdus(&self, pdus: &Vec<Pdu>) -> Vec<CdisPdu> {
        let cdis_pdus: Vec<CdisPdu> = pdus.iter()
            .map(|pdu| CdisPdu::encode(pdu) )
            .collect();
        cdis_pdus
    }
}