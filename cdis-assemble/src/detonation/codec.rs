use crate::codec::Codec;
use crate::detonation::model::{Detonation, DetonationUnits};
use crate::records::codec::{
    decode_entity_coordinate_vector, decode_munition_descriptor, decode_world_coordinates,
    encode_entity_coordinate_vector, encode_munition_descriptor, encode_world_coordinates,
};
use crate::records::model::{
    CdisVariableParameter, DescriptorQuantityRate, DescriptorWarheadFuse, EntityId, EntityType,
    LinearVelocity,
};
use crate::types::model::UVINT8;
use dis_rs::BodyRaw;
use dis_rs::detonation::model::DetonationDescriptor;
use dis_rs::enumerations::{DetonationResult, EntityKind, ExplosiveMaterialCategories};
use dis_rs::model::{EventId, ExpendableDescriptor, ExplosionDescriptor};

type Counterpart = dis_rs::detonation::model::Detonation;

impl Detonation {
    #[must_use]
    pub fn encode(item: &Counterpart) -> Self {
        let (location_in_world_coordinates, units_world) =
            encode_world_coordinates(&item.location_in_world_coordinates);

        let (location_in_entity_coordinates, units_entity) =
            encode_entity_coordinate_vector(&item.location_in_entity_coordinates);

        let units = DetonationUnits {
            world_location_altitude: units_world,
            location_entity_coordinates: units_entity,
        };

        let (descriptor_entity_type, descriptor_warhead_fuse, descriptor_quantity_rate) =
            encode_detonation_descriptor(&item.descriptor);

        let variable_parameters = item
            .variable_parameters
            .iter()
            .map(CdisVariableParameter::encode)
            .collect();

        Detonation {
            units,
            source_entity_id: EntityId::encode(&item.source_entity_id),
            target_entity_id: EntityId::encode(&item.target_entity_id),
            exploding_entity_id: EntityId::encode(&item.exploding_entity_id),
            event_id: EntityId::from(&item.event_id),
            entity_linear_velocity: LinearVelocity::encode(&item.velocity),
            location_in_world_coordinates,
            descriptor_entity_type,
            descriptor_warhead_fuse,
            descriptor_quantity_rate,
            location_in_entity_coordinates,
            detonation_results: UVINT8::from(u8::from(item.detonation_result)),
            variable_parameters,
        }
    }

    #[must_use]
    pub fn decode(&self) -> Counterpart {
        let location_in_world_coordinates = decode_world_coordinates(
            &self.location_in_world_coordinates,
            self.units.world_location_altitude,
        );

        let descriptor = decode_detonation_descriptor(
            &self.descriptor_entity_type,
            self.descriptor_warhead_fuse.as_ref(),
            self.descriptor_quantity_rate.as_ref(),
        );

        let location_in_entity_coordinates = decode_entity_coordinate_vector(
            &self.location_in_entity_coordinates,
            self.units.location_entity_coordinates,
        );

        let variable_parameters = self
            .variable_parameters
            .iter()
            .map(crate::codec::Codec::decode)
            .collect();

        Counterpart::builder()
            .with_source_entity_id(self.source_entity_id.decode())
            .with_target_entity_id(self.target_entity_id.decode())
            .with_exploding_entity_id(self.exploding_entity_id.decode())
            .with_event_id(EventId::from(&self.event_id))
            .with_velocity(self.entity_linear_velocity.decode())
            .with_world_location(location_in_world_coordinates)
            .with_descriptor(descriptor)
            .with_entity_location(location_in_entity_coordinates)
            .with_detonation_result(DetonationResult::from(self.detonation_results.value))
            .with_variable_parameters(variable_parameters)
            .build()
    }
}

fn encode_detonation_descriptor(
    item: &DetonationDescriptor,
) -> (
    EntityType,
    Option<DescriptorWarheadFuse>,
    Option<DescriptorQuantityRate>,
) {
    match item {
        DetonationDescriptor::Munition(munition) => encode_munition_descriptor(munition),
        DetonationDescriptor::Expendable(expendable) => {
            (EntityType::encode(&expendable.entity_type), None, None)
        }
        // TODO SISO-STD-023-2023: [13.4] & [11.21] do not describe an encoding for the Explosion
        //      Descriptor record. Only the Munition Descriptor record is covered by the standard
        //      (the Expendable Descriptor record is also implicitly covered as a subset of the
        //      Munition Descriptor). The document is ambiguous, but seems to suggest that the
        //      Explosion Descriptor should be encoded as a Munition Descriptor with
        //      Warhead/Fuze/Quantity/Rate all zero.
        DetonationDescriptor::Explosion(explosion) => {
            (EntityType::encode(&explosion.entity_type), None, None)
        }
    }
}

fn decode_detonation_descriptor(
    descriptor_entity_type: &EntityType,
    descriptor_warhead_fuse: Option<&DescriptorWarheadFuse>,
    descriptor_quantity_rate: Option<&DescriptorQuantityRate>,
) -> DetonationDescriptor {
    let entity_type = descriptor_entity_type.decode();

    match entity_type.kind {
        EntityKind::Munition => DetonationDescriptor::Munition(decode_munition_descriptor(
            descriptor_entity_type,
            descriptor_warhead_fuse,
            descriptor_quantity_rate,
        )),
        EntityKind::Expendable => {
            DetonationDescriptor::Expendable(ExpendableDescriptor { entity_type })
        }
        // EntityType describes the non-munition/-expendable entity that has exploded
        _ => DetonationDescriptor::Explosion(ExplosionDescriptor {
            entity_type,
            explosive_material: ExplosiveMaterialCategories::default(),
            explosive_force: f32::default(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::{CodecOptions, CodecStateResult, DecoderState, EncoderState};
    use crate::detonation::model::{Detonation, DetonationUnits};
    use crate::records::model::{
        DescriptorQuantityRate, DescriptorWarheadFuse, EntityCoordinateVector, EntityId,
        EntityType, LinearVelocity, UnitsDekameters, UnitsMeters, WorldCoordinates,
    };
    use crate::types::model::{SVINT16, SVINT24, UVINT8, UVINT16};
    use crate::{BodyProperties, CdisBody};
    use dis_rs::BodyRaw;
    use dis_rs::detonation::builder::DetonationBuilder;
    use dis_rs::detonation::model::{Detonation as DisDetonation, DetonationDescriptor};
    use dis_rs::enumerations::{
        DetonationResult, EntityKind, ExplosiveMaterialCategories, MunitionDescriptorFuse,
        MunitionDescriptorWarhead, PlatformDomain,
    };
    use dis_rs::model::{
        EntityId as DisEntityId, EntityType as DisEntityType, EventId, ExplosionDescriptor,
        Location, PduBody, VectorF32,
    };

    fn create_basic_dis_detonation_body() -> DetonationBuilder {
        DisDetonation::builder()
            .with_source_entity_id(DisEntityId::new(10, 10, 10))
            .with_target_entity_id(DisEntityId::new(20, 20, 20))
            .with_exploding_entity_id(DisEntityId::new(10, 10, 500))
            .with_event_id(EventId::new(10, 10, 1))
            .with_descriptor(DetonationDescriptor::Explosion(ExplosionDescriptor {
                entity_type: DisEntityType::default()
                    .with_kind(EntityKind::Munition)
                    .with_domain(PlatformDomain::Land),
                explosive_material: ExplosiveMaterialCategories::Alcohol,
                explosive_force: 20.0,
            }))
            .with_velocity(VectorF32::new(10.0, 10.0, 10.0))
            .with_world_location(Location::new(20000.0, 20000.0, 20000.0))
    }

    #[test]
    fn detonation_body_encode_units_centimeters() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update();

        let dis_body = create_basic_dis_detonation_body()
            .with_entity_location(VectorF32::new(50.0, 0.0, 0.0))
            .with_detonation_result(DetonationResult::Detonation)
            .build()
            .into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::Detonation(detonation) = cdis_body {
            assert_eq!(
                detonation.units.world_location_altitude,
                UnitsDekameters::Dekameter
            );
            assert_eq!(
                detonation.units.location_entity_coordinates,
                UnitsMeters::Centimeter
            );
            assert_eq!(detonation.detonation_results.value, 5);
        } else {
            panic!()
        }
    }

    #[test]
    fn detonation_body_encode_units_meters() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update();

        let dis_body = create_basic_dis_detonation_body()
            .with_entity_location(VectorF32::new(33000.0, 0.0, 0.0))
            .build()
            .into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::Detonation(detonation) = cdis_body {
            assert_eq!(
                detonation.units.world_location_altitude,
                UnitsDekameters::Dekameter
            );
            assert_eq!(
                detonation.units.location_entity_coordinates,
                UnitsMeters::Meter
            );
        } else {
            panic!()
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn detonation_body_decode_units_centimeters() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_full_update();

        let cdis_body = Detonation {
            units: DetonationUnits {
                world_location_altitude: UnitsDekameters::Dekameter,
                location_entity_coordinates: UnitsMeters::Centimeter,
            },
            source_entity_id: EntityId::new(
                UVINT16::from(10),
                UVINT16::from(10),
                UVINT16::from(10),
            ),
            target_entity_id: EntityId::new(
                UVINT16::from(20),
                UVINT16::from(20),
                UVINT16::from(20),
            ),
            exploding_entity_id: EntityId::new(
                UVINT16::from(10),
                UVINT16::from(10),
                UVINT16::from(500),
            ),
            event_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(1)),
            entity_linear_velocity: LinearVelocity::new(
                SVINT16::from(1),
                SVINT16::from(1),
                SVINT16::from(1),
            ),
            location_in_world_coordinates: WorldCoordinates::new(
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
            location_in_entity_coordinates: EntityCoordinateVector::new(
                SVINT16::from(10),
                SVINT16::from(10),
                SVINT16::from(10),
            ),
            detonation_results: UVINT8::from(5),
            variable_parameters: vec![],
        }
        .into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let PduBody::Detonation(detonation) = dis_body {
            assert_eq!(detonation.source_entity_id, DisEntityId::new(10, 10, 10));
            assert_eq!(detonation.target_entity_id, DisEntityId::new(20, 20, 20));
            assert_eq!(
                detonation.exploding_entity_id,
                DisEntityId::new(10, 10, 500)
            );
            assert_eq!(detonation.event_id, EventId::new(10, 10, 1));
            if let DetonationDescriptor::Munition(munition) = detonation.descriptor {
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
            assert_eq!(
                detonation
                    .location_in_entity_coordinates
                    .first_vector_component,
                0.10
            );
            assert_eq!(
                detonation
                    .location_in_entity_coordinates
                    .second_vector_component,
                0.10
            );
            assert_eq!(
                detonation
                    .location_in_entity_coordinates
                    .third_vector_component,
                0.10
            );
        } else {
            panic!()
        }
    }

    #[test]
    fn detonation_body_decode_units_meters() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_full_update();

        let cdis_body = Detonation {
            units: DetonationUnits {
                world_location_altitude: UnitsDekameters::Dekameter,
                location_entity_coordinates: UnitsMeters::Meter,
            },
            source_entity_id: EntityId::new(
                UVINT16::from(10),
                UVINT16::from(10),
                UVINT16::from(10),
            ),
            target_entity_id: EntityId::new(
                UVINT16::from(20),
                UVINT16::from(20),
                UVINT16::from(20),
            ),
            exploding_entity_id: EntityId::new(
                UVINT16::from(10),
                UVINT16::from(10),
                UVINT16::from(500),
            ),
            event_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(1)),
            entity_linear_velocity: LinearVelocity::new(
                SVINT16::from(1),
                SVINT16::from(1),
                SVINT16::from(1),
            ),
            location_in_world_coordinates: WorldCoordinates::new(
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
            location_in_entity_coordinates: EntityCoordinateVector::new(
                SVINT16::from(10),
                SVINT16::from(10),
                SVINT16::from(10),
            ),
            detonation_results: UVINT8::from(5),
            variable_parameters: vec![],
        }
        .into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let PduBody::Detonation(detonation) = dis_body {
            assert_eq!(
                detonation
                    .location_in_entity_coordinates
                    .first_vector_component,
                10.0
            );
            assert_eq!(
                detonation
                    .location_in_entity_coordinates
                    .second_vector_component,
                10.0
            );
            assert_eq!(
                detonation
                    .location_in_entity_coordinates
                    .third_vector_component,
                10.0
            );
        } else {
            panic!()
        }
    }
}
