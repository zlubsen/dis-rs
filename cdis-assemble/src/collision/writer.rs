use crate::collision::model::Collision;
use crate::constants::ONE_BIT;
use crate::writing::{write_value_unsigned, SerializeCdis};
use crate::{BitBuffer, SerializeCdisPdu};
use dis_rs::enumerations::CollisionType;

impl SerializeCdisPdu for Collision {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(
            buf,
            cursor,
            ONE_BIT,
            self.units.location_entity_coordinates.into(),
        );
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.units.mass.into());

        let cursor = self.issuing_entity_id.serialize(buf, cursor);
        let cursor = self.colliding_entity_id.serialize(buf, cursor);
        let cursor = self.event_id.serialize(buf, cursor);

        // CollisionType holds more than two values; all other than Elastic and Inelastic default to Inelastic (0 wire value)
        let collision_type: u8 = match self.collision_type {
            CollisionType::Elastic => 1,
            _ => 0,
        };
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, collision_type);

        let cursor = self.velocity.serialize(buf, cursor);
        let cursor = self.mass.serialize(buf, cursor);
        let cursor = self.location.serialize(buf, cursor);

        cursor
    }
}

#[cfg(test)]
mod tests {
    use crate::collision::model::{Collision, CollisionUnits};
    use crate::records::model::{
        EntityCoordinateVector, EntityId, LinearVelocity, UnitsMass, UnitsMeters,
    };
    use crate::types::model::{SVINT16, UVINT16, UVINT32};
    use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
    use bitvec::array::BitArray;
    use dis_rs::enumerations::CollisionType;

    #[test]
    fn serialize_collision() {
        let cdis_body = Collision {
            units: CollisionUnits {
                location_entity_coordinates: UnitsMeters::Centimeter,
                mass: UnitsMass::Grams,
            },
            issuing_entity_id: EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(1)),
            colliding_entity_id: EntityId::new(
                UVINT16::from(2),
                UVINT16::from(2),
                UVINT16::from(2),
            ),
            event_id: EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(3)),
            collision_type: CollisionType::Inelastic,
            velocity: LinearVelocity::new(SVINT16::from(1), SVINT16::from(1), SVINT16::from(1)),
            mass: UVINT32::from(100),
            location: EntityCoordinateVector::new(
                SVINT16::from(1),
                SVINT16::from(1),
                SVINT16::from(1),
            ),
        }
        .into_cdis_body();

        let mut buf: BitBuffer = BitArray::ZERO;
        let cursor = cdis_body.serialize(&mut buf, 0);

        assert_eq!(cursor, cdis_body.body_length());

        #[rustfmt::skip]
        #[allow(clippy::unusual_byte_groupings)]
        #[allow(clippy::unreadable_literal)]
        let expected = [
            0b00_000000,
            0b0001_0000,
            0b000001_00,
            0b00000001,
            0b_00000000,
            0b10_000000,
            0b0010_0000,
            0b000010_00,
            0b00000001,
            0b_00000000,
            0b01_000000,
            0b0011_0_000,
            0b0000001_0,
            0b00000000,
            0b1_0000000,
            0b001_00011,
            0b00100_000,
            0b0000001_0,
            0b00000000,
            0b1_0000000,
            0b001_00000,
        ];
        // fields                  ^u^ entityid                                       ^ entityid                                   ^ eventid                                        ^c^ velocity 1,1,1                                 ^ mass         ^ entity location                                ^ no remainder
        // bits                    ^2^ 3x 10                                          ^ 3x 10                                      ^ 3x 10                                          ^1^ 3x 10                                          ^ 10           ^ 3x 10                                          ^
        // values                  ^0^ 1,1,1                                          ^ 2,2,2                                      ^ 1,1,3                                          ^0^ 1,1,1                                          ^ 100          ^ 1 1 1                                          ^

        assert_eq!(buf.data[..21], expected);
    }
}
