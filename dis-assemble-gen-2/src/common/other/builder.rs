use crate::common::other::model::Other;
use crate::model::EntityId;

pub struct OtherBuilder(Other);

impl Default for OtherBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl OtherBuilder {
    #[must_use]
    pub fn new() -> Self {
        OtherBuilder(Other::default())
    }

    #[must_use]
    pub fn new_from_body(body: Other) -> Self {
        OtherBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> Other {
        self.0
    }

    #[must_use]
    pub fn with_origin(mut self, origin: Option<EntityId>) -> Self {
        self.0.originating_entity_id = origin;
        self
    }

    #[must_use]
    pub fn with_receiver(mut self, receiver: Option<EntityId>) -> Self {
        self.0.receiving_entity_id = receiver;
        self
    }

    #[must_use]
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.0.body = body;
        self
    }
}
