use crate::common::{BodyInfo, Interaction};
use crate::common::model::EntityId;
use crate::constants::FOUR_OCTETS;
use crate::enumerations::VariableRecordType;
use crate::{PduBody, PduType};

pub const BASE_DATA_QUERY_BODY_LENGTH: u16 = 28;

#[derive(Debug, PartialEq)]
pub struct DataQuery {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub time_interval: u32,
    pub fixed_datum_records: Vec<VariableRecordType>,
    pub variable_datum_records: Vec<VariableRecordType>,
}

impl Default for DataQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl DataQuery {
    pub fn new() -> Self {
        Self {
            originating_id: Default::default(),
            receiving_id: Default::default(),
            request_id: 0,
            time_interval: 0,
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

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.request_id = request_id;
        self
    }

    pub fn with_time_interval(mut self, time_interval: u32) -> Self {
        self.time_interval = time_interval;
        self
    }

    pub fn with_fixed_datums(mut self, fixed_datum_records: Vec<VariableRecordType>) -> Self {
        self.fixed_datum_records = fixed_datum_records;
        self
    }

    pub fn with_variable_datums(mut self, variable_datum_records: Vec<VariableRecordType>) -> Self {
        self.variable_datum_records = variable_datum_records;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::DataQuery(self)
    }
}

impl BodyInfo for DataQuery {
    fn body_length(&self) -> u16 {
        BASE_DATA_QUERY_BODY_LENGTH +
            (FOUR_OCTETS * self.fixed_datum_records.len()) as u16 +
            (FOUR_OCTETS * self.variable_datum_records.len()) as u16
    }

    fn body_type(&self) -> PduType {
        PduType::DataQuery
    }
}

impl Interaction for DataQuery {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}