use dis_rs::model::ClockTime;
use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::records::model::{CdisRecord, EntityId};
use crate::types::model::{UVINT32, VarInt};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct StartResume {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub real_world_time: ClockTime,
    pub simulation_time: ClockTime,
    pub request_id: UVINT32,
}

impl BodyProperties for StartResume {
    type FieldsPresent = ();
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 0;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        0
    }

    fn body_length_bits(&self) -> usize {
        const CLOCK_TIME_LENGTH_BITS: usize = 64;
        self.originating_id.record_length()
            + self.receiving_id.record_length()
            + CLOCK_TIME_LENGTH_BITS
            + CLOCK_TIME_LENGTH_BITS
            + self.request_id.record_length()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::StartResume(self)
    }
}

impl CdisInteraction for StartResume {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}