use nom::IResult;
use nom::number::complete::be_u32;
use crate::common::action_response::model::ActionResponse;
use crate::common::parser::{datum_specification, entity_id};
use crate::enumerations::{RequestStatus};
use crate::common::model::PduBody;

pub(crate) fn action_response_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, request_status) = be_u32(input)?;
    let request_status = RequestStatus::from(request_status);
    let (input, datums) = datum_specification(input)?;

    let body = ActionResponse::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id)
        .with_request_status(request_status)
        .with_fixed_datums(datums.fixed_datum_records)
        .with_variable_datums(datums.variable_datum_records)
        .build();

    Ok((input, body.into_pdu_body()))
}