use bitvec::field::BitField;
use crate::BitBuffer;
use crate::SerializeCdis;
use crate::types::model::{SVINT12, Svint12BitSize, SVINT13, Svint13BitSize, SVINT14, Svint14BitSize, SVINT16, Svint16BitSize, SVINT24, Svint24BitSize, UVINT16, Uvint16BitSize, UVINT32, Uvint32BitSize, UVINT8, Uvint8BitSize};

impl SerializeCdis for UVINT8 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_position = cursor + Uvint8BitSize::FLAG_BITS;
        buf[cursor..next_position].store_be(self.flag_bits_value());
        let cursor = next_position;
        let next_position = cursor+self.bit_size();
        buf[cursor..next_position].store_be(self.value);

        next_position
    }
}

impl SerializeCdis for UVINT16 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_position = cursor + Uvint16BitSize::FLAG_SIZE;
        buf[cursor..next_position].store_be(self.flag_bits_value());
        let cursor = next_position;
        let next_position = cursor+self.bit_size();
        buf[cursor..next_position].store_be(self.value);

        next_position
    }
}

impl SerializeCdis for UVINT32 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_cursor = cursor + Uvint32BitSize::FLAG_SIZE;
        buf[cursor..next_cursor].store_be(self.flag_bits_value());
        let cursor = next_cursor;
        let next_cursor = cursor+self.bit_size();
        buf[cursor..next_cursor].store_be(self.value);

        next_cursor
    }
}

impl SerializeCdis for SVINT12 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_cursor = cursor + Svint12BitSize::FLAG_SIZE;
        buf[cursor..next_cursor].store_be(self.flag_bits_value());
        let cursor = next_cursor;
        let next_cursor = next_cursor + 1;
        let sign_bit = self.value.is_negative();
        buf[cursor..next_cursor].store_be(if sign_bit {1u8} else { 0u8});
        let cursor = next_cursor;
        let next_cursor = next_cursor + self.bit_size() - 1;
        let field_value = - (if sign_bit { self.min_value() } else { 0 } - self.value);
        buf[cursor..next_cursor].store_be(field_value);

        next_cursor
    }
}

impl SerializeCdis for SVINT13 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_cursor = cursor + Svint13BitSize::FLAG_SIZE;
        buf[cursor..next_cursor].store_be(self.flag_bits_value());
        let cursor = next_cursor;
        let next_cursor = next_cursor + 1;
        let sign_bit = self.value.is_negative();
        buf[cursor..next_cursor].store_be(if sign_bit {1u8} else { 0u8});
        let cursor = next_cursor;
        let next_cursor = next_cursor + self.bit_size() - 1;
        let field_value = - (if sign_bit { self.min_value() } else { 0 } - self.value);
        buf[cursor..next_cursor].store_be(field_value);

        next_cursor
    }
}

impl SerializeCdis for SVINT14 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_cursor = cursor + Svint14BitSize::FLAG_SIZE;
        buf[cursor..next_cursor].store_be(self.flag_bits_value());
        let cursor = next_cursor;
        let next_cursor = next_cursor + 1;
        let sign_bit = self.value.is_negative();
        buf[cursor..next_cursor].store_be(if sign_bit {1u8} else { 0u8});
        let cursor = next_cursor;
        let next_cursor = next_cursor + self.bit_size() - 1;
        let field_value = - (if sign_bit { self.min_value() } else { 0 } - self.value);
        buf[cursor..next_cursor].store_be(field_value);

        next_cursor
    }
}

impl SerializeCdis for SVINT16 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_cursor = cursor + Svint16BitSize::FLAG_SIZE;
        buf[cursor..next_cursor].store_be(self.flag_bits_value());
        let cursor = next_cursor;
        let next_cursor = next_cursor + 1;
        let sign_bit = self.value.is_negative();
        buf[cursor..next_cursor].store_be(if sign_bit {1u8} else { 0u8});
        let cursor = next_cursor;
        let next_cursor = next_cursor + self.bit_size() - 1;
        let field_value = - (if sign_bit { self.min_value() } else { 0 } - self.value);
        buf[cursor..next_cursor].store_be(field_value);

        next_cursor
    }
}

impl SerializeCdis for SVINT24 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_cursor = cursor + Svint24BitSize::FLAG_SIZE;
        buf[cursor..next_cursor].store_be(self.flag_bits_value());
        let cursor = next_cursor;
        let next_cursor = next_cursor + 1;
        let sign_bit = self.value.is_negative();
        buf[cursor..next_cursor].store_be(if sign_bit {1u8} else { 0u8});
        let cursor = next_cursor;
        let next_cursor = next_cursor + self.bit_size() - 1;
        let field_value = - (if sign_bit { self.min_value() } else { 0 } - self.value);
        buf[cursor..next_cursor].store_be(field_value);

        next_cursor
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::{BitArray};
    use crate::{BitBuffer, SerializeCdis};
    use crate::types::model::{SVINT12, Svint12BitSize, UVINT16, Uvint16BitSize, UVINT8, Uvint8BitSize};

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