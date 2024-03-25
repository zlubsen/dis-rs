use crate::constants::{EIGHT_BITS, FOUR_BITS, TWO_BITS};

/// 10.2.1 UVINT8
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UVINT8 {
    bit_size: Uvint8BitSize,
    pub value: u8,
}

impl UVINT8 {
    /// Construct a new UVINT8 with the given bit size definition and value.
    /// There is no validation on whether the bit size and value match.
    /// As such, this constructor is mainly for testing purposes,
    /// hence it is not part of the public API of the library.
    pub(crate) fn new_scaled(bit_size: Uvint8BitSize, value: u8) -> Self {
        Self {
            bit_size,
            value,
        }
    }

    pub(crate) fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    pub(crate) fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }
}

impl From<u8> for UVINT8 {
    fn from(value: u8) -> Self {
        let bit_size = match value {
            0..=15 => Uvint8BitSize::Four,
            ..=u8::MAX => Uvint8BitSize::Eight,
        };

        Self {
            bit_size,
            value
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Uvint8BitSize {
    Four,
    Eight,
}

impl Uvint8BitSize {
    pub const FLAG_BITS: usize = 1;

    pub fn bit_size(&self) -> usize {
        match self {
            Uvint8BitSize::Four => { FOUR_BITS }
            Uvint8BitSize::Eight => { EIGHT_BITS }
        }
    }
}

// impl BitSize for Uvint8BitSize {
//     const FLAG_BITS: usize = 1;
//
//     fn bit_size(&self) -> usize {
//         match self {
//             Uvint8BitSize::Four => { FOUR_BITS }
//             Uvint8BitSize::Eight => { EIGHT_BITS }
//         }
//     }
// }

impl From<u8> for Uvint8BitSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Uvint8BitSize::Four,
            _ => Uvint8BitSize::Eight,
        }
    }
}

impl From<Uvint8BitSize> for u8 {
    fn from(value: Uvint8BitSize) -> Self {
        match value {
            Uvint8BitSize::Four => { 0 }
            Uvint8BitSize::Eight => { 1 }
        }
    }
}

/// 10.2.2 UVINT16
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UVINT16 {
    bit_size: Uvint16BitSize,
    pub value: u16,
}

impl UVINT16 {
    pub(crate) fn new(bit_size: Uvint16BitSize, value: u16) -> Self {
        Self {
            bit_size,
            value,
        }
    }

    pub(crate) fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    pub(crate) fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }
}

impl From<u16> for UVINT16 {
    fn from(value: u16) -> Self {
        let bit_size = match value {
            0..=255 => Uvint16BitSize::Eight,
            ..=2_047 => Uvint16BitSize::Eleven,
            ..=16_383 => Uvint16BitSize::Fourteen,
            ..=u16::MAX => Uvint16BitSize::Sixteen,
        };

        Self {
            bit_size,
            value
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Uvint16BitSize {
    Eight,
    Eleven,
    Fourteen,
    Sixteen,
}

impl Uvint16BitSize {
    pub const FLAG_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Uvint16BitSize::Eight => { 8 }
            Uvint16BitSize::Eleven => { 11 }
            Uvint16BitSize::Fourteen => { 14 }
            Uvint16BitSize::Sixteen => { 16 }
        }
    }
}

impl From<u8> for Uvint16BitSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Uvint16BitSize::Eight,
            1 => Uvint16BitSize::Eleven,
            2 => Uvint16BitSize::Fourteen,
            _ => Uvint16BitSize::Sixteen,
        }
    }
}

impl From<Uvint16BitSize> for u8 {
    fn from(value: Uvint16BitSize) -> Self {
        match value {
            Uvint16BitSize::Eight => { 0b00 }
            Uvint16BitSize::Eleven => { 0b01 }
            Uvint16BitSize::Fourteen => { 0b10 }
            Uvint16BitSize::Sixteen => { 0b11 }
        }
    }
}

/// 10.2.3 UVINT32
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UVINT32 {
    bit_size: Uvint32BitSize,
    pub value: u32,
}

impl UVINT32 {
    pub(crate) fn new(bit_size: Uvint32BitSize, value: u32) -> Self {
        Self {
            bit_size,
            value,
        }
    }

    pub(crate) fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    pub(crate) fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }
}

impl From<u32> for UVINT32 {
    fn from(value: u32) -> Self {
        let bit_size = match value {
            0..=255 => Uvint32BitSize::Eight,
            ..=32_767 => Uvint32BitSize::Fifteen,
            ..=262_143 => Uvint32BitSize::Eighteen,
            ..=u32::MAX => Uvint32BitSize::ThirtyTwo,
        };

        Self {
            bit_size,
            value
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Uvint32BitSize {
    Eight,
    Fifteen,
    Eighteen,
    ThirtyTwo,
}

impl Uvint32BitSize {
    pub const FLAG_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Uvint32BitSize::Eight => { 8 }
            Uvint32BitSize::Fifteen => { 15 }
            Uvint32BitSize::Eighteen => { 18 }
            Uvint32BitSize::ThirtyTwo => { 32 }
        }
    }
}

impl From<u8> for Uvint32BitSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Uvint32BitSize::Eight,
            1 => Uvint32BitSize::Fifteen,
            2 => Uvint32BitSize::Eighteen,
            _ => Uvint32BitSize::ThirtyTwo,
        }
    }
}

impl From<Uvint32BitSize> for u8 {
    fn from(value: Uvint32BitSize) -> Self {
        match value {
            Uvint32BitSize::Eight => { 0b00 }
            Uvint32BitSize::Fifteen => { 0b01 }
            Uvint32BitSize::Eighteen => { 0b10 }
            Uvint32BitSize::ThirtyTwo => { 0b11 }
        }
    }
}

/// 10.2.4 SVINT12
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SVINT12 {
    bit_size: Svint12BitSize,
    pub value: i16,
}

impl SVINT12 {
    pub(crate) fn new(bit_size: Svint12BitSize, value: i16) -> Self {
        Self {
            bit_size,
            value,
        }
    }

    pub(crate) fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    pub(crate) fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    pub(crate) const fn flag_bits_size(&self) -> usize {
        Svint12BitSize::FLAG_SIZE
    }

    pub(crate) fn min_value(&self) -> i16 {
        self.bit_size.min_value()
    }

    pub(crate) fn max_value(&self) -> i16 {
        self.bit_size.max_value()
    }
}

impl From<i16> for SVINT12 {
    fn from(value: i16) -> Self {
        let bit_size = match value {
            -4..=3 => Svint12BitSize::Three,
            -32..=31 => Svint12BitSize::Six,
            -256..=255 => Svint12BitSize::Nine,
            -2_048..=2_047 => Svint12BitSize::Twelve,
            _ => { Svint12BitSize::Twelve }
        };

        Self {
            bit_size,
            value
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Svint12BitSize {
    Three,
    Six,
    Nine,
    Twelve,
}

impl Svint12BitSize {
    pub const FLAG_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint12BitSize::Three => { 3 }
            Svint12BitSize::Six => { 6 }
            Svint12BitSize::Nine => { 9 }
            Svint12BitSize::Twelve => { 12 }
        }
    }

    pub fn min_value(&self) -> i16 {
        match self {
            Svint12BitSize::Three => { -4 }
            Svint12BitSize::Six => { -32 }
            Svint12BitSize::Nine => { -256 }
            Svint12BitSize::Twelve => { -2_048 }
        }
    }

    pub fn max_value(&self) -> i16 {
        match self {
            Svint12BitSize::Three => { 3 }
            Svint12BitSize::Six => { 31 }
            Svint12BitSize::Nine => { 255 }
            Svint12BitSize::Twelve => { 2_047 }
        }
    }
}

impl From<u8> for Svint12BitSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Svint12BitSize::Three,
            1 => Svint12BitSize::Six,
            2 => Svint12BitSize::Nine,
            _ => Svint12BitSize::Twelve,
        }
    }
}

impl From<Svint12BitSize> for u8 {
    fn from(value: Svint12BitSize) -> Self {
        match value {
            Svint12BitSize::Three => { 0b00 }
            Svint12BitSize::Six => { 0b01 }
            Svint12BitSize::Nine => { 0b10 }
            Svint12BitSize::Twelve => { 0b11 }
        }
    }
}

/// 10.2.5 SVINT13
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SVINT13 {
    bit_size: Svint13BitSize,
    pub value: i16,
}

impl SVINT13 {
    pub(crate) fn new(bit_size: Svint13BitSize, value: i16) -> Self {
        Self {
            bit_size,
            value,
        }
    }

    pub(crate) fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    pub(crate) fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    pub(crate) const fn flag_bits_size(&self) -> usize {
        Svint13BitSize::FLAG_SIZE
    }
}

impl From<i16> for SVINT13 {
    fn from(value: i16) -> Self {
        let bit_size = match value {
            -16..=15 => Svint13BitSize::Five,
            -64..=63 => Svint13BitSize::Seven,
            -512..=511 => Svint13BitSize::Ten,
            -4_096..=4_095 => Svint13BitSize::Thirteen,
            _ => { Svint13BitSize::Thirteen }
        };

        Self {
            bit_size,
            value
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Svint13BitSize {
    Five,
    Seven,
    Ten,
    Thirteen,
}

impl Svint13BitSize {
    pub const FLAG_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint13BitSize::Five => { 5 }
            Svint13BitSize::Seven => { 7 }
            Svint13BitSize::Ten => { 10 }
            Svint13BitSize::Thirteen => { 13 }
        }
    }

    pub fn min_value(&self) -> i16 {
        match self {
            Svint13BitSize::Five => { -16 }
            Svint13BitSize::Seven => { -64 }
            Svint13BitSize::Ten => { -512 }
            Svint13BitSize::Thirteen => { -4_096 }
        }
    }

    pub fn max_value(&self) -> i16 {
        match self {
            Svint13BitSize::Five => { 15 }
            Svint13BitSize::Seven => { 63 }
            Svint13BitSize::Ten => { 511 }
            Svint13BitSize::Thirteen => { 4_095 }
        }
    }
}

impl From<u8> for Svint13BitSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Svint13BitSize::Five,
            1 => Svint13BitSize::Seven,
            2 => Svint13BitSize::Ten,
            _ => Svint13BitSize::Thirteen,
        }
    }
}

impl From<Svint13BitSize> for u8 {
    fn from(value: Svint13BitSize) -> Self {
        match value {
            Svint13BitSize::Five => { 0b00 }
            Svint13BitSize::Seven => { 0b01 }
            Svint13BitSize::Ten => { 0b10 }
            Svint13BitSize::Thirteen => { 0b11 }
        }
    }
}

/// 10.2.6 SVINT14
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SVINT14 {
    bit_size: Svint14BitSize,
    pub value: i16,
}

impl SVINT14 {
    pub(crate) fn new(bit_size: Svint14BitSize, value: i16) -> Self {
        Self {
            bit_size,
            value,
        }
    }

    pub(crate) fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    pub(crate) fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    pub(crate) const fn flag_bits_size(&self) -> usize {
        Svint14BitSize::FLAG_SIZE
    }
}

impl From<i16> for SVINT14 {
    fn from(value: i16) -> Self {
        let bit_size = match value {
            -8..=7 => Svint14BitSize::Four,
            -64..=63 => Svint14BitSize::Seven,
            -256..=255 => Svint14BitSize::Nine,
            -8_192..=8_191 => Svint14BitSize::Fourteen,
            _ => { Svint14BitSize::Fourteen }
        };

        Self {
            bit_size,
            value
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Svint14BitSize {
    Four,
    Seven,
    Nine,
    Fourteen,
}

impl Svint14BitSize {
    pub const FLAG_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint14BitSize::Four => { 4 }
            Svint14BitSize::Seven => { 7 }
            Svint14BitSize::Nine => { 9 }
            Svint14BitSize::Fourteen => { 14 }
        }
    }

    pub fn min_value(&self) -> i16 {
        match self {
            Svint14BitSize::Four => { -8 }
            Svint14BitSize::Seven => { -64 }
            Svint14BitSize::Nine => { -256 }
            Svint14BitSize::Fourteen => { -8_192 }
        }
    }

    pub fn max_value(&self) -> i16 {
        match self {
            Svint14BitSize::Four => { 7 }
            Svint14BitSize::Seven => { 63 }
            Svint14BitSize::Nine => { 255 }
            Svint14BitSize::Fourteen => { 8_191 }
        }
    }
}

impl From<u8> for Svint14BitSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Svint14BitSize::Four,
            1 => Svint14BitSize::Seven,
            2 => Svint14BitSize::Nine,
            _ => Svint14BitSize::Fourteen,
        }
    }
}

impl From<Svint14BitSize> for u8 {
    fn from(value: Svint14BitSize) -> Self {
        match value {
            Svint14BitSize::Four => { 0b00 }
            Svint14BitSize::Seven => { 0b01 }
            Svint14BitSize::Nine => { 0b10 }
            Svint14BitSize::Fourteen => { 0b11 }
        }
    }
}

/// 10.2.7 SVINT16
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SVINT16 {
    bit_size: Svint16BitSize,
    pub value: i16,
}

impl SVINT16 {
    pub(crate) fn new(bit_size: Svint16BitSize, value: i16) -> Self {
        Self {
            bit_size,
            value,
        }
    }

    pub(crate) fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    pub(crate) fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    pub(crate) const fn flag_bits_size(&self) -> usize {
        Svint16BitSize::FLAG_SIZE
    }
}

impl From<i16> for SVINT16 {
    fn from(value: i16) -> Self {
        let bit_size = match value {
            -128..=127 => Svint16BitSize::Eight,
            -2_048..=2_047 => Svint16BitSize::Twelve,
            -4_096..=4_095 => Svint16BitSize::Thirteen,
            i16::MIN..=i16::MAX => Svint16BitSize::Sixteen,
        };

        Self {
            bit_size,
            value
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Svint16BitSize {
    Eight,
    Twelve,
    Thirteen,
    Sixteen,
}

impl Svint16BitSize {
    pub const FLAG_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint16BitSize::Eight => { 8 }
            Svint16BitSize::Twelve => { 12 }
            Svint16BitSize::Thirteen => { 13 }
            Svint16BitSize::Sixteen => { 16 }
        }
    }

    pub fn min_value(&self) -> i16 {
        match self {
            Svint16BitSize::Eight => { -128 }
            Svint16BitSize::Twelve => { -2_048 }
            Svint16BitSize::Thirteen => { -4_096 }
            Svint16BitSize::Sixteen => { i16::MIN }
        }
    }

    pub fn max_value(&self) -> i16 {
        match self {
            Svint16BitSize::Eight => { 127 }
            Svint16BitSize::Twelve => { 2_047 }
            Svint16BitSize::Thirteen => { 4_095 }
            Svint16BitSize::Sixteen => { i16::MAX }
        }
    }
}

impl From<u8> for Svint16BitSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Svint16BitSize::Eight,
            1 => Svint16BitSize::Twelve,
            2 => Svint16BitSize::Thirteen,
            _ => Svint16BitSize::Sixteen,
        }
    }
}

impl From<Svint16BitSize> for u8 {
    fn from(value: Svint16BitSize) -> Self {
        match value {
            Svint16BitSize::Eight => { 0b00 }
            Svint16BitSize::Twelve => { 0b01 }
            Svint16BitSize::Thirteen => { 0b10 }
            Svint16BitSize::Sixteen => { 0b11 }
        }
    }
}

/// 10.2.8 SVINT24
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SVINT24 {
    bit_size: Svint24BitSize,
    pub value: i32,
}

impl SVINT24 {
    pub(crate) fn new(bit_size: Svint24BitSize, value: i32) -> Self {
        Self {
            bit_size,
            value,
        }
    }

    pub(crate) fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    pub(crate) fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    pub(crate) const fn flag_bits_size(&self) -> usize {
        Svint24BitSize::FLAG_SIZE
    }
}

impl From<i32> for SVINT24 {
    fn from(value: i32) -> Self {
        let bit_size = match value {
            -32_768..=32_767 => Svint24BitSize::Sixteen,
            -262_144..=262_143 => Svint24BitSize::Nineteen,
            -1_048_576..=1_048_575 => Svint24BitSize::TwentyOne,
            -8_388_608..=8_388_607 => Svint24BitSize::TwentyFour,
            _ => { Svint24BitSize::TwentyFour }
        };

        Self {
            bit_size,
            value
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Svint24BitSize {
    Sixteen,
    Nineteen,
    TwentyOne,
    TwentyFour,
}

impl Svint24BitSize {
    pub const FLAG_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint24BitSize::Sixteen => { 16 }
            Svint24BitSize::Nineteen => { 19 }
            Svint24BitSize::TwentyOne => { 21 }
            Svint24BitSize::TwentyFour => { 24 }
        }
    }

    pub fn min_value(&self) -> i32 {
        match self {
            Svint24BitSize::Sixteen => { -32_768 }
            Svint24BitSize::Nineteen => { -262_144 }
            Svint24BitSize::TwentyOne => { -1_048_576 }
            Svint24BitSize::TwentyFour => { 8_388_608 }
        }
    }

    pub fn max_value(&self) -> i32 {
        match self {
            Svint24BitSize::Sixteen => { 32_767 }
            Svint24BitSize::Nineteen => { 262_143 }
            Svint24BitSize::TwentyOne => { 1_048_575 }
            Svint24BitSize::TwentyFour => { 8_388_607 }
        }
    }
}

impl From<u8> for Svint24BitSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Svint24BitSize::Sixteen,
            1 => Svint24BitSize::Nineteen,
            2 => Svint24BitSize::TwentyOne,
            _ => Svint24BitSize::TwentyFour,
        }
    }
}

impl From<Svint24BitSize> for u8 {
    fn from(value: Svint24BitSize) -> Self {
        match value {
            Svint24BitSize::Sixteen => { 0b00 }
            Svint24BitSize::Nineteen => { 0b01 }
            Svint24BitSize::TwentyOne => { 0b10 }
            Svint24BitSize::TwentyFour => { 0b11 }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CdisFloat {
    mantissa: i32,
    exponent: i8,
}

impl CdisFloat {
    pub fn new(mantissa: i32, exponent: i8) -> Self {
        Self {
            mantissa,
            exponent,
        }
    }

    pub fn to_value(&self) -> f64 {
        self.mantissa as f64 * ((10^(self.exponent)) as f64)
    }
}

// pub(crate) trait VarInt {
//     type Size: BitSize + From<Self::InnerType>;
//     type InnerType;
//
//     fn new(bit_size: Self::Size, value: Self::InnerType) -> Self;
//     fn bit_size(&self) -> usize;
//     fn flag_bits_value(&self) -> Self::InnerType;
//     fn flag_bits_size() -> usize;
// }
//
// pub(crate) trait BitSize {
//     const FLAG_BITS: usize;
//
//     fn bit_size(&self) -> usize;
// }

// fn test() {
//     let aap = Uvint8Type {
//         varint_type: Zero,
//         value: 5,
//     };
// }
//
// type Uvint8Type = VarInt<Uvint8Def>;
//
// pub struct VarInt<T: TypeDef> {
//     varint_type: T,
//     value: T::Output,
// }
//
// impl From<T:Output> for VarInt<T> {
//     fn from(value: T) -> Self {
//     }
// }
//
// pub trait TypeDef {
//     type Output;
//     fn flag_bits_size() -> usize;
//     fn sign_bit() -> Option<bool>;
//     fn bit_size(&self) -> usize;
//     fn min_value(&self) -> Self::Output;
//     fn max_value(&self) -> Self::Output;
// }
//
// pub enum Uvint8Def {
//     Zero,
//     One,
// }
//
// impl TypeDef for Uvint8Def {
//     type Output = u8;
//
//     fn flag_bits_size() -> usize {
//         ONE_BIT
//     }
//
//     fn sign_bit() -> Option<bool> {
//         None
//     }
//
//     fn bit_size(&self) -> usize {
//         match self {
//             Uvint8Def::Zero => { 4 }
//             Uvint8Def::One => { 8 }
//         }
//     }
//
//     fn min_value(&self) -> Self::Output {
//         match self {
//             Uvint8Def::Zero => { 0 }
//             Uvint8Def::One => { 0 }
//         }
//     }
//
//     fn max_value(&self) -> Self::Output {
//         match self {
//             Uvint8Def::Zero => { 15 }
//             Uvint8Def::One => { 255 }
//         }
//     }
// }
//
// impl From<u8> for Uvint8Def {
//     fn from(value: u8) -> Self {
//         match value {
//             0 => Uvint8Def::Zero,
//             _ => Uvint8Def::One,
//         }
//     }
// }

// impl VarInt for UVINT8 {
//     type Size = Uvint8BitSize;
//     type InnerType = u8;
//
//     fn new(bit_size: Self::Size, value: Self::InnerType) -> Self {
//         Self {
//             bit_size,
//             value,
//         }
//     }
//
//     fn bit_size(&self) -> usize {
//         self.bit_size.bit_size()
//     }
//
//     fn flag_bits_value(&self) -> Self::InnerType {
//         self.bit_size.into()
//     }
//
//     fn flag_bits_size() -> usize {
//         Self::Size::FLAG_BITS
//     }
// }