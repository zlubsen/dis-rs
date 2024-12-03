use crate::enumerations::{StopFreezeFrozenBehavior, StopFreezeReason};
use crate::model::{ClockTime, EntityId};
use crate::stop_freeze::model::StopFreeze;

pub struct StopFreezeBuilder(StopFreeze);

impl StopFreezeBuilder {
    #[must_use]
    pub fn new() -> Self {
        StopFreezeBuilder(StopFreeze::default())
    }

    #[must_use]
    pub fn new_from_body(body: StopFreeze) -> Self {
        StopFreezeBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> StopFreeze {
        self.0
    }

    #[must_use]
    pub fn with_origination_id(mut self, originating_id: EntityId) -> Self {
        self.0.originating_id = originating_id;
        self
    }

    #[must_use]
    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.0.receiving_id = receiving_id;
        self
    }

    #[must_use]
    pub fn with_real_world_time(mut self, real_world_time: ClockTime) -> Self {
        self.0.real_world_time = real_world_time;
        self
    }

    #[must_use]
    pub fn with_reason(mut self, reason: StopFreezeReason) -> Self {
        self.0.reason = reason;
        self
    }

    #[must_use]
    pub fn with_frozen_behavior(mut self, frozen_behavior: StopFreezeFrozenBehavior) -> Self {
        self.0.frozen_behavior = frozen_behavior;
        self
    }

    #[must_use]
    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }
}
