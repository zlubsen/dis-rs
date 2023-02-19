use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, ClockTime};
use crate::{PduBody, PduType};

const START_RESUME_BODY_LENGTH : u16 = 32;

pub struct StartResume {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub real_world_time: ClockTime,
    pub simulation_time: ClockTime,
    pub request_id: u32,
}

impl Default for StartResume {
    fn default() -> Self {
        Self::new()
    }
}

impl StartResume {
    pub fn new() -> Self {
        Self {
            originating_id: Default::default(),
            receiving_id: Default::default(),
            real_world_time: ClockTime::default(),
            simulation_time: ClockTime::default(),
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

    pub fn with_simulation_time(mut self, simulation_time: ClockTime) -> Self {
        self.simulation_time = simulation_time;
        self
    }

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.request_id = request_id;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::StartResume(self)
    }
}

impl BodyInfo for StartResume {
    fn body_length(&self) -> u16 {
        START_RESUME_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::StartResume
    }
}

impl Interaction for StartResume {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}