use crate::comment_r::model::CommentR;
use crate::common::model::{EntityId, VariableDatum};

pub struct CommentRBuilder(CommentR);

impl CommentRBuilder {
    #[must_use]
    pub fn new() -> Self {
        CommentRBuilder(CommentR::default())
    }

    #[must_use]
    pub fn new_from_body(body: CommentR) -> Self {
        CommentRBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> CommentR {
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
    pub fn with_variable_datums(mut self, variable_datum_records: Vec<VariableDatum>) -> Self {
        self.0.variable_datum_records = variable_datum_records;
        self
    }
}
