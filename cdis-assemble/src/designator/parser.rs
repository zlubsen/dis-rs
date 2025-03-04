use crate::constants::{FOUR_BITS, ONE_BIT, SIXTEEN_BITS, TWO_BITS};
use crate::designator::model::{Designator, DesignatorFieldsPresent, DesignatorUnits};
use crate::parsing::{parse_field_when_present, BitInput};
use crate::records::parser::{
    entity_coordinate_vector, entity_identification, linear_acceleration, world_coordinates,
};
use crate::types::parser::{uvint16, uvint32};
use crate::{BodyProperties, CdisBody};
use dis_rs::enumerations::{DeadReckoningAlgorithm, DesignatorSystemName};
use nom::bits::complete::take;
use nom::IResult;

#[allow(clippy::redundant_closure)]
pub(crate) fn designator_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, units): (BitInput, u8) = take(TWO_BITS)(input)?;
    let units = DesignatorUnits::from(units);
    let (input, full_update_flag): (BitInput, u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    let (input, designating_entity_id) = entity_identification(input)?;

    let (input, code_name): (BitInput, Option<u16>) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresent::DESIGNATOR_DETAILS_BIT,
        take(SIXTEEN_BITS),
    )(input)?;
    let code_name = code_name.map(|code| DesignatorSystemName::from(code));
    let (input, designated_entity_id) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresent::DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY_BIT,
        entity_identification,
    )(input)?;

    let (input, designator_code) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresent::DESIGNATOR_DETAILS_BIT,
        uvint16,
    )(input)?;
    let (input, designator_power) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresent::DESIGNATOR_DETAILS_BIT,
        uvint32,
    )(input)?;
    let (input, designator_wavelength) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresent::DESIGNATOR_DETAILS_BIT,
        uvint32,
    )(input)?;

    let (input, spot_wrt_designated_entity) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresent::DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY_BIT,
        entity_coordinate_vector,
    )(input)?;

    let (input, designator_spot_location) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresent::DESIGNATOR_SPOT_LOCATION_BIT,
        world_coordinates,
    )(input)?;

    let (input, dr_algorithm): (BitInput, Option<u8>) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresent::ENTITY_DR_AND_LINEAR_ACCELERATION_BIT,
        take(FOUR_BITS),
    )(input)?;
    let dr_algorithm = dr_algorithm.map(|algo| DeadReckoningAlgorithm::from(algo));

    let (input, dr_entity_linear_acceleration) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresent::ENTITY_DR_AND_LINEAR_ACCELERATION_BIT,
        linear_acceleration,
    )(input)?;

    Ok((
        input,
        Designator {
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
        }
        .into_cdis_body(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::designator::model::DesignatorUnits;
    use crate::designator::parser::designator_body;
    use crate::records::model::{EntityId, UnitsDekameters, UnitsMeters};
    use crate::types::model::UVINT16;
    use crate::CdisBody;

    #[test]
    fn parse_designator_no_fields_present() {
        #[rustfmt::skip]
        #[allow(clippy::unusual_byte_groupings)]
        #[allow(clippy::unreadable_literal)]
        let input = [
            0b0000_00_0_0,
            0b00000000,
            0b1_0000000,
            0b001_00000,
            0b00001_000,
        ];
        // fields              ^fl  ^u ^f^ entityid                                       ^ remainder
        // bits                ^ 4  ^2 ^f^ 3x 10                                          ^
        // values              ^ 0  ^0 ^f^ 1, 1, 1                                        ^
        let ((_input, cursor), body) = designator_body((&input, 0)).unwrap();

        assert_eq!(cursor, 5); // cursor position in last byte of input
        if let CdisBody::Designator(designator) = body {
            assert_eq!(
                designator.units,
                DesignatorUnits {
                    location_wrt_entity_units: UnitsMeters::Centimeter,
                    world_location_altitude: UnitsDekameters::Centimeter,
                }
            );
            assert!(!designator.full_update_flag);
            assert_eq!(
                designator.designating_entity_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(1))
            );
            assert!(designator.code_name.is_none());
            assert!(designator.designated_entity_id.is_none());
            assert!(designator.designator_code.is_none());
            assert!(designator.designator_power.is_none());
            assert!(designator.designator_wavelength.is_none());
            assert!(designator.spot_wrt_designated_entity.is_none());
            assert!(designator.designator_spot_location.is_none());
            assert!(designator.dr_algorithm.is_none());
            assert!(designator.dr_entity_linear_acceleration.is_none());
        } else {
            panic!()
        }
    }
}
