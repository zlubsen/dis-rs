use num_traits::Zero;
use dis_rs::enumerations::{DetonationResult, EntityKind, ExplosiveMaterialCategories, MunitionDescriptorFuse, MunitionDescriptorWarhead};
use dis_rs::model::{DescriptorRecord, EventId, MunitionDescriptor};
use crate::codec::Codec;
use crate::detonation::model::{Detonation, DetonationUnits, ExplosiveForceFloat};
use crate::records::codec::{decode_entity_coordinate_vector, decode_world_coordinates, encode_entity_coordinate_vector, encode_world_coordinates};
use crate::records::model::{CdisVariableParameter, EntityId, EntityType, LinearVelocity};
use crate::types::model::{CdisFloat, UVINT16, UVINT8};

type Counterpart = dis_rs::detonation::model::Detonation;

impl Detonation {
    pub fn encode(item: &Counterpart) -> Self {
        let (location_in_world_coordinates, units_world) =
            encode_world_coordinates(&item.location_in_world_coordinates);
        let (location_in_entity_coordinates, units_entity) =
            encode_entity_coordinate_vector(&item.location_in_entity_coordinates);
        let units = DetonationUnits {
            world_location_altitude: units_world,
            location_entity_coordinates: units_entity,
        };
        let (descriptor_entity_type,
            descriptor_warhead,
            descriptor_fuze,
            descriptor_quantity,
            descriptor_rate,
            descriptor_explosive_material,
            descriptor_explosive_force) = encode_detonation_descriptor(&item.descriptor);
        let detonation_result: u8 = item.detonation_result.into();
        let variable_parameters = item.variable_parameters.iter()
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
            descriptor_warhead,
            descriptor_fuze,
            descriptor_quantity,
            descriptor_rate,
            descriptor_explosive_material,
            descriptor_explosive_force,
            location_in_entity_coordinates,
            detonation_results: UVINT8::from(detonation_result),
            variable_parameters,
        }
    }

    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_source_entity_id(self.source_entity_id.decode())
            .with_target_entity_id(self.target_entity_id.decode())
            .with_exploding_entity_id(self.exploding_entity_id.decode())
            .with_event_id(EventId::from(&self.event_id))
            .with_velocity(self.entity_linear_velocity.decode())
            .with_world_location(decode_world_coordinates(&self.location_in_world_coordinates, self.units.world_location_altitude))
            .with_descriptor(decode_detonation_descriptor(self, self.descriptor_entity_type.decode()))
            .with_entity_location(decode_entity_coordinate_vector(&self.location_in_entity_coordinates, self.units.location_entity_coordinates))
            .with_detonation_result(DetonationResult::from(self.detonation_results.value))
            .with_variable_parameters(self.variable_parameters.iter()
                .map(|vp| vp.decode() )
                .collect())
            .build()
    }
}

fn encode_detonation_descriptor(item : &DescriptorRecord) -> (EntityType, Option<u16>, Option<u16>, Option<u8>, Option<u8>, Option<UVINT16>, Option<ExplosiveForceFloat>) {
    match item {
        DescriptorRecord::Munition { entity_type, munition } => {
            let warhead = Some(munition.warhead.into());
            let fuze = Some(munition.fuse.into());
            let quantity = if munition.quantity.is_zero() { None } else { Some(munition.quantity.min(u8::MAX as u16) as u8) };
            let rate = if munition.rate.is_zero() { None } else { Some(munition.rate.min(u8::MAX as u16) as u8) };
            (EntityType::encode(entity_type), warhead, fuze, quantity, rate, None, None)
        }
        DescriptorRecord::Expendable { entity_type } => {
            (EntityType::encode(entity_type), None, None, None, None, None, None)
        }
        DescriptorRecord::Explosion { entity_type, explosive_material, explosive_force} => {
            let explosive_material: u16 = (*explosive_material).into();
            let explosive_material = UVINT16::from(explosive_material);
            (EntityType::encode(entity_type), None, None, None, None, Some(explosive_material), Some(ExplosiveForceFloat::from_float(*explosive_force)))
        }
    }
}

fn decode_detonation_descriptor(detonation_body: &Detonation, entity_type: dis_rs::model::EntityType) -> DescriptorRecord {
    match entity_type.kind {
        EntityKind::Munition => {
            DescriptorRecord::new_munition(entity_type, MunitionDescriptor::default()
                .with_warhead(MunitionDescriptorWarhead::from(detonation_body.descriptor_warhead.unwrap_or_default()))
                .with_fuse(MunitionDescriptorFuse::from(detonation_body.descriptor_fuze.unwrap_or_default()))
                .with_quantity(detonation_body.descriptor_quantity.unwrap_or_default() as u16)
                .with_rate(detonation_body.descriptor_rate.unwrap_or_default() as u16))
        }
        EntityKind::Expendable => {
            DescriptorRecord::new_expendable(entity_type)
        }
        _ => {
            let explosive_material = detonation_body.descriptor_explosive_material.map(|record| record.value).unwrap_or_default();
            let explosive_force = detonation_body.descriptor_explosive_force.map(|float| float.to_float()).unwrap_or_default();
            DescriptorRecord::new_explosion(entity_type, ExplosiveMaterialCategories::from(explosive_material), explosive_force)
        }
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::detonation::builder::DetonationBuilder;
    use dis_rs::detonation::model::Detonation as DisDetonation;
    use dis_rs::enumerations::{DetonationResult, EntityKind, ExplosiveMaterialCategories, MunitionDescriptorFuse, MunitionDescriptorWarhead, PlatformDomain};
    use dis_rs::model::{DescriptorRecord, EntityId as DisEntityId, EntityType as DisEntityType, EventId, Location, PduBody, SimulationAddress, VectorF32};
    use crate::{BodyProperties, CdisBody};
    use crate::codec::{CodecOptions, CodecStateResult, DecoderState, EncoderState};
    use crate::detonation::model::{Detonation, DetonationUnits};
    use crate::records::model::{EntityCoordinateVector, EntityId, EntityType, LinearVelocity, UnitsDekameters, UnitsMeters, WorldCoordinates};
    use crate::types::model::{SVINT16, SVINT24, UVINT16, UVINT8};

    fn create_basic_dis_detonation_body() -> DetonationBuilder {
        DisDetonation::builder()
            .with_source_entity_id(DisEntityId::new(10, 10, 10))
            .with_target_entity_id(DisEntityId::new(20, 20, 20))
            .with_exploding_entity_id(DisEntityId::new(10, 10, 500))
            .with_event_id(EventId::new(SimulationAddress::new(10, 10), 1))
            .with_descriptor(DescriptorRecord::new_explosion(DisEntityType::default().with_kind(EntityKind::Munition).with_domain(PlatformDomain::Land), ExplosiveMaterialCategories::Alcohol, 20.0))
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
            .build().into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::Detonation(detonation) = cdis_body {
            assert_eq!(detonation.units.world_location_altitude, UnitsDekameters::Dekameter);
            assert_eq!(detonation.units.location_entity_coordinates, UnitsMeters::Centimeter);
            assert_eq!(detonation.detonation_results.value, 5);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn detonation_body_encode_units_meters() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update();

        let dis_body = create_basic_dis_detonation_body()
            .with_entity_location(VectorF32::new(33000.0, 0.0, 0.0))
            .build().into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::Detonation(detonation) = cdis_body {
            assert_eq!(detonation.units.world_location_altitude, UnitsDekameters::Dekameter);
            assert_eq!(detonation.units.location_entity_coordinates, UnitsMeters::Meter);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn detonation_body_decode_units_centimeters() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_full_update();

        let cdis_body = Detonation {
            units: DetonationUnits {
                world_location_altitude: UnitsDekameters::Dekameter,
                location_entity_coordinates: UnitsMeters::Centimeter,
            },
            source_entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
            target_entity_id: EntityId::new(UVINT16::from(20), UVINT16::from(20), UVINT16::from(20)),
            exploding_entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(500)),
            event_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(1)),
            entity_linear_velocity: LinearVelocity::new(SVINT16::from(1), SVINT16::from(1), SVINT16::from(1)),
            location_in_world_coordinates: WorldCoordinates::new(620384200f32, 59652240f32, SVINT24::from(1987)),
            descriptor_entity_type: EntityType::new(2, 2, 0,
                UVINT8::from(0), UVINT8::from(0), UVINT8::from(0), UVINT8::from(0)),
            descriptor_warhead: Some(u16::from(MunitionDescriptorWarhead::Dummy)),
            descriptor_fuze: Some(u16::from(MunitionDescriptorFuse::Dummy_8110)),
            descriptor_quantity: Some(1),
            descriptor_rate: Some(0),
            descriptor_explosive_material: None,
            descriptor_explosive_force: None,
            location_in_entity_coordinates: EntityCoordinateVector::new(SVINT16::from(10), SVINT16::from(10), SVINT16::from(10)),
            detonation_results: UVINT8::from(5),
            variable_parameters: vec![],
        }.into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let PduBody::Detonation(detonation) = dis_body {
            assert_eq!(detonation.source_entity_id, DisEntityId::new(10, 10, 10));
            assert_eq!(detonation.target_entity_id, DisEntityId::new(20, 20, 20));
            assert_eq!(detonation.exploding_entity_id, DisEntityId::new(10, 10, 500));
            assert_eq!(detonation.event_id, EventId::new(SimulationAddress::new(10, 10), 1));
            if let DescriptorRecord::Munition { entity_type, munition} = detonation.descriptor {
                assert_eq!(entity_type.kind, EntityKind::Munition);
                assert_eq!(entity_type.domain, PlatformDomain::Air);
                assert_eq!(entity_type.category, 0);
                assert_eq!(munition.warhead, MunitionDescriptorWarhead::Dummy);
                assert_eq!(munition.fuse, MunitionDescriptorFuse::Dummy_8110);
                assert_eq!(munition.quantity, 1);
                assert_eq!(munition.rate, 0);
            } else { assert!(false) };
            assert_eq!(detonation.location_in_entity_coordinates.first_vector_component, 0.10);
            assert_eq!(detonation.location_in_entity_coordinates.second_vector_component, 0.10);
            assert_eq!(detonation.location_in_entity_coordinates.third_vector_component, 0.10);
        } else { assert!(false) };
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
            source_entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
            target_entity_id: EntityId::new(UVINT16::from(20), UVINT16::from(20), UVINT16::from(20)),
            exploding_entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(500)),
            event_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(1)),
            entity_linear_velocity: LinearVelocity::new(SVINT16::from(1), SVINT16::from(1), SVINT16::from(1)),
            location_in_world_coordinates: WorldCoordinates::new(620384200f32, 59652240f32, SVINT24::from(1987)),
            descriptor_entity_type: EntityType::new(2, 2, 0,
                                                    UVINT8::from(0), UVINT8::from(0), UVINT8::from(0), UVINT8::from(0)),
            descriptor_warhead: Some(u16::from(MunitionDescriptorWarhead::Dummy)),
            descriptor_fuze: Some(u16::from(MunitionDescriptorFuse::Dummy_8110)),
            descriptor_quantity: Some(1),
            descriptor_rate: Some(0),
            descriptor_explosive_material: None,
            descriptor_explosive_force: None,
            location_in_entity_coordinates: EntityCoordinateVector::new(SVINT16::from(10), SVINT16::from(10), SVINT16::from(10)),
            detonation_results: UVINT8::from(5),
            variable_parameters: vec![],
        }.into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let PduBody::Detonation(detonation) = dis_body {
            assert_eq!(detonation.location_in_entity_coordinates.first_vector_component, 10.0);
            assert_eq!(detonation.location_in_entity_coordinates.second_vector_component, 10.0);
            assert_eq!(detonation.location_in_entity_coordinates.third_vector_component, 10.0);
        } else { assert!(false) };
    }
}