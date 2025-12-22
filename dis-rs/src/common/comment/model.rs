use crate::common::comment::builder::CommentBuilder;
use crate::common::model::{
    length_padded_to_num, EntityId, PduBody, VariableDatum, BASE_VARIABLE_DATUM_LENGTH,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::PduType;
use crate::BodyRaw;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const BASE_COMMENT_BODY_LENGTH: u16 = 20;

/// 5.6.5.13 Comment PDU
///
/// 7.5.13 Comment PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Comment {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl BodyRaw for Comment {
    type Builder = CommentBuilder;

    fn builder() -> CommentBuilder {
        CommentBuilder::new()
    }

    fn into_builder(self) -> CommentBuilder {
        CommentBuilder::new_from_body(self)
    }

    fn into_pdu_body(self) -> PduBody {
        PduBody::Comment(self)
    }
}

impl BodyInfo for Comment {
    fn body_length(&self) -> u16 {
        BASE_COMMENT_BODY_LENGTH
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
        PduType::Comment
    }
}

impl Interaction for Comment {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}

impl From<Comment> for PduBody {
    #[inline]
    fn from(value: Comment) -> Self {
        value.into_pdu_body()
    }
}
