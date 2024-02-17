use crate::common::{BodyInfo, Interaction};
use crate::common::collision::builder::CollisionBuilder;
use crate::common::model::{EntityId, EventId, PduBody, VectorF32};
use crate::enumerations::{CollisionType, PduType};

const COLLISION_BODY_LENGTH : u16 = 60;

/// 5.3.3 Collision PDU
///
/// 7.2.3 Collision PDU
#[derive(Debug, Default, PartialEq)]
pub struct Collision {
    pub issuing_entity_id: EntityId,
    pub colliding_entity_id: EntityId,
    pub event_id: EventId,
    pub collision_type: CollisionType,
    pub velocity: VectorF32,
    pub mass: f32,
    pub location: VectorF32,
}

impl Collision {
    pub fn builder() -> CollisionBuilder {
        CollisionBuilder::new()
    }

    pub fn into_builder(self) -> CollisionBuilder {
        CollisionBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Collision(self)
    }
}

impl BodyInfo for Collision {
    fn body_length(&self) -> u16 {
        COLLISION_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::Collision
    }
}

impl Interaction for Collision {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.issuing_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.colliding_entity_id)
    }
}