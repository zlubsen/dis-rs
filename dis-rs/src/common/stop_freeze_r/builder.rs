use crate::enumerations::{RequiredReliabilityService, StopFreezeFrozenBehavior, StopFreezeReason};
use crate::model::{ClockTime, EntityId};
use crate::stop_freeze_r::model::StopFreezeR;

pub struct StopFreezeRBuilder(StopFreezeR);

impl StopFreezeRBuilder {
    pub fn new() -> Self {
        StopFreezeRBuilder(StopFreezeR::default())
    }

    pub fn new_from_body(body: StopFreezeR) -> Self {
        StopFreezeRBuilder(body)
    }

    pub fn build(self) -> StopFreezeR {
        self.0
    }

    pub fn with_origination_id(mut self, originating_id: EntityId) -> Self {
        self.0.originating_id = originating_id;
        self
    }

    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.0.receiving_id = receiving_id;
        self
    }

    pub fn with_real_world_time(mut self, real_world_time: ClockTime) -> Self {
        self.0.real_world_time = real_world_time;
        self
    }

    pub fn with_reason(mut self, reason: StopFreezeReason) -> Self {
        self.0.reason = reason;
        self
    }

    pub fn with_frozen_behavior(mut self, frozen_behavior: StopFreezeFrozenBehavior) -> Self {
        self.0.frozen_behavior = frozen_behavior;
        self
    }

    pub fn with_required_reliability_service(mut self, required_reliability_service: RequiredReliabilityService) -> Self {
        self.0.required_reliability_service = required_reliability_service;
        self
    }

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }
}
