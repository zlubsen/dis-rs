use crate::{EntityId, EventId, PduBody, PduType, VectorF32};
use crate::common::{BodyInfo, Interaction};

const COLLISION_ELASTIC_BODY_LENGTH : u16 = 88;

pub struct CollisionElastic {
    pub issuing_entity_id: EntityId,
    pub colliding_entity_id: EntityId,
    pub event_id: EventId,
    pub velocity: VectorF32,
    pub mass: f32,
    pub location: VectorF32,
    pub intermediate_result_xx: f32,
    pub intermediate_result_xy: f32,
    pub intermediate_result_xz: f32,
    pub intermediate_result_yy: f32,
    pub intermediate_result_yz: f32,
    pub intermediate_result_zz: f32,
    pub unit_surface_normal: VectorF32,
    pub coefficient_of_restitution: f32,
}

impl CollisionElastic {
    pub fn new() -> Self {
        Self {
            issuing_entity_id: Default::default(),
            colliding_entity_id: Default::default(),
            event_id: Default::default(),
            velocity: Default::default(),
            mass: 0.0,
            location: Default::default(),
            intermediate_result_xx: 0.0,
            intermediate_result_xy: 0.0,
            intermediate_result_xz: 0.0,
            intermediate_result_yy: 0.0,
            intermediate_result_yz: 0.0,
            intermediate_result_zz: 0.0,
            unit_surface_normal: Default::default(),
            coefficient_of_restitution: 0.0,
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

    pub fn with_intermediate_result_xx(mut self, intermediate_result_xx: f32) -> Self {
        self.intermediate_result_xx = intermediate_result_xx;
        self
    }

    pub fn with_intermediate_result_xy(mut self, intermediate_result_xy: f32) -> Self {
        self.intermediate_result_xy = intermediate_result_xy;
        self
    }

    pub fn with_intermediate_result_xz(mut self, intermediate_result_xz: f32) -> Self {
        self.intermediate_result_xz = intermediate_result_xz;
        self
    }

    pub fn with_intermediate_result_yy(mut self, intermediate_result_yy: f32) -> Self {
        self.intermediate_result_yy = intermediate_result_yy;
        self
    }

    pub fn with_intermediate_result_yz(mut self, intermediate_result_yz: f32) -> Self {
        self.intermediate_result_yz = intermediate_result_yz;
        self
    }

    pub fn with_intermediate_result_zz(mut self, intermediate_result_zz: f32) -> Self {
        self.intermediate_result_zz = intermediate_result_zz;
        self
    }

    pub fn with_unit_surface_normal(mut self, unit_surface_normal: VectorF32) -> Self {
        self.unit_surface_normal = unit_surface_normal;
        self
    }

    pub fn with_coefficient_of_restitution(mut self, coefficient_of_restitution: f32) -> Self {
        self.coefficient_of_restitution = coefficient_of_restitution;
        self
    }

    pub fn as_pdu_body(self) -> PduBody {
        PduBody::CollisionElastic(self)
    }
}

impl BodyInfo for CollisionElastic {
    fn body_length(&self) -> u16 {
        COLLISION_ELASTIC_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::CollisionElastic
    }
}

impl Interaction for CollisionElastic {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.issuing_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.colliding_entity_id)
    }
}