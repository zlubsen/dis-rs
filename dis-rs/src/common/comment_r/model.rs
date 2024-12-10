use crate::comment_r::builder::CommentRBuilder;
use crate::common::model::{
    length_padded_to_num, EntityId, PduBody, VariableDatum, BASE_VARIABLE_DATUM_LENGTH,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::PduType;

const BASE_COMMENT_R_BODY_LENGTH: u16 = 20;

/// 5.12.4.13 Comment-R PDU
///
/// 7.11.13 Comment-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommentR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl CommentR {
    #[must_use]
    pub fn builder() -> CommentRBuilder {
        CommentRBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> CommentRBuilder {
        CommentRBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::CommentR(self)
    }
}

impl BodyInfo for CommentR {
    fn body_length(&self) -> u16 {
        BASE_COMMENT_R_BODY_LENGTH
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
        PduType::CommentR
    }
}

impl Interaction for CommentR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
