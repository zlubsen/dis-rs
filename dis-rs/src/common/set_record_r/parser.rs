use nom::IResult;
use nom::number::complete::{be_u16, be_u32, be_u8};
use crate::common::model::{PduBody};
use crate::common::parser::{entity_id, record_specification};
use crate::enumerations::RequiredReliabilityService;
use crate::set_record_r::model::SetRecordR;

pub(crate) fn set_record_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, required_reliability_service) = be_u8(input)?;
    let required_reliability_service = RequiredReliabilityService::from(required_reliability_service);
    let (input, _padding) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, _padding) = be_u32(input)?;
    let (input, record_specification) = record_specification(input)?;

    let body = SetRecordR::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id)
        .with_required_reliability_service(required_reliability_service)
        .with_record_specification(record_specification)
        .build();

    Ok((input, body.into_pdu_body()))
}