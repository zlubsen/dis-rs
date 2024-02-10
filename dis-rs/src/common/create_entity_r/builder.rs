use crate::common::model::EntityId;
use crate::create_entity_r::model::CreateEntityR;
use crate::enumerations::RequiredReliabilityService;

pub struct CreateEntityRBuilder(CreateEntityR);

impl CreateEntityRBuilder {
    pub fn new() -> Self {
        CreateEntityRBuilder(CreateEntityR::default())
    }

    pub fn new_from_body(body: CreateEntityR) -> Self {
        CreateEntityRBuilder(body)
    }

    pub fn build(self) -> CreateEntityR {
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

    pub fn with_required_reliability_service(mut self, required_reliability_service: RequiredReliabilityService) -> Self {
        self.0.required_reliability_service = required_reliability_service;
        self
    }

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }

}