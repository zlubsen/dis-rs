use crate::common::{BodyInfo, Interaction};
use crate::common::create_entity::builder::CreateEntityBuilder;
use crate::common::model::{EntityId, PduBody};
use crate::enumerations::PduType;

const CREATE_ENTITY_BODY_LENGTH : u16 = 16;

#[derive(Debug, Default, PartialEq)]
pub struct CreateEntity {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
}

impl CreateEntity {
    pub fn builder() -> CreateEntityBuilder {
        CreateEntityBuilder::new()
    }

    pub fn into_builder(body: CreateEntity) -> CreateEntityBuilder {
        CreateEntityBuilder::new_from_body(body)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::CreateEntity(self)
    }
}

impl BodyInfo for CreateEntity {
    fn body_length(&self) -> u16 {
        CREATE_ENTITY_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::CreateEntity
    }
}

impl Interaction for CreateEntity {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}