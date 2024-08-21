use std::any::type_name;
use bitvec::array::BitArray;
use bitvec::order::Msb0;
use bitvec::macros::internal::funty::Integral;
use bitvec::field::BitField;
use crate::{CdisBody, CdisPdu};
use crate::constants::{EIGHT_BITS, MTU_BITS, ONE_BIT, SIXTEEN_BITS};

pub type BitBuffer = BitArray<[u8; MTU_BITS], Msb0>;

pub fn create_bit_buffer() -> BitBuffer {
    let buf: BitBuffer = BitArray::ZERO;
    buf
}

pub trait SerializeCdisPdu {
    fn serialize(&self, buf : &mut BitBuffer, cursor : usize) -> usize;
}

pub trait SerializeCdis {
    fn serialize(&self, buf : &mut BitBuffer, cursor:  usize) -> usize;
}

impl SerializeCdisPdu for CdisPdu {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.header.serialize(buf, cursor);
        let cursor = self.body.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdisPdu for CdisBody {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = match self {
            CdisBody::Unsupported(_body) => { cursor }
            CdisBody::EntityState(body) => { body.serialize(buf, cursor) }
            CdisBody::Fire(body) => { body.serialize(buf, cursor) }
            CdisBody::Detonation(body) => { body.serialize(buf, cursor) }
            CdisBody::Collision(body) => { body.serialize(buf, cursor) }
            CdisBody::CreateEntity(body) => { body.serialize(buf, cursor) }
            CdisBody::RemoveEntity(body) => { body.serialize(buf, cursor) }
            CdisBody::StartResume(body) => { body.serialize(buf, cursor) }
            CdisBody::StopFreeze(body) => { body.serialize(buf, cursor) }
            CdisBody::Acknowledge(body) => { body.serialize(buf, cursor) }
            CdisBody::ActionRequest(body) => { body.serialize(buf, cursor) }
            CdisBody::ActionResponse(body) => { body.serialize(buf, cursor) }
            CdisBody::DataQuery(body) => { body.serialize(buf, cursor) }
            CdisBody::SetData(body) => { body.serialize(buf, cursor) }
            CdisBody::Data(body) => { body.serialize(buf, cursor) }
            CdisBody::EventReport(body) => { body.serialize(buf, cursor) }
            CdisBody::Comment(body) => { body.serialize(buf, cursor) }
            CdisBody::ElectromagneticEmission(body) => { body.serialize(buf, cursor) }
            CdisBody::Designator(body) => { body.serialize(buf, cursor) }
            // CdisBody::Transmitter => {}
            CdisBody::Signal(body) => { body.serialize(buf, cursor) }
            CdisBody::Receiver(body) => { body.serialize(buf, cursor) }
            // CdisBody::Iff => {}
            _ => { cursor }
        };

        cursor
    }
}

/// Write `value` to the BitBuffer `buf`, at the position of `cursor` with length `bit_size`.
/// This is an internal function, to write 'whole sequences of bits' of positive values.
/// Use ``write_value_unsigned`` and ``write_value_signed``.
/// C-DIS negative values in 2's complement have to be written manually, consisting of a sign bit and the value bits.
///
/// Returns the new cursor position.
fn write_value_with_length<T: Integral>(buf: &mut BitBuffer, cursor: usize, bit_size: usize, value: T) -> usize {
    let next_cursor = cursor + bit_size;
    buf[cursor..next_cursor].store_be(value);
    next_cursor
}

/// Write an unsigned value to the BitBuffer `buf`, at the position of the `cursor`, with `bit_size` bits in length.
pub(crate) fn write_value_unsigned<T: num::Unsigned + Integral>(buf: &mut BitBuffer, cursor: usize, bit_size: usize, value: T) -> usize {
    write_value_with_length(buf, cursor, bit_size, value)
}

#[allow(clippy::let_and_return)]
pub(crate) fn write_value_signed<T: num::FromPrimitive + num::Signed + num::Zero + Integral>(buf: &mut BitBuffer, cursor: usize, bit_size: usize, value: T) -> usize {
    let cursor = write_value_with_length(
        buf, cursor, ONE_BIT, u8::from(value.is_negative()));
    let value_bits = - (if value.is_negative() {
        T::from_isize((-2isize).pow(bit_size as u32 - 1))
            .unwrap_or_else(|| panic!("Cannot determine minimum value for type {}", type_name::<T>()))
            - value
    } else { T::zero() - value });
    let cursor = write_value_with_length(
        buf, cursor, bit_size - ONE_BIT, value_bits);

    cursor
}

/// Helper function that checks if the provided `Option` is `Some`, and then serializes the contained value.
/// Field must implement trait `SerializeCdis`.
pub(crate) fn serialize_when_present<I: SerializeCdis>(field: &Option<I>, buf: &mut BitBuffer, cursor: usize) -> usize {
    if let Some(inner) = field { inner.serialize(buf, cursor) } else { cursor }
}

impl SerializeCdis for u8 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        write_value_unsigned(buf, cursor, EIGHT_BITS, *self)
    }
}

impl SerializeCdis for u16 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        write_value_unsigned(buf, cursor, SIXTEEN_BITS, *self)
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::BitArray;
    use crate::constants::{SIX_BITS, SIXTEEN_BITS};
    use crate::writing::{BitBuffer, write_value_signed, write_value_unsigned};

    #[test]
    fn write_value_unsigned_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_value_unsigned(&mut buf, 0, SIX_BITS, 0x00u8);
        assert_eq!(cursor, 6);
        assert_eq!(buf.data[0], 0x00);
    }

    #[test]
    fn write_value_unsigned_positive() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_value_unsigned(&mut buf, 0, SIX_BITS, 15u8);
        assert_eq!(cursor, 6);
        assert_eq!(buf.data[0], 0x3C);
    }

    #[test]
    fn write_value_signed_negative() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_value_signed(&mut buf, 0, SIXTEEN_BITS, -32768);
        assert_eq!(cursor, 16);
        assert_eq!(buf.data[0..2], [0x80, 0x00]);
    }

    #[test]
    fn write_value_signed_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_value_signed(&mut buf, 0, SIXTEEN_BITS, 0);
        assert_eq!(cursor, 16);
        assert_eq!(buf.data[0..2], [0x00, 0x00]);
    }

    #[test]
    fn write_value_signed_negative_positive() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_value_signed(&mut buf, 0, SIXTEEN_BITS, 32767);
        assert_eq!(cursor, 16);
        assert_eq!(buf.data[0..2], [0x7F, 0xFF]);
    }

    #[test]
    fn write_value_signed_full_bit_size() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_value_signed(&mut buf, 0, SIXTEEN_BITS, -1);
        assert_eq!(cursor, 16);
        assert_eq!(buf.data[0..2], [0xFF, 0xFF]);
    }
}
