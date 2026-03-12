use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::create_entity_r::builder::CreateEntityRBuilder;
use crate::enumerations::{PduType, RequiredReliabilityService};
use crate::BodyRaw;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const CREATE_ENTITY_R_BODY_LENGTH: u16 = 20;

/// 5.12.4.2 Create Entity-R PDU
///
/// 7.11.2 Create Entity-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateEntityR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    pub request_id: u32,
}

impl BodyRaw for CreateEntityR {
    type Builder = CreateEntityRBuilder;

    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    fn into_builder(self) -> Self::Builder {
        Self::Builder::new_from_body(self)
    }

    fn into_pdu_body(self) -> PduBody {
        PduBody::CreateEntityR(self)
    }
}

impl BodyInfo for CreateEntityR {
    fn body_length(&self) -> u16 {
        CREATE_ENTITY_R_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::CreateEntityR
    }
}

impl Interaction for CreateEntityR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
