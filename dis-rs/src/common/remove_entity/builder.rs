use crate::model::EntityId;
use crate::remove_entity::model::RemoveEntity;

pub struct RemoveEntityBuilder(RemoveEntity);

impl RemoveEntityBuilder {
    #[must_use]
    pub fn new() -> Self {
        RemoveEntityBuilder(RemoveEntity::default())
    }

    #[must_use]
    pub fn new_from_body(body: RemoveEntity) -> Self {
        RemoveEntityBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> RemoveEntity {
        self.0
    }

    #[must_use]
    pub fn with_origination_id(mut self, originating_id: EntityId) -> Self {
        self.0.originating_id = originating_id;
        self
    }

    #[must_use]
    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.0.receiving_id = receiving_id;
        self
    }

    #[must_use]
    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }
}
