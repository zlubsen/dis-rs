use nom::IResult;
use nom::number::complete::{be_u16, be_u32, be_u8};
use crate::common::parser::entity_id;
use crate::common::model::PduBody;
use crate::common::create_entity_r::model::CreateEntityR;
use crate::enumerations::RequiredReliabilityService;

pub fn create_entity_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, required_reliability_service) = be_u8(input)?;
    let required_reliability_service = RequiredReliabilityService::from(required_reliability_service);
    let (input, _padding) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, request_id) = be_u32(input)?;

    let body = CreateEntityR::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_required_reliability_service(required_reliability_service)
        .with_request_id(request_id)
        .build();

    Ok((input, body.into_pdu_body()))
}