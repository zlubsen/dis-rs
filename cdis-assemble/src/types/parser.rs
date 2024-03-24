use nom::IResult;
use nom::bits::complete::take;
use nom::complete::bool;
use crate::constants::{ONE_BIT, TWO_BITS};
use crate::types::model::{SVINT12, Svint12BitSize, SVINT13, Svint13BitSize, SVINT14, Svint14BitSize, SVINT16, Svint16BitSize, SVINT24, Svint24BitSize, UVINT16, Uvint16BitSize, UVINT32, Uvint32BitSize, UVINT8, Uvint8BitSize};

pub(crate) fn uvint8(input: (&[u8], usize)) -> IResult<(&[u8], usize), UVINT8> {
    let (input, flag_bits):((&[u8], usize), u8) = take(ONE_BIT)(input)?;
    let num_bits_to_parse = Uvint8BitSize::from(flag_bits).bit_size();
    let (input, field_value):((&[u8], usize), u8) = take(num_bits_to_parse)(input)?;

    Ok((input, UVINT8::from(field_value)))
}

pub(crate) fn uvint16(input: (&[u8], usize)) -> IResult<(&[u8], usize), UVINT16> {
    let (input, flag_bits):((&[u8], usize), u8) = take(TWO_BITS)(input)?;
    let num_bits_to_parse = Uvint16BitSize::from(flag_bits).bit_size();
    let (input, field_value):((&[u8], usize), u16) = take(num_bits_to_parse)(input)?;

    Ok((input, UVINT16::from(field_value)))
}

pub(crate) fn uvint32(input: (&[u8], usize)) -> IResult<(&[u8], usize), UVINT32> {
    let (input, flag_bits):((&[u8], usize), u8) = take(TWO_BITS)(input)?;
    let num_bits_to_parse = Uvint32BitSize::from(flag_bits).bit_size();
    let (input, field_value):((&[u8], usize), u32) = take(num_bits_to_parse)(input)?;

    Ok((input, UVINT32::from(field_value)))
}

pub(crate) fn svint12(input: (&[u8], usize)) -> IResult<(&[u8], usize), SVINT12> {
    let (input, flag_bits) : ((&[u8], usize), u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint12BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : ((&[u8], usize), bool) = bool(input)?;
    let (input, field_value) : ((&[u8], usize), i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT12::from(field_value)))
}

pub(crate) fn svint13(input: (&[u8], usize)) -> IResult<(&[u8], usize), SVINT13> {
    let (input, flag_bits) : ((&[u8], usize), u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint13BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : ((&[u8], usize), bool) = bool(input)?;
    let (input, field_value) : ((&[u8], usize), i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT13::from(field_value)))
}

pub(crate) fn svint14(input: (&[u8], usize)) -> IResult<(&[u8], usize), SVINT14> {
    let (input, flag_bits) : ((&[u8], usize), u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint14BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : ((&[u8], usize), bool) = bool(input)?;
    let (input, field_value) : ((&[u8], usize), i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT14::from(field_value)))
}

pub(crate) fn svint16(input: (&[u8], usize)) -> IResult<(&[u8], usize), SVINT16> {
    let (input, flag_bits) : ((&[u8], usize), u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint16BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : ((&[u8], usize), bool) = bool(input)?;
    let (input, field_value) : ((&[u8], usize), i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT16::from(field_value)))
}

pub(crate) fn svint24(input: (&[u8], usize)) -> IResult<(&[u8], usize), SVINT24> {
    let (input, flag_bits) : ((&[u8], usize), u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint24BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit) : ((&[u8], usize), bool) = bool(input)?;
    let (input, field_value) : ((&[u8], usize), i32) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT24::from(field_value)))
}

#[cfg(test)]
mod tests {
    use crate::types::parser::{svint12, uvint16, uvint32, uvint8};
    use crate::types::model::{SVINT12, Svint12BitSize, UVINT16, Uvint16BitSize, UVINT32, Uvint32BitSize, UVINT8, Uvint8BitSize};

    // #[test]
    // fn parse_generic_varint_bit_flag_zero() {
    //     let input = [0b00001000];
    //     let expected = UVINT8::new(Uvint8BitSize::Four, 1);
    //     let (input, actual) : ((&[u8], usize), UVINT8) = varint((&input, 0)).unwrap();
    //
    //     assert_eq!(expected, actual);
    // }

    #[test]
    fn parse_uvint8_bit_flag_zero() {
        let input = [0b00001000];
        let expected = UVINT8::new(Uvint8BitSize::Four, 1);
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
}