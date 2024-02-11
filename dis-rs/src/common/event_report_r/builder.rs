use crate::enumerations::EventType;
use crate::event_report_r::model::EventReportR;
use crate::model::{EntityId, FixedDatum, VariableDatum};

pub struct EventReportRBuilder(EventReportR);

impl EventReportRBuilder {
    pub fn new() -> Self {
        EventReportRBuilder(EventReportR::default())
    }

    pub fn new_from_body(body: EventReportR) -> Self {
        EventReportRBuilder(body)
    }

    pub fn build(self) -> EventReportR {
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

    pub fn with_event_type(mut self, event_type: EventType) -> Self {
        self.0.event_type = event_type;
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
