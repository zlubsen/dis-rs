use crate::common::model::{EntityId, PduBody};
use crate::common::remove_entity_r::builder::RemoveEntityRBuilder;
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{PduType, RequiredReliabilityService};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const REMOVE_ENTITY_R_BODY_LENGTH: u16 = 20;

/// 5.12.4.3 Remove Entity-R PDU
///
/// 7.11.3 Remove Entity-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RemoveEntityR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    pub request_id: u32,
}

impl RemoveEntityR {
    #[must_use]
    pub fn builder() -> RemoveEntityRBuilder {
        RemoveEntityRBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> RemoveEntityRBuilder {
        RemoveEntityRBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::RemoveEntityR(self)
    }
}

impl BodyInfo for RemoveEntityR {
    fn body_length(&self) -> u16 {
        REMOVE_ENTITY_R_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::RemoveEntityR
    }
}

impl Interaction for RemoveEntityR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}

impl From<RemoveEntityR> for PduBody {
    #[inline]
    fn from(value: RemoveEntityR) -> Self {
        value.into_pdu_body()
    }
}
