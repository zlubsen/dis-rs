use crate::common::action_response::builder::ActionResponseBuilder;
use crate::common::model::{length_padded_to_num, BASE_VARIABLE_DATUM_LENGTH, FIXED_DATUM_LENGTH};
use crate::common::model::{EntityId, FixedDatum, PduBody, VariableDatum};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{PduType, RequestStatus};

pub const BASE_ACTION_RESPONSE_BODY_LENGTH: u16 = 28;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionResponse {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub request_status: RequestStatus,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl ActionResponse {
    pub fn builder() -> ActionResponseBuilder {
        ActionResponseBuilder::new()
    }

    pub fn into_builder(self) -> ActionResponseBuilder {
        ActionResponseBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ActionResponse(self)
    }
}

impl BodyInfo for ActionResponse {
    fn body_length(&self) -> u16 {
        BASE_ACTION_RESPONSE_BODY_LENGTH
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
        PduType::ActionResponse
    }
}

impl Interaction for ActionResponse {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
