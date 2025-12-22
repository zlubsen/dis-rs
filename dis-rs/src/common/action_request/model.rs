use crate::common::action_request::builder::ActionRequestBuilder;
use crate::common::model::{length_padded_to_num, BASE_VARIABLE_DATUM_LENGTH, FIXED_DATUM_LENGTH};
use crate::common::model::{EntityId, FixedDatum, PduBody, VariableDatum};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{ActionId, PduType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const BASE_ACTION_REQUEST_BODY_LENGTH: u16 = 28;

/// 5.6.5.7 Action Request PDU
///
/// 7.5.7 Action Request PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ActionRequest {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub action_id: ActionId,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl ActionRequest {
    #[must_use]
    pub fn builder() -> ActionRequestBuilder {
        ActionRequestBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> ActionRequestBuilder {
        ActionRequestBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ActionRequest(self)
    }
}

impl BodyInfo for ActionRequest {
    fn body_length(&self) -> u16 {
        BASE_ACTION_REQUEST_BODY_LENGTH
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
        PduType::ActionRequest
    }
}

impl Interaction for ActionRequest {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}

impl From<ActionRequest> for PduBody {
    #[inline]
    fn from(value: ActionRequest) -> Self {
        value.into_pdu_body()
    }
}
