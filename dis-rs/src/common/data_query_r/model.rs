use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::constants::FOUR_OCTETS;
use crate::data_query_r::builder::DataQueryRBuilder;
use crate::enumerations::{PduType, RequiredReliabilityService, VariableRecordType};
use crate::BodyRaw;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const BASE_DATA_QUERY_R_BODY_LENGTH: u16 = 32;

/// 5.12.4.9 Data Query-R PDU
///
/// 7.11.9 Data Query-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DataQueryR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    pub request_id: u32,
    pub time_interval: u32,
    pub fixed_datum_records: Vec<VariableRecordType>,
    pub variable_datum_records: Vec<VariableRecordType>,
}

impl BodyRaw for DataQueryR {
    type Builder = DataQueryRBuilder;

    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    fn into_builder(self) -> Self::Builder {
        Self::Builder::new_from_body(self)
    }

    fn into_pdu_body(self) -> PduBody {
        PduBody::DataQueryR(self)
    }
}

impl BodyInfo for DataQueryR {
    fn body_length(&self) -> u16 {
        BASE_DATA_QUERY_R_BODY_LENGTH
            + (FOUR_OCTETS * self.fixed_datum_records.len()) as u16
            + (FOUR_OCTETS * self.variable_datum_records.len()) as u16
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
