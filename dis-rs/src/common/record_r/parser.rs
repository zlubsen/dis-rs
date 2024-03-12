use nom::IResult;
use nom::number::complete::{be_u32, be_u8};
use crate::common::model::PduBody;
use crate::common::parser::{entity_id, record_specification};
use crate::enumerations::{EventType, RequiredReliabilityService};
use crate::record_r::model::RecordR;

pub(crate) fn record_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, required_reliability_service) = be_u8(input)?;
    let required_reliability_service = RequiredReliabilityService::from(required_reliability_service);
    let (input, _padding) = be_u8(input)?;
    let (input, event_type) = be_u32(input)?;
    let event_type = EventType::from(event_type);
    let (input, response_serial_number) = be_u32(input)?;
    let (input, record_specification) = record_specification(input)?;

    let body = RecordR::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id)
        .with_required_reliability_service(required_reliability_service)
        .with_event_type(event_type)
        .with_response_serial_number(response_serial_number)
        .with_record_specification(record_specification)
        .build();

    Ok((input, body.into_pdu_body()))
}