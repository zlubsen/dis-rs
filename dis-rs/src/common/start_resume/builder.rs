use crate::model::{ClockTime, EntityId};
use crate::start_resume::model::StartResume;

pub struct StartResumeBuilder(StartResume);

impl Default for StartResumeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StartResumeBuilder {
    #[must_use]
    pub fn new() -> Self {
        StartResumeBuilder(StartResume::default())
    }

    #[must_use]
    pub fn new_from_body(body: StartResume) -> Self {
        StartResumeBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> StartResume {
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
    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }
}
