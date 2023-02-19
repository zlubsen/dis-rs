use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, EventId};
use crate::enumerations::CollisionType;
use crate::{PduBody, PduType, VectorF32};

const COLLISION_BODY_LENGTH : u16 = 60;

pub struct Collision {
    pub issuing_entity_id: EntityId,
    pub colliding_entity_id: EntityId,
    pub event_id: EventId,
    pub collision_type: CollisionType,
    pub velocity: VectorF32,
    pub mass: f32,
    pub location: VectorF32,
}

impl Default for Collision {
     fn default() -> Self {
         Self::new()
     }
}

impl Collision {
    pub fn new() -> Self {
        Self {
            issuing_entity_id: Default::default(),
            colliding_entity_id: Default::default(),
            event_id: Default::default(),
            collision_type: Default::default(),
            velocity: Default::default(),
            mass: 0.0,
            location: Default::default()
        }
    }

    pub fn with_issuing_entity_id(mut self, issuing_entity_id: EntityId) -> Self {
        self.issuing_entity_id = issuing_entity_id;
        self
    }

    pub fn with_colliding_entity_id(mut self, colliding_entity_id: EntityId) -> Self {
        self.colliding_entity_id = colliding_entity_id;
        self
    }

    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.event_id = event_id;
        self
    }

    pub fn with_collision_type(mut self, collision_type: CollisionType) -> Self {
        self.collision_type = collision_type;
        self
    }

    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    pub fn with_location(mut self, location: VectorF32) -> Self {
        self.location = location;
        self
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