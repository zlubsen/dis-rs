use crate::common::model::EntityId;
use crate::data_query_r::model::DataQueryR;
use crate::enumerations::{RequiredReliabilityService, VariableRecordType};

pub struct DataQueryRBuilder(DataQueryR);

impl DataQueryRBuilder {
    pub fn new() -> Self {
        DataQueryRBuilder(DataQueryR::default())
    }

    pub fn new_from_body(body: DataQueryR) -> Self {
        DataQueryRBuilder(body)
    }

    pub fn build(self) -> DataQueryR {
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

    pub fn with_time_interval(mut self, time_interval: u32) -> Self {
        self.0.time_interval = time_interval;
        self
    }

    pub fn with_fixed_datums(mut self, fixed_datum_records: Vec<VariableRecordType>) -> Self {
        self.0.fixed_datum_records = fixed_datum_records;
        self
    }

    pub fn with_variable_datums(mut self, variable_datum_records: Vec<VariableRecordType>) -> Self {
        self.0.variable_datum_records = variable_datum_records;
        self
    }
}
