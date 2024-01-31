use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, VariableDatum, BASE_VARIABLE_DATUM_LENGTH, length_padded_to_num_bytes, PduBody};
use crate::enumerations::PduType;
use crate::constants::EIGHT_OCTETS;

pub const BASE_COMMENT_BODY_LENGTH: u16 = 20;

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl Default for Comment {
    fn default() -> Self {
        Self::new()
    }
}

impl Comment {
    pub fn new() -> Self {
        Self {
            originating_id: Default::default(),
            receiving_id: Default::default(),
            variable_datum_records: vec![],
        }
    }

    pub fn with_origination_id(mut self, originating_id: EntityId) -> Self {
        self.originating_id = originating_id;
        self
    }

    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.receiving_id = receiving_id;
        self
    }

    pub fn with_variable_datums(mut self, variable_datum_records: Vec<VariableDatum>) -> Self {
        self.variable_datum_records = variable_datum_records;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Comment(self)
    }
}

impl BodyInfo for Comment {
    fn body_length(&self) -> u16 {
        BASE_COMMENT_BODY_LENGTH +
            (self.variable_datum_records.iter().map(|datum| {
                let padded_record = length_padded_to_num_bytes(
                    BASE_VARIABLE_DATUM_LENGTH as usize + datum.datum_value.len(),
                    EIGHT_OCTETS);
                padded_record.record_length_bytes as u16
            } ).sum::<u16>())
    }

    fn body_type(&self) -> PduType {
        PduType::Comment
    }
}

impl Interaction for Comment {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}