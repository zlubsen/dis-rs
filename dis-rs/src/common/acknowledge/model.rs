use crate::common::acknowledge::builder::AcknowledgeBuilder;
use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{AcknowledgeFlag, PduType, ResponseFlag};
use crate::BodyRaw;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const ACKNOWLEDGE_BODY_LENGTH: u16 = 20;

/// 5.6.5.6 Acknowledge PDU
///
/// 7.5.6 Acknowledge PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Acknowledge {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: ResponseFlag,
    pub request_id: u32,
}

impl BodyRaw for Acknowledge {
    type Builder = AcknowledgeBuilder;

    #[must_use]
    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    #[must_use]
    fn into_builder(self) -> Self::Builder {
        AcknowledgeBuilder::new_from_body(self)
    }

    #[must_use]
    fn into_pdu_body(self) -> PduBody {
        PduBody::Acknowledge(self)
    }
}

impl BodyInfo for Acknowledge {
    fn body_length(&self) -> u16 {
        ACKNOWLEDGE_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::Acknowledge
    }
}

impl Interaction for Acknowledge {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
