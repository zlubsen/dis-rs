use nom::complete::take;
use nom::IResult;
use crate::{BodyProperties, CdisBody, parsing};
use crate::constants::{EIGHT_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS};
use crate::fire::model::{Fire, FireFieldsPresent};
use crate::parsing::BitInput;
use crate::records::model::{Units};
use crate::records::parser::{entity_identification, entity_type, linear_velocity, world_coordinates};
use crate::types::parser::{uvint32};

#[allow(clippy::redundant_closure)]
pub(crate) fn fire_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present) : (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, units) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let units = Units::from(units);

    let (input, firing_entity_id) = entity_identification(input)?;
    let (input, target_entity_id) = entity_identification(input)?;
    let (input, munition_expandable_entity_id) = entity_identification(input)?;
    let (input, event_id) = entity_identification(input)?;

    let (input, fire_mission_index) = parsing::parse_field_when_present(
        fields_present, FireFieldsPresent::FIRE_MISSION_INDEX_BIT, uvint32)(input)?;

    let (input, location_world_coordinates) = world_coordinates(input)?;
    let (input, descriptor_entity_type) = entity_type(input)?;

    let (input, descriptor_warhead) = parsing::parse_field_when_present(
        fields_present, FireFieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT, take(SIXTEEN_BITS))(input)?;
    let (input, descriptor_fuze) = parsing::parse_field_when_present(
        fields_present, FireFieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT, take(SIXTEEN_BITS))(input)?;
    let (input, descriptor_quantity) = parsing::parse_field_when_present(
        fields_present, FireFieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT, take(EIGHT_BITS))(input)?;
    let (input, descriptor_rate) = parsing::parse_field_when_present(
        fields_present, FireFieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT, take(EIGHT_BITS))(input)?;

    let (input, velocity) = linear_velocity(input)?;

    let (input, range) = parsing::parse_field_when_present(
        fields_present, FireFieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT, uvint32)(input)?;

    Ok((input, Fire {
        units,
        firing_entity_id,
        target_entity_id,
        munition_expandable_entity_id,
        event_id,
        fire_mission_index,
        location_world_coordinates,
        descriptor_entity_type,
        descriptor_warhead,
        descriptor_fuze,
        descriptor_quantity,
        descriptor_rate,
        velocity,
        range
    }.into_cdis_body()))
}

#[cfg(test)]
mod tests {
    // use crate::CdisBody;
    // use crate::fire::parser::fire_body;
    // use crate::records::model::{EntityId, Units};
    // use crate::types::model::UVINT16;

    #[test]
    fn parse_fire_no_fields_present() {
        // let input = [0b00000000, 0b00000_1_0_0, 0b00000001, 0b1_0000000, 0b011_00000, 0b00011_000, 0b0_0000000];
        // let ((_input, cursor), body) = fire_body((&input, 0)).unwrap();
        //
        // // assert_eq!(cursor, 1); // cursor position in last byte of input
        //
        // if let CdisBody::Fire(fire) = body {
        //     assert_eq!(fire.units, Units::Dekameter);
        //     assert_eq!(fire.firing_entity_id, EntityId::new(UVINT16::from(3), UVINT16::from(3), UVINT16::from(3)));
        // }
        assert!(false)
    }
}