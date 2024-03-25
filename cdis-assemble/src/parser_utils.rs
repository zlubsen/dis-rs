use bitvec::field::BitField;
use bitvec::macros::internal::funty::Integral;
use crate::BitBuffer;

/// Write `value` to the BitBuffer `buf`, at the position of `cursor` with length `bit_size`.
pub(crate) fn write_value_with_length<T: Integral>(buf: &mut BitBuffer, cursor: usize, bit_size: usize, value: T) -> usize {
    let next_cursor = cursor + bit_size;
    buf[cursor..next_cursor].store_be(value);
    next_cursor
}