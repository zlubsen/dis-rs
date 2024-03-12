use nom::IResult;
use nom::number::complete::be_u32;
use crate::common::parser::{datum_specification, entity_id};
use crate::common::set_data::model::SetData;
use crate::common::model::PduBody;

pub(crate) fn set_data_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, _padding) = be_u32(input)?;
    let (input, datums) = datum_specification(input)?;

    let body = SetData::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id)
        .with_fixed_datums(datums.fixed_datum_records)
        .with_variable_datums(datums.variable_datum_records)
        .build();

    Ok((input, body.into_pdu_body()))
}