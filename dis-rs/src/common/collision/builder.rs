use crate::common::collision::model::Collision;
use crate::common::model::{EntityId, EventId, VectorF32};
use crate::enumerations::CollisionType;

pub struct CollisionBuilder(Collision);

impl CollisionBuilder {
    pub fn new() -> Self {
        CollisionBuilder(Collision::default())
    }

    pub fn new_from_body(body: Collision) -> Self {
        CollisionBuilder(body)
    }

    pub fn build(self) -> Collision {
        self.0
    }

    pub fn with_issuing_entity_id(mut self, issuing_entity_id: EntityId) -> Self {
        self.0.issuing_entity_id = issuing_entity_id;
        self
    }

    pub fn with_colliding_entity_id(mut self, colliding_entity_id: EntityId) -> Self {
        self.0.colliding_entity_id = colliding_entity_id;
        self
    }

    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.0.event_id = event_id;
        self
    }

    pub fn with_collision_type(mut self, collision_type: CollisionType) -> Self {
        self.0.collision_type = collision_type;
        self
    }

    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.0.velocity = velocity;
        self
    }

    pub fn with_mass(mut self, mass: f32) -> Self {
        self.0.mass = mass;
        self
    }

    pub fn with_location(mut self, location: VectorF32) -> Self {
        self.0.location = location;
        self
    }
}