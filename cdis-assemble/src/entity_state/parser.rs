use std::ops::BitAnd;
use nom::bits::complete::take;
use nom::error::{ErrorKind, ParseError};
use nom::IResult;
use crate::constants::{ONE_BIT, THIRTEEN_BITS};
use crate::entity_state::model::EntityState;
use crate::records::model::{EntityId, EntityType, Units};
use crate::records::parser::entity_type;
use crate::types::model::UVINT16;

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

pub(crate) fn entity_state_body(input: (&[u8], usize)) -> IResult<(&[u8], usize), EntityState> {
    let (input, fields_present) : ((&[u8], usize), u16) = take(THIRTEEN_BITS)(input)?;
    let (input, units) : ((&[u8], usize), u8) = take(ONE_BIT)(input)?;
    let units = Units::from(units);
    let (input, full_update_flag) : ((&[u8], usize), u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    // let (input, entity_type) = parse_field_when_present(
    //     full_update_flag, fields_present, FPF_BIT_ENTITY_TYPE,
    //     entity_type)(input)?;
    // let (input, entity_type) = if full_update_flag | field_present(fields_present, FPF_BIT_ENTITY_TYPE) {
    //     let (input, full_update_flag) = entity_type(input)?;
    //     (input, Some(full_update_flag))
    // } else { (input, None) };

    Ok((input, EntityState {
        units,
        full_update_flag,
        entity_id: EntityId {
            site: UVINT16::from(1),
            application: UVINT16::from(1),
            entity: UVINT16::from(1),
        },
        force_id: None,
        entity_type: None,
        alternate_entity_type: None,
        entity_linear_velocity: None,
        entity_location: None,
        entity_orientation: None,
        entity_appearance: None,
        dr_algorithm: Default::default(),
        dr_params_other: None,
        dr_params_entity_linear_acceleration: None,
        dr_params_entity_angular_velocity: None,
        entity_marking: None,
        capabilities: None,
        variable_parameters: None,
    }))
}

pub fn parse_field_when_present<O, T, E: for<'a> ParseError<(&'a [u8], usize),>, F>(
    full_update: bool, fields_present: T, mask: T, f: F
) -> impl Fn((&[u8], usize)) -> IResult<(&[u8], usize), Option<O>, E>
    where
        T: Copy + BitAnd + PartialEq + Default,
        <T as BitAnd>::Output: PartialEq<T>,
        F: Fn((&[u8], usize)) -> IResult<(&[u8], usize), O, E>, {
    move |input: (&[u8], usize)| {
        if full_update | field_present(fields_present, mask) {
            let result = f(input);
            match result {
                Ok((input, result)) => { Ok((input, Some(result))) }
                Err(err) => { Err(err) }
            }
        } else { Ok((input, None)) }
    }
}

// fn parse_field_when_present<'a, T, I, Oi, Oo, E, F>(full_update: bool, fields_present: T, mask: T, mut f: F)
//     -> impl FnMut(ParseInput<'a>) -> IResult<(ParseInput<'a>, Oo), E> where
//     T: Copy + BitAnd + Shl,
//     F: Fn(ParseInput<'a>) -> IResult<ParseInput<'a>, Oi, E>,
//     E: ParseError<ParseInput<'a>>, {
//     move |input: ParseInput<'a>| {
//         if full_update | field_present(fields_present, mask) {
//             let result = f.parse(input).finish();
//
//             match result {
//                 Ok((input, result)) => { Ok(input,Some(result)) }
//                 Err(err) => { Err(err) }
//             }
//         } else { Ok((input, None)) }
//     }
// }

// fn parse_field_when_present<T, I, O, E>(full_update: bool, fields_present: T, mask: T, dummy: O) -> impl Fn(I) -> Result<(I, Option<O>), E> + 'static {
//     move |input: I | {
//         if full_update {
//             Ok((input, Some(dummy)))
//         } else { Err(nom::Err::Error(nom::error::ErrorKind::AlphaNumeric)) }
//     }
// }

type ParseInput<'a> = (&'a[u8], usize);

fn field_present<T>(fields_present: T, mask: T) -> bool
    where T: BitAnd + PartialEq + Default,
          <T as BitAnd>::Output: PartialEq<T>, {
    (fields_present & mask) != Default::default()
}

#[cfg(test)]
mod tests {
    use crate::entity_state::parser::{field_present, parse_field_when_present};
    use crate::records::parser::entity_type;

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
    fn parse_when_present_entity_type() {
        let fields = 0b00000001u8;
        let mask = 0x01u8;
        let input : [u8; 4] = [0b00000000, 0b01000000, 0b00010000, 0b00000100];

        let actual = parse_field_when_present(false, fields, mask, entity_type)((&input, 0));

        assert!(actual.is_ok());
        assert!(actual.unwrap().1.is_some())
    }
}