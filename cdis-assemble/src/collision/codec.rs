use crate::collision::model::{Collision, CollisionUnits};
use crate::records::codec::encode_entity_coordinate_vector;
use crate::records::model::{EntityId, LinearVelocity, UnitsMass};
use crate::types::model::UVINT32;

type Counterpart = dis_rs::collision::model::Collision;

impl Collision {
    pub fn encode(item: &Counterpart) -> Self {
        let (location, units_location) =
            encode_entity_coordinate_vector(&item.location);
        let (mass, units_mass) = {
            if item.mass > 65.535 {
                let mass = u32::from_f32(item.mass * 1000.0).unwrap_or_else(|| u32::MAX);
                (mass, UnitsMass::Grams)
            } else {
                let mass = u32::from_f32(item.mass).unwrap_or_else(|| u32::MAX);
                (mass, UnitsMass::Kilograms)
            }
        };
        let units = CollisionUnits {
            location_entity_coordinates: units_location,
            mass: units_mass,
        };

        Collision {
            units,
            issuing_entity_id: EntityId::encode(&item.issuing_entity_id),
            colliding_entity_id: EntityId::encode(&item.colliding_entity_id),
            event_id: EntityId::from(&item.event_id),
            collision_type: item.collision_type,
            velocity: LinearVelocity::encode(&item.velocity),
            mass: UVINT32::from(mass),
            location,
        }
    }

    pub fn decode(&self) -> Counterpart {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::collision::builder::CollisionBuilder;
    use dis_rs::collision::model::Collision;
    use dis_rs::enumerations::{EntityKind, PlatformDomain};
    use dis_rs::model::{EntityId as DisEntityId, EntityType as DisEntityType, EventId, Location, PduBody, SimulationAddress, VectorF32};
    use crate::{BodyProperties, CdisBody};
    use crate::codec::{CodecOptions, CodecStateResult, DecoderState, EncoderState};
    use crate::records::model::{EntityCoordinateVector, EntityId, EntityType, LinearVelocity, UnitsDekameters, UnitsMass};
    use crate::types::model::{UVINT32};

    fn create_basic_dis_collision_body() -> CollisionBuilder {
        Collision::builder()
            .with_issuing_entity_id(DisEntityId::new(20, 20, 20))
            .with_colliding_entity_id(DisEntityId::new(10, 10, 500))
            .with_event_id(EventId::new(SimulationAddress::new(10, 10), 1))
        //     .with_velocity(VectorF32::new(10.0, 10.0, 10.0))
        //     .with_world_location(Location::new(20000.0, 20000.0, 20000.0))
    }

    #[test]
    fn detonation_body_encode_units_centimeters() {}
}