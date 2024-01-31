use crate::common::comment::model::Comment;
use crate::common::model::{EntityId, VariableDatum};

pub struct CommentBuilder(Comment);

impl CommentBuilder {
    pub fn new() -> Self {
        CommentBuilder(Comment::default())
    }

    pub fn new_from_body(body: Comment) -> Self {
        CommentBuilder(body)
    }

    pub fn build(self) -> Comment {
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

    pub fn with_variable_datums(mut self, variable_datum_records: Vec<VariableDatum>) -> Self {
        self.0.variable_datum_records = variable_datum_records;
        self
    }
}