use crate::enumerations::RequiredReliabilityService;
use crate::model::{ClockTime, EntityId};
use crate::start_resume_r::model::StartResumeR;

pub struct StartResumeRBuilder(StartResumeR);

impl StartResumeRBuilder {
    #[must_use]
    pub fn new() -> Self {
        StartResumeRBuilder(StartResumeR::default())
    }

    #[must_use]
    pub fn new_from_body(body: StartResumeR) -> Self {
        StartResumeRBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> StartResumeR {
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
    pub fn with_simulation_time(mut self, simulation_time: ClockTime) -> Self {
        self.0.simulation_time = simulation_time;
        self
    }

    #[must_use]
    pub fn with_required_reliability_service(
        mut self,
        required_reliability_service: RequiredReliabilityService,
    ) -> Self {
        self.0.required_reliability_service = required_reliability_service;
        self
    }

    #[must_use]
    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }
}
