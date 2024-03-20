use dis_rs::enumerations::PduType;
use dis_rs::model::PduStatus;
use dis_rs::model::{TimeStamp};
use crate::records::model::CDISProtocolVersion::{Reserved, SISO_023_2023, StandardDis};
use crate::types::UVINT8;

pub struct CDISHeader {
    pub protocol_version: CDISProtocolVersion,
    pub exercise_id: UVINT8,
    pub pdu_type: PduType,
    pub timestamp: TimeStamp, // TODO decide if CDISTimeStamp is needed
    pub length: u16,
    pub pdu_status: PduStatus,
}

#[allow(non_camel_case_types)]
pub enum CDISProtocolVersion {
    StandardDis,
    SISO_023_2023,
    Reserved(u8),
}

impl From<u8> for CDISProtocolVersion {
    fn from(value: u8) -> Self {
        match value {
            0 => StandardDis,
            1 => SISO_023_2023,
            reserved => Reserved(reserved),
        }
    }
}

impl From<CDISProtocolVersion> for u8 {
    fn from(value: CDISProtocolVersion) -> Self {
        match value {
            StandardDis => { 0 }
            SISO_023_2023 => { 1 }
            Reserved(reserved) => { reserved }
        }
    }
}
