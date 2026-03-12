use crate::acknowledge_r::model::AcknowledgeR;
use crate::common::model::EntityId;
use crate::enumerations::{AcknowledgeFlag, ResponseFlag};

pub struct AcknowledgeRBuilder(AcknowledgeR);

impl Default for AcknowledgeRBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AcknowledgeRBuilder {
    #[must_use]
    pub fn new() -> Self {
        AcknowledgeRBuilder(AcknowledgeR::default())
    }

    #[must_use]
    pub fn new_from_body(body: AcknowledgeR) -> Self {
        AcknowledgeRBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> AcknowledgeR {
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
    pub fn with_acknowledge_flag(mut self, acknowledge_flag: AcknowledgeFlag) -> Self {
        self.0.acknowledge_flag = acknowledge_flag;
        self
    }

    #[must_use]
    pub fn with_response_flag(mut self, response_flag: ResponseFlag) -> Self {
        self.0.response_flag = response_flag;
        self
    }

    #[must_use]
    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }
}
