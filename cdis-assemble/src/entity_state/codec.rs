use crate::codec::Codec;
use crate::entity_state::model::{CdisDRParametersOther, CdisEntityCapabilities, EntityState};
use crate::records::codec::encode_world_coordinates;
use crate::records::model::{AngularVelocity, CdisEntityMarking, CdisVariableParameter, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation};
use crate::types::model::{UVINT32, UVINT8};

impl Codec for EntityState {
    type Counterpart = dis_rs::entity_state::model::EntityState;
    const SCALING: f32 = 0.0;

    fn encode(item: &Self::Counterpart) -> Self {
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
                .map(|vp| CdisVariableParameter::encode(vp) )
                .collect(),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        todo!()
    }
}
