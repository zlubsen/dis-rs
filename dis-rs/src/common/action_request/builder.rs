use crate::common::action_request::model::ActionRequest;
use crate::common::model::{EntityId, FixedDatum, VariableDatum};
use crate::enumerations::ActionId;

pub struct ActionRequestBuilder(ActionRequest);

impl Default for ActionRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRequestBuilder {
    #[must_use]
    pub fn new() -> Self {
        ActionRequestBuilder(ActionRequest::default())
    }

    #[must_use]
    pub fn new_from_body(body: ActionRequest) -> Self {
        ActionRequestBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> ActionRequest {
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
