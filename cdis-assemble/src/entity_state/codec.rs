use dis_rs::entity_state::model::{DrParameters, EntityAppearance, EntityMarking};
use dis_rs::enumerations::{EntityMarkingCharacterSet, ForceId};
use crate::codec::Codec;
use crate::entity_state::model::{CdisDRParametersOther, CdisEntityCapabilities, EntityState};
use crate::records::codec::{decode_world_coordinates, encode_world_coordinates};
use crate::records::model::{AngularVelocity, CdisEntityMarking, CdisVariableParameter, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation};
use crate::types::model::{UVINT32, UVINT8};

impl Codec for EntityState {
    type Counterpart = dis_rs::entity_state::model::EntityState;
    const SCALING: f32 = 0.0;

    fn encode(item: &Self::Counterpart) -> Self {
        // Covers full update mode
        let (entity_location, units) = encode_world_coordinates(&item.entity_location);
        let entity_location = Some(entity_location);
        Self {
            units,
            full_update_flag: true,
            entity_id: EntityId::encode(&item.entity_id),
            force_id: Some(UVINT8::from(u8::from(item.force_id))),
            entity_type: Some(EntityType::encode(&item.entity_type)),
            alternate_entity_type: Some(EntityType::encode(&item.alternative_entity_type)),
            entity_linear_velocity: Some(LinearVelocity::encode(&item.entity_linear_velocity)),
            entity_location,
            entity_orientation: Some(Orientation::encode(&item.entity_orientation)),
            entity_appearance: Some((&item.entity_appearance).into()),
            dr_algorithm: item.dead_reckoning_parameters.algorithm,
            dr_params_other: Some(CdisDRParametersOther::from(&item.dead_reckoning_parameters.other_parameters)),
            dr_params_entity_linear_acceleration: Some(LinearAcceleration::encode(&item.dead_reckoning_parameters.linear_acceleration)),
            dr_params_entity_angular_velocity: Some(AngularVelocity::encode(&item.dead_reckoning_parameters.angular_velocity)),
            entity_marking: Some(CdisEntityMarking::new(item.entity_marking.marking_string.clone())),
            capabilities: Some(CdisEntityCapabilities(UVINT32::from(u32::from(item.entity_capabilities)))),
            variable_parameters: item.variable_parameters.iter()
                .map(CdisVariableParameter::encode )
                .collect(),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        // pre-compute, decoded value is needed two times
        let entity_type = self.entity_type.unwrap_or_default().decode();

        // Covers full update mode
        Self::Counterpart::builder()
            .with_entity_id(self.entity_id.decode())
            .with_force_id(ForceId::from(self.force_id.unwrap_or_default().value))
            .with_entity_type(entity_type)
            .with_alternative_entity_type(self.alternate_entity_type.unwrap_or_default().decode())
            .with_velocity(self.entity_linear_velocity.unwrap_or_default().decode())
            .with_location(self.entity_location
                .map(| world_coordinates | decode_world_coordinates(&world_coordinates, self.units) )
                .unwrap_or_default())
            .with_orientation(self.entity_orientation.unwrap_or_default().decode())
            .with_appearance(self.entity_appearance.as_ref()
                .map(|cdis| EntityAppearance::from_bytes(cdis.0, &entity_type))
                .unwrap_or_default())
            .with_dead_reckoning_parameters(DrParameters::default()
                .with_algorithm(self.dr_algorithm)
                .with_parameters(self.dr_params_other.clone().unwrap_or_default().decode(self.dr_algorithm))
                .with_linear_acceleration(self.dr_params_entity_linear_acceleration.unwrap_or_default().decode())
                .with_angular_velocity(self.dr_params_entity_angular_velocity.unwrap_or_default().decode()))
            .with_marking(EntityMarking::new(self.entity_marking.clone().unwrap_or_default().marking, EntityMarkingCharacterSet::ASCII))
            .with_capabilities(dis_rs::entity_capabilities_from_bytes(self.capabilities.clone().unwrap_or_default().0.value, &entity_type))
            .with_variable_parameters(self.variable_parameters.iter()
                .map(|vp| vp.decode() )
                .collect())
            .build()
    }
}