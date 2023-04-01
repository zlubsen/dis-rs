use nom::IResult;
use crate::common::parser::{datum_specification, entity_id};
use crate::common::comment::model::Comment;
use crate::common::model::PduBody;

pub fn comment_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, datums) = datum_specification(input)?;

    let body = Comment::new()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_variable_datums(datums.variable_datum_records);

    Ok((input, body.into_pdu_body()))
}