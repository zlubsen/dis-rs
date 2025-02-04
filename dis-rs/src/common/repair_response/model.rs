use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{PduType, RepairResponseRepairResult};
use crate::repair_response::builder::RepairResponseBuilder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const REPAIR_RESPONSE_BASE_BODY_LENGTH: u16 = 16;

/// 5.5.10 Repair Response PDU
///
/// 7.4.7 Repair Response PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RepairResponse {
    pub receiving_id: EntityId,
    pub repairing_id: EntityId,
    pub repair_result: RepairResponseRepairResult,
}

impl RepairResponse {
    #[must_use]
    pub fn builder() -> RepairResponseBuilder {
        RepairResponseBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> RepairResponseBuilder {
        RepairResponseBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::RepairResponse(self)
    }
}

impl BodyInfo for RepairResponse {
    fn body_length(&self) -> u16 {
        REPAIR_RESPONSE_BASE_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::RepairResponse
    }
}

impl Interaction for RepairResponse {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.repairing_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
