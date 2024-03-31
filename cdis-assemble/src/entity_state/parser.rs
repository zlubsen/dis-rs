use nom::bits::complete::take;
use nom::IResult;
use nom::multi::count;
use dis_rs::enumerations::{DeadReckoningAlgorithm, ForceId};
use crate::constants::{FOUR_BITS, HUNDRED_TWENTY_BITS, ONE_BIT, THIRTEEN_BITS, THIRTY_TWO_BITS};
use crate::entity_state::model::{CdisEntityAppearance, CdisEntityCapabilities, EntityState};
use crate::parser_utils;
use crate::parser_utils::BitInput;
use crate::records::model::Units;
use crate::records::parser::{angular_velocity, entity_identification, entity_marking, entity_type, linear_acceleration, linear_velocity, orientation, variable_parameter, world_coordinates};
use crate::types::parser::{uvint32, uvint8};

const FPF_BIT_FORCE_ID: u16 = 12; // 0x1000;
const FPF_BIT_VP: u16 = 11; // 0x0800;
const FPF_BIT_ENTITY_TYPE: u16 = 10; // 0x0400;
const FPF_BIT_ALT_ENTITY_TYPE: u16 = 9; // 0x0200;
const FPF_BIT_LIN_VELOCITY: u16 = 8; // 0x0100;
const FPF_BIT_ENTITY_LOCATION: u16 = 7; // 0x0080;
const FPF_BIT_ENTITY_ORIENTATION: u16 = 6; // 0x0040;
const FPF_BIT_ENTITY_APPEARANCE: u16 = 5; // 0x0020;
const FPF_BIT_DR_OTHER: u16 = 4; // 0x0010;
const FPF_BIT_DR_LIN_ACCELERATION: u16 = 3; // 0x0008;
const FPF_BIT_DR_ANG_VELOCITY: u16 = 2; // 0x0004;
const FPF_BIT_MARKING: u16 = 1; // 0x0002;
const FPF_BIT_CAPABILITIES: u16 = 0; // 0x0001;

pub(crate) fn entity_state_body(input: BitInput) -> IResult<BitInput, EntityState> {
    let (input, fields_present) : (BitInput, u16) = take(THIRTEEN_BITS)(input)?;
    let (input, units) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let units = Units::from(units);
    let (input, full_update_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    let (input, entity_id) = entity_identification(input)?;
    let (input, force_id) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_FORCE_ID, uvint8)(input)?;
    let force_id = parser_utils::varint_to_type::<_, _, ForceId>(force_id);
    let (input, number_of_var_params) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_VP, uvint8)(input)?;
    let number_of_var_params = parser_utils::varint_to_type::<_, _, usize>(number_of_var_params);

    let (input, primary_entity_type) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_ENTITY_TYPE, entity_type)(input)?;
    let (input, alternate_entity_type) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_ALT_ENTITY_TYPE, entity_type)(input)?;

    let (input, entity_linear_velocity) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_LIN_VELOCITY, linear_velocity)(input)?;
    let (input, entity_location) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_ENTITY_LOCATION, world_coordinates)(input)?;
    let (input, entity_orientation) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_ENTITY_ORIENTATION, orientation)(input)?;

    let (input, entity_appearance) : (BitInput, Option<u32>) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_ENTITY_APPEARANCE, take(THIRTY_TWO_BITS))(input)?;
    let entity_appearance = if let Some(appearance) = entity_appearance {
        Some(CdisEntityAppearance(appearance))
    } else { None };

    let (input, dr_algorithm) : (BitInput, u8) = take(FOUR_BITS)(input)?;
    let dr_algorithm = DeadReckoningAlgorithm::from(dr_algorithm);
    let (input, dr_params_other) : (BitInput, Option<u128>) = parser_utils::parse_field_when_present(
    full_update_flag, fields_present, FPF_BIT_DR_OTHER, take(HUNDRED_TWENTY_BITS))(input)?;
    let (input, dr_params_entity_linear_acceleration) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_DR_LIN_ACCELERATION, linear_acceleration)(input)?;
    let (input, dr_params_entity_angular_velocity) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_DR_ANG_VELOCITY, angular_velocity)(input)?;

    let (input, entity_marking) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_MARKING, entity_marking)(input)?;
    let (input, capabilities) = parser_utils::parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_CAPABILITIES, uvint32)(input)?;
    let capabilities = if let Some(capabilities) = capabilities {
        Some(CdisEntityCapabilities(capabilities))
    } else { None };

    let (input, variable_parameters) = if let Some(num_params) = number_of_var_params {
        count(variable_parameter, num_params)(input)?
    } else {
        (input, vec![])
    };


    Ok((input, EntityState {
        units,
        full_update_flag,
        entity_id,
        force_id,
        entity_type: primary_entity_type,
        alternate_entity_type,
        entity_linear_velocity,
        entity_location,
        entity_orientation,
        entity_appearance,
        dr_algorithm,
        dr_params_other,
        dr_params_entity_linear_acceleration,
        dr_params_entity_angular_velocity,
        entity_marking,
        capabilities,
        variable_parameters,
    }))
}

#[cfg(test)]
mod tests {
    use crate::parser_utils::{field_present, parse_field_when_present};
    use crate::records::parser::entity_identification;

    #[test]
    fn field_present_u8_true() {
        let fields = 0b00000010u8;
        let mask = 0x2u8;

        assert!(field_present(fields, mask));
    }

    #[test]
    fn field_present_u32_true() {
        let fields = 0x02004010u32;
        let mask = 0x10u32;

        assert!(field_present(fields, mask));
    }

    #[test]
    fn field_present_u32_false() {
        let fields = 0x02004010u32;
        let mask = 0x01u32;

        assert!(!field_present(fields, mask));
    }

    #[test]
    fn field_present_u8_false() {
        let fields = 0b00000100u8;
        let mask = 0x2u8;

        assert!(!field_present(fields, mask));
    }

    #[test]
    fn parse_when_present_entity_id() {
        let fields = 0b00000001u8;
        let mask = 0x01u8;
        let input : [u8; 4] = [0b00000000, 0b01000000, 0b00010000, 0b00000100];

        // entity_identification is in reality always present, but is an easy example for a test.
        let actual = parse_field_when_present(
            false, fields, mask,
            entity_identification)((&input, 0));

        assert!(actual.is_ok());
        let entity = actual.unwrap().1;
        assert!(entity.is_some());
        let entity = entity.unwrap();
        assert_eq!(1u16, entity.site.value);
        assert_eq!(1u16, entity.application.value);
        assert_eq!(1u16, entity.entity.value);
    }

    #[test]
    fn parse_when_present_entity_id_not_present() {
        let fields = 0b00010000u8;
        let mask = 0x01u8;
        let input : [u8; 4] = [0b00000000, 0b01000000, 0b00010000, 0b00000100];

        // entity_identification is in reality always present, but is an easy example for a test.
        let actual = parse_field_when_present(
            false, fields, mask,
            entity_identification)((&input, 0));

        assert!(actual.is_ok());
        assert!(actual.unwrap().1.is_none())
    }
}