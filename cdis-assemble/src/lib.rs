use bitvec::prelude::{BitArray, Msb0};
use crate::constants::MTU_BITS;
use crate::entity_state::model::{EntityState};
use crate::records::model::CdisHeader;

pub mod types;
pub mod records;
pub mod entity_state;
pub mod constants;
pub(crate) mod parser_utils;


pub(crate) type BitBuffer = BitArray<[u8; MTU_BITS], Msb0>;

trait SerializeCdisPdu {
    fn serialize(&self, buf : &mut BitBuffer, cursor : usize) -> usize;
}

trait SerializeCdis {
    fn serialize(&self, buf : &mut BitBuffer, cursor:  usize) -> usize;
}

pub struct CdisPdu {
    header: CdisHeader,
    body: CdisBody,
}

pub enum CdisBody {
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
    Iff
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
