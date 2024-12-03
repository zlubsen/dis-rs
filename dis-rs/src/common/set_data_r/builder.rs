use crate::enumerations::RequiredReliabilityService;
use crate::model::{EntityId, FixedDatum, VariableDatum};
use crate::set_data_r::model::SetDataR;

pub struct SetDataRBuilder(SetDataR);

impl SetDataRBuilder {
    #[must_use]
    pub fn new() -> Self {
        SetDataRBuilder(SetDataR::default())
    }

    #[must_use]
    pub fn new_from_body(body: SetDataR) -> Self {
        SetDataRBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> SetDataR {
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
    pub fn with_required_reliability_service(
        mut self,
        required_reliability_service: RequiredReliabilityService,
    ) -> Self {
        self.0.required_reliability_service = required_reliability_service;
        self
    }

    #[must_use]
    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }

    #[must_use]
    pub fn with_fixed_datums(mut self, fixed_datum_records: Vec<FixedDatum>) -> Self {
        self.0.fixed_datum_records = fixed_datum_records;
        self
    }

    #[must_use]
    pub fn with_variable_datums(mut self, variable_datum_records: Vec<VariableDatum>) -> Self {
        self.0.variable_datum_records = variable_datum_records;
        self
    }
}
