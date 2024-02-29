use nom::IResult;
use crate::common::parser::{entity_id};
use crate::common::model::PduBody;
use crate::resupply_cancel::model::ResupplyCancel;

pub(crate) fn resupply_cancel_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, requesting_id) = entity_id(input)?;
    let (input, servicing_id) = entity_id(input)?;

    let body = ResupplyCancel::builder()
        .with_requesting_id(requesting_id)
        .with_servicing_id(servicing_id)
        .build();

    Ok((input, body.into_pdu_body()))
}