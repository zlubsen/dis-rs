use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
use crate::resupply_cancel::builder::ResupplyCancelBuilder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const RESUPPLY_CANCEL_BASE_BODY_LENGTH: u16 = 12;

/// 5.5.8 Resupply Cancel PDU
///
/// 7.4.5 Resupply Cancel PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResupplyCancel {
    pub requesting_id: EntityId,
    pub servicing_id: EntityId,
}

impl ResupplyCancel {
    #[must_use]
    pub fn builder() -> ResupplyCancelBuilder {
        ResupplyCancelBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> ResupplyCancelBuilder {
        ResupplyCancelBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ResupplyCancel(self)
    }
}

impl BodyInfo for ResupplyCancel {
    fn body_length(&self) -> u16 {
        RESUPPLY_CANCEL_BASE_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::ResupplyCancel
    }
}

impl Interaction for ResupplyCancel {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.requesting_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.servicing_id)
    }
}
