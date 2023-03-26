use nom::IResult;
use nom::number::complete::{be_u16, be_u32};
use crate::common::parser::entity_id;
use crate::{AcknowledgeFlag, PduBody, ResponseFlag};
use crate::common::action_request::model::ActionRequest;

pub fn action_request_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, acknowledge_flag) = be_u16(input)?;
    let acknowledge_flag = AcknowledgeFlag::from(acknowledge_flag);
    let (input, response_flag) = be_u16(input)?;
    let response_flag = ResponseFlag::from(response_flag);
    let (input, request_id) = be_u32(input)?;

    let body = ActionRequest::new()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id);

    Ok((input, body.into_pdu_body()))
}