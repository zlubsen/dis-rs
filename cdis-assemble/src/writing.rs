use crate::constants::{EIGHT_BITS, MTU_BITS, SIXTEEN_BITS};
use crate::{CdisBody, CdisPdu};
use bitvec::array::BitArray;
use bitvec::field::BitField;
use bitvec::macros::internal::funty::Integral;
use bitvec::order::Msb0;

pub type BitBuffer = BitArray<[u8; MTU_BITS], Msb0>;

#[must_use]
pub fn create_bit_buffer() -> BitBuffer {
    let buf: BitBuffer = BitArray::ZERO;
    buf
}

pub trait SerializeCdis {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize;
}

impl SerializeCdis for CdisPdu {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.header.serialize(buf, cursor);
        let cursor = self.body.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for CdisBody {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = match self {
            CdisBody::Unsupported(_body) => cursor,
            CdisBody::EntityState(body) => body.serialize(buf, cursor),
            CdisBody::Fire(body) => body.serialize(buf, cursor),
            CdisBody::Detonation(body) => body.serialize(buf, cursor),
            CdisBody::Collision(body) => body.serialize(buf, cursor),
            CdisBody::CreateEntity(body) => body.serialize(buf, cursor),
            CdisBody::RemoveEntity(body) => body.serialize(buf, cursor),
            CdisBody::StartResume(body) => body.serialize(buf, cursor),
            CdisBody::StopFreeze(body) => body.serialize(buf, cursor),
            CdisBody::Acknowledge(body) => body.serialize(buf, cursor),
            CdisBody::ActionRequest(body) => body.serialize(buf, cursor),
            CdisBody::ActionResponse(body) => body.serialize(buf, cursor),
            CdisBody::DataQuery(body) => body.serialize(buf, cursor),
            CdisBody::SetData(body) => body.serialize(buf, cursor),
            CdisBody::Data(body) => body.serialize(buf, cursor),
            CdisBody::EventReport(body) => body.serialize(buf, cursor),
            CdisBody::Comment(body) => body.serialize(buf, cursor),
            CdisBody::ElectromagneticEmission(body) => body.serialize(buf, cursor),
            CdisBody::Designator(body) => body.serialize(buf, cursor),
            CdisBody::Transmitter(body) => body.serialize(buf, cursor),
            CdisBody::Signal(body) => body.serialize(buf, cursor),
            CdisBody::Receiver(body) => body.serialize(buf, cursor),
            CdisBody::Iff(body) => body.serialize(buf, cursor),
        };

        cursor
    }
}

/// Write `value` to the `BitBuffer` `buf`, at the position of `cursor` with length `bit_size`.
/// Returns the new cursor position.
pub(crate) fn write_integer_bits<T: Integral>(
    buf: &mut BitBuffer,
    cursor: usize,
    bit_size: usize,
    value: T,
) -> usize {
    let next_cursor = cursor + bit_size;
    buf[cursor..next_cursor].store_be(value);
    next_cursor
}

/// Helper function that checks if the provided `Option` is `Some`, and then serializes the contained value.
/// Field must implement trait `SerializeCdis`.
#[allow(clippy::ref_option)]
pub(crate) fn serialize_when_present<I: SerializeCdis>(
    field: &Option<I>,
    buf: &mut BitBuffer,
    cursor: usize,
) -> usize {
    if let Some(inner) = field {
        inner.serialize(buf, cursor)
    } else {
        cursor
    }
}

impl SerializeCdis for u8 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        write_integer_bits(buf, cursor, EIGHT_BITS, *self)
    }
}

impl SerializeCdis for u16 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        write_integer_bits(buf, cursor, SIXTEEN_BITS, *self)
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{SIX_BITS, SIXTEEN_BITS};
    use crate::writing::{BitBuffer, write_integer_bits};
    use bitvec::prelude::BitArray;

    #[test]
    fn write_value_unsigned_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_integer_bits(&mut buf, 0, SIX_BITS, 0x00u8);
        assert_eq!(cursor, 6);
        assert_eq!(buf.data[0], 0x00);
    }

    #[test]
    fn write_value_unsigned_positive() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_integer_bits(&mut buf, 0, SIX_BITS, 15u8);
        assert_eq!(cursor, 6);
        assert_eq!(buf.data[0], 0x3C);
    }

    #[test]
    fn write_value_signed_negative() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_integer_bits(&mut buf, 0, SIXTEEN_BITS, -32768);
        assert_eq!(cursor, 16);
        assert_eq!(buf.data[0..2], [0x80, 0x00]);
    }

    #[test]
    fn write_value_signed_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_integer_bits(&mut buf, 0, SIXTEEN_BITS, 0);
        assert_eq!(cursor, 16);
        assert_eq!(buf.data[0..2], [0x00, 0x00]);
    }

    #[test]
    fn write_value_signed_negative_positive() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_integer_bits(&mut buf, 0, SIXTEEN_BITS, 32767);
        assert_eq!(cursor, 16);
        assert_eq!(buf.data[0..2], [0x7F, 0xFF]);
    }

    #[test]
    fn write_value_signed_full_bit_size() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let cursor = write_integer_bits(&mut buf, 0, SIXTEEN_BITS, -1);
        assert_eq!(cursor, 16);
        assert_eq!(buf.data[0..2], [0xFF, 0xFF]);
    }
}
