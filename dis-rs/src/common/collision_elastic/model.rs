use crate::common::model::{EntityId, EventId, PduBody, VectorF32};
use crate::enumerations::PduType;
use crate::common::{BodyInfo, Interaction};
use crate::common::collision_elastic::builder::CollisionElasticBuilder;

const COLLISION_ELASTIC_BODY_LENGTH : u16 = 88;

/// 5.3.4 Collision-Elastic PDU
///
/// 7.2.4 Collision-Elastic PDU
#[derive(Debug, Default, PartialEq)]
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
    pub fn builder() -> CollisionElasticBuilder {
        CollisionElasticBuilder::new()
    }

    pub fn into_builder(self) -> CollisionElasticBuilder {
        CollisionElasticBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
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