use crate::action_response_r::builder::ActionResponseRBuilder;
use crate::common::model::{length_padded_to_num, BASE_VARIABLE_DATUM_LENGTH, FIXED_DATUM_LENGTH};
use crate::common::model::{EntityId, FixedDatum, PduBody, VariableDatum};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{PduType, RequestStatus};

pub const BASE_ACTION_RESPONSE_R_BODY_LENGTH: u16 = 28;

/// 5.12.4.7 Action Request-R PDU
///
/// 7.11.8 Action Response-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionResponseR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub request_status: RequestStatus,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl ActionResponseR {
    pub fn builder() -> ActionResponseRBuilder {
        ActionResponseRBuilder::new()
    }

    pub fn into_builder(self) -> ActionResponseRBuilder {
        ActionResponseRBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ActionResponseR(self)
    }
}

impl BodyInfo for ActionResponseR {
    fn body_length(&self) -> u16 {
        BASE_ACTION_RESPONSE_R_BODY_LENGTH
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
        PduType::ActionResponseR
    }
}

impl Interaction for ActionResponseR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
