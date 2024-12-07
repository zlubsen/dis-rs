use crate::common::model::EntityId;
use crate::enumerations::RequiredReliabilityService;
use crate::model::RecordSpecification;
use crate::set_record_r::model::SetRecordR;

pub struct SetRecordRBuilder(SetRecordR);

impl SetRecordRBuilder {
    #[must_use]
    pub fn new() -> Self {
        SetRecordRBuilder(SetRecordR::default())
    }

    #[must_use]
    pub fn new_from_body(body: SetRecordR) -> Self {
        SetRecordRBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> SetRecordR {
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

    #[must_use]
    pub fn with_required_reliability_service(
        mut self,
        required_reliability_service: RequiredReliabilityService,
    ) -> Self {
        self.0.required_reliability_service = required_reliability_service;
        self
    }

    #[must_use]
    pub fn with_record_specification(mut self, record_specification: RecordSpecification) -> Self {
        self.0.record_specification = record_specification;
        self
    }
}
