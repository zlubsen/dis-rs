use crate::action_request_r::builder::ActionRequestRBuilder;
use crate::common::model::{length_padded_to_num, BASE_VARIABLE_DATUM_LENGTH, FIXED_DATUM_LENGTH};
use crate::common::model::{EntityId, FixedDatum, PduBody, VariableDatum};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{ActionId, PduType, RequiredReliabilityService};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const BASE_ACTION_REQUEST_R_BODY_LENGTH: u16 = 36;

/// 5.12.4.7 Action Request-R PDU
///
/// 7.11.7 Action Request-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ActionRequestR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    pub request_id: u32,
    pub action_id: ActionId,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl ActionRequestR {
    #[must_use]
    pub fn builder() -> ActionRequestRBuilder {
        ActionRequestRBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> ActionRequestRBuilder {
        ActionRequestRBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ActionRequestR(self)
    }
}

impl BodyInfo for ActionRequestR {
    fn body_length(&self) -> u16 {
        BASE_ACTION_REQUEST_R_BODY_LENGTH
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
        PduType::ActionRequestR
    }
}

impl Interaction for ActionRequestR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
