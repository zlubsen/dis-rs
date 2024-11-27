use crate::common::model::EntityId;
use crate::enumerations::{RecordQueryREventType, RequiredReliabilityService};
use crate::model::TimeStamp;
use crate::record_query_r::model::{RecordQueryR, RecordQuerySpecification};

pub struct RecordQueryRBuilder(RecordQueryR);

impl RecordQueryRBuilder {
    pub fn new() -> Self {
        RecordQueryRBuilder(RecordQueryR::default())
    }

    pub fn new_from_body(body: RecordQueryR) -> Self {
        RecordQueryRBuilder(body)
    }

    pub fn build(self) -> RecordQueryR {
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

    pub fn with_event_type(mut self, event_type: RecordQueryREventType) -> Self {
        self.0.event_type = event_type;
        self
    }

    pub fn with_time(mut self, time: TimeStamp) -> Self {
        self.0.time = time;
        self
    }

    pub fn with_record_query_specification(
        mut self,
        record_query_specification: RecordQuerySpecification,
    ) -> Self {
        self.0.record_query_specification = record_query_specification;
        self
    }
}
