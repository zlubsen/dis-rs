use bitvec::field::BitField;
use crate::BitBuffer;
use crate::SerializeCdis;
use crate::types::model::{UVINT16, UVINT32, UVINT8};

impl SerializeCdis for UVINT8 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_position = cursor+self.flag_bits_size();
        buf[cursor..next_position].store_be(self.flag_bits_value());
        let cursor = next_position;
        let next_position = cursor+self.bit_size();
        buf[cursor..next_position].store_be(self.value);

        next_position
    }
}

impl SerializeCdis for UVINT16 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_position = cursor+self.flag_bits_size();
        buf[cursor..next_position].store_be(self.flag_bits_value());
        let cursor = next_position;
        let next_position = cursor+self.bit_size();
        buf[cursor..next_position].store_be(self.value);

        next_position
    }
}

impl SerializeCdis for UVINT32 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let next_position = cursor+self.flag_bits_size();
        buf[cursor..next_position].store_be(self.flag_bits_value());
        let cursor = next_position;
        let next_position = cursor+self.bit_size();
        buf[cursor..next_position].store_be(self.value);

        next_position
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::{BitArray};
    use crate::{BitBuffer, SerializeCdis};
    use crate::types::model::{UVINT16, Uvint16BitSize, UVINT8, Uvint8BitSize};

    #[test]
    fn serialize_uvint8_bit_flag_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT8::new(Uvint8BitSize::Four, 1);
        let expected : [u8; 1] = [0b00001000];
        let next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[0], buf.as_raw_slice()[0]);
    }

    #[test]
    fn serialize_uvint8_bit_flag_one() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT8::new(Uvint8BitSize::Eight, 129);
        let expected : [u8; 2] = [0b11000000, 0b10000000];
        let next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..2], buf.as_raw_slice()[..2]);
    }

    #[test]
    fn serialize_uvint16_bit_flag_zero() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT16::new(Uvint16BitSize::Eight, 1);
        let expected : [u8; 2] = [0b00000000, 0b01000000];
        let next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..2], buf.as_raw_slice()[..2]);
    }

    #[test]
    fn serialize_uvint16_bit_flag_three() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = UVINT16::new(Uvint16BitSize::Sixteen, 32767);
        let expected : [u8; 3] = [0b11011111, 0b11111111, 0b11000000];
        let next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..3], buf.as_raw_slice()[..3]);
    }
}