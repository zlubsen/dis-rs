use crate::constants::TWO_BITS;
use crate::parsing::BitInput;
use crate::records::parser::{entity_identification, fixed_datum, variable_datum};
use crate::set_data::model::{SetData, SetDataFieldsPresent};
use crate::types::parser::{uvint32, uvint8};
use crate::{parsing, BodyProperties, CdisBody};
use dis_rs::model::DatumSpecification;
use nom::complete::take;
use nom::multi::count;
use nom::IResult;

pub(crate) fn set_data_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present): (BitInput, u8) = take(TWO_BITS)(input)?;

    let (input, originating_id) = entity_identification(input)?;
    let (input, receiving_id) = entity_identification(input)?;
    let (input, request_id) = uvint32(input)?;

    let (input, number_of_fixed_datums) = parsing::parse_field_when_present(
        fields_present,
        SetDataFieldsPresent::FIXED_DATUMS_BIT,
        uvint8,
    )(input)?;
    let number_of_fixed_datums = parsing::varint_to_type::<_, _, usize>(number_of_fixed_datums);
    let (input, number_of_var_datums) = parsing::parse_field_when_present(
        fields_present,
        SetDataFieldsPresent::VARIABLE_DATUMS_BIT,
        uvint8,
    )(input)?;
    let number_of_var_datums = parsing::varint_to_type::<_, _, usize>(number_of_var_datums);

    let (input, fixed_datums) = if let Some(num_datums) = number_of_fixed_datums {
        count(fixed_datum, num_datums)(input)?
    } else {
        (input, vec![])
    };
    let (input, variable_datums) = if let Some(num_datums) = number_of_var_datums {
        count(variable_datum, num_datums)(input)?
    } else {
        (input, vec![])
    };

    Ok((
        input,
        SetData {
            originating_id,
            receiving_id,
            request_id,
            datum_specification: DatumSpecification::new(fixed_datums, variable_datums),
        }
        .into_cdis_body(),
    ))
}
