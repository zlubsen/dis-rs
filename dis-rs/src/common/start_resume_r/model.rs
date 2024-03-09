use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, ClockTime, PduBody};
use crate::enumerations::{PduType, RequiredReliabilityService};
use crate::start_resume_r::builder::StartResumeRBuilder;

const START_RESUME_R_BODY_LENGTH : u16 = 36;

/// 5.12.4.4 Start/Resume-R PDU
///
/// 7.11.4 Start/Resume-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StartResumeR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub real_world_time: ClockTime,
    pub simulation_time: ClockTime,
    pub required_reliability_service: RequiredReliabilityService,
    pub request_id: u32,
}

impl StartResumeR {
    pub fn builder() -> StartResumeRBuilder {
        StartResumeRBuilder::new()
    }

    pub fn into_builder(self) -> StartResumeRBuilder {
        StartResumeRBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::StartResumeR(self)
    }
}

impl BodyInfo for StartResumeR {
    fn body_length(&self) -> u16 {
        START_RESUME_R_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::StartResumeR
    }
}

impl Interaction for StartResumeR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}