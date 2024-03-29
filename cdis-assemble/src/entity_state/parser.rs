use std::ops::BitAnd;
use nom::bits::complete::take;
use nom::IResult;
use dis_rs::enumerations::{DeadReckoningAlgorithm, ForceId};
use crate::constants::{EIGHT_BITS, FOUR_BITS, HUNDRED_TWENTY_BITS, NINE_BITS, ONE_BIT, THIRTEEN_BITS};
use crate::entity_state::model::EntityState;
use crate::records::model::{EntityId, Units};
use crate::records::parser::{angular_velocity, entity_identification, entity_marking, entity_type, linear_acceleration, linear_velocity, orientation, world_coordinates};
use crate::types::model::{UVINT16, VarInt};
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
    let (input, force_id) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_FORCE_ID, uvint8)(input)?;
    let force_id = varint_to_type::<_, _, ForceId>(force_id);
    let (input, number_of_var_params) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_VP, uvint8)(input)?;
    let number_of_var_params = varint_to_type::<_, _, usize>(number_of_var_params);

    let (input, primary_entity_type) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_ENTITY_TYPE, entity_type)(input)?;
    let (input, alternate_entity_type) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_ALT_ENTITY_TYPE, entity_type)(input)?;

    let (input, entity_linear_velocity) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_LIN_VELOCITY, linear_velocity)(input)?;
    let (input, entity_location) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_ENTITY_LOCATION, world_coordinates)(input)?;
    let (input, entity_orientation) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_ENTITY_ORIENTATION, orientation)(input)?;
    // TODO: expose entity_appearance from dis_rs
    // let (input, entity_appearance) = parse_field_when_present(
        // full_update_flag, fields_present, FPF_BIT_ENTITY_APPEARANCE, appearance...)(input)?;

    let (input, dr_algorithm) : (BitInput, u8) = take(FOUR_BITS)(input)?;
    let dr_algorithm = DeadReckoningAlgorithm::from(dr_algorithm);
    let (input, dr_params_other) : (BitInput, Option<u128>) = parse_field_when_present(
    full_update_flag, fields_present, FPF_BIT_DR_OTHER, take(HUNDRED_TWENTY_BITS))(input)?;

    let (input, dr_params_entity_linear_acceleration) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_DR_LIN_ACCELERATION, linear_acceleration)(input)?;
    let (input, dr_params_entity_angular_velocity) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_DR_ANG_VELOCITY, angular_velocity)(input)?;

    let (input, entity_marking) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_MARKING, entity_marking)(input)?;
    let (input, capabilities) = parse_field_when_present(
        full_update_flag, fields_present, FPF_BIT_CAPABILITIES, uvint32)(input)?;
    // TODO convert u32 to capabilities
    // let capabilities = varint_to_type::<_, _, u32>(capabilities);

    // TODO var params

    Ok((input, EntityState {
        units,
        full_update_flag,
        entity_id: EntityId {
            site: UVINT16::from(1),
            application: UVINT16::from(1),
            entity: UVINT16::from(1),
        },
        force_id,
        entity_type: primary_entity_type,
        alternate_entity_type,
        entity_linear_velocity,
        entity_location,
        entity_orientation,
        entity_appearance: None,
        dr_algorithm,
        dr_params_other: None,
        dr_params_entity_linear_acceleration: Default::default(),
        dr_params_entity_angular_velocity: Default::default(),
        entity_marking,
        capabilities,
        variable_parameters: None,
    }))
}

/// This is a 'conditional parser', which applies the provided parser `f` when either a full update is needed (indicated by the `full_update` flag)
/// or when `mask` applied (bitwise OR) to the `fields_present` flags yields a none-zero value.
///
/// The function returns the output of parser `f` as an `Option`.
pub fn parse_field_when_present<'a, O, T, F>(
    full_update: bool, fields_present: T, mask: T, f: F
) -> impl Fn(BitInput<'a>) -> IResult<BitInput, Option<O>>
    where
        O: std::fmt::Debug,
        T: Copy + BitAnd + PartialEq + Default,
        <T as BitAnd>::Output: PartialEq<T>,
        F: Fn(BitInput<'a>) -> IResult<BitInput<'a>, O>, {
    move |input: BitInput<'a>| {
        if full_update | field_present(fields_present, mask) {
            let result = f(input);
            match result {
                Ok((input, result)) => { Ok((input, Some(result))) }
                Err(err) => { Err(err) }
            }
        } else { Ok((input, None)) }
    }
}

type BitInput<'a> = (&'a[u8], usize);

/// Helper function to match presents of a bit position in a bitfield.
///
/// Returns `true` when `fields_present` OR `mask` yields a non-zero value.
/// Works with the basic numerical types (u8, u16, u32, i..).
fn field_present<T>(fields_present: T, mask: T) -> bool
    where T: BitAnd + PartialEq + Default,
          <T as BitAnd>::Output: PartialEq<T>, {
    (fields_present & mask) != Default::default()
}

fn varint_to_type<V, I, T>(enum_value: Option<V>) -> Option<T>
where V: VarInt<InnerType = I>,
      T: From<I> {
    if let Some(value) = enum_value {
        let inner = value.value();
        Some(T::from(inner))
    } else { None }
}

#[cfg(test)]
mod tests {
    use crate::entity_state::parser::{field_present, parse_field_when_present};
    use crate::records::parser::{entity_identification};

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