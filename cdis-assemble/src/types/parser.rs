use crate::constants::{ONE_BIT, THIRTY_TWO_BITS, TWO_BITS};
use crate::parsing::BitInput;
use crate::types::model::{
    Svint12BitSize, Svint13BitSize, Svint14BitSize, Svint16BitSize, Svint24BitSize, Uvint16BitSize,
    Uvint32BitSize, Uvint8BitSize, SVINT12, SVINT13, SVINT14, SVINT16, SVINT24, UVINT16, UVINT32,
    UVINT8,
};
use dis_rs::model::ClockTime;
use nom::bits::complete::take;
use nom::complete::bool;
use nom::IResult;

pub(crate) fn uvint8(input: BitInput) -> IResult<BitInput, UVINT8> {
    let (input, flag_bits): (BitInput, u8) = take(ONE_BIT)(input)?;
    let num_bits_to_parse = Uvint8BitSize::from(flag_bits).bit_size();
    let (input, field_value): (BitInput, u8) = take(num_bits_to_parse)(input)?;

    Ok((input, UVINT8::from(field_value)))
}

pub(crate) fn uvint16(input: BitInput) -> IResult<BitInput, UVINT16> {
    let (input, flag_bits): (BitInput, u8) = take(TWO_BITS)(input)?;
    let num_bits_to_parse = Uvint16BitSize::from(flag_bits).bit_size();
    let (input, field_value): (BitInput, u16) = take(num_bits_to_parse)(input)?;

    Ok((input, UVINT16::from(field_value)))
}

pub(crate) fn uvint32(input: BitInput) -> IResult<BitInput, UVINT32> {
    let (input, flag_bits): (BitInput, u8) = take(TWO_BITS)(input)?;
    let num_bits_to_parse = Uvint32BitSize::from(flag_bits).bit_size();
    let (input, field_value): (BitInput, u32) = take(num_bits_to_parse)(input)?;

    Ok((input, UVINT32::from(field_value)))
}

pub(crate) fn svint12(input: BitInput) -> IResult<BitInput, SVINT12> {
    let (input, flag_bits): (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint12BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit): (BitInput, bool) = bool(input)?;
    let (input, field_value): (BitInput, i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT12::from(field_value)))
}

pub(crate) fn svint13(input: BitInput) -> IResult<BitInput, SVINT13> {
    let (input, flag_bits): (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint13BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit): (BitInput, bool) = bool(input)?;
    let (input, field_value): (BitInput, i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT13::from(field_value)))
}

pub(crate) fn svint14(input: BitInput) -> IResult<BitInput, SVINT14> {
    let (input, flag_bits): (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint14BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit): (BitInput, bool) = bool(input)?;
    let (input, field_value): (BitInput, i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT14::from(field_value)))
}

pub(crate) fn svint16(input: BitInput) -> IResult<BitInput, SVINT16> {
    let (input, flag_bits): (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint16BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit): (BitInput, bool) = bool(input)?;
    let (input, field_value): (BitInput, i16) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT16::from(field_value)))
}

pub(crate) fn svint24(input: BitInput) -> IResult<BitInput, SVINT24> {
    let (input, flag_bits): (BitInput, u8) = take(TWO_BITS)(input)?;
    let bit_size = Svint24BitSize::from(flag_bits);
    let num_bits_to_parse = bit_size.bit_size() - 1;
    let (input, sign_bit): (BitInput, bool) = bool(input)?;
    let (input, field_value): (BitInput, i32) = take(num_bits_to_parse)(input)?;
    let field_value = if sign_bit { bit_size.min_value() } else { 0 } + field_value;

    Ok((input, SVINT24::from(field_value)))
}

/// Parses a C-DIS Clock Time Record (11.4).
pub(crate) fn clock_time(input: BitInput) -> IResult<BitInput, ClockTime> {
    let (input, hour): (BitInput, i32) = take(THIRTY_TWO_BITS)(input)?;
    let (input, time_past_hour): (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;

    let time = ClockTime::new(hour, time_past_hour);
    Ok((input, time))
}

#[cfg(test)]
mod tests {
    use crate::constants::{FOURTEEN_BITS, THREE_BITS};
    use crate::parsing::{take_signed, BitInput};
    use crate::types::model::VarInt;
    use crate::types::model::{
        CdisFloat, Svint12BitSize, Uvint16BitSize, Uvint32BitSize, Uvint8BitSize, SVINT12, UVINT16,
        UVINT32, UVINT8,
    };
    use crate::types::parser::{svint12, uvint16, uvint32, uvint8};
    use crate::writing::write_value_signed;
    use crate::BitBuffer;
    use nom::IResult;

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

    pub struct TestFloat {
        mantissa: i32,
        exponent: i8,
    }

    impl CdisFloat for TestFloat {
        type Mantissa = i32;
        type Exponent = i8;
        type InnerFloat = f32;

        const MANTISSA_BITS: usize = FOURTEEN_BITS;
        const EXPONENT_BITS: usize = THREE_BITS;

        fn new(mantissa: Self::Mantissa, exponent: Self::Exponent) -> Self {
            Self { mantissa, exponent }
        }

        fn from_float(float: Self::InnerFloat) -> Self {
            let mut mantissa = float;
            let mut exponent = 0i32;
            let max_mantissa = 2f32.powi(Self::MANTISSA_BITS as i32) - 1.0;
            while (mantissa > max_mantissa) & (exponent as usize <= Self::EXPONENT_BITS) {
                mantissa /= 10.0;
                exponent += 1;
            }

            Self {
                mantissa: mantissa as Self::Mantissa,
                exponent: exponent as Self::Exponent,
            }
        }

        fn to_float(&self) -> Self::InnerFloat {
            self.mantissa as f32 * 10f32.powf(f32::from(self.exponent))
        }

        fn parse(input: BitInput) -> IResult<BitInput, Self> {
            let (input, mantissa) = take_signed(Self::MANTISSA_BITS)(input)?;
            let (input, exponent) = take_signed(Self::EXPONENT_BITS)(input)?;

            Ok((
                input,
                Self {
                    mantissa: mantissa as Self::Mantissa,
                    exponent: exponent as Self::Exponent,
                },
            ))
        }

        #[allow(clippy::let_and_return)]
        fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
            let cursor = write_value_signed(buf, cursor, Self::MANTISSA_BITS, self.mantissa);
            let cursor = write_value_signed(buf, cursor, Self::EXPONENT_BITS, self.exponent);

            cursor
        }
    }

    #[test]
    fn parse_cdis_float() {
        let input = [0b00000000, 0b00000100, 0b10000000];

        let (_input, float) = TestFloat::parse((&input, 0)).unwrap();
        assert_eq!(float.mantissa, 1);
        assert_eq!(float.exponent, 1);
        let expected = 1f32 * 10f32.powi(1);
        assert_eq!(float.to_float(), expected);
    }

    #[test]
    fn cdis_float_from_f32() {
        let float = 1234567f32;
        let cdis_float = TestFloat::from_float(float);

        assert_eq!(cdis_float.mantissa, 12345);
        assert_eq!(cdis_float.exponent, 2);
    }
}
