use bitvec::prelude::{BitArray, Msb0};
use crate::constants::MTU_BITS;
use crate::entity_state::model::{EntityState};
use crate::records::model::CdisHeader;
use crate::unsupported::Unsupported;

pub mod types;
pub mod records;
pub mod entity_state;
pub mod unsupported;
pub mod constants;
pub(crate) mod utils;
pub(crate) mod parsing;
pub(crate) mod writing;

pub use parsing::parse;

pub(crate) type BitBuffer = BitArray<[u8; MTU_BITS], Msb0>;

trait SerializeCdisPdu {
    fn serialize(&self, buf : &mut BitBuffer, cursor : usize) -> usize;
}

trait SerializeCdis {
    fn serialize(&self, buf : &mut BitBuffer, cursor:  usize) -> usize;
}

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

pub struct CdisPdu {
    header: CdisHeader,
    body: CdisBody,
}

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

pub enum CdisError {
    ParseError(String), // the parsing of a CDIS PDU resulted in an error
    InsufficientHeaderLength(u16), // the input was too small to contain a valid CDIS header; (u16 found)
    InsufficientPduLength(u16, u16), // the input was too small to contain a valid CDIS PDU based on the header and parsing; (u16 expected, u16 found)
    InsufficientBufferSize(u16, usize), // the buffer for serialisation has insufficient capacity to hold the provided CDIS PDU; (u16 PDU size, usize available capacity)
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
