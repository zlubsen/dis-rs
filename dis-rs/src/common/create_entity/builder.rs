use crate::common::create_entity::model::CreateEntity;
use crate::common::model::EntityId;

pub struct CreateEntityBuilder(CreateEntity);

impl CreateEntityBuilder {
    pub fn new() -> Self {
        CreateEntityBuilder(CreateEntity::default())
    }

    pub fn new_from_body(body: CreateEntity) -> Self {
        CreateEntityBuilder(body)
    }

    pub fn build(self) -> CreateEntity {
        self.0
    }

    pub fn with_origination_id(mut self, originating_id: EntityId) -> Self {
        self.0.originating_id = originating_id;
        self
    }

    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.0.receiving_id = receiving_id;
        self
    }

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }
}
