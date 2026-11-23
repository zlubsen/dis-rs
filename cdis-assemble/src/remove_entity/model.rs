use crate::records::model::{CdisRecord, EntityId};
use crate::types::model::{UVINT32, VarInt};
use crate::{BodyProperties, CdisBody, CdisInteraction};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RemoveEntity {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: UVINT32,
}

impl BodyProperties for RemoveEntity {
    type FieldsPresentFlags = ();
    type FieldsPresentField = ();

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
