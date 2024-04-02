use crate::BitBuffer;
use crate::constants::ONE_BIT;
use crate::utils::write_value_with_length;
use crate::SerializeCdis;
use crate::types::model::{CdisFloat, SVINT12, SVINT13, SVINT14, SVINT16, SVINT24, UVINT16, UVINT32, UVINT8};
use crate::types::model::VarInt;

impl SerializeCdis for UVINT8 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_with_length(
            buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for UVINT16 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_with_length(
            buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for UVINT32 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_with_length(
            buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for SVINT12 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_with_length(
            buf, cursor, ONE_BIT, if self.value.is_negative() { 1u8 } else { 0u8 });
        let field_value = - (if self.value.is_negative() { self.min_value() } else { 0 } - self.value);
        let cursor = write_value_with_length(
            buf, cursor, self.bit_size() - 1, field_value);

        cursor
    }
}

impl SerializeCdis for SVINT13 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_with_length(
            buf, cursor, ONE_BIT, if self.value.is_negative() { 1u8 } else { 0u8 });
        let field_value = - (if self.value.is_negative() { self.min_value() } else { 0 } - self.value);
        let cursor = write_value_with_length(
            buf, cursor, self.bit_size() - 1, field_value);

        cursor
    }
}

impl SerializeCdis for SVINT14 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_with_length(
            buf, cursor, ONE_BIT, if self.value.is_negative() { 1u8 } else { 0u8 });
        let field_value = - (if self.value.is_negative() { self.min_value() } else { 0 } - self.value);
        let cursor = write_value_with_length(
            buf, cursor, self.bit_size() - 1, field_value);

        cursor
    }
}

impl SerializeCdis for SVINT16 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_with_length(
            buf, cursor, ONE_BIT, if self.value.is_negative() { 1u8 } else { 0u8 });
        let field_value = - (if self.value.is_negative() { self.min_value() } else { 0 } - self.value);
        let cursor = write_value_with_length(
            buf, cursor, self.bit_size() - 1, field_value);

        cursor
    }
}

impl SerializeCdis for SVINT24 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_with_length(
            buf, cursor, ONE_BIT, u8::from(self.value.is_negative()));
        let field_value = - (if self.value.is_negative() { self.min_value() } else { 0 } - self.value);
        let cursor = write_value_with_length(
            buf, cursor, self.bit_size() - 1, field_value);

        cursor
    }
}

pub(crate) fn serialize_cdis_float<T: CdisFloat>(float: T, buf: &mut BitBuffer, cursor: usize) -> usize {
    let cursor = write_value_with_length(
        buf, cursor, ONE_BIT, u8::from(float.mantissa().is_negative()));
    let mantissa_value = - (if float.mantissa().is_negative() { i32::MIN } else { 0 } - float.mantissa());
    let cursor = write_value_with_length(
        buf, cursor, float.mantissa_bit_size(), mantissa_value);

    let cursor = write_value_with_length(
        buf, cursor, ONE_BIT, u8::from(float.exponent().is_negative()));
    let exponent_value = - (if float.exponent().is_negative() { i8::MIN } else { 0 } - float.exponent());
    let cursor = write_value_with_length(
        buf, cursor, float.exponent_bit_size(), exponent_value);

    cursor
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::{BitArray};
    use crate::{BitBuffer, SerializeCdis};
    use crate::types::model::{SVINT12, Svint12BitSize, UVINT16, Uvint16BitSize, UVINT8, Uvint8BitSize};
    use crate::types::model::VarInt;

    const ONE_BYTE: usize = 1;
    const TWO_BYTES: usize = 2;
    const THREE_BYTES: usize = 3;

    #[test]
    fn serialize_uvint8_bit_flag_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT8::new(Uvint8BitSize::Four, 1);
        let expected : [u8; ONE_BYTE] = [0b00001000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[0], buf.as_raw_slice()[0]);
    }

    #[test]
    fn serialize_uvint8_bit_flag_one() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT8::new(Uvint8BitSize::Eight, 129);
        let expected : [u8; TWO_BYTES] = [0b11000000, 0b10000000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..2], buf.as_raw_slice()[..2]);
    }

    #[test]
    fn serialize_uvint16_bit_flag_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT16::new(Uvint16BitSize::Eight, 1);
        let expected : [u8; TWO_BYTES] = [0b00000000, 0b01000000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..TWO_BYTES], buf.as_raw_slice()[..TWO_BYTES]);
    }

    #[test]
    fn serialize_uvint16_bit_flag_three() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT16::new(Uvint16BitSize::Sixteen, 32767);
        let expected : [u8; THREE_BYTES] = [0b11011111, 0b11111111, 0b11000000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..THREE_BYTES], buf.as_raw_slice()[..THREE_BYTES]);
    }

    #[test]
    fn serialize_svint12_bit_flag_zero_positive() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = SVINT12::new(Svint12BitSize::Three, 1);
        let expected : [u8; ONE_BYTE] = [0b00001000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..ONE_BYTE], buf.as_raw_slice()[..ONE_BYTE]);
    }

    #[test]
    fn serialize_svint12_bit_flag_zero_negative() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = SVINT12::new(Svint12BitSize::Three, -3);
        let expected : [u8; ONE_BYTE] = [0b00101000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..ONE_BYTE], buf.as_raw_slice()[..ONE_BYTE]);
    }

    #[test]
    fn serialize_svint12_bit_flag_three_positive() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = SVINT12::new(Svint12BitSize::Twelve, 2047);
        let expected : [u8; TWO_BYTES] = [0b11011111, 0b11111100];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..TWO_BYTES], buf.as_raw_slice()[..TWO_BYTES]);
    }

    #[test]
    fn serialize_svint12_bit_flag_three_negative() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = SVINT12::new(Svint12BitSize::Twelve, -2047);
        let expected : [u8; TWO_BYTES] = [0b11100000, 0b00000100];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..TWO_BYTES], buf.as_raw_slice()[..TWO_BYTES]);
    }
}