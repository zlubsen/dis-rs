use crate::records::model::{CdisRecord, EntityId};
use crate::types::model::{VarInt, UVINT32};
use crate::{BodyProperties, CdisBody, CdisInteraction};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RemoveEntity {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: UVINT32,
}

impl BodyProperties for RemoveEntity {
    type FieldsPresent = ();
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 0;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        0
    }

    fn body_length_bits(&self) -> usize {
        self.originating_id.record_length()
            + self.receiving_id.record_length()
            + self.request_id.record_length()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::RemoveEntity(self)
    }
}

impl CdisInteraction for RemoveEntity {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
