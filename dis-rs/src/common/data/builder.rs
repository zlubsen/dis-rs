use crate::common::data::model::Data;
use crate::common::model::{EntityId, FixedDatum, VariableDatum};

pub struct DataBuilder(Data);

impl DataBuilder {
    pub fn new() -> Self {
        DataBuilder(Data::default())
    }

    pub fn new_from_body(body: Data) -> Self {
        DataBuilder(body)
    }

    pub fn build(self) -> Data {
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

    pub fn with_fixed_datums(mut self, fixed_datum_records: Vec<FixedDatum>) -> Self {
        self.0.fixed_datum_records = fixed_datum_records;
        self
    }

    pub fn with_variable_datums(mut self, variable_datum_records: Vec<VariableDatum>) -> Self {
        self.0.variable_datum_records = variable_datum_records;
        self
    }
}
