use crate::common::{BodyInfo, Interaction};
use crate::common::model::EntityId;
use crate::common::model::ClockTime;
use crate::enumerations::{StopFreezeReason, StopFreezeFrozenBehavior};
use crate::PduType;

const STOP_FREEZE_BODY_LENGTH : u16 = 28;

pub struct StopFreeze {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub real_world_time: ClockTime,
    pub reason: StopFreezeReason,
    pub frozen_behavior: StopFreezeFrozenBehavior,
    pub request_id: u32,
}

impl StopFreeze {
    pub fn new() -> Self {
        Self {
            originating_id: Default::default(),
            receiving_id: Default::default(),
            real_world_time: Default::default(),
            reason: Default::default(),
            frozen_behavior: StopFreezeFrozenBehavior {
                run_simulation_clock: false,
                transmit_updates: false,
                process_updates: false,
            },
            request_id: 0,
        }
    }

    pub fn with_origination_id(mut self, originating_id: EntityId) -> Self {
        self.originating_id = originating_id;
        self
    }

    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.receiving_id = receiving_id;
        self
    }

    pub fn with_real_world_time(mut self, real_world_time: ClockTime) -> Self {
        self.real_world_time = real_world_time;
        self
    }

    pub fn with_reason(mut self, reason: StopFreezeReason) -> Self {
        self.reason = reason;
        self
    }

    pub fn with_frozen_behavior(mut self, frozen_behavior: StopFreezeFrozenBehavior) -> Self {
        self.frozen_behavior = frozen_behavior;
        self
    }

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.request_id = request_id;
        self
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