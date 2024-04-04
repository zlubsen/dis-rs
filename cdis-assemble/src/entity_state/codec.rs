use dis_rs::model::VectorF32;
use crate::codec::Codec;
use crate::constants::{METERS_TO_DECIMETERS, RADIANS_SEC_TO_DEGREES_SEC};
use crate::entity_state::model::{CdisDRParametersOther, CdisEntityCapabilities, EntityState};
use crate::records::model::{AngularVelocity, CdisEntityMarking, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, Units, WorldCoordinates};
use crate::types::model::{SVINT12, SVINT14, UVINT32, UVINT8};

impl Codec for EntityState {
    type Counterpart = dis_rs::entity_state::model::EntityState;

    fn encode(item: Self::Counterpart) -> Self {
        Self {
            units: Units::Centimeter,
            full_update_flag: true,
            entity_id: EntityId::encode(item.entity_id),
            force_id: Some(UVINT8::from(u8::from(item.force_id))),
            entity_type: Some(EntityType::encode(item.entity_type)),
            alternate_entity_type: Some(EntityType::encode(item.alternative_entity_type)),
            entity_linear_velocity: Some(LinearVelocity::encode(item.entity_linear_velocity)),
            // TODO test / validate if correct
            entity_location: Some(WorldCoordinates::from(item.entity_location)),
            // TODO check if this is the correct conversion - and move to the record (codec trait)
            entity_orientation: Some(Orientation::new(item.entity_orientation.psi as u16, item.entity_orientation.theta as u16, item.entity_orientation.phi as u16)),
            entity_appearance: Some((&item.entity_appearance).into()),
            dr_algorithm: item.dead_reckoning_parameters.algorithm,
            dr_params_other: Some(CdisDRParametersOther::from(item.dead_reckoning_parameters.other_parameters)),
            dr_params_entity_linear_acceleration: Some(LinearAcceleration::from(item.dead_reckoning_parameters.linear_acceleration)),
            dr_params_entity_angular_velocity: Some(AngularVelocity::from(item.dead_reckoning_parameters.angular_velocity)),
            entity_marking: Some(CdisEntityMarking::new(item.entity_marking.marking_string.clone())),
            capabilities: Some(CdisEntityCapabilities(UVINT32::from(u32::from(item.entity_capabilities)))),
            variable_parameters: vec![],
        }
    }

    fn decode(&self) -> Self::Counterpart {
        todo!()
    }
}
