use num_traits::Zero;
use dis_rs::enumerations::{DetonationResult, EntityKind, MunitionDescriptorFuse, MunitionDescriptorWarhead};
use dis_rs::model::{DescriptorRecord, EventId, MunitionDescriptor};
use crate::codec::Codec;
use crate::detonation::model::{Detonation, DetonationUnits};
use crate::records::codec::{decode_entity_coordinate_vector, decode_world_coordinates, encode_entity_coordinate_vector, encode_world_coordinates};
use crate::records::model::{CdisVariableParameter, EntityId, EntityType, LinearVelocity};
use crate::types::model::UVINT8;

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
            descriptor_rate) = encode_detonation_descriptor(&item.descriptor);
        let detonation_result: u8 = item.detonation_result.into();
        let variable_parameters = item.variable_parameters.iter()
            .map(|vp| CdisVariableParameter::encode(vp) )
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

fn encode_detonation_descriptor(item : &DescriptorRecord) -> (EntityType, Option<u16>, Option<u16>, Option<u8>, Option<u8>) {
    match item {
        DescriptorRecord::Munition { entity_type, munition } => {
            let warhead = Some(munition.warhead.into());
            let fuze = Some(munition.fuse.into());
            let quantity = if munition.quantity.is_zero() { None } else { Some(munition.quantity.min(u8::MAX as u16) as u8) };
            let rate = if munition.rate.is_zero() { None } else { Some(munition.rate.min(u8::MAX as u16) as u8) };
            (EntityType::encode(entity_type), warhead, fuze, quantity, rate)
        }
        DescriptorRecord::Expendable { entity_type } => {
            (EntityType::encode(entity_type), None, None, None, None)
        }
        DescriptorRecord::Explosion { entity_type, explosive_material: _, explosive_force: _} => {
            (EntityType::encode(entity_type), None, None, None, None)
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
            DescriptorRecord::new_munition(entity_type, MunitionDescriptor::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::detonation::builder::DetonationBuilder;
    use dis_rs::detonation::model::Detonation as DisDetonation;
    use dis_rs::model::{EntityId as DisEntityId, EntityType as DisEntityType, EventId, Location, MunitionDescriptor, PduBody, SimulationAddress};

    fn create_basic_dis_detonation_body() -> DetonationBuilder {
        DisDetonation::builder()
            .with_firing_entity_id(DisEntityId::new(10, 10, 10))
            .with_target_entity_id(DisEntityId::new(20, 20, 20))
            .with_entity_id(DisEntityId::new(10, 10, 500))
            .with_event_id(EventId::new(SimulationAddress::new(10, 10), 1))
            .with_location_in_world(Location::new(20000.0, 20000.0, 20000.0))
    }

    #[test]
    fn detonation_body_encode_units_centimeters() {
        assert!(false);
    }

    #[test]
    fn detonation_body_encode_units_meters() {
        assert!(false);
    }

    #[test]
    fn detonation_body_decode_units_centimeters() {
        assert!(false);
    }

    #[test]
    fn detonation_body_decode_units_meters() {
        assert!(false);
    }
}