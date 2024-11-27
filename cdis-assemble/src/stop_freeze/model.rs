use crate::records::model::{CdisRecord, EntityId};
use crate::types::model::{VarInt, UVINT32};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use dis_rs::enumerations::{StopFreezeFrozenBehavior, StopFreezeReason};
use dis_rs::model::ClockTime;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct StopFreeze {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub real_world_time: ClockTime,
    pub reason: StopFreezeReason,
    pub frozen_behavior: StopFreezeFrozenBehavior,
    pub request_id: UVINT32,
}

impl BodyProperties for StopFreeze {
    type FieldsPresent = ();
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 0;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        0
    }

    fn body_length_bits(&self) -> usize {
        const CLOCK_TIME_LENGTH_BITS: usize = 64;
        const FIXED_FIELDS_LENGTH_BITS: usize = 7; // frozen_behavior 3(!) and reason 4
        self.originating_id.record_length()
            + self.receiving_id.record_length()
            + CLOCK_TIME_LENGTH_BITS
            + FIXED_FIELDS_LENGTH_BITS
            + self.request_id.record_length()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::StopFreeze(self)
    }
}

impl CdisInteraction for StopFreeze {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
