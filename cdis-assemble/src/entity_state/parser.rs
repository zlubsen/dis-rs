use nom::bits::complete::take;
use nom::IResult;
use nom::multi::count;
use dis_rs::enumerations::{DeadReckoningAlgorithm};
use crate::constants::{FOUR_BITS, HUNDRED_TWENTY_BITS, ONE_BIT, THIRTEEN_BITS, THIRTY_TWO_BITS};
use crate::entity_state::model::{CdisEntityAppearance, CdisEntityCapabilities, EntityState, EntityStateFieldsPresent as FPF};
use crate::{BodyProperties, CdisBody, utils};
use crate::utils::BitInput;
use crate::records::model::Units;
use crate::records::parser::{angular_velocity, entity_identification, entity_marking, entity_type, linear_acceleration, linear_velocity, orientation, variable_parameter, world_coordinates};
use crate::types::parser::{uvint32, uvint8};

pub(crate) fn entity_state_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present) : (BitInput, u16) = take(THIRTEEN_BITS)(input)?;
    let (input, units) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let units = Units::from(units);
    let (input, full_update_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    let (input, entity_id) = entity_identification(input)?;
    let (input, force_id) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::FORCE_ID_BIT, uvint8)(input)?;
    let (input, number_of_var_params) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::VP_BIT, uvint8)(input)?;
    let number_of_var_params = utils::varint_to_type::<_, _, usize>(number_of_var_params);

    let (input, primary_entity_type) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::ENTITY_TYPE_BIT, entity_type)(input)?;
    let (input, alternate_entity_type) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::ALT_ENTITY_TYPE_BIT, entity_type)(input)?;

    let (input, entity_linear_velocity) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::LINEAR_VELOCITY_BIT, linear_velocity)(input)?;
    let (input, entity_location) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::ENTITY_LOCATION_BIT, world_coordinates)(input)?;
    let (input, entity_orientation) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::ENTITY_ORIENTATION_BIT, orientation)(input)?;

    let (input, entity_appearance) : (BitInput, Option<u32>) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::ENTITY_APPEARANCE_BIT, take(THIRTY_TWO_BITS))(input)?;
    let entity_appearance = if let Some(appearance) = entity_appearance {
        Some(CdisEntityAppearance(appearance))
    } else { None };

    let (input, dr_algorithm) : (BitInput, u8) = take(FOUR_BITS)(input)?;
    let dr_algorithm = DeadReckoningAlgorithm::from(dr_algorithm);
    let (input, dr_params_other) : (BitInput, Option<u128>) = utils::parse_field_when_present(
    full_update_flag, fields_present, FPF::DR_OTHER_BIT, take(HUNDRED_TWENTY_BITS))(input)?;
    let (input, dr_params_entity_linear_acceleration) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::DR_LINEAR_ACCELERATION_BIT, linear_acceleration)(input)?;
    let (input, dr_params_entity_angular_velocity) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::DR_ANGULAR_VELOCITY_BIT, angular_velocity)(input)?;

    let (input, entity_marking) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::MARKING_BIT, entity_marking)(input)?;
    let (input, capabilities) = utils::parse_field_when_present(
        full_update_flag, fields_present, FPF::CAPABILITIES_BIT, uvint32)(input)?;
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
    }.into_cdis_body()))
}

#[cfg(test)]
mod tests {
    use crate::utils::{field_present, parse_field_when_present};
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