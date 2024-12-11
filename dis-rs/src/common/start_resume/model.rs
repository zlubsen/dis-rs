use crate::common::model::{ClockTime, EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
use crate::start_resume::builder::StartResumeBuilder;
use serde::{Deserialize, Serialize};

const START_RESUME_BODY_LENGTH: u16 = 32;

/// 5.6.5.4 Start/Resume PDU
///
/// 7.5.4 Start/Resume PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StartResume {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub real_world_time: ClockTime,
    pub simulation_time: ClockTime,
    pub request_id: u32,
}

impl StartResume {
    #[must_use]
    pub fn builder() -> StartResumeBuilder {
        StartResumeBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> StartResumeBuilder {
        StartResumeBuilder::new_from_body(self)
    }

    #[must_use]
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
