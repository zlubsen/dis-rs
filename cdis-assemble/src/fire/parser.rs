use crate::constants::{EIGHT_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS};
use crate::fire::model::{Fire, FireFieldsPresent};
use crate::parsing::BitInput;
use crate::records::model::UnitsDekameters;
use crate::records::parser::{
    entity_identification, entity_type, linear_velocity, world_coordinates,
};
use crate::types::parser::uvint32;
use crate::{parsing, BodyProperties, CdisBody};
use nom::bits::complete::take;
use nom::IResult;

#[allow(clippy::redundant_closure)]
pub(crate) fn fire_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, units): (BitInput, u8) = take(ONE_BIT)(input)?;
    let units = UnitsDekameters::from(units);

    let (input, firing_entity_id) = entity_identification(input)?;
    let (input, target_entity_id) = entity_identification(input)?;
    let (input, munition_expandable_entity_id) = entity_identification(input)?;
    let (input, event_id) = entity_identification(input)?;

    let (input, fire_mission_index) = parsing::parse_field_when_present(
        fields_present,
        FireFieldsPresent::FIRE_MISSION_INDEX_BIT,
        uvint32,
    )(input)?;

    let (input, location_world_coordinates) = world_coordinates(input)?;
    let (input, descriptor_entity_type) = entity_type(input)?;
    let (input, descriptor_warhead) = parsing::parse_field_when_present(
        fields_present,
        FireFieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT,
        take(SIXTEEN_BITS),
    )(input)?;
    let (input, descriptor_fuze) = parsing::parse_field_when_present(
        fields_present,
        FireFieldsPresent::DESCRIPTOR_WARHEAD_FUZE_BIT,
        take(SIXTEEN_BITS),
    )(input)?;
    let (input, descriptor_quantity) = parsing::parse_field_when_present(
        fields_present,
        FireFieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT,
        take(EIGHT_BITS),
    )(input)?;
    let (input, descriptor_rate) = parsing::parse_field_when_present(
        fields_present,
        FireFieldsPresent::DESCRIPTOR_QUANTITY_RATE_BIT,
        take(EIGHT_BITS),
    )(input)?;

    let (input, velocity) = linear_velocity(input)?;

    let (input, range) =
        parsing::parse_field_when_present(fields_present, FireFieldsPresent::RANGE_BIT, uvint32)(
            input,
        )?;

    Ok((
        input,
        Fire {
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
            range,
        }
        .into_cdis_body(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::fire::parser::fire_body;
    use crate::records::model::{EntityId, EntityType, LinearVelocity, UnitsDekameters};
    use crate::types::model::{SVINT16, SVINT24, UVINT16, UVINT8};
    use crate::CdisBody;

    #[test]
    fn parse_fire_no_fields_present() {
        #[rustfmt::skip]
        #[allow(clippy::unusual_byte_groupings)]
        #[allow(clippy::unreadable_literal)]
        let input = [
            0b0000_1_000,
            0b0000001_0,
            0b00000000,
            0b1_0000000,
            0b001_00000,
            0b00010_000,
            0b0000010_0,
            0b00000001,
            0b0_0000000,
            0b001_00000,
            0b00001_000,
            0b0000010_0,
            0b00000000,
            0b1_0000000,
            0b001_00000,
            0b00011_000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b0000_0000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b0000_0000,
            0b00000000,
            0b000001_00,
            0b10_0010_00,
            0b0000000_0,
            0b0000_0000,
            0b0_00000_00,
            0b000_00000,
            0b00001_000,
            0b0000001_0,
            0b00000000,
            0b1_0000000,
        ];
        // fields               ^fl ^u^ entityid                                       ^ entityid                                       ^ entityid                                   ^ eventid                                        ^ location                                                                                                                  ^ entity_type                                                   ^ velocity                                       ^ remainder
        // bits                 ^ 4 ^1^ 3x 10                                          ^ 3x 10                                          ^ 3x 10                                      ^ 3x 10                                          ^ 31,32,18                                                                                                                  ^ 4 + 4 + 9 + 5 + 5 + 5 + 5   ; (no descriptor 16 + 16 + 8 + 8) ^ 3x 10                                          ^
        // values               ^ 0 ^1^ 1, 1, 1                                        ^ 2, 2, 2                                        ^ 1, 1, 2                                    ^ 1, 1, 3                                        ^ 1,1,1                                                                                                                     ^ 2, 2, 0, 0, 0, 0, 0                                           ^ 1, 1, 1                                        ^
        let ((_input, cursor), body) = fire_body((&input, 0)).unwrap();

        assert_eq!(cursor, 1); // cursor position in last byte of input
        if let CdisBody::Fire(fire) = body {
            assert_eq!(fire.units, UnitsDekameters::Dekameter);
            assert_eq!(
                fire.firing_entity_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(1))
            );
            assert_eq!(
                fire.target_entity_id,
                EntityId::new(UVINT16::from(2), UVINT16::from(2), UVINT16::from(2))
            );
            assert_eq!(
                fire.munition_expandable_entity_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(2))
            );
            assert_eq!(
                fire.event_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(3))
            );
            assert_eq!(fire.location_world_coordinates.latitude, 0.0);
            assert_eq!(fire.location_world_coordinates.longitude, 0.0);
            assert_eq!(
                fire.location_world_coordinates.altitude_msl,
                SVINT24::from(1)
            );
            assert_eq!(
                fire.descriptor_entity_type,
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
            assert_eq!(
                fire.velocity,
                LinearVelocity::new(SVINT16::from(1), SVINT16::from(1), SVINT16::from(1))
            );
            assert!(fire.descriptor_warhead.is_none());
            assert!(fire.range.is_none());
        } else {
            panic!()
        }
    }
}
