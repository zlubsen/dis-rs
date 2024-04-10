use cdis_assemble::{CdisPdu, Codec};
use dis_rs::model::Pdu;
use dis_rs::{parse};
use crate::config::GatewayMode;

struct Encoder {
    mode: GatewayMode,
    // hold a buffer/map of received PDUs to look up which fields can be left out
}

impl Encoder {
    fn new(mode: GatewayMode) -> Self {
        Self {
            mode
        }
    }

    // fn encode_buffers(&self, bytes_in: &[u8], bytes_out: &[u8]) {
    //     let pdus = parse(bytes_in);
    //     let cdis_pdus = match pdus {
    //         Ok(pdus) => {
    //             self.encode(&pdus)
    //         }
    //         Err(err) => {
    //             println!("{}", err);
    //             Vec::new()
    //         }
    //     };
    // }

    fn encode(&self, pdus: &Vec<Pdu>) -> Vec<CdisPdu> {
        let cdis_pdus: Vec<CdisPdu> = pdus.iter()
            .map(|pdu| CdisPdu::encode(pdu) )
            .collect();
        cdis_pdus
    }
}