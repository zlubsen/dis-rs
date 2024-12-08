use crate::common::create_entity::model::CreateEntity;
use crate::common::model::EntityId;

pub struct CreateEntityBuilder(CreateEntity);

impl Default for CreateEntityBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateEntityBuilder {
    #[must_use]
    pub fn new() -> Self {
        CreateEntityBuilder(CreateEntity::default())
    }

    #[must_use]
    pub fn new_from_body(body: CreateEntity) -> Self {
        CreateEntityBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> CreateEntity {
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
