use crate::constants::{FOUR_BITS, HUNDRED_TWENTY_BITS, ONE_BIT, THIRTEEN_BITS, THIRTY_TWO_BITS};
use crate::entity_state::model::{
    CdisDRParametersOther, CdisEntityAppearance, CdisEntityCapabilities, EntityState,
    EntityStateFieldsPresent as FPF,
};
use crate::parsing::BitInput;
use crate::records::model::UnitsDekameters;
use crate::records::parser::{
    angular_velocity, entity_identification, entity_marking, entity_type, linear_acceleration,
    linear_velocity, orientation, variable_parameter, world_coordinates,
};
use crate::types::parser::{uvint32, uvint8};
use crate::{parsing, BodyProperties, CdisBody};
use dis_rs::enumerations::DeadReckoningAlgorithm;
use nom::bits::complete::take;
use nom::multi::count;
use nom::IResult;

#[allow(clippy::redundant_closure)]
pub(crate) fn entity_state_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present): (BitInput, u16) = take(THIRTEEN_BITS)(input)?;
    let (input, units): (BitInput, u8) = take(ONE_BIT)(input)?;
    let units = UnitsDekameters::from(units);
    let (input, full_update_flag): (BitInput, u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    let (input, entity_id) = entity_identification(input)?;
    let (input, force_id) =
        parsing::parse_field_when_present(fields_present, FPF::FORCE_ID_BIT, uvint8)(input)?;
    let (input, number_of_var_params) =
        parsing::parse_field_when_present(fields_present, FPF::VP_BIT, uvint8)(input)?;
    let number_of_var_params = parsing::varint_to_type::<_, _, usize>(number_of_var_params);

    let (input, primary_entity_type) =
        parsing::parse_field_when_present(fields_present, FPF::ENTITY_TYPE_BIT, entity_type)(
            input,
        )?;
    let (input, alternate_entity_type) =
        parsing::parse_field_when_present(fields_present, FPF::ALT_ENTITY_TYPE_BIT, entity_type)(
            input,
        )?;

    let (input, entity_linear_velocity) = parsing::parse_field_when_present(
        fields_present,
        FPF::LINEAR_VELOCITY_BIT,
        linear_velocity,
    )(input)?;
    let (input, entity_location) = parsing::parse_field_when_present(
        fields_present,
        FPF::ENTITY_LOCATION_BIT,
        world_coordinates,
    )(input)?;
    let (input, entity_orientation) = parsing::parse_field_when_present(
        fields_present,
        FPF::ENTITY_ORIENTATION_BIT,
        orientation,
    )(input)?;

    let (input, entity_appearance): (BitInput, Option<u32>) = parsing::parse_field_when_present(
        fields_present,
        FPF::ENTITY_APPEARANCE_BIT,
        take(THIRTY_TWO_BITS),
    )(input)?;
    // #[allow(clippy::redundant_closure)]
    let entity_appearance = entity_appearance.map(|appearance| CdisEntityAppearance(appearance));

    let (input, dr_algorithm): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let dr_algorithm = DeadReckoningAlgorithm::from(dr_algorithm);
    let (input, dr_params_other): (BitInput, Option<u128>) = parsing::parse_field_when_present(
        fields_present,
        FPF::DR_OTHER_BIT,
        take(HUNDRED_TWENTY_BITS),
    )(input)?;
    let dr_params_other = dr_params_other.map(|bytes| CdisDRParametersOther::from(bytes));

    let (input, dr_params_entity_linear_acceleration) = parsing::parse_field_when_present(
        fields_present,
        FPF::DR_LINEAR_ACCELERATION_BIT,
        linear_acceleration,
    )(input)?;
    let (input, dr_params_entity_angular_velocity) = parsing::parse_field_when_present(
        fields_present,
        FPF::DR_ANGULAR_VELOCITY_BIT,
        angular_velocity,
    )(input)?;

    let (input, entity_marking) =
        parsing::parse_field_when_present(fields_present, FPF::MARKING_BIT, entity_marking)(input)?;
    let (input, capabilities) =
        parsing::parse_field_when_present(fields_present, FPF::CAPABILITIES_BIT, uvint32)(input)?;
    let capabilities = capabilities.map(|cap| CdisEntityCapabilities(cap));

    let (input, variable_parameters) = if let Some(num_params) = number_of_var_params {
        count(variable_parameter, num_params)(input)?
    } else {
        (input, vec![])
    };

    Ok((
        input,
        EntityState {
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
        }
        .into_cdis_body(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::entity_state::parser::entity_state_body;
    use crate::records::model::{EntityId, UnitsDekameters};
    use crate::types::model::UVINT16;
    use crate::CdisBody;

    #[test]
    fn parse_entity_state_no_fields_present() {
        #[rustfmt::skip]
        #[allow(clippy::unusual_byte_groupings)]
        #[allow(clippy::unreadable_literal)]
        let input = [
            0b00000000,
            0b00000_1_0_0,
            0b00000001,
            0b1_0000000,
            0b011_00000,
            0b00011_000,
            0b0_0000000,
        ];
        let ((_input, cursor), body) = entity_state_body((&input, 0)).unwrap();

        assert_eq!(cursor, 1); // cursor position in last byte of input

        if let CdisBody::EntityState(es) = body {
            assert_eq!(es.units, UnitsDekameters::Dekameter);
            assert_eq!(
                es.entity_id,
                EntityId::new(UVINT16::from(3), UVINT16::from(3), UVINT16::from(3))
            );
            assert!(!es.full_update_flag);
        }
    }
}
