use nom::IResult;
use nom::bits::complete::take;
use nom::complete::bool;
use crate::constants::{ONE_BIT, TWO_BITS};
use crate::parsing::BitInput;
use crate::parsing::take_signed;
use crate::types::model::{CdisFloat, SVINT12, Svint12BitSize, SVINT13, Svint13BitSize, SVINT14, Svint14BitSize, SVINT16, Svint16BitSize, SVINT24, Svint24BitSize, UVINT16, Uvint16BitSize, UVINT32, Uvint32BitSize, UVINT8, Uvint8BitSize};

pub(crate) fn uvint8(input: BitInput) -> IResult<BitInput, UVINT8> {
    let (input, flag_bits):(BitInput, u8) = take(ONE_BIT)(input)?;
    let num_bits_to_parse = Uvint8BitSize::from(flag_bits).bit_size();
    let (input, field_value):(BitInput, u8) = take(num_bits_to_parse)(input)?;

    Ok((input, UVINT8::from(field_value)))
}

pub(crate) fn uvint16(input: BitInput) -> IResult<BitInput, UVINT16> {
    let (input, flag_bits):(BitInput, u8) = take(TWO_BITS)(input)?;
    let num_bits_to_parse = Uvint16BitSize::from(flag_bits).bit_size();
    let (input, field_value):(BitInput, u16) = take(num_bits_to_parse)(input)?;

    Ok((input, UVINT16::from(field_value)))
}

pub(crate) fn uvint32(input: BitInput) -> IResult<BitInput, UVINT32> {
    let (input, flag_bits):(BitInput, u8) = take(TWO_BITS)(input)?;
    let num_bits_to_parse = Uvint32BitSize::from(flag_bits).bit_size();
    let (input, field_value):(BitInput, u32) = take(num_bits_to_parse)(input)?;

    Ok((input, UVINT32::from(field_value)))
}

pub(crate) fn svint12(input: BitInput) -> IResult<BitInput, SVINT12> {
    let (input, flag_bits) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint12BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : (BitInput, bool) = bool(input)?;
    let (input, field_value) : (BitInput, i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT12::from(field_value)))
}

pub(crate) fn svint13(input: BitInput) -> IResult<BitInput, SVINT13> {
    let (input, flag_bits) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint13BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : (BitInput, bool) = bool(input)?;
    let (input, field_value) : (BitInput, i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT13::from(field_value)))
}

pub(crate) fn svint14(input: BitInput) -> IResult<BitInput, SVINT14> {
    let (input, flag_bits) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint14BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : (BitInput, bool) = bool(input)?;
    let (input, field_value) : (BitInput, i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT14::from(field_value)))
}

pub(crate) fn svint16(input: BitInput) -> IResult<BitInput, SVINT16> {
    let (input, flag_bits) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint16BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : (BitInput, bool) = bool(input)?;
    let (input, field_value) : (BitInput, i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT16::from(field_value)))
}

pub(crate) fn svint24(input: BitInput) -> IResult<BitInput, SVINT24> {
    let (input, flag_bits) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint24BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : (BitInput, bool) = bool(input)?;
    let (input, field_value) : (BitInput, i32) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT24::from(field_value)))
}

/// Parses a C-DIS custom floating point value, based on the concrete implementation `T` of trait `CdisFloat`.
pub(crate) fn cdis_float<T>(input: BitInput) -> IResult<BitInput, T>
where T: CdisFloat {
    let (input, mantissa) : (BitInput, isize) = take_signed(T::MANTISSA_BITS)(input)?;
    let mantissa = mantissa as i32;
    let (input, exponent) : (BitInput, isize) = take_signed(T::EXPONENT_BITS)(input)?;
    let exponent = exponent as i8;

    Ok((input, T::new(mantissa, exponent)))
}

#[cfg(test)]
mod tests {
    use crate::types::parser::{svint12, uvint16, uvint32, uvint8};
    use crate::types::model::{SVINT12, Svint12BitSize, UVINT16, Uvint16BitSize, UVINT32, Uvint32BitSize, UVINT8, Uvint8BitSize};
    use crate::types::model::VarInt;

    #[test]
    fn parse_uvint8_bit_flag_zero() {
        let input = [0b00001000];
        let expected = UVINT8::new(Uvint8BitSize::Four, 1u8);
        let (_input, actual) = uvint8((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_uvint8_bit_flag_one() {
        let input = [0b11000000, 0b10000000];
        let expected = UVINT8::new(Uvint8BitSize::Eight, 129);
        let (_input, actual) = uvint8((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_uvint16_bit_flag_zero() {
        let input = [0b00000000, 0b01000000];
        let expected = UVINT16::new(Uvint16BitSize::Eight, 1);
        let (_input, actual) = uvint16((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_uvint16_bit_flag_three() {
        let input = [0b11011111, 0b11111111, 0b11000000];
        let expected = UVINT16::new(Uvint16BitSize::Sixteen, 32767);
        let (_input, actual) = uvint16((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_uvint32_bit_flag_zero() {
        let input = [0b00000000, 0b01000000];
        let expected = UVINT32::new(Uvint32BitSize::Eight, 1);
        let (_input, actual) = uvint32((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_uvint32_bit_flag_three() {
        let input = [0b11100000, 0b00000000, 0b00000000, 0b00000000, 0b01000000];
        let expected = UVINT32::new(Uvint32BitSize::ThirtyTwo, 2_147_483_649);
        let (_input, actual) = uvint32((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_svint12_bit_flag_zero_positive() {
        let input = [0b00001000];
        let expected = SVINT12::new(Svint12BitSize::Three, 1);
        let (_input, actual) = svint12((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_svint12_bit_flag_zero_negative() {
        let input = [0b00100000];
        let expected = SVINT12::new(Svint12BitSize::Three, -4);
        let (_input, actual) = svint12((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_svint12_bit_flag_three_positive() {
        let input = [0b11010000, 0b00000100];
        let expected = SVINT12::new(Svint12BitSize::Twelve, 1025);
        let (_input, actual) = svint12((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_svint12_bit_flag_three_negative() {
        let input = [0b11100000, 0b00000100];
        let expected = SVINT12::new(Svint12BitSize::Twelve, -2047);
        let (_input, actual) = svint12((&input, 0)).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_cdis_float() {
        assert!(false)
    }
}