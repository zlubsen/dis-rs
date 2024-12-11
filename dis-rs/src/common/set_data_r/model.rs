use crate::common::model::{
    length_padded_to_num, EntityId, FixedDatum, PduBody, VariableDatum, BASE_VARIABLE_DATUM_LENGTH,
    FIXED_DATUM_LENGTH,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{PduType, RequiredReliabilityService};
use crate::set_data_r::builder::SetDataRBuilder;
use serde::{Deserialize, Serialize};

pub const BASE_SET_DATA_R_BODY_LENGTH: u16 = 28;

/// 5.12.4.10 Set Data-R PDU
///
/// 7.11.10 Set Data-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SetDataR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    pub request_id: u32,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl SetDataR {
    #[must_use]
    pub fn builder() -> SetDataRBuilder {
        SetDataRBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> SetDataRBuilder {
        SetDataRBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::SetDataR(self)
    }
}

impl BodyInfo for SetDataR {
    fn body_length(&self) -> u16 {
        BASE_SET_DATA_R_BODY_LENGTH
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
        PduType::SetDataR
    }
}

impl Interaction for SetDataR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
