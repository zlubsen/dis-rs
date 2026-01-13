use crate::common::data_query::model::DataQuery;
use crate::common::model::PduBody;
use crate::common::parser::entity_id;
use crate::enumerations::VariableRecordType;
use crate::BodyRaw;
use nom::multi::count;
use nom::number::complete::be_u32;
use nom::{IResult, Parser};

pub(crate) fn data_query_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, time_interval) = be_u32(input)?;

    let (input, num_of_fixed_datums) = be_u32(input)?;
    let (input, num_of_variable_datums) = be_u32(input)?;
    let (input, fixed_datum_ids) = count(be_u32, num_of_fixed_datums as usize).parse(input)?;
    let fixed_datum_ids = fixed_datum_ids
        .iter()
        .map(|id| VariableRecordType::from(*id))
        .collect();
    let (input, variable_datum_ids) =
        count(be_u32, num_of_variable_datums as usize).parse(input)?;
    let variable_datum_ids = variable_datum_ids
        .iter()
        .map(|id| VariableRecordType::from(*id))
        .collect();

    let body = DataQuery::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id)
        .with_time_interval(time_interval)
        .with_fixed_datums(fixed_datum_ids)
        .with_variable_datums(variable_datum_ids)
        .build();

    Ok((input, body.into_pdu_body()))
}
