use crate::codec::Codec;
use crate::entity_state::model::{CdisDRParametersOther, CdisEntityCapabilities, EntityState};
use crate::records::model::{AngularVelocity, CdisEntityMarking, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, Units, WorldCoordinates};
use crate::types::model::{UVINT32, UVINT8};

impl Codec for EntityState {
    type Counterpart = dis_rs::entity_state::model::EntityState;
    const SCALING: f32 = 0.0;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            // TODO - when/how to decide what the Unit should be
            units: Units::Centimeter,
            full_update_flag: true,
            entity_id: EntityId::encode(&item.entity_id),
            force_id: Some(UVINT8::from(u8::from(item.force_id))),
            entity_type: Some(EntityType::encode(&item.entity_type)),
            alternate_entity_type: Some(EntityType::encode(&item.alternative_entity_type)),
            entity_linear_velocity: Some(LinearVelocity::encode(&item.entity_linear_velocity)),
            // TODO test / validate if correct
            entity_location: Some(WorldCoordinates::from(item.entity_location.clone())),
            entity_orientation: Some(Orientation::encode(&item.entity_orientation)),
            entity_appearance: Some((&item.entity_appearance).into()),
            dr_algorithm: item.dead_reckoning_parameters.algorithm,
            dr_params_other: Some(CdisDRParametersOther::from(&item.dead_reckoning_parameters.other_parameters)),
            dr_params_entity_linear_acceleration: Some(LinearAcceleration::encode(&item.dead_reckoning_parameters.linear_acceleration)),
            dr_params_entity_angular_velocity: Some(AngularVelocity::encode(&item.dead_reckoning_parameters.angular_velocity)),
            entity_marking: Some(CdisEntityMarking::new(item.entity_marking.marking_string.clone())),
            capabilities: Some(CdisEntityCapabilities(UVINT32::from(u32::from(item.entity_capabilities)))),
            variable_parameters: vec![],
        }
    }

    fn decode(&self) -> Self::Counterpart {
        todo!()
    }
}
