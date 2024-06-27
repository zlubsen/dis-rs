use nom::complete::take;
use nom::IResult;
use nom::multi::count;
use crate::{BodyProperties, CdisBody, parsing};
use crate::constants::{EIGHT_BITS, SIXTEEN_BITS, THREE_BITS, TWO_BITS};
use crate::detonation::model::{Detonation, DetonationFieldsPresent, DetonationUnits};
use crate::parsing::BitInput;
use crate::records::parser::{entity_coordinate_vector, entity_identification, entity_type, linear_velocity, variable_parameter, world_coordinates};
use crate::types::parser::uvint8;

pub(crate) fn detonation_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present) : (BitInput, u8) = take(THREE_BITS)(input)?;
    let (input, units) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let units = DetonationUnits::from(units);

    let (input, source_entity_id) = entity_identification(input)?;
    let (input, target_entity_id) = entity_identification(input)?;
    let (input, exploding_entity_id) = entity_identification(input)?;
    let (input, event_id) = entity_identification(input)?;

    let (input, entity_linear_velocity) = linear_velocity(input)?;
    let (input, location_in_world_coordinates) = world_coordinates(input)?;

    let (input, descriptor_entity_type) = entity_type(input)?;
    let (input, descriptor_warhead) = parsing::parse_field_when_present(
        fields_present, DetonationFieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT, take(SIXTEEN_BITS))(input)?;
    let (input, descriptor_fuze) = parsing::parse_field_when_present(
        fields_present, DetonationFieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT, take(SIXTEEN_BITS))(input)?;
    let (input, descriptor_quantity) = parsing::parse_field_when_present(
        fields_present, DetonationFieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT, take(EIGHT_BITS))(input)?;
    let (input, descriptor_rate) = parsing::parse_field_when_present(
        fields_present, DetonationFieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT, take(EIGHT_BITS))(input)?;

    let (input, location_in_entity_coordinates) = entity_coordinate_vector(input)?;
    let (input, detonation_results) = uvint8(input)?;

    let (input, number_of_var_params) = parsing::parse_field_when_present(
        fields_present, DetonationFieldsPresent::VARIABLE_PARAMETERS_BIT, take(EIGHT_BITS))(input)?;

    let (input, variable_parameters) = if let Some(num_params) = number_of_var_params {
        count(variable_parameter, num_params)(input)?
    } else {
        (input, vec![])
    };

    Ok((input, Detonation {
        units,
        source_entity_id,
        target_entity_id,
        exploding_entity_id,
        event_id,
        entity_linear_velocity,
        location_in_world_coordinates,
        descriptor_entity_type,
        descriptor_warhead,
        descriptor_fuze,
        descriptor_quantity,
        descriptor_rate,
        location_in_entity_coordinates,
        detonation_results,
        variable_parameters,
    }.into_cdis_body()))
}