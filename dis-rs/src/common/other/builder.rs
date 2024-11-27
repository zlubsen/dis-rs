use crate::common::other::model::Other;
use crate::model::EntityId;

pub struct OtherBuilder(Other);

impl OtherBuilder {
    pub fn new() -> Self {
        OtherBuilder(Other::default())
    }

    pub fn new_from_body(body: Other) -> Self {
        OtherBuilder(body)
    }

    pub fn build(self) -> Other {
        self.0
    }

    pub fn with_origin(mut self, origin: Option<EntityId>) -> Self {
        self.0.originating_entity_id = origin;
        self
    }

    pub fn with_receiver(mut self, receiver: Option<EntityId>) -> Self {
        self.0.receiving_entity_id = receiver;
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.0.body = body;
        self
    }
}
