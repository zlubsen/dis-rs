use thiserror::Error;
use dis_rs::model::TimeStamp;
use crate::entity_state::model::EntityState;
use crate::records::model::{CdisHeader, CdisRecord};
use crate::unsupported::Unsupported;

pub mod types;
pub mod records;
pub mod entity_state;
pub mod unsupported;
pub mod constants;
pub(crate) mod parsing;
pub(crate) mod writing;
pub(crate) mod codec;

pub use parsing::parse;
pub use writing::SerializeCdisPdu;
pub use writing::BitBuffer;
pub use writing::create_bit_buffer;
pub use codec::Codec;

trait BodyProperties {
    type FieldsPresent;
    type FieldsPresentOutput;
    const FIELDS_PRESENT_LENGTH: usize;
    fn fields_present_field(&self) -> Self::FieldsPresentOutput;

    fn body_length_bits(&self) -> usize;

    fn fields_present_length(&self) -> usize {
        Self::FIELDS_PRESENT_LENGTH
    }

    fn into_cdis_body(self) -> CdisBody;
}

#[derive(Clone, Debug, PartialEq)]
pub struct CdisPdu {
    pub header: CdisHeader,
    pub body: CdisBody,
}

impl CdisPdu {
    pub fn finalize_from_parts(header: CdisHeader, body: CdisBody, time_stamp: Option<impl Into<TimeStamp>>) -> Self {
        let time_stamp: TimeStamp = if let Some(time_stamp) = time_stamp {
            time_stamp.into()
        } else { header.timestamp };
        Self {
            header: CdisHeader {
                timestamp: time_stamp,
                length: (header.record_length() + body.body_length()) as u16,
                ..header
            },
            body
        }
    }

    pub fn pdu_length(&self) -> usize {
        self.header.record_length()
        + self.body.body_length()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CdisBody {
    Unsupported(Unsupported),
    EntityState(EntityState),
    Fire,
    Detonation,
    Collision,
    CreateEntity,
    RemoveEntity,
    StartResume,
    StopFreeze,
    Acknowledge,
    ActionRequest,
    ActionResponse,
    DataQuery,
    SetData,
    Data,
    EventReport,
    Comment,
    ElectromagneticEmission,
    Designator,
    Transmitter,
    Signal,
    Receiver,
    Iff,
}

impl CdisBody {
    pub fn body_length(&self) -> usize {
        match self {
            CdisBody::Unsupported(_) => { 0 }
            CdisBody::EntityState(body) => { body.body_length_bits() }
            CdisBody::Fire => { 0 }
            CdisBody::Detonation => { 0 }
            CdisBody::Collision => { 0 }
            CdisBody::CreateEntity => { 0 }
            CdisBody::RemoveEntity => { 0 }
            CdisBody::StartResume => { 0 }
            CdisBody::StopFreeze => { 0 }
            CdisBody::Acknowledge => { 0 }
            CdisBody::ActionRequest => { 0 }
            CdisBody::ActionResponse => { 0 }
            CdisBody::DataQuery => { 0 }
            CdisBody::SetData => { 0 }
            CdisBody::Data => { 0 }
            CdisBody::EventReport => { 0 }
            CdisBody::Comment => { 0 }
            CdisBody::ElectromagneticEmission => { 0 }
            CdisBody::Designator => { 0 }
            CdisBody::Transmitter => { 0 }
            CdisBody::Signal => { 0 }
            CdisBody::Receiver => { 0 }
            CdisBody::Iff => { 0 }
        }
    }
}

// TODO review messages (bits vs bytes)
#[derive(Clone, Debug, PartialEq, Error)]
pub enum CdisError {
    #[error("{0}")]
    ParseError(String), // the parsing of a CDIS PDU resulted in an error
    #[error("The buffer does not contain enough bytes for a valid C-DIS header. {0} bits available.")]
    InsufficientHeaderLength(u16), // the input was too small to contain a valid CDIS header; (u16 found)
    #[error("C-DIS PDU has insufficient length. Expected {0}, found {1}.")]
    InsufficientPduLength(u16, u16), // the input was too small to contain a valid CDIS PDU based on the header and parsing; (u16 expected, u16 found)
    #[error("C-DIS PDU is larger than size of the buffer for serialisation. Needs {0} bits, available {1} bits.")]
    InsufficientBufferSize(u16, usize), // the buffer for serialisation has insufficient capacity to hold the provided CDIS PDU; (u16 PDU size, usize available capacity)
    #[error("Encountered a C-DIS PDU of an unsupported type: {0}.")]
    UnsupportedPdu(u8), // encountered a CDIS PDU of an unsupported type; (u8 PduType found)
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
