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

#[cfg(test)]
mod tests {
    use crate::CdisBody;
    use crate::fire::parser::fire_body;
    use crate::records::model::{EntityId, EntityType, LinearVelocity, UnitsDekameters};
    use crate::types::model::{SVINT16, SVINT24, UVINT16, UVINT8};

    #[test]
    fn parse_detonation_no_fields_present() {
        // let input = [0b0000_1_000, 0b0000001_0, 0b00000000, 0b1_0000000, 0b001_00000, 0b00010_000, 0b0000010_0, 0b00000001, 0b0_0000000, 0b001_00000, 0b00001_000, 0b0000010_0, 0b00000000, 0b1_0000000, 0b001_00000, 0b00011_000, 0b00000000, 0b00000000, 0b00000000, 0b0000_0000, 0b00000000, 0b00000000, 0b00000000, 0b0000_0000, 0b00000000, 0b000001_00, 0b10_0010_00, 0b0000000_0, 0b0000_0000, 0b0_00000_00, 0b000_00000, 0b00001_000, 0b0000001_0, 0b00000000, 0b1_0000000];
        // //                      ^ fl ^u^ entityid                                       ^ entityid                                       ^ entityid                                   ^ eventid                                        ^ location                                    ^31                                              ^32                          ^ entity_type                                                   ^ velocity                                       ^ remainder
        // //                      flags 4; units 1; entity/event ids 12x ten bits; location: 31 + 32 + 18; entity_type 4 + 4 + 9 + 5 + 5 + 5 + 5; (no descriptor 16 + 16 + 8 + 8); velocity 10 + 10 + 10; (no range)
        // //                      0       ; 1     ; 1, 1, 1, 2, 2, 2, 1, 1, 2, 1, 1, 3 ; 1, 1, 1               ; 2, 2, 0, 0, 0, 0, 0               ;                                   ; 1, 1, 1           ;
        // let ((_input, cursor), body) = fire_body((&input, 0)).unwrap();
        //
        // assert_eq!(cursor, 1); // cursor position in last byte of input
        // if let CdisBody::Fire(fire) = body {
        //     assert_eq!(fire.units, UnitsDekameters::Dekameter);
        //     assert_eq!(fire.firing_entity_id, EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(1)));
        //     assert_eq!(fire.target_entity_id, EntityId::new(UVINT16::from(2), UVINT16::from(2), UVINT16::from(2)));
        //     assert_eq!(fire.munition_expandable_entity_id, EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(2)));
        //     assert_eq!(fire.event_id, EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(3)));
        //     assert_eq!(fire.location_world_coordinates.latitude, 0.0);
        //     assert_eq!(fire.location_world_coordinates.longitude, 0.0);
        //     assert_eq!(fire.location_world_coordinates.altitude_msl, SVINT24::from(1));
        //     assert_eq!(fire.descriptor_entity_type,
        //                EntityType::new(2, 2, 0,
        //                                UVINT8::from(0), UVINT8::from(0), UVINT8::from(0), UVINT8::from(0)));
        //     assert_eq!(fire.velocity, LinearVelocity::new(SVINT16::from(1), SVINT16::from(1), SVINT16::from(1)));
        //     assert!(fire.descriptor_warhead.is_none());
        //     assert!(fire.range.is_none());
        // } else { assert!(false) }
    }
}