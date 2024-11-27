use crate::comment_r::model::CommentR;
use crate::common::model::PduBody;
use crate::common::parser::{datum_specification, entity_id};
use nom::IResult;

pub(crate) fn comment_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, datums) = datum_specification(input)?;

    let body = CommentR::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_variable_datums(datums.variable_datum_records)
        .build();

    Ok((input, body.into_pdu_body()))
}
