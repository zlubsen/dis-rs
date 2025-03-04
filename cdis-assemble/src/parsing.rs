use crate::acknowledge::parser::acknowledge_body;
use crate::action_request::parser::action_request_body;
use crate::action_response::parser::action_response_body;
use crate::collision::parser::collision_body;
use crate::comment::parser::comment_body;
use crate::constants::ONE_BIT;
use crate::create_entity::parser::create_entity_body;
use crate::data::parser::data_body;
use crate::data_query::parser::data_query_body;
use crate::designator::parser::designator_body;
use crate::detonation::parser::detonation_body;
use crate::electromagnetic_emission::parser::electromagnetic_emission_body;
use crate::entity_state::parser::entity_state_body;
use crate::event_report::parser::event_report_body;
use crate::fire::parser::fire_body;
use crate::iff::parser::iff_body;
use crate::receiver::parser::receiver_body;
use crate::records::model::CdisHeader;
use crate::records::parser::cdis_header;
use crate::remove_entity::parser::remove_entity_body;
use crate::set_data::parser::set_data_body;
use crate::signal::parser::signal_body;
use crate::start_resume::parser::start_resume_body;
use crate::stop_freeze::parser::stop_freeze_body;
use crate::transmitter::parser::transmitter_body;
use crate::types::model::VarInt;
use crate::unsupported::Unsupported;
use crate::{CdisBody, CdisError, CdisPdu};
use dis_rs::enumerations::PduType;
use nom::bits::complete::take;
use nom::multi::many1;
use nom::{IResult, Parser};
use std::ops::BitAnd;

/// Attempts to parse the provided buffer for CDIS PDUs
///
/// # Errors
/// Returns a `CdisError` when parsing fails.
pub fn parse(input: &[u8]) -> Result<Vec<CdisPdu>, CdisError> {
    parse_multiple_cdis_pdu(input)
}

pub(crate) fn parse_multiple_cdis_pdu(input: &[u8]) -> Result<Vec<CdisPdu>, CdisError> {
    match many1(bitscdis_pdu).parse((input, 0)) {
        Ok((_, pdus)) => Ok(pdus),
        Err(err) => Err(CdisError::ParseError(err.to_string())), // TODO not very descriptive / error means we can not match any PDUs
    }
}

pub(crate) fn cdis_pdu(input: BitInput) -> IResult<BitInput, CdisPdu> {
    let (input, header) = cdis_header(input)?;
    let (input, body) = cdis_body(&header)(input)?;

    Ok((input, CdisPdu { header, body }))
}

pub(crate) fn cdis_body(
    header: &CdisHeader,
) -> impl Fn(BitInput) -> IResult<BitInput, CdisBody> + '_ {
    move |input: BitInput| {
        let (input, body): (BitInput, CdisBody) = match header.pdu_type {
            PduType::EntityState => entity_state_body(input)?,
            PduType::Fire => fire_body(input)?,
            PduType::Detonation => detonation_body(input)?,
            PduType::Collision => collision_body(input)?,
            PduType::CreateEntity => create_entity_body(input)?,
            PduType::RemoveEntity => remove_entity_body(input)?,
            PduType::StartResume => start_resume_body(input)?,
            PduType::StopFreeze => stop_freeze_body(input)?,
            PduType::Acknowledge => acknowledge_body(input)?,
            PduType::ActionRequest => action_request_body(input)?,
            PduType::ActionResponse => action_response_body(input)?,
            PduType::DataQuery => data_query_body(input)?,
            PduType::SetData => set_data_body(input)?,
            PduType::Data => data_body(input)?,
            PduType::EventReport => event_report_body(input)?,
            PduType::Comment => comment_body(input)?,
            PduType::ElectromagneticEmission => electromagnetic_emission_body(input)?,
            PduType::Designator => designator_body(input)?,
            PduType::Transmitter => transmitter_body(input)?,
            PduType::Signal => signal_body(input)?,
            PduType::Receiver => receiver_body(input)?,
            PduType::IFF => iff_body(input)?,
            // Unsupported PDUs in CDIS v1
            PduType::Other => (input, CdisBody::Unsupported(Unsupported)),
            PduType::Unspecified(_val) => (input, CdisBody::Unsupported(Unsupported)),
            _val => (input, CdisBody::Unsupported(Unsupported)),
        };

        Ok((input, body))
    }
}

pub(crate) type BitInput<'a> = (&'a [u8], usize);

/// This is a 'conditional parser', which applies the provided parser `f` when either a full update is needed (indicated by the `full_update` flag)
/// or when `mask` applied (bitwise OR) to the `fields_present` flags yields a none-zero value.
///
/// The function returns the output of parser `f` as an `Option`.
pub(crate) fn parse_field_when_present<'a, O, T, F>(
    fields_present: T,
    mask: T,
    f: F,
) -> impl Fn(BitInput<'a>) -> IResult<BitInput<'a>, Option<O>>
where
    O: std::fmt::Debug,
    T: Copy + BitAnd + PartialEq + Default,
    <T as BitAnd>::Output: PartialEq<T>,
    F: Fn(BitInput<'a>) -> IResult<BitInput<'a>, O>,
{
    move |input: BitInput<'a>| {
        if field_present(fields_present, mask) {
            let result = f(input);
            match result {
                Ok((input, result)) => Ok((input, Some(result))),
                Err(err) => Err(err),
            }
        } else {
            Ok((input, None))
        }
    }
}

/// Helper function to match presents of a bit position in a bitfield.
///
/// Returns `true` when `fields_present` OR `mask` yields a non-zero value.
/// Works with the basic numerical types (u8, u16, u32, i..).
pub(crate) fn field_present<T>(fields_present: T, mask: T) -> bool
where
    T: BitAnd + PartialEq + Default,
    <T as BitAnd>::Output: PartialEq<T>,
{
    (fields_present & mask) != Default::default()
}

/// Conversion function to convert the inner type of an `Option<T>` as
/// returned by a conditional parser to another type.
/// Useful for transforming a `VarInt` to a standard Rust type such as `u8`.
///
/// Returns `None` or `Some` with the converted type
pub(crate) fn varint_to_type<V, I, T>(enum_value: Option<V>) -> Option<T>
where
    V: VarInt<InnerType = I>,
    T: From<I>,
{
    if let Some(value) = enum_value {
        let inner = value.value();
        Some(T::from(inner))
    } else {
        None
    }
}

/// Parse a signed value from the bit stream, formatted in `count` bits.
/// MSB is the sign bit, the remaining bits form the value.
/// This function then converts these two components to a signed value of type `isize`.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub(crate) fn take_signed(count: usize) -> impl Fn(BitInput) -> IResult<BitInput, isize> {
    move |input| {
        let (input, sign_bit): (BitInput, isize) = take(ONE_BIT)(input)?;
        let (input, value_bits): (BitInput, isize) = take(count - ONE_BIT)(input)?;

        let max_value = 2usize.pow((count - 1) as u32) - 1;
        let min_value = -(max_value as isize + 1);
        let value = if sign_bit != 0 {
            min_value + value_bits
        } else {
            value_bits
        };

        Ok((input, value))
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::THREE_BITS;
    use crate::parsing::{field_present, parse_field_when_present, take_signed};
    use crate::records::parser::entity_identification;

    #[test]
    fn take_signed_positive_min() {
        let input = [0b0000_0000];
        let (_input, value) = take_signed(THREE_BITS)((&input, 0)).unwrap();

        assert_eq!(0, value);
    }

    #[test]
    fn take_signed_positive_max() {
        let input = [0b0110_0000];
        let (_input, value) = take_signed(THREE_BITS)((&input, 0)).unwrap();

        assert_eq!(3, value);
    }

    #[test]
    fn take_signed_negative_min() {
        let input = [0b1000_0000];
        let (_input, value) = take_signed(THREE_BITS)((&input, 0)).unwrap();

        assert_eq!(-4, value);
    }

    #[test]
    fn take_signed_negative_max() {
        let input = [0b1110_0000];
        let (_input, value) = take_signed(THREE_BITS)((&input, 0)).unwrap();

        assert_eq!(-1, value);
    }

    #[test]
    fn field_present_u8_true() {
        let fields = 0b0000_0010_u8;
        let mask = 0x2u8;

        assert!(field_present(fields, mask));
    }

    #[test]
    fn field_present_u32_true() {
        let fields = 0x0200_4010_u32;
        let mask = 0x10u32;

        assert!(field_present(fields, mask));
    }

    #[test]
    fn parse_when_present_entity_id() {
        let fields = 0b0000_0001_u8;
        let mask = 0x01u8;
        let input: [u8; 4] = [0b0000_0000, 0b0100_0000, 0b0001_0000, 0b0000_0100];

        // entity_identification is in reality always present, but is an easy example for a test.
        let actual = parse_field_when_present(fields, mask, entity_identification)((&input, 0));

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
        let fields = 0b0001_0000_u8;
        let mask = 0x01u8;
        let input: [u8; 4] = [0b0000_0000, 0b0100_0000, 0b0001_0000, 0b0000_0100];

        // entity_identification is in reality always present, but is an easy example for a test.
        let actual = parse_field_when_present(fields, mask, entity_identification)((&input, 0));

        assert!(actual.is_ok());
        assert!(actual.unwrap().1.is_none());
    }

    #[test]
    fn field_present_u32_false() {
        let fields = 0x0200_4010_u32;
        let mask = 0x01u32;

        assert!(!field_present(fields, mask));
    }

    #[test]
    fn field_present_u8_false() {
        let fields = 0b0000_0100_u8;
        let mask = 0x2u8;

        assert!(!field_present(fields, mask));
    }
}
