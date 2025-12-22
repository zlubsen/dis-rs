use crate::common::collision_elastic::builder::CollisionElasticBuilder;
use crate::common::model::{EntityId, EventId, PduBody, VectorF32};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const COLLISION_ELASTIC_BODY_LENGTH: u16 = 88;

/// 5.3.4 Collision-Elastic PDU
///
/// 7.2.4 Collision-Elastic PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[must_use]
    pub fn builder() -> CollisionElasticBuilder {
        CollisionElasticBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> CollisionElasticBuilder {
        CollisionElasticBuilder::new_from_body(self)
    }

    #[must_use]
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

impl From<CollisionElastic> for PduBody {
    #[inline]
    fn from(value: CollisionElastic) -> Self {
        value.into_pdu_body()
    }
}
