use crate::common::model::{
    length_padded_to_num, EntityId, FixedDatum, PduBody, VariableDatum, BASE_VARIABLE_DATUM_LENGTH,
    FIXED_DATUM_LENGTH,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::data_r::builder::DataRBuilder;
use crate::enumerations::{PduType, RequiredReliabilityService};

pub const BASE_DATA_R_BODY_LENGTH: u16 = 28;

/// 5.12.4.11 Data-R PDU
///
/// 7.11.11 Data-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DataR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl DataR {
    #[must_use]
    pub fn builder() -> DataRBuilder {
        DataRBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> DataRBuilder {
        DataRBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::DataR(self)
    }
}

impl BodyInfo for DataR {
    fn body_length(&self) -> u16 {
        BASE_DATA_R_BODY_LENGTH
            + (FIXED_DATUM_LENGTH * self.fixed_datum_records.len() as u16)
            + (self
                .variable_datum_records
                .iter()
                .map(|datum| {
                    let padded_record = length_padded_to_num(
                        BASE_VARIABLE_DATUM_LENGTH as usize + datum.datum_value.len(),
                        EIGHT_OCTETS,
                    );
                    padded_record.record_length as u16
                })
                .sum::<u16>())
    }

    fn body_type(&self) -> PduType {
        PduType::DataR
    }
}

impl Interaction for DataR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
