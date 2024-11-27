use crate::common::model::{EntityId, RecordSpecification};
use crate::enumerations::{EventType, RequiredReliabilityService};
use crate::record_r::model::RecordR;

pub struct RecordRBuilder(RecordR);

impl RecordRBuilder {
    pub fn new() -> Self {
        RecordRBuilder(RecordR::default())
    }

    pub fn new_from_body(body: RecordR) -> Self {
        RecordRBuilder(body)
    }

    pub fn build(self) -> RecordR {
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

    pub fn with_required_reliability_service(
        mut self,
        required_reliability_service: RequiredReliabilityService,
    ) -> Self {
        self.0.required_reliability_service = required_reliability_service;
        self
    }

    pub fn with_event_type(mut self, event_type: EventType) -> Self {
        self.0.event_type = event_type;
        self
    }

    pub fn with_response_serial_number(mut self, response_serial_number: u32) -> Self {
        self.0.response_serial_number = response_serial_number;
        self
    }

    pub fn with_record_specification(mut self, record_specification: RecordSpecification) -> Self {
        self.0.record_specification = record_specification;
        self
    }
}
