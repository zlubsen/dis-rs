use crate::common::model::{
    EntityId, EventId, ExpendableDescriptor, Location, MunitionDescriptor, PduBody, VectorF32,
};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
use crate::fire::builder::FireBuilder;
use crate::BodyRaw;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const FIRE_BODY_LENGTH: u16 = 84;

/// 5.4.3 Fire PDU
///
/// 7.3.2 Fire PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Fire {
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub entity_id: EntityId,
    pub event_id: EventId,
    pub fire_mission_index: u32,
    pub location_in_world: Location,
    pub descriptor: FireDescriptor,
    pub velocity: VectorF32,
    pub range: f32,
}

impl BodyRaw for Fire {
    type Builder = FireBuilder;

    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    fn into_builder(self) -> Self::Builder {
        Self::Builder::new_from_body(self)
    }

    fn into_pdu_body(self) -> PduBody {
        PduBody::Fire(self)
    }
}

impl BodyInfo for Fire {
    fn body_length(&self) -> u16 {
        FIRE_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::Fire
    }
}

impl Interaction for Fire {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.firing_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.target_entity_id)
    }
}

/// 6.2.19 Fire Descriptor record
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FireDescriptor {
    #[cfg_attr(feature = "serde", serde(rename = "munition"))]
    Munition(MunitionDescriptor),
    #[cfg_attr(feature = "serde", serde(rename = "expendable"))]
    Expendable(ExpendableDescriptor),
}

impl Default for FireDescriptor {
    fn default() -> Self {
        Self::Munition(MunitionDescriptor::default())
    }
}

impl From<MunitionDescriptor> for FireDescriptor {
    #[inline]
    fn from(value: MunitionDescriptor) -> Self {
        Self::Munition(value)
    }
}

impl From<ExpendableDescriptor> for FireDescriptor {
    #[inline]
    fn from(value: ExpendableDescriptor) -> Self {
        Self::Expendable(value)
    }
}
