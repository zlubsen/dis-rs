use dis_rs::model::EventId;
use crate::codec::Codec;
use crate::collision::model::{Collision, CollisionUnits};
use crate::records::codec::{decode_entity_coordinate_vector, encode_entity_coordinate_vector};
use crate::records::model::{EntityId, LinearVelocity, UnitsMass};
use crate::types::model::UVINT32;

use num_traits::FromPrimitive;

type Counterpart = dis_rs::collision::model::Collision;

impl Collision {
    pub fn encode(item: &Counterpart) -> Self {
        let (location, units_location) =
            encode_entity_coordinate_vector(&item.location);
        let (mass, units_mass) = encode_collision_mass(item.mass);
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
        Counterpart::builder()
            .with_issuing_entity_id(self.issuing_entity_id.decode())
            .with_colliding_entity_id(self.colliding_entity_id.decode())
            .with_event_id(EventId::from(&self.event_id))
            .with_collision_type(self.collision_type)
            .with_velocity(self.velocity.decode())
            .with_mass(decode_collision_mass(self.mass.value, self.units.mass))
            .with_location(decode_entity_coordinate_vector(&self.location, self.units.location_entity_coordinates))
            .build()
    }
}

/// Encode DIS mass field to C-DIS mass type and units.
///
/// DIS has a f32 value in Kilograms. C-DIS has a UVINT32 in either Grams or Kilograms as indicated by the UnitsMeters enum.
fn encode_collision_mass(mass: f32) -> (UVINT32, UnitsMass) {
    const MAX_NUMBER_AS_GRAMS_IN_KG: f32 = 65.535;
    if mass > MAX_NUMBER_AS_GRAMS_IN_KG {
        let mass = u32::from_f32(mass * 1000.0).unwrap_or_else(|| u32::MAX);
        (UVINT32::from(mass), UnitsMass::Grams)
    } else {
        let mass = u32::from_f32(mass).unwrap_or_else(|| u32::MAX);
        (UVINT32::from(mass), UnitsMass::Kilograms)
    }
}

fn decode_collision_mass(mass: u32, unit: UnitsMass) -> f32 {
    let mass = f32::from_u32(mass).unwrap_or(0.0);
    match unit {
        UnitsMass::Grams => { mass / 1000.0 }
        UnitsMass::Kilograms => { mass }
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

    fn create_basic_dis_collision_body() -> CollisionBuilder {
        Collision::builder()
            .with_issuing_entity_id(DisEntityId::new(20, 20, 20))
            .with_colliding_entity_id(DisEntityId::new(10, 10, 500))
            .with_event_id(EventId::new(SimulationAddress::new(10, 10), 1))
        //     .with_velocity(VectorF32::new(10.0, 10.0, 10.0))
        //     .with_world_location(Location::new(20000.0, 20000.0, 20000.0))
    }

    #[test]
    fn detonation_body_encode_units_centimeters() {
        todo!()
    }
}