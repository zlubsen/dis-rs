use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
use crate::remove_entity::builder::RemoveEntityBuilder;
use crate::BodyRaw;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const REMOVE_ENTITY_BODY_LENGTH: u16 = 16;

/// 5.6.5.3 Remove Entity PDU
///
/// 7.5.3 Remove Entity PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RemoveEntity {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
}

impl BodyRaw for RemoveEntity {
    type Builder = RemoveEntityBuilder;

    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    fn into_builder(self) -> Self::Builder {
        Self::Builder::new_from_body(self)
    }

    fn into_pdu_body(self) -> PduBody {
        PduBody::RemoveEntity(self)
    }
}

impl BodyInfo for RemoveEntity {
    fn body_length(&self) -> u16 {
        REMOVE_ENTITY_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::RemoveEntity
    }
}

impl Interaction for RemoveEntity {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
