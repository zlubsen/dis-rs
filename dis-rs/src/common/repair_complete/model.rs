use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{PduType, RepairCompleteRepair};
use crate::repair_complete::builder::RepairCompleteBuilder;

const REPAIR_COMPLETE_BASE_BODY_LENGTH: u16 = 16;

/// 5.5.9 Repair Complete PDU
///
/// 7.4.6 Repair Complete PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RepairComplete {
    pub receiving_id: EntityId,
    pub repairing_id: EntityId,
    pub repair: RepairCompleteRepair,
}

impl RepairComplete {
    #[must_use]
    pub fn builder() -> RepairCompleteBuilder {
        RepairCompleteBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> RepairCompleteBuilder {
        RepairCompleteBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::RepairComplete(self)
    }
}

impl BodyInfo for RepairComplete {
    fn body_length(&self) -> u16 {
        REPAIR_COMPLETE_BASE_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::RepairComplete
    }
}

impl Interaction for RepairComplete {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.repairing_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
