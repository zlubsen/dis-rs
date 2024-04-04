use bitvec::array::BitArray;
use bitvec::order::Msb0;
use bitvec::macros::internal::funty::Integral;
use bitvec::field::BitField;
use crate::{CdisBody, CdisPdu};
use crate::constants::MTU_BITS;

pub(crate) type BitBuffer = BitArray<[u8; MTU_BITS], Msb0>;

pub trait SerializeCdisPdu {
    fn serialize(&self, buf : &mut BitBuffer, cursor : usize) -> usize;
}

pub trait SerializeCdis {
    fn serialize(&self, buf : &mut BitBuffer, cursor:  usize) -> usize;
}

impl SerializeCdisPdu for CdisPdu {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.header.serialize(buf, cursor);
        let cursor = self.body.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdisPdu for CdisBody {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = match self {
            CdisBody::EntityState(body) => { body.serialize(buf, cursor) }
            // CdisBody::Fire => {}
            // CdisBody::Detonation => {}
            // CdisBody::Collision => {}
            // CdisBody::CreateEntity => {}
            // CdisBody::RemoveEntity => {}
            // CdisBody::StartResume => {}
            // CdisBody::StopFreeze => {}
            // CdisBody::Acknowledge => {}
            // CdisBody::ActionRequest => {}
            // CdisBody::ActionResponse => {}
            // CdisBody::DataQuery => {}
            // CdisBody::SetData => {}
            // CdisBody::Data => {}
            // CdisBody::EventReport => {}
            // CdisBody::Comment => {}
            // CdisBody::ElectromagneticEmission => {}
            // CdisBody::Designator => {}
            // CdisBody::Transmitter => {}
            // CdisBody::Signal => {}
            // CdisBody::Receiver => {}
            // CdisBody::Iff => {}
            _ => { cursor }
        };

        cursor
    }
}

/// Write `value` to the BitBuffer `buf`, at the position of `cursor` with length `bit_size`.
pub(crate) fn write_value_with_length<T: Integral>(buf: &mut BitBuffer, cursor: usize, bit_size: usize, value: T) -> usize {
    let next_cursor = cursor + bit_size;
    buf[cursor..next_cursor].store_be(value);
    next_cursor
}

/// Helper function that checks if the provided `Option` is `Some`, and then serializes the contained value.
/// Field must implement trait `SerializeCdis`.
pub(crate) fn serialize_when_present<I: SerializeCdis>(field: &Option<I>, buf: &mut BitBuffer, cursor: usize) -> usize {
    if let Some(inner) = field { inner.serialize(buf, cursor) } else { cursor }
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_value_with_length_positive() {
        assert!(false)
    }

    #[test]
    fn write_value_with_length_negative() {
        assert!(false)
    }
}
