use crate::action_request_r::model::ActionRequestR;
use crate::common::model::{EntityId, FixedDatum, VariableDatum};
use crate::enumerations::{ActionId, RequiredReliabilityService};

pub struct ActionRequestRBuilder(ActionRequestR);

impl ActionRequestRBuilder {
    #[must_use]
    pub fn new() -> Self {
        ActionRequestRBuilder(ActionRequestR::default())
    }

    #[must_use]
    pub fn new_from_body(body: ActionRequestR) -> Self {
        ActionRequestRBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> ActionRequestR {
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
    pub fn with_action_id(mut self, action_id: ActionId) -> Self {
        self.0.action_id = action_id;
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
