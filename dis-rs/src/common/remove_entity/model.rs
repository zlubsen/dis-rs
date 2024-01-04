use crate::common::{BodyInfo, Interaction};
use crate::common::model::EntityId;
use crate::{PduBody, PduType};

const REMOVE_ENTITY_BODY_LENGTH : u16 = 16;

#[derive(Debug, PartialEq)]
pub struct RemoveEntity {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
}

impl Default for RemoveEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoveEntity {
    pub fn new() -> Self {
        Self {
            originating_id: Default::default(),
            receiving_id: Default::default(),
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

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.request_id = request_id;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::RemoveEntity(self)
    }
}

impl BodyInfo for RemoveEntity {
    fn body_length(&self) -> u16 {
        REMOVE_ENTITY_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::RemoveEntity
    }
}

impl Interaction for RemoveEntity {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}