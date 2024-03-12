use crate::common::{BodyInfo, Interaction};
use crate::common::model::{ClockTime, EntityId, PduBody};
use crate::enumerations::{StopFreezeReason, StopFreezeFrozenBehavior, PduType, RequiredReliabilityService};
use crate::stop_freeze_r::builder::StopFreezeRBuilder;

const STOP_FREEZE_R_BODY_LENGTH : u16 = 28;

/// 5.12.4.5 Stop/Freeze-R PDU
///
/// 7.11.5 Stop/Freeze-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StopFreezeR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub real_world_time: ClockTime,
    pub reason: StopFreezeReason,
    pub frozen_behavior: StopFreezeFrozenBehavior,
    pub required_reliability_service: RequiredReliabilityService,
    pub request_id: u32,
}

impl StopFreezeR {
    pub fn builder() -> StopFreezeRBuilder {
        StopFreezeRBuilder::new()
    }

    pub fn into_builder(self) -> StopFreezeRBuilder {
        StopFreezeRBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::StopFreezeR(self)
    }
}

impl BodyInfo for StopFreezeR {
    fn body_length(&self) -> u16 {
        STOP_FREEZE_R_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::StopFreezeR
    }
}

impl Interaction for StopFreezeR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}