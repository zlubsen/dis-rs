use crate::codec::Codec;
use crate::fire::model::Fire;
use crate::records::codec::{
    decode_munition_descriptor, decode_world_coordinates, encode_munition_descriptor,
    encode_world_coordinates,
};
use crate::records::model::{
    DescriptorQuantityRate, DescriptorWarheadFuse, EntityId, EntityType, LinearVelocity,
};
use crate::types::model::UVINT32;
use dis_rs::enumerations::EntityKind;
use dis_rs::fire::model::FireDescriptor;
use dis_rs::model::{EventId, ExpendableDescriptor, MunitionDescriptor};
use dis_rs::{BodyRaw, NO_FIRE_MISSION};

type Counterpart = dis_rs::fire::model::Fire;

impl Fire {
    #[must_use]
    pub fn encode(item: &Counterpart) -> Self {
        let fire_mission_index = if item.fire_mission_index != NO_FIRE_MISSION {
            Some(UVINT32::from(item.fire_mission_index))
        } else {
            None
        };

        let (location_world_coordinates, units) = encode_world_coordinates(&item.location_in_world);

        let (descriptor_entity_type, descriptor_warhead_fuse, descriptor_quantity_rate) =
            encode_fire_descriptor(&item.descriptor);

        let range = if item.range.is_normal() {
            Some(UVINT32::from(item.range.round() as u32))
        } else {
            None
        };

        Fire {
            units,
            firing_entity_id: EntityId::encode(&item.firing_entity_id),
            target_entity_id: EntityId::encode(&item.target_entity_id),
            munition_expendable_entity_id: EntityId::encode(&item.entity_id),
            event_id: EntityId::from(&item.event_id),
            fire_mission_index,
            location_world_coordinates,
            descriptor_entity_type,
            descriptor_warhead_fuse,
            descriptor_quantity_rate,
            velocity: LinearVelocity::encode(&item.velocity),
            range,
        }
    }

    #[must_use]
    pub fn decode(&self) -> Counterpart {
        let fire_mission_index = {
            if let Some(fmi) = self.fire_mission_index {
                fmi.value
            } else {
                NO_FIRE_MISSION
            }
        };

        let location_in_world_coordinates =
            decode_world_coordinates(&self.location_world_coordinates, self.units);

        let descriptor = decode_fire_descriptor(
            &self.descriptor_entity_type,
            self.descriptor_warhead_fuse.as_ref(),
            self.descriptor_quantity_rate.as_ref(),
        );

        Counterpart::builder()
            .with_firing_entity_id(self.firing_entity_id.decode())
            .with_target_entity_id(self.target_entity_id.decode())
            .with_entity_id(self.munition_expendable_entity_id.decode())
            .with_event_id(EventId::from(&self.event_id))
            .with_fire_mission_index(fire_mission_index)
            .with_location_in_world(location_in_world_coordinates)
            .with_descriptor(descriptor)
            .with_velocity(self.velocity.decode())
            .with_range(self.range.unwrap_or_default().value as f32)
            .build()
    }
}

fn encode_fire_descriptor(
    item: &FireDescriptor,
) -> (
    EntityType,
    Option<DescriptorWarheadFuse>,
    Option<DescriptorQuantityRate>,
) {
    match item {
        FireDescriptor::Munition(munition) => encode_munition_descriptor(munition),
        FireDescriptor::Expendable(expendable) => {
            (EntityType::encode(&expendable.entity_type), None, None)
        }
    }
}

fn decode_fire_descriptor(
    descriptor_entity_type: &EntityType,
    descriptor_warhead_fuse: Option<&DescriptorWarheadFuse>,
    descriptor_quantity_rate: Option<&DescriptorQuantityRate>,
) -> FireDescriptor {
    let entity_type = descriptor_entity_type.decode();

    match entity_type.kind {
        EntityKind::Munition => FireDescriptor::Munition(decode_munition_descriptor(
            descriptor_entity_type,
            descriptor_warhead_fuse,
            descriptor_quantity_rate,
        )),
        EntityKind::Expendable => FireDescriptor::Expendable(ExpendableDescriptor { entity_type }),
        _ => {
            // Fire PDU should not have other EntityKinds than Munition or Expendable, so any others
            // are decoded into munition by default
            FireDescriptor::Munition(MunitionDescriptor::default().with_entity_type(entity_type))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::{CodecOptions, CodecStateResult, DecoderState, EncoderState};
    use crate::fire::model::{Fire, FireFieldsPresentFlags};
    use crate::records::model::{
        DescriptorQuantityRate, DescriptorWarheadFuse, EntityId, EntityType, LinearVelocity,
        UnitsDekameters, WorldCoordinates,
    };
    use crate::types::model::{SVINT16, SVINT24, UVINT8, UVINT16, UVINT32};
    use crate::{BodyProperties, CdisBody};
    use dis_rs::BodyRaw;
    use dis_rs::enumerations::{
        EntityKind, MunitionDescriptorFuse, MunitionDescriptorWarhead, PlatformDomain,
    };
    use dis_rs::fire::builder::FireBuilder;
    use dis_rs::fire::model::{Fire as DisFire, FireDescriptor};
    use dis_rs::model::{
        EntityId as DisEntityId, EntityType as DisEntityType, EventId, ExpendableDescriptor,
        Location, MunitionDescriptor, PduBody,
    };

    fn create_basic_dis_fire_body() -> FireBuilder {
        DisFire::builder()
            .with_firing_entity_id(DisEntityId::new(10, 10, 10))
            .with_target_entity_id(DisEntityId::new(20, 20, 20))
            .with_entity_id(DisEntityId::new(10, 10, 500))
            .with_event_id(EventId::new(10, 10, 1))
            .with_location_in_world(Location::new(20000.0, 20000.0, 20000.0))
    }

    #[test]
    fn fire_body_encode_munition() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update();

        let dis_body = create_basic_dis_fire_body()
            .with_descriptor(FireDescriptor::Munition(
                MunitionDescriptor::default()
                    .with_entity_type(
                        DisEntityType::default()
                            .with_kind(EntityKind::Munition)
                            .with_domain(PlatformDomain::Air),
                    )
                    .with_warhead(MunitionDescriptorWarhead::Dummy)
                    .with_fuse(MunitionDescriptorFuse::Dummy_8110)
                    .with_quantity(1)
                    .with_rate(1),
            ))
            .with_range(10000.0)
            .build()
            .into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::Fire(fire) = cdis_body {
            assert_ne!(
                fire.fields_present_field()
                    & FireFieldsPresentFlags::DESCRIPTOR_WARHEAD_FUSE.bits(),
                0u8
            );
            assert_ne!(
                fire.fields_present_field()
                    & FireFieldsPresentFlags::DESCRIPTOR_QUANTITY_RATE.bits(),
                0u8
            );

            assert_eq!(fire.units, UnitsDekameters::Dekameter);
            assert_eq!(
                fire.firing_entity_id,
                EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10))
            );
            assert_eq!(
                fire.target_entity_id,
                EntityId::new(UVINT16::from(20), UVINT16::from(20), UVINT16::from(20))
            );
            assert_eq!(
                fire.munition_expendable_entity_id,
                EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(500))
            );
            assert_eq!(
                fire.event_id,
                EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(1))
            );

            assert_eq!(
                fire.descriptor_entity_type,
                EntityType::new(
                    2,
                    2,
                    0,
                    UVINT8::from(0),
                    UVINT8::from(0),
                    UVINT8::from(0),
                    UVINT8::from(0)
                )
            );
            assert_eq!(fire.descriptor_warhead_fuse.unwrap().warhead, 4002u16);
            assert_eq!(fire.descriptor_warhead_fuse.unwrap().fuse, 8110u16);
            assert_eq!(fire.descriptor_quantity_rate.unwrap().quantity, 1);
            assert_eq!(fire.descriptor_quantity_rate.unwrap().rate, 1);
            assert_eq!(fire.range.unwrap(), UVINT32::from(10000));
        } else {
            panic!()
        }
    }

    #[test]
    fn fire_body_encode_munition_no_qnt_rt() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update();

        let dis_body = create_basic_dis_fire_body()
            .with_descriptor(FireDescriptor::Munition(
                MunitionDescriptor::default()
                    .with_entity_type(
                        DisEntityType::default()
                            .with_kind(EntityKind::Munition)
                            .with_domain(PlatformDomain::Air),
                    )
                    .with_warhead(MunitionDescriptorWarhead::Dummy)
                    .with_fuse(MunitionDescriptorFuse::Dummy_8110)
                    .with_quantity(0)
                    .with_rate(0),
            ))
            .build()
            .into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::Fire(fire) = cdis_body {
            assert_ne!(
                fire.fields_present_field()
                    & FireFieldsPresentFlags::DESCRIPTOR_WARHEAD_FUSE.bits(),
                0u8
            );
            assert_eq!(
                fire.fields_present_field()
                    & FireFieldsPresentFlags::DESCRIPTOR_QUANTITY_RATE.bits(),
                0u8
            );

            assert_eq!(fire.units, UnitsDekameters::Dekameter);
            assert_eq!(
                fire.descriptor_entity_type,
                EntityType::new(
                    2,
                    2,
                    0,
                    UVINT8::from(0),
                    UVINT8::from(0),
                    UVINT8::from(0),
                    UVINT8::from(0)
                )
            );
            assert_eq!(fire.descriptor_warhead_fuse.unwrap().warhead, 4002u16);
            assert_eq!(fire.descriptor_warhead_fuse.unwrap().fuse, 8110u16);
            assert!(fire.descriptor_quantity_rate.is_none());
        } else {
            panic!()
        }
    }

    #[test]
    fn fire_body_encode_expendable() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update();

        let dis_body = create_basic_dis_fire_body()
            .with_descriptor(FireDescriptor::Expendable(ExpendableDescriptor {
                entity_type: DisEntityType::default()
                    .with_kind(EntityKind::Expendable)
                    .with_domain(PlatformDomain::Air),
            }))
            .with_range(10000.0)
            .build()
            .into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::Fire(fire) = cdis_body {
            assert_eq!(
                fire.fields_present_field()
                    & FireFieldsPresentFlags::DESCRIPTOR_WARHEAD_FUSE.bits(),
                0u8
            );
            assert_eq!(
                fire.fields_present_field()
                    & FireFieldsPresentFlags::DESCRIPTOR_QUANTITY_RATE.bits(),
                0u8
            );

            assert_eq!(fire.units, UnitsDekameters::Dekameter);
            assert_eq!(
                fire.descriptor_entity_type,
                EntityType::new(
                    8,
                    2,
                    0,
                    UVINT8::from(0),
                    UVINT8::from(0),
                    UVINT8::from(0),
                    UVINT8::from(0)
                )
            );
            assert!(fire.descriptor_warhead_fuse.is_none());
            assert!(fire.descriptor_quantity_rate.is_none());
            assert_eq!(fire.range.unwrap(), UVINT32::from(10000));
        } else {
            panic!()
        }
    }

    #[test]
    fn fire_body_decode_munition() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_full_update();

        let cdis_body = Fire {
            units: UnitsDekameters::Dekameter,
            firing_entity_id: EntityId::new(
                UVINT16::from(10),
                UVINT16::from(10),
                UVINT16::from(10),
            ),
            target_entity_id: EntityId::new(
                UVINT16::from(20),
                UVINT16::from(20),
                UVINT16::from(20),
            ),
            munition_expendable_entity_id: EntityId::new(
                UVINT16::from(10),
                UVINT16::from(10),
                UVINT16::from(500),
            ),
            event_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(1)),
            fire_mission_index: Some(UVINT32::from(100)),
            location_world_coordinates: WorldCoordinates::new(
                620_384_200,
                59_652_240,
                SVINT24::from(1987),
            ),
            descriptor_entity_type: EntityType::new(
                2,
                2,
                0,
                UVINT8::from(0),
                UVINT8::from(0),
                UVINT8::from(0),
                UVINT8::from(0),
            ),
            descriptor_warhead_fuse: Some(DescriptorWarheadFuse {
                warhead: u16::from(MunitionDescriptorWarhead::Dummy),
                fuse: u16::from(MunitionDescriptorFuse::Dummy_8110),
            }),
            descriptor_quantity_rate: Some(DescriptorQuantityRate {
                quantity: 1,
                rate: 0,
            }),
            velocity: LinearVelocity::new(SVINT16::from(100), SVINT16::from(100), SVINT16::from(0)),
            range: Some(UVINT32::from(10000)),
        }
        .into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let PduBody::Fire(fire) = dis_body {
            assert_eq!(fire.firing_entity_id, DisEntityId::new(10, 10, 10));
            assert_eq!(fire.target_entity_id, DisEntityId::new(20, 20, 20));
            assert_eq!(fire.entity_id, DisEntityId::new(10, 10, 500));
            assert_eq!(fire.event_id, EventId::new(10, 10, 1));
            assert_eq!(fire.fire_mission_index, 100);
            if let FireDescriptor::Munition(munition) = fire.descriptor {
                assert_eq!(munition.entity_type.kind, EntityKind::Munition);
                assert_eq!(munition.entity_type.domain, PlatformDomain::Air);
                assert_eq!(munition.entity_type.category, 0);
                assert_eq!(munition.warhead, MunitionDescriptorWarhead::Dummy);
                assert_eq!(munition.fuse, MunitionDescriptorFuse::Dummy_8110);
                assert_eq!(munition.quantity, 1);
                assert_eq!(munition.rate, 0);
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }

    #[test]
    fn fire_body_decode_expendable() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_full_update();

        let cdis_body = Fire {
            units: UnitsDekameters::Dekameter,
            firing_entity_id: EntityId::new(
                UVINT16::from(10),
                UVINT16::from(10),
                UVINT16::from(10),
            ),
            target_entity_id: EntityId::new(
                UVINT16::from(20),
                UVINT16::from(20),
                UVINT16::from(20),
            ),
            munition_expendable_entity_id: EntityId::new(
                UVINT16::from(10),
                UVINT16::from(10),
                UVINT16::from(500),
            ),
            event_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(1)),
            fire_mission_index: Some(UVINT32::from(100)),
            location_world_coordinates: WorldCoordinates::new(
                620_384_200,
                59_652_240,
                SVINT24::from(1987),
            ),
            descriptor_entity_type: EntityType::new(
                8,
                2,
                0,
                UVINT8::from(0),
                UVINT8::from(0),
                UVINT8::from(0),
                UVINT8::from(0),
            ),
            descriptor_warhead_fuse: Some(DescriptorWarheadFuse {
                warhead: u16::from(MunitionDescriptorWarhead::Dummy),
                fuse: u16::from(MunitionDescriptorFuse::Dummy_8110),
            }),

            descriptor_quantity_rate: Some(DescriptorQuantityRate {
                quantity: 1,
                rate: 1,
            }),
            velocity: LinearVelocity::new(SVINT16::from(100), SVINT16::from(100), SVINT16::from(0)),
            range: Some(UVINT32::from(10000)),
        }
        .into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let PduBody::Fire(fire) = dis_body {
            assert_eq!(fire.firing_entity_id, DisEntityId::new(10, 10, 10));
            assert_eq!(fire.target_entity_id, DisEntityId::new(20, 20, 20));
            assert_eq!(fire.entity_id, DisEntityId::new(10, 10, 500));
            assert_eq!(fire.event_id, EventId::new(10, 10, 1));
            assert_eq!(fire.fire_mission_index, 100);
            if let FireDescriptor::Expendable(expendable) = fire.descriptor {
                assert_eq!(expendable.entity_type.kind, EntityKind::Expendable);
                assert_eq!(expendable.entity_type.domain, PlatformDomain::Air);
                assert_eq!(expendable.entity_type.category, 0);
            } else {
                panic!()
            }
        }
    }
}
