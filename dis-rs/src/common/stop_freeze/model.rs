use crate::common::{BodyInfo, Interaction};
use crate::common::model::{ClockTime, EntityId, PduBody};
use crate::enumerations::{StopFreezeReason, StopFreezeFrozenBehavior, PduType};
use crate::stop_freeze::builder::StopFreezeBuilder;

const STOP_FREEZE_BODY_LENGTH : u16 = 28;

/// 5.6.5.5 Stop/Freeze PDU
///
/// 7.5.5 Stop/Freeze PDU
#[derive(Debug, Default, PartialEq)]
pub struct StopFreeze {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub real_world_time: ClockTime,
    pub reason: StopFreezeReason,
    pub frozen_behavior: StopFreezeFrozenBehavior,
    pub request_id: u32,
}

impl StopFreeze {
    pub fn builder() -> StopFreezeBuilder {
        StopFreezeBuilder::new()
    }

    pub fn into_builder(self) -> StopFreezeBuilder {
        StopFreezeBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::StopFreeze(self)
    }
}

impl BodyInfo for StopFreeze {
    fn body_length(&self) -> u16 {
        STOP_FREEZE_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::StopFreeze
    }
}

impl Interaction for StopFreeze {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}