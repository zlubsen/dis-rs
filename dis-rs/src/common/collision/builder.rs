use crate::common::collision::model::Collision;
use crate::common::model::{EntityId, EventId, VectorF32};
use crate::enumerations::CollisionType;

pub struct CollisionBuilder(Collision);

impl CollisionBuilder {
    #[must_use]
    pub fn new() -> Self {
        CollisionBuilder(Collision::default())
    }

    #[must_use]
    pub fn new_from_body(body: Collision) -> Self {
        CollisionBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> Collision {
        self.0
    }

    #[must_use]
    pub fn with_issuing_entity_id(mut self, issuing_entity_id: EntityId) -> Self {
        self.0.issuing_entity_id = issuing_entity_id;
        self
    }

    #[must_use]
    pub fn with_colliding_entity_id(mut self, colliding_entity_id: EntityId) -> Self {
        self.0.colliding_entity_id = colliding_entity_id;
        self
    }

    #[must_use]
    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.0.event_id = event_id;
        self
    }

    #[must_use]
    pub fn with_collision_type(mut self, collision_type: CollisionType) -> Self {
        self.0.collision_type = collision_type;
        self
    }

    #[must_use]
    pub fn with_velocity(mut self, velocity: VectorF32) -> Self {
        self.0.velocity = velocity;
        self
    }

    #[must_use]
    pub fn with_mass(mut self, mass: f32) -> Self {
        self.0.mass = mass;
        self
    }

    #[must_use]
    pub fn with_location(mut self, location: VectorF32) -> Self {
        self.0.location = location;
        self
    }
}
