use crate::codec::Codec;
use crate::entity_state::model::EntityState;
use crate::records::model::{EntityId, EntityType, LinearVelocity, Orientation, Units, WorldCoordinates};
use crate::types::model::{UVINT8};

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
            entity_location: Some(WorldCoordinates::from(item.entity_location)),
            // TODO check if this is the correct conversion - and move to the record (codec trait)
            entity_orientation: Some(Orientation::new(item.entity_orientation.psi as u16, item.entity_orientation.theta as u16, item.entity_orientation.phi as u16)),
            entity_appearance: None,
            dr_algorithm: Default::default(),
            dr_params_other: None,
            dr_params_entity_linear_acceleration: None,
            dr_params_entity_angular_velocity: None,
            entity_marking: None,
            capabilities: None,
            variable_parameters: vec![],
        }
    }

    fn decode(&self) -> Self::Counterpart {
        todo!()
    }
}
