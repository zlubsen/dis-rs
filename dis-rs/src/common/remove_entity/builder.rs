use crate::model::EntityId;
use crate::remove_entity::model::RemoveEntity;

pub struct RemoveEntityBuilder(RemoveEntity);

impl RemoveEntityBuilder {
    pub fn new() -> Self {
        RemoveEntityBuilder(RemoveEntity::default())
    }

    pub fn new_from_body(body: RemoveEntity) -> Self {
        RemoveEntityBuilder(body)
    }

    pub fn build(self) -> RemoveEntity {
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
