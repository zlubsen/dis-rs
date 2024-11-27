use crate::common::collision_elastic::model::CollisionElastic;
use crate::common::model::{EntityId, EventId, VectorF32};

pub struct CollisionElasticBuilder(CollisionElastic);

impl CollisionElasticBuilder {
    pub fn new() -> Self {
        CollisionElasticBuilder(CollisionElastic::default())
    }

    pub fn new_from_body(body: CollisionElastic) -> Self {
        CollisionElasticBuilder(body)
    }

    pub fn build(self) -> CollisionElastic {
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

    pub fn with_intermediate_result_xx(mut self, intermediate_result_xx: f32) -> Self {
        self.0.intermediate_result_xx = intermediate_result_xx;
        self
    }

    pub fn with_intermediate_result_xy(mut self, intermediate_result_xy: f32) -> Self {
        self.0.intermediate_result_xy = intermediate_result_xy;
        self
    }

    pub fn with_intermediate_result_xz(mut self, intermediate_result_xz: f32) -> Self {
        self.0.intermediate_result_xz = intermediate_result_xz;
        self
    }

    pub fn with_intermediate_result_yy(mut self, intermediate_result_yy: f32) -> Self {
        self.0.intermediate_result_yy = intermediate_result_yy;
        self
    }

    pub fn with_intermediate_result_yz(mut self, intermediate_result_yz: f32) -> Self {
        self.0.intermediate_result_yz = intermediate_result_yz;
        self
    }

    pub fn with_intermediate_result_zz(mut self, intermediate_result_zz: f32) -> Self {
        self.0.intermediate_result_zz = intermediate_result_zz;
        self
    }

    pub fn with_unit_surface_normal(mut self, unit_surface_normal: VectorF32) -> Self {
        self.0.unit_surface_normal = unit_surface_normal;
        self
    }

    pub fn with_coefficient_of_restitution(mut self, coefficient_of_restitution: f32) -> Self {
        self.0.coefficient_of_restitution = coefficient_of_restitution;
        self
    }
}
