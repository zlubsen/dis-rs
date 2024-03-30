use bitvec::field::BitField;
use bitvec::macros::internal::funty::Integral;
use nom::IResult;
use std::ops::BitAnd;
use crate::BitBuffer;
use crate::types::model::VarInt;

/// Write `value` to the BitBuffer `buf`, at the position of `cursor` with length `bit_size`.
pub(crate) fn write_value_with_length<T: Integral>(buf: &mut BitBuffer, cursor: usize, bit_size: usize, value: T) -> usize {
    let next_cursor = cursor + bit_size;
    buf[cursor..next_cursor].store_be(value);
    next_cursor
}

pub(crate) type BitInput<'a> = (&'a[u8], usize);

/// This is a 'conditional parser', which applies the provided parser `f` when either a full update is needed (indicated by the `full_update` flag)
/// or when `mask` applied (bitwise OR) to the `fields_present` flags yields a none-zero value.
///
/// The function returns the output of parser `f` as an `Option`.
pub(crate) fn parse_field_when_present<'a, O, T, F>(
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

/// Helper function to match presents of a bit position in a bitfield.
///
/// Returns `true` when `fields_present` OR `mask` yields a non-zero value.
/// Works with the basic numerical types (u8, u16, u32, i..).
pub(crate) fn field_present<T>(fields_present: T, mask: T) -> bool
    where T: BitAnd + PartialEq + Default,
          <T as BitAnd>::Output: PartialEq<T>, {
    (fields_present & mask) != Default::default()
}

/// Conversion function to convert the inner type of an `Option<T>` as
/// returned by a conditional parser to another type.
/// Useful for transforming a `VarInt` to a standard Rust type such as `u8`.
///
/// Returns `None` or `Some` with the converted type
pub(crate) fn varint_to_type<V, I, T>(enum_value: Option<V>) -> Option<T>
where V: VarInt<InnerType = I>,
      T: From<I> {
    if let Some(value) = enum_value {
        let inner = value.value();
        Some(T::from(inner))
    } else { None }
}
