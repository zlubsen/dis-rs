use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, FixedDatum, VariableDatum, BASE_VARIABLE_DATUM_LENGTH, FIXED_DATUM_LENGTH, length_padded_to_num_bytes, PduBody};
use crate::enumerations::PduType;
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::EventType;

pub const BASE_EVENT_REPORT_BODY_LENGTH: u16 = 28;

#[derive(Debug, PartialEq)]
pub struct EventReport {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub event_type: EventType,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl Default for EventReport {
    fn default() -> Self {
        Self::new()
    }
}

impl EventReport {
    pub fn new() -> Self {
        Self {
            originating_id: Default::default(),
            receiving_id: Default::default(),
            event_type: Default::default(),
            fixed_datum_records: vec![],
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

    pub fn with_event_type(mut self, event_type: EventType) -> Self {
        self.event_type = event_type;
        self
    }

    pub fn with_fixed_datums(mut self, fixed_datum_records: Vec<FixedDatum>) -> Self {
        self.fixed_datum_records = fixed_datum_records;
        self
    }

    pub fn with_variable_datums(mut self, variable_datum_records: Vec<VariableDatum>) -> Self {
        self.variable_datum_records = variable_datum_records;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::EventReport(self)
    }
}

impl BodyInfo for EventReport {
    fn body_length(&self) -> u16 {
        BASE_EVENT_REPORT_BODY_LENGTH +
            (FIXED_DATUM_LENGTH * self.fixed_datum_records.len() as u16) +
            (self.variable_datum_records.iter().map(|datum| {
                let padded_record = length_padded_to_num_bytes(
                    BASE_VARIABLE_DATUM_LENGTH as usize + datum.datum_value.len(),
                    EIGHT_OCTETS);
                padded_record.record_length_bytes as u16
            } ).sum::<u16>())
    }

    fn body_type(&self) -> PduType {
        PduType::EventReport
    }
}

impl Interaction for EventReport {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}