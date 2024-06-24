use num::{ToPrimitive, Zero};
use dis_rs::model::{DescriptorRecord};
use dis_rs::NO_FIRE_MISSION;
use crate::codec::Codec;
use crate::fire::model::Fire;
use crate::records::codec::encode_world_coordinates;
use crate::records::model::{EntityId, EntityType, LinearVelocity};
use crate::types::model::{UVINT32};

type Counterpart = dis_rs::fire::model::Fire;

impl Fire {
    pub fn encode(item: &Counterpart) -> Self {
        let fire_mission_index = if item.fire_mission_index != NO_FIRE_MISSION as u32 {
            Some(UVINT32::from(item.fire_mission_index))
        } else { None };
        let (location_world_coordinates, units) = encode_world_coordinates(&item.location_in_world);
        let (descriptor_entity_type,
            descriptor_warhead,
            descriptor_fuze,
            descriptor_quantity,
            descriptor_rate) =
                encode_fire_descriptor(&item.descriptor);
        let range = if item.range.is_normal() {
            if let Some(range) = item.range.to_u32() {
                Some(UVINT32::from(range))
            } else { None }
        } else { None };

        Fire {
            units,
            firing_entity_id: EntityId::encode(&item.firing_entity_id),
            target_entity_id: EntityId::encode(&item.target_entity_id),
            munition_expandable_entity_id: EntityId::encode(&item.entity_id),
            event_id: EntityId::from(&item.event_id),
            fire_mission_index,
            location_world_coordinates,
            descriptor_entity_type,
            descriptor_warhead,
            descriptor_fuze,
            descriptor_quantity,
            descriptor_rate,
            velocity: LinearVelocity::encode(&item.velocity),
            range,
        }
    }

    pub fn decode(&self) -> Counterpart {

    }
}

fn encode_fire_descriptor(item : &DescriptorRecord) -> (EntityType, Option<u16>, Option<u16>, Option<u8>, Option<u8>) {
    match item {
        DescriptorRecord::Munition { entity_type, munition } => {
            let warhead = Some(munition.warhead.into());
            let fuze = Some(munition.fuse.into());
            let quantity = if munition.quantity.is_zero() { None } else { Some(munition.quantity as u8) }; // FIXME u16 to u8
            let rate = if munition.rate.is_zero() { None } else { Some(munition.rate as u8) }; // FIXME u16 to u8
            (EntityType::encode(entity_type), warhead, fuze, quantity, rate)
        }
        DescriptorRecord::Expendable { entity_type } => {
            (EntityType::encode(entity_type), None, None, None, None)
        }
        DescriptorRecord::Explosion { entity_type, explosive_material, explosive_force} => {
            (EntityType::encode(entity_type), None, None, None, None)
        }
    }
}
