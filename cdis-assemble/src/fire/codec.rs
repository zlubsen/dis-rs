use num::{ToPrimitive, Zero};
use dis_rs::enumerations::{EntityKind, MunitionDescriptorFuse, MunitionDescriptorWarhead};
use dis_rs::model::{DescriptorRecord, EventId, MunitionDescriptor};
use dis_rs::NO_FIRE_MISSION;
use crate::codec::Codec;
use crate::fire::model::Fire;
use crate::records::codec::{decode_world_coordinates, encode_world_coordinates};
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
        let entity_type = self.descriptor_entity_type.decode();
        let fire_mission_index = {
            if let Some(fmi) = self.fire_mission_index {
                fmi.value
            } else { NO_FIRE_MISSION }
        };
        let descriptor = decode_fire_descriptor(&self, entity_type);
        Counterpart::builder()
            .with_firing_entity_id(self.firing_entity_id.decode())
            .with_target_entity_id(self.target_entity_id.decode())
            .with_entity_id(self.munition_expandable_entity_id.decode())
            .with_event_id(EventId::from(&self.event_id))
            .with_fire_mission_index(fire_mission_index)
            .with_location_in_world(decode_world_coordinates(&self.location_world_coordinates, self.units))
            .with_descriptor(descriptor)
            .with_velocity(self.velocity.decode())
            .with_range(self.range.unwrap_or_default().value as f32)
            .build()
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

fn decode_fire_descriptor(fire_body: &Fire, entity_type: dis_rs::model::EntityType) -> DescriptorRecord {
    match entity_type.kind {
        EntityKind::Munition => {
            DescriptorRecord::new_munition(entity_type, MunitionDescriptor::default()
                .with_warhead(MunitionDescriptorWarhead::from(fire_body.descriptor_warhead.unwrap_or_default()))
                .with_fuse(MunitionDescriptorFuse::from(fire_body.descriptor_fuze.unwrap_or_default()))
                .with_quantity(fire_body.descriptor_quantity.unwrap_or_default() as u16)
                .with_rate(fire_body.descriptor_rate.unwrap_or_default() as u16))
        }
        EntityKind::Expendable => {
            DescriptorRecord::new_expendable(entity_type)
        }
        _ => {
            DescriptorRecord::new_munition(entity_type, MunitionDescriptor::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::enumerations::{EntityKind, MunitionDescriptorFuse, MunitionDescriptorWarhead, PlatformDomain};
    use dis_rs::fire::builder::FireBuilder;
    use dis_rs::fire::model::Fire as DisFire;
    use dis_rs::model::{EntityId as DisEntityId, EntityType as DisEntityType, EventId, Location, MunitionDescriptor, SimulationAddress};
    use crate::CdisBody;
    use crate::codec::{CodecOptions, CodecStateResult, EncoderState};
    use crate::records::model::{EntityId, EntityType, Units};
    use crate::types::model::{UVINT16, UVINT32, UVINT8};

    fn create_basic_dis_fire_body() -> FireBuilder {
        DisFire::builder()
            .with_firing_entity_id(DisEntityId::new(10, 10, 10))
            .with_target_entity_id(DisEntityId::new(20, 20, 20))
            .with_entity_id(DisEntityId::new(10, 10, 500))
            .with_event_id(EventId::new(SimulationAddress::new(10, 10), 1))
            .with_location_in_world(Location::new(20000.0, 20000.0, 20000.0))
    }

    #[test]
    fn fire_body_encode_munition() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update().use_guise(true);

        let dis_body = create_basic_dis_fire_body()
            .with_munition_descriptor(
                DisEntityType::default()
                    .with_kind(EntityKind::Munition)
                    .with_domain(PlatformDomain::Air),
                MunitionDescriptor::default()
                    .with_warhead(MunitionDescriptorWarhead::Dummy)
                    .with_fuse(MunitionDescriptorFuse::Dummy_8110)
                    .with_quantity(1)
                    .with_rate(1))
            .with_range(10000.0)
            .build().into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::Fire(fire) = cdis_body {
            assert_eq!(fire.units, Units::Dekameter);
            assert_eq!(fire.firing_entity_id, EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)));
            assert_eq!(fire.target_entity_id, EntityId::new(UVINT16::from(20), UVINT16::from(20), UVINT16::from(20)));
            assert_eq!(fire.munition_expandable_entity_id, EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(500)));
            assert_eq!(fire.event_id, EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(1)));

            assert_eq!(fire.descriptor_entity_type, EntityType::new(2, 2, 0,
                UVINT8::from(0), UVINT8::from(0), UVINT8::from(0), UVINT8::from(0)));
            assert_eq!(fire.descriptor_warhead.unwrap(), 4002u16);
            assert_eq!(fire.descriptor_fuze.unwrap(), 8110u16);
            assert_eq!(fire.descriptor_quantity.unwrap(), 1);
            assert_eq!(fire.descriptor_rate.unwrap(), 1);
            assert_eq!(fire.range.unwrap(), UVINT32::from(10000));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn fire_body_encode_expandable() {
        assert!(false);
    }

    #[test]
    fn fire_body_decode_munition() {
        assert!(false);
    }

    #[test]
    fn fire_body_decode_expandable() {
        assert!(false);
    }
}
