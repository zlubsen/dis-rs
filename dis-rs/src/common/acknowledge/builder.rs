use crate::common::acknowledge::model::Acknowledge;
use crate::common::model::EntityId;
use crate::enumerations::{AcknowledgeFlag, ResponseFlag};

pub struct AcknowledgeBuilder(Acknowledge);

impl AcknowledgeBuilder {
    pub fn new() -> Self {
        AcknowledgeBuilder(Acknowledge::default())
    }

    pub fn new_from_body(body: Acknowledge) -> Self {
        AcknowledgeBuilder(body)
    }

    pub fn build(self) -> Acknowledge {
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

    pub fn with_acknowledge_flag(mut self, acknowledge_flag: AcknowledgeFlag) -> Self {
        self.0.acknowledge_flag = acknowledge_flag;
        self
    }

    pub fn with_response_flag(mut self, response_flag: ResponseFlag) -> Self {
        self.0.response_flag = response_flag;
        self
    }

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }
}
