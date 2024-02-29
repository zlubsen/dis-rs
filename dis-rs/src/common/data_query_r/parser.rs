use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_u16, be_u32, be_u8};
use crate::common::parser::entity_id;
use crate::common::model::PduBody;
use crate::data_query_r::model::DataQueryR;
use crate::enumerations::{RequiredReliabilityService, VariableRecordType};

pub(crate) fn data_query_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, required_reliability_service) = be_u8(input)?;
    let required_reliability_service = RequiredReliabilityService::from(required_reliability_service);
    let (input, _padding) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, time_interval) = be_u32(input)?;

    let (input, num_of_fixed_datums) = be_u32(input)?;
    let (input, num_of_variable_datums) = be_u32(input)?;
    let (input, fixed_datum_ids) = count(be_u32, num_of_fixed_datums as usize)(input)?;
    let fixed_datum_ids = fixed_datum_ids.iter().map(|id| VariableRecordType::from(*id)).collect();
    let (input, variable_datum_ids) = count(be_u32, num_of_variable_datums as usize)(input)?;
    let variable_datum_ids = variable_datum_ids.iter().map(|id| VariableRecordType::from(*id)).collect();

    let body = DataQueryR::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_required_reliability_service(required_reliability_service)
        .with_request_id(request_id)
        .with_time_interval(time_interval)
        .with_fixed_datums(fixed_datum_ids)
        .with_variable_datums(variable_datum_ids)
        .build();

    Ok((input, body.into_pdu_body()))
}