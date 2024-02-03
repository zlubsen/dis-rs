use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, PduBody};
use crate::enumerations::PduType;
use crate::remove_entity::builder::RemoveEntityBuilder;

const REMOVE_ENTITY_BODY_LENGTH : u16 = 16;

#[derive(Debug, Default, PartialEq)]
pub struct RemoveEntity {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
}

impl RemoveEntity {
    pub fn builder() -> RemoveEntityBuilder {
        RemoveEntityBuilder::new()
    }

    pub fn into_builder(self) -> RemoveEntityBuilder {
        RemoveEntityBuilder::new_from_body(self)
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