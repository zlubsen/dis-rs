use crate::records::model::{AngularVelocity, CdisEntityMarking, EntityCoordinateVector, EntityId, EntityType, LinearVelocity, Orientation, WorldCoordinates};
use crate::{BitBuffer, SerializeCdis};
use crate::constants::{FOUR_BITS, NINE_BITS, ONE_BIT, THIRTEEN_BITS};
use crate::parser_utils::write_value_with_length;

impl SerializeCdis for AngularVelocity {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EntityCoordinateVector {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EntityId {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.site.serialize(buf, cursor);
        let cursor = self.application.serialize(buf, cursor);
        let cursor = self.entity.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EntityType {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(buf, cursor, FOUR_BITS, self.kind);
        let cursor = write_value_with_length(buf, cursor, FOUR_BITS, self.domain);
        let cursor = write_value_with_length(buf, cursor, NINE_BITS, self.country);

        let cursor = self.category.serialize(buf, cursor);
        let cursor = self.subcategory.serialize(buf, cursor);
        let cursor = self.specific.serialize(buf, cursor);
        let cursor = self.extra.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for LinearVelocity {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for WorldCoordinates {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        self.latitude
        self.longitude
        let cursor = self.altitude_msl.serialize(buf, cursor);
        cursor
    }
}

impl SerializeCdis for Orientation {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(buf, cursor, THIRTEEN_BITS, self.psi);
        let cursor = write_value_with_length(buf, cursor, THIRTEEN_BITS, self.theta);
        let cursor = write_value_with_length(buf, cursor, THIRTEEN_BITS, self.phi);
        cursor
    }
}

impl SerializeCdis for CdisEntityMarking {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_with_length(buf, cursor, FOUR_BITS, self.marking.len());
        let cursor = write_value_with_length(buf, cursor, ONE_BIT, self.char_encoding.encoding());
        let codes: Vec<u8> = self.marking.chars()
            .map(|char| self.char_encoding.u8_from_char(char) )
            .collect();
        let cursor = codes.iter().fold(cursor, |cur, code| {
            write_value_with_length(buf, cur, self.char_encoding.bit_size(), *code)
        });
        cursor
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::{BitArray};
    use crate::{BitBuffer, SerializeCdis};
    use crate::records::model::CdisEntityMarking;

    const FOUR_BYTES: usize = 4;

    #[test]
    fn serialize_marking_five_bit_encoding() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = CdisEntityMarking::from("ABCDE");
        let expected: [u8; FOUR_BYTES] = [0b01010000, 0b01000100, 0b00110010, 0b00010100];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..FOUR_BYTES], buf.as_raw_slice()[..FOUR_BYTES]);
    }

    fn serialize_marking_six_bit_encoding() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = CdisEntityMarking::from("AAJJ");
        let expected: [u8; FOUR_BYTES] = [0b01001000, 0b00100000, 0b10010100, 0b01010000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..FOUR_BYTES], buf.as_raw_slice()[..FOUR_BYTES]);
    }
}