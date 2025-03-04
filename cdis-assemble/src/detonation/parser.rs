use crate::constants::{EIGHT_BITS, FOUR_BITS, SIXTEEN_BITS, TWO_BITS};
use crate::detonation::model::{
    Detonation, DetonationFieldsPresent, DetonationUnits, ExplosiveForceFloat,
};
use crate::parsing::BitInput;
use crate::records::parser::{
    entity_coordinate_vector, entity_identification, entity_type, linear_velocity,
    variable_parameter, world_coordinates,
};
use crate::types::model::CdisFloat;
use crate::types::parser::{uvint16, uvint8};
use crate::{parsing, BodyProperties, CdisBody};
use nom::bits::complete::take;
use nom::multi::count;
use nom::{IResult, Parser};

pub(crate) fn detonation_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, units): (BitInput, u8) = take(TWO_BITS)(input)?;
    let units = DetonationUnits::from(units);

    let (input, source_entity_id) = entity_identification(input)?;
    let (input, target_entity_id) = entity_identification(input)?;
    let (input, exploding_entity_id) = entity_identification(input)?;
    let (input, event_id) = entity_identification(input)?;

    let (input, entity_linear_velocity) = linear_velocity(input)?;
    let (input, location_in_world_coordinates) = world_coordinates(input)?;

    let (input, descriptor_entity_type) = entity_type(input)?;
    let (input, descriptor_warhead) = parsing::parse_field_when_present(
        fields_present,
        DetonationFieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT,
        take(SIXTEEN_BITS),
    )(input)?;
    let (input, descriptor_fuze) = parsing::parse_field_when_present(
        fields_present,
        DetonationFieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT,
        take(SIXTEEN_BITS),
    )(input)?;
    let (input, descriptor_quantity) = parsing::parse_field_when_present(
        fields_present,
        DetonationFieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT,
        take(EIGHT_BITS),
    )(input)?;
    let (input, descriptor_rate) = parsing::parse_field_when_present(
        fields_present,
        DetonationFieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT,
        take(EIGHT_BITS),
    )(input)?;
    let (input, descriptor_explosive_material) = parsing::parse_field_when_present(
        fields_present,
        DetonationFieldsPresent::DESCRIPTOR_EXPLOSIVE_BIT,
        uvint16,
    )(input)?;
    let (input, descriptor_explosive_force) = parsing::parse_field_when_present(
        fields_present,
        DetonationFieldsPresent::DESCRIPTOR_EXPLOSIVE_BIT,
        ExplosiveForceFloat::parse,
    )(input)?;

    let (input, location_in_entity_coordinates) = entity_coordinate_vector(input)?;
    let (input, detonation_results) = uvint8(input)?;

    let (input, number_of_var_params) = parsing::parse_field_when_present(
        fields_present,
        DetonationFieldsPresent::VARIABLE_PARAMETERS_BIT,
        take(EIGHT_BITS),
    )(input)?;

    let (input, variable_parameters) = if let Some(num_params) = number_of_var_params {
        count(variable_parameter, num_params).parse(input)?
    } else {
        (input, vec![])
    };

    Ok((
        input,
        Detonation {
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
            descriptor_explosive_material,
            descriptor_explosive_force,
            location_in_entity_coordinates,
            detonation_results,
            variable_parameters,
        }
        .into_cdis_body(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::detonation::parser::detonation_body;
    use crate::records::model::{
        EntityCoordinateVector, EntityId, EntityType, LinearVelocity, UnitsDekameters, UnitsMeters,
    };
    use crate::types::model::{SVINT16, SVINT24, UVINT16, UVINT8};
    use crate::CdisBody;
    use dis_rs::enumerations::DetonationResult;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn parse_detonation_no_fields_present() {
        // TODO added explosive descriptor field present flag - update test when standardized, or remove this comment
        #[rustfmt::skip]
        #[allow(clippy::unusual_byte_groupings)]
        #[allow(clippy::unreadable_literal)]
        let input: [u8; 39] = [
            0b0000_10_00,
            0b00000001_,
            0b00000000,
            0b01_000000,
            0b0001_0000,
            0b000010_00,
            0b00000010_,
            0b00000000,
            0b10_000000,
            0b0001_0000,
            0b000001_00,
            0b00000010_,
            0b00000000,
            0b01_000000,
            0b0001_0000,
            0b000011_00,
            0b00000001_,
            0b00000000,
            0b01_000000,
            0b0001_0000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b000_00000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b000_00000,
            0b00000000,
            0b00001_001,
            0b0_0010_000,
            0b000000_00,
            0b000_00000_,
            0b00000_000,
            0b00_000000,
            0b0000_0000,
            0b000000_00,
            0b00000000_,
            0b00101_000,
        ];
        // fields                 ^fl ^u ^ entityid                                       ^ entityid                                       ^ entityid                                   ^ eventid                                        ^ velocity 1,1,1                                 ^ world location                                                                                                            ^ entity type                                                   ^ entity location                            ^ results ^ remainder
        // bits                   ^4  ^2 ^ 3x 10                                          ^ 3x 10                                          ^ 3x 10                                      ^ 3x 10                                          ^ 3x 10                                          ^ 31,32,18                                                                                                                  ^ 4,4,9,5,5,5,5                                                 ^ 3x 10                                      ^ 5       ^
        // values                 ^0  ^1 ^ 1,1,1                                          ^ 2,2,2                                          ^ 1,1,2                                      ^ 1,1,3                                          ^ 1,1,1                                          ^ 0 0 0                                                                                                                     ^ 2,2,0,0,0,0,0                                                 ^ 0 0 0                                      ^ 5       ^

        let ((_input, cursor), body) = detonation_body((&input, 0)).unwrap();

        assert_eq!(cursor, 5); // cursor position in last byte of input
        if let CdisBody::Detonation(detonation) = body {
            assert_eq!(
                detonation.units.world_location_altitude,
                UnitsDekameters::Dekameter
            );
            assert_eq!(
                detonation.units.location_entity_coordinates,
                UnitsMeters::Centimeter
            );

            assert_eq!(
                detonation.source_entity_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(1))
            );
            assert_eq!(
                detonation.target_entity_id,
                EntityId::new(UVINT16::from(2), UVINT16::from(2), UVINT16::from(2))
            );
            assert_eq!(
                detonation.exploding_entity_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(2))
            );
            assert_eq!(
                detonation.event_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(3))
            );
            assert_eq!(
                detonation.entity_linear_velocity,
                LinearVelocity::new(SVINT16::from(1), SVINT16::from(1), SVINT16::from(1))
            );
            assert_eq!(detonation.location_in_world_coordinates.latitude, 0.0);
            assert_eq!(detonation.location_in_world_coordinates.longitude, 0.0);
            assert_eq!(
                detonation.location_in_world_coordinates.altitude_msl,
                SVINT24::from(1)
            );
            assert_eq!(
                detonation.descriptor_entity_type,
                EntityType::new(
                    2,
                    2,
                    0,
                    UVINT8::from(0),
                    UVINT8::from(0),
                    UVINT8::from(0),
                    UVINT8::from(0)
                )
            );
            assert!(detonation.descriptor_warhead.is_none());
            assert!(detonation.descriptor_quantity.is_none());
            assert_eq!(
                detonation.location_in_entity_coordinates,
                EntityCoordinateVector::new(SVINT16::from(0), SVINT16::from(0), SVINT16::from(0))
            );
            assert_eq!(
                detonation.detonation_results,
                UVINT8::from(u8::from(DetonationResult::Detonation))
            );
        } else {
            panic!()
        }
    }
}
