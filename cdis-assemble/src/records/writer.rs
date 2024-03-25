use bitvec::field::BitField;
use bitvec::macros::internal::funty::Integral;
use crate::records::model::{AngularVelocity, EntityCoordinateVector, EntityId, EntityType, LinearVelocity};
use crate::{BitBuffer, SerializeCdis};
use crate::constants::{FOUR_BITS, NINE_BITS};
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