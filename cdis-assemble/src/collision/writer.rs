use dis_rs::enumerations::CollisionType;
use crate::collision::model::Collision;
use crate::{BitBuffer, SerializeCdisPdu};
use crate::constants::ONE_BIT;
use crate::writing::{SerializeCdis, write_value_unsigned};

impl SerializeCdisPdu for Collision {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.units.location_entity_coordinates.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.units.mass.into());

        let cursor = self.issuing_entity_id.serialize(buf, cursor);
        let cursor = self.colliding_entity_id.serialize(buf, cursor);
        let cursor = self.event_id.serialize(buf, cursor);

        // CollisionType holds more than two values; all other than Elastic and Inelastic default to Inelastic (0 wire value)
        let collision_type : u8 = match self.collision_type {
            CollisionType::Elastic => { 1 }
            _ => { 0 }
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
    #[test]
    fn serialize_collision() {
        todo!()
    }
}