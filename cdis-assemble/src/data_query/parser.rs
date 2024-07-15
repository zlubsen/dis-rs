use nom::complete::take;
use nom::IResult;
use nom::multi::count;
use dis_rs::enumerations::VariableRecordType;
use crate::{BodyProperties, CdisBody, parsing};
use crate::constants::{THIRTY_TWO_BITS, TWENTY_SIX_BITS, TWO_BITS};
use crate::data_query::model::{DataQuery, DataQueryFieldsPresent};
use crate::parsing::BitInput;
use crate::records::model::CdisTimeStamp;
use crate::records::parser::{entity_identification};
use crate::types::parser::{uvint32, uvint8};

pub(crate) fn data_query_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present) : (BitInput, u8) = take(TWO_BITS)(input)?;

    let (input, originating_id) = entity_identification(input)?;
    let (input, receiving_id) = entity_identification(input)?;
    let (input, request_id) = uvint32(input)?;
    let (input, time_interval) : (BitInput, u32) = take(TWENTY_SIX_BITS)(input)?;
    let time_interval = CdisTimeStamp::from(time_interval);

    let (input, number_of_fixed_datums) = parsing::parse_field_when_present(
        fields_present, DataQueryFieldsPresent::FIXED_DATUMS_BIT, uvint8)(input)?;
    let number_of_fixed_datums = parsing::varint_to_type::<_, _, usize>(number_of_fixed_datums);
    let (input, number_of_var_datums) = parsing::parse_field_when_present(
        fields_present, DataQueryFieldsPresent::VARIABLE_DATUMS_BIT, uvint8)(input)?;
    let number_of_var_datums = parsing::varint_to_type::<_, _, usize>(number_of_var_datums);

    let (input, fixed_datum_ids) : (BitInput, Vec<u32>) = if let Some(num_datums) = number_of_fixed_datums {
        count(take(THIRTY_TWO_BITS), num_datums)(input)?
    } else {
        (input, vec![])
    };
    let (input, variable_datum_ids) : (BitInput, Vec<u32>) = if let Some(num_datums) = number_of_var_datums {
        count(take(THIRTY_TWO_BITS), num_datums)(input)?
    } else {
        (input, vec![])
    };

    Ok((input, DataQuery {
        originating_id,
        receiving_id,
        request_id,
        time_interval,
        fixed_datum_ids: fixed_datum_ids.iter().map(|id| VariableRecordType::from(*id)).collect(),
        variable_datum_ids: variable_datum_ids.iter().map(|id| VariableRecordType::from(*id)).collect(),
    }.into_cdis_body()))
}