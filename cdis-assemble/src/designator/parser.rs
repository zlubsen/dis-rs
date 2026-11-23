use crate::constants::{FOUR_BITS, ONE_BIT, SIXTEEN_BITS, TWO_BITS};
use crate::designator::model::{
    DesignatedEntityIdAndSpotLocationWrtEntity, Designator, DesignatorDetails,
    DesignatorFieldsPresentFlags, DesignatorUnits, EntityDrAndLinearAcceleration,
};
use crate::parsing::{BitInput, parse_field_when_present};
use crate::records::parser::{
    entity_coordinate_vector, entity_identification, linear_acceleration, world_coordinates,
};
use crate::types::parser::{uvint16, uvint32};
use crate::{BodyProperties, CdisBody};
use dis_rs::enumerations::{DeadReckoningAlgorithm, DesignatorSystemName};
use nom::IResult;
use nom::bits::complete::take;

#[allow(clippy::redundant_closure)]
pub(crate) fn designator_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    // Fields Present Flags
    let (input, fields_present): (BitInput, u8) = take(FOUR_BITS)(input)?;

    // Units
    let (input, units): (BitInput, u8) = take(TWO_BITS)(input)?;
    let units = DesignatorUnits::from(units);

    // Full Update Flag
    let (input, full_update_flag): (BitInput, u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    // Designating Entity
    let (input, designating_entity_id) = entity_identification(input)?;

    // Code Name
    let (input, code_name): (BitInput, Option<u16>) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresentFlags::DESIGNATOR_DETAILS.bits(),
        take(SIXTEEN_BITS),
    )(input)?;
    let code_name = code_name.map(|code| DesignatorSystemName::from(code));

    // Designated Entity
    let (input, designated_entity_id) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresentFlags::DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY.bits(),
        entity_identification,
    )(input)?;

    // Designator - Code
    let (input, designator_code) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresentFlags::DESIGNATOR_DETAILS.bits(),
        uvint16,
    )(input)?;
    // Designator - Power
    let (input, designator_power) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresentFlags::DESIGNATOR_DETAILS.bits(),
        uvint32,
    )(input)?;
    // Designator - Wavelength
    let (input, designator_wavelength) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresentFlags::DESIGNATOR_DETAILS.bits(),
        uvint32,
    )(input)?;

    // Designator Spot Location wrt Designated Entity
    let (input, spot_wrt_designated_entity) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresentFlags::DESIGNATED_ENTITY_ID_AND_SPOT_LOCATION_WRT_ENTITY.bits(),
        entity_coordinate_vector,
    )(input)?;

    // Designator Spot Location (World Coordinates)
    let (input, designator_spot_location) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresentFlags::DESIGNATOR_SPOT_LOCATION.bits(),
        world_coordinates,
    )(input)?;

    // Dead Reckoning Algorithm
    let (input, dr_algorithm): (BitInput, Option<u8>) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresentFlags::ENTITY_DR_AND_LINEAR_ACCELERATION.bits(),
        take(FOUR_BITS),
    )(input)?;
    let dr_algorithm = dr_algorithm.map(|algo| DeadReckoningAlgorithm::from(algo));

    // Entity Linear Acceleration
    let (input, dr_entity_linear_acceleration) = parse_field_when_present(
        fields_present,
        DesignatorFieldsPresentFlags::ENTITY_DR_AND_LINEAR_ACCELERATION.bits(),
        linear_acceleration,
    )(input)?;

    Ok((
        input,
        Designator {
            units,
            full_update_flag,
            designating_entity_id,
            designated_entity_id_and_spot_location_wrt_entity:
                DesignatedEntityIdAndSpotLocationWrtEntity::new(
                    designated_entity_id,
                    spot_wrt_designated_entity,
                ),
            designator_details: DesignatorDetails::new(
                code_name,
                designator_code,
                designator_power,
                designator_wavelength,
            ),
            designator_spot_location,
            entity_dr_and_linear_acceleration: EntityDrAndLinearAcceleration::new(
                dr_algorithm,
                dr_entity_linear_acceleration,
            ),
        }
        .into_cdis_body(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::CdisBody;
    use crate::designator::model::DesignatorUnits;
    use crate::designator::parser::designator_body;
    use crate::records::model::{EntityId, UnitsDekameters, UnitsMeters};
    use crate::types::model::UVINT16;

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
                    location_wrt_entity: UnitsMeters::Centimeter,
                    world_location_altitude: UnitsDekameters::Centimeter,
                }
            );
            assert!(!designator.full_update_flag);
            assert_eq!(
                designator.designating_entity_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(1))
            );
            assert!(
                designator
                    .designated_entity_id_and_spot_location_wrt_entity
                    .is_none()
            );
            assert!(designator.designator_details.is_none());
            assert!(designator.designator_spot_location.is_none());
            assert!(designator.entity_dr_and_linear_acceleration.is_none());
        } else {
            panic!()
        }
    }
}
