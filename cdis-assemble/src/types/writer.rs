use crate::writing::{BitBuffer, write_value_signed, write_value_unsigned};
use crate::writing::SerializeCdis;
use crate::types::model::{CdisFloat, SVINT12, SVINT13, SVINT14, SVINT16, SVINT24, UVINT16, UVINT32, UVINT8};
use crate::types::model::VarInt;

impl SerializeCdis for UVINT8 {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_unsigned(
            buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for UVINT16 {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_unsigned(
            buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for UVINT32 {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_unsigned(
            buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for SVINT12 {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_signed(buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for SVINT13 {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_signed(buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for SVINT14 {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_signed(buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for SVINT16 {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_signed(buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

impl SerializeCdis for SVINT24 {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf, cursor, self.flag_bits_size(), self.flag_bits_value());
        let cursor = write_value_signed(buf, cursor, self.bit_size(), self.value);

        cursor
    }
}

#[allow(clippy::let_and_return)]
pub(crate) fn serialize_cdis_float<T: CdisFloat>(buf: &mut BitBuffer, cursor: usize, float: &T) -> usize {
    let cursor = write_value_signed(buf, cursor, float.mantissa_bit_size(), float.mantissa());
    let cursor = write_value_signed(buf, cursor, float.exponent_bit_size(), float.exponent());

    cursor
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::BitArray;
    use crate::records::model::ParameterValueFloat;
    use crate::writing::SerializeCdis;
    use crate::types::model::{CdisFloat, SVINT12, UVINT16, UVINT8};
    use crate::types::writer::serialize_cdis_float;
    use crate::writing::BitBuffer;

    const ONE_BYTE: usize = 1;
    const TWO_BYTES: usize = 2;
    const THREE_BYTES: usize = 3;

    #[test]
    fn serialize_uvint8_bit_flag_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT8::from(1);
        let expected : [u8; ONE_BYTE] = [0b00001000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[0], buf.as_raw_slice()[0]);
    }

    #[test]
    fn serialize_uvint8_bit_flag_one() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT8::from(129);
        let expected : [u8; TWO_BYTES] = [0b11000000, 0b10000000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..2], buf.as_raw_slice()[..2]);
    }

    #[test]
    fn serialize_uvint16_bit_flag_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT16::from(1);
        let expected : [u8; TWO_BYTES] = [0b00000000, 0b01000000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..TWO_BYTES], buf.as_raw_slice()[..TWO_BYTES]);
    }

    #[test]
    fn serialize_uvint16_bit_flag_three() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT16::from(32767);
        let expected : [u8; THREE_BYTES] = [0b11011111, 0b11111111, 0b11000000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..THREE_BYTES], buf.as_raw_slice()[..THREE_BYTES]);
    }

    #[test]
    fn serialize_svint12_bit_flag_zero_positive() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = SVINT12::from(1);
        let expected : [u8; ONE_BYTE] = [0b00001000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..ONE_BYTE], buf.as_raw_slice()[..ONE_BYTE]);
    }

    #[test]
    fn serialize_svint12_bit_flag_zero_negative() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = SVINT12::from(-3);
        let expected : [u8; ONE_BYTE] = [0b00101000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..ONE_BYTE], buf.as_raw_slice()[..ONE_BYTE]);
    }

    #[test]
    fn serialize_svint12_bit_flag_three_positive() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = SVINT12::from(2047);
        let expected : [u8; TWO_BYTES] = [0b11011111, 0b11111100];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..TWO_BYTES], buf.as_raw_slice()[..TWO_BYTES]);
    }

    #[test]
    fn serialize_svint12_bit_flag_three_negative() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = SVINT12::from(-2047);
        let expected : [u8; TWO_BYTES] = [0b11100000, 0b00000100];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..TWO_BYTES], buf.as_raw_slice()[..TWO_BYTES]);
    }

    #[test]
    fn serialize_cdis_float_one_and_one() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = ParameterValueFloat::new(1, 1);
        // ParameterValueFloat has 15 bit mantissa and 3 bit exponent fields
        let expected: [u8; THREE_BYTES] = [0b00000000, 0b00000010, 0b01000000];
        let cursor = serialize_cdis_float(&mut buf, 0, &input);

        assert_eq!(cursor, 18);
        assert_eq!(expected, buf.as_raw_slice()[..THREE_BYTES]);

    }
}