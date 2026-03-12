use crate::common::model::PduBody;
use crate::common::parser::entity_id;
use crate::enumerations::{RecordQueryREventType, RequiredReliabilityService, VariableRecordType};
use crate::model::TimeStamp;
use crate::record_query_r::model::{RecordQueryR, RecordQuerySpecification};
use crate::BodyRaw;
use nom::multi::count;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::{IResult, Parser};

pub(crate) fn record_query_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, required_reliability_service) = be_u8(input)?;
    let required_reliability_service =
        RequiredReliabilityService::from(required_reliability_service);
    let (input, _padding) = be_u8(input)?;
    let (input, event_type) = be_u16(input)?;
    let event_type = RecordQueryREventType::from(event_type);
    let (input, time) = be_u32(input)?;
    let time = TimeStamp::from(time);
    let (input, record_query_specification) = record_query_specification(input)?;

    let body = RecordQueryR::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id)
        .with_required_reliability_service(required_reliability_service)
        .with_event_type(event_type)
        .with_time(time)
        .with_record_query_specification(record_query_specification)
        .build();

    Ok((input, body.into_pdu_body()))
}

pub(crate) fn record_query_specification(input: &[u8]) -> IResult<&[u8], RecordQuerySpecification> {
    let (input, record_count) = be_u32(input)?;
    let (input, records) = count(be_u32, record_count as usize).parse(input)?;
    let records = records
        .iter()
        .map(|record| VariableRecordType::from(*record))
        .collect();

    Ok((
        input,
        RecordQuerySpecification::default().with_record_ids(records),
    ))
}
