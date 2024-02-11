use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, PduBody};
use crate::constants::FOUR_OCTETS;
use crate::data_query_r::builder::DataQueryRBuilder;
use crate::enumerations::{PduType, RequiredReliabilityService, VariableRecordType};

pub const BASE_DATA_QUERY_R_BODY_LENGTH: u16 = 32;

#[derive(Debug, Default, PartialEq)]
pub struct DataQueryR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    pub request_id: u32,
    pub time_interval: u32,
    pub fixed_datum_records: Vec<VariableRecordType>,
    pub variable_datum_records: Vec<VariableRecordType>,
}

impl DataQueryR {
    pub fn builder() -> DataQueryRBuilder {
        DataQueryRBuilder::new()
    }

    pub fn into_builder(self) -> DataQueryRBuilder {
        DataQueryRBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::DataQueryR(self)
    }
}

impl BodyInfo for DataQueryR {
    fn body_length(&self) -> u16 {
        BASE_DATA_QUERY_R_BODY_LENGTH +
            (FOUR_OCTETS * self.fixed_datum_records.len()) as u16 +
            (FOUR_OCTETS * self.variable_datum_records.len()) as u16
    }

    fn body_type(&self) -> PduType {
        PduType::DataQueryR
    }
}

impl Interaction for DataQueryR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}