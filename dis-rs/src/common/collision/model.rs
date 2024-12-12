use crate::common::collision::builder::CollisionBuilder;
use crate::common::model::{EntityId, EventId, PduBody, VectorF32};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{CollisionType, PduType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const COLLISION_BODY_LENGTH: u16 = 60;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[must_use]
    pub fn builder() -> CollisionBuilder {
        CollisionBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> CollisionBuilder {
        CollisionBuilder::new_from_body(self)
    }

    #[must_use]
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
