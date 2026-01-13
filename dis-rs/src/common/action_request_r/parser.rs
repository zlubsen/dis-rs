use crate::action_request_r::model::ActionRequestR;
use crate::common::model::PduBody;
use crate::common::parser::{datum_specification, entity_id};
use crate::enumerations::{ActionId, RequiredReliabilityService};
use crate::BodyRaw;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::IResult;

pub(crate) fn action_request_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, required_reliability_service) = be_u8(input)?;
    let required_reliability_service =
        RequiredReliabilityService::from(required_reliability_service);
    let (input, _padding) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, action_id) = be_u32(input)?;
    let action_id = ActionId::from(action_id);
    let (input, _padding) = be_u32(input)?;
    let (input, datums) = datum_specification(input)?;

    let body = ActionRequestR::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_required_reliability_service(required_reliability_service)
        .with_request_id(request_id)
        .with_action_id(action_id)
        .with_fixed_datums(datums.fixed_datum_records)
        .with_variable_datums(datums.variable_datum_records)
        .build();

    Ok((input, body.into_pdu_body()))
}
