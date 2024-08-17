use nom::complete::take;
use nom::IResult;
use dis_rs::enumerations::{DeadReckoningAlgorithm, DesignatorSystemName};
use crate::{BodyProperties, CdisBody, parsing};
use crate::constants::{FOUR_BITS, ONE_BIT, SIXTEEN_BITS, TWO_BITS};
use crate::designator::model::{Designator, DesignatorFieldsPresent, DesignatorUnits};
use crate::parsing::{BitInput, parse_field_when_present};
use crate::records::model::{UnitsDekameters, UnitsMeters};
use crate::records::parser::{entity_coordinate_vector, entity_identification, linear_acceleration, world_coordinates};
use crate::types::parser::{uvint16, uvint32, uvint8};

#[allow(clippy::redundant_closure)]
pub(crate) fn designator_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present) : (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, units) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let units = DesignatorUnits::from(units);
    let (input, full_update_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    let (input, designating_entity_id) = entity_identification(input)?;

    let (input, code_name) : (BitInput, Option<u16>) = parse_field_when_present(
        fields_present, DesignatorFieldsPresent::DESIGNATOR_DETAILS_BIT, take(SIXTEEN_BITS))(input)?;
    let code_name = code_name.map(|code| DesignatorSystemName::from(code));
    let (input, designated_entity_id) = parse_field_when_present(
        fields_present, DesignatorFieldsPresent::DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY_BIT, entity_identification)(input)?;

    let (input, designator_code) = parse_field_when_present(
        fields_present, DesignatorFieldsPresent::DESIGNATOR_DETAILS_BIT, uvint16)(input)?;
    let (input, designator_power) = parse_field_when_present(
        fields_present, DesignatorFieldsPresent::DESIGNATOR_DETAILS_BIT, uvint32)(input)?;
    let (input, designator_wavelength) = parse_field_when_present(
        fields_present, DesignatorFieldsPresent::DESIGNATOR_DETAILS_BIT, uvint32)(input)?;

    let (input, spot_wrt_designated_entity) = parse_field_when_present(
        fields_present, DesignatorFieldsPresent::DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY_BIT, entity_coordinate_vector)(input)?;

    let (input, designator_spot_location) = parse_field_when_present(
        fields_present, DesignatorFieldsPresent::DESIGNATOR_SPOT_LOCATION_BIT, world_coordinates)(input)?;

    let (input, dr_algorithm) : (BitInput, Option<u8>) = parse_field_when_present(
        fields_present, DesignatorFieldsPresent::ENTITY_DR_AND_LINEAR_ACCELERATION_BIT, take(FOUR_BITS))(input)?;
    let dr_algorithm = dr_algorithm.map(|algo| DeadReckoningAlgorithm::from(algo));

    let (input, dr_entity_linear_acceleration) = parse_field_when_present(
        fields_present, DesignatorFieldsPresent::ENTITY_DR_AND_LINEAR_ACCELERATION_BIT, linear_acceleration)(input)?;

    Ok((input, Designator {
        units,
        full_update_flag,
        designating_entity_id,
        code_name,
        designated_entity_id,
        designator_code,
        designator_power,
        designator_wavelength,
        spot_wrt_designated_entity,
        designator_spot_location,
        dr_algorithm,
        dr_entity_linear_acceleration,
    }.into_cdis_body()))
}