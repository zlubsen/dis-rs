use crate::common::data_query::builder::DataQueryBuilder;
use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::constants::FOUR_OCTETS;
use crate::enumerations::{PduType, VariableRecordType};

pub const BASE_DATA_QUERY_BODY_LENGTH: u16 = 28;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DataQuery {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub time_interval: u32,
    pub fixed_datum_records: Vec<VariableRecordType>,
    pub variable_datum_records: Vec<VariableRecordType>,
}

impl DataQuery {
    #[must_use]
    pub fn builder() -> DataQueryBuilder {
        DataQueryBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> DataQueryBuilder {
        DataQueryBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::DataQuery(self)
    }
}

impl BodyInfo for DataQuery {
    fn body_length(&self) -> u16 {
        BASE_DATA_QUERY_BODY_LENGTH
            + (FOUR_OCTETS * self.fixed_datum_records.len()) as u16
            + (FOUR_OCTETS * self.variable_datum_records.len()) as u16
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
