use crate::records::model::{CdisRecord, EntityId};
use crate::types::model::{UVINT32, VarInt};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use dis_rs::enumerations::{AcknowledgeFlag, ResponseFlag};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Acknowledge {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: ResponseFlag,
    pub request_id: UVINT32,
}

impl BodyProperties for Acknowledge {
    type FieldsPresentFlags = ();
    type FieldsPresentField = ();

    fn body_length_bits(&self) -> usize {
        const FIXED_FLAGS_LENGTH_BITS: usize = 5; // Acknowledge flag (3) and Response flag (2)
        self.originating_id.record_length()
            + self.receiving_id.record_length()
            + FIXED_FLAGS_LENGTH_BITS
            + self.request_id.record_length()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::Acknowledge(self)
    }
}

impl CdisInteraction for Acknowledge {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
