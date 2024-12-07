use crate::constants::{EIGHT_BITS, FOUR_BITS, ONE_BIT, TWO_BITS};
use crate::parsing::BitInput;
use crate::BitBuffer;
use nom::IResult;
use num_traits::FromPrimitive;

pub(crate) trait VarInt {
    type BitSize;
    type InnerType;
    #[allow(dead_code)]
    fn new(bit_size: Self::BitSize, value: Self::InnerType) -> Self;
    fn bit_size(&self) -> usize;
    fn flag_bits_value(&self) -> u8;
    fn flag_bits_size(&self) -> usize;
    fn value(&self) -> Self::InnerType;
    #[allow(dead_code)]
    fn max_value(&self) -> Self::InnerType;
    #[allow(dead_code)]
    fn min_value(&self) -> Self::InnerType;
    fn record_length(&self) -> usize {
        self.flag_bits_size() + self.bit_size()
    }
}

/// 10.2.1 UVINT8
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UVINT8 {
    bit_size: Uvint8BitSize,
    pub value: u8,
}

impl Default for UVINT8 {
    fn default() -> Self {
        UVINT8::from(0)
    }
}

impl VarInt for UVINT8 {
    type BitSize = Uvint8BitSize;
    type InnerType = u8;

    /// Construct a new UVINT8 with the given bit size definition and value.
    /// There is no validation on whether the bit size and value match.
    /// As such, this constructor is mainly for testing purposes,
    /// hence it is not part of the public API of the library.
    fn new(bit_size: Self::BitSize, value: Self::InnerType) -> Self {
        Self { bit_size, value }
    }

    fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    fn flag_bits_value(&self) -> Self::InnerType {
        self.bit_size.into()
    }

    fn flag_bits_size(&self) -> usize {
        Self::BitSize::FLAG_BITS_SIZE
    }

    fn value(&self) -> Self::InnerType {
        self.value
    }

    fn max_value(&self) -> Self::InnerType {
        Self::InnerType::MAX
    }

    fn min_value(&self) -> Self::InnerType {
        Self::InnerType::MIN
    }
}

impl From<u8> for UVINT8 {
    fn from(value: u8) -> Self {
        let bit_size = match value {
            0..=15 => Uvint8BitSize::Four,
            16..=u8::MAX => Uvint8BitSize::Eight,
        };

        Self { bit_size, value }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum Uvint8BitSize {
    #[default]
    Four,
    Eight,
}

impl Uvint8BitSize {
    pub const FLAG_BITS_SIZE: usize = ONE_BIT;

    pub fn bit_size(&self) -> usize {
        match self {
            Uvint8BitSize::Four => FOUR_BITS,
            Uvint8BitSize::Eight => EIGHT_BITS,
        }
    }
}

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
            Uvint8BitSize::Four => 0,
            Uvint8BitSize::Eight => 1,
        }
    }
}

/// 10.2.2 UVINT16
#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct UVINT16 {
    bit_size: Uvint16BitSize,
    pub value: u16,
}

impl Default for UVINT16 {
    fn default() -> Self {
        UVINT16::from(0)
    }
}

impl VarInt for UVINT16 {
    type BitSize = Uvint16BitSize;
    type InnerType = u16;

    fn new(bit_size: Self::BitSize, value: Self::InnerType) -> Self {
        Self { bit_size, value }
    }

    fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    fn flag_bits_size(&self) -> usize {
        Self::BitSize::FLAG_BITS_SIZE
    }

    fn value(&self) -> Self::InnerType {
        self.value
    }

    fn max_value(&self) -> Self::InnerType {
        Self::InnerType::MAX
    }

    fn min_value(&self) -> Self::InnerType {
        Self::InnerType::MIN
    }
}

impl From<u16> for UVINT16 {
    fn from(value: u16) -> Self {
        let bit_size = match value {
            0..=255 => Uvint16BitSize::Eight,
            256..=2_047 => Uvint16BitSize::Eleven,
            2_048..=16_383 => Uvint16BitSize::Fourteen,
            16_384..=u16::MAX => Uvint16BitSize::Sixteen,
        };

        Self { bit_size, value }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub(crate) enum Uvint16BitSize {
    Eight,
    Eleven,
    Fourteen,
    Sixteen,
}

impl Uvint16BitSize {
    pub const FLAG_BITS_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Uvint16BitSize::Eight => 8,
            Uvint16BitSize::Eleven => 11,
            Uvint16BitSize::Fourteen => 14,
            Uvint16BitSize::Sixteen => 16,
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
            Uvint16BitSize::Eight => 0b00,
            Uvint16BitSize::Eleven => 0b01,
            Uvint16BitSize::Fourteen => 0b10,
            Uvint16BitSize::Sixteen => 0b11,
        }
    }
}

/// 10.2.3 UVINT32
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UVINT32 {
    bit_size: Uvint32BitSize,
    pub value: u32,
}

impl Default for UVINT32 {
    fn default() -> Self {
        UVINT32::from(0)
    }
}

impl VarInt for UVINT32 {
    type BitSize = Uvint32BitSize;
    type InnerType = u32;

    fn new(bit_size: Self::BitSize, value: Self::InnerType) -> Self {
        Self { bit_size, value }
    }

    fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    fn flag_bits_size(&self) -> usize {
        Self::BitSize::FLAG_BITS_SIZE
    }

    fn value(&self) -> Self::InnerType {
        self.value
    }

    fn max_value(&self) -> Self::InnerType {
        Self::InnerType::MAX
    }

    fn min_value(&self) -> Self::InnerType {
        Self::InnerType::MIN
    }
}

impl From<u32> for UVINT32 {
    fn from(value: u32) -> Self {
        let bit_size = match value {
            0..=255 => Uvint32BitSize::Eight,
            256..=32_767 => Uvint32BitSize::Fifteen,
            32_768..=262_143 => Uvint32BitSize::Eighteen,
            262_144..=u32::MAX => Uvint32BitSize::ThirtyTwo,
        };

        Self { bit_size, value }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum Uvint32BitSize {
    #[default]
    Eight,
    Fifteen,
    Eighteen,
    ThirtyTwo,
}

impl Uvint32BitSize {
    pub const FLAG_BITS_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Uvint32BitSize::Eight => 8,
            Uvint32BitSize::Fifteen => 15,
            Uvint32BitSize::Eighteen => 18,
            Uvint32BitSize::ThirtyTwo => 32,
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
            Uvint32BitSize::Eight => 0b00,
            Uvint32BitSize::Fifteen => 0b01,
            Uvint32BitSize::Eighteen => 0b10,
            Uvint32BitSize::ThirtyTwo => 0b11,
        }
    }
}

/// 10.2.4 SVINT12
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SVINT12 {
    bit_size: Svint12BitSize,
    pub value: i16,
}

impl VarInt for SVINT12 {
    type BitSize = Svint12BitSize;
    type InnerType = i16;

    fn new(bit_size: Self::BitSize, value: Self::InnerType) -> Self {
        Self { bit_size, value }
    }

    fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    fn flag_bits_size(&self) -> usize {
        Svint12BitSize::FLAG_BITS_SIZE
    }

    fn value(&self) -> Self::InnerType {
        self.value
    }

    fn max_value(&self) -> Self::InnerType {
        self.bit_size.max_value()
    }

    fn min_value(&self) -> Self::InnerType {
        self.bit_size.min_value()
    }
}

impl From<i16> for SVINT12 {
    fn from(value: i16) -> Self {
        let bit_size = match value {
            -4..=3 => Svint12BitSize::Three,
            -32..=31 => Svint12BitSize::Six,
            -256..=255 => Svint12BitSize::Nine,
            -2_048..=2_047 => Svint12BitSize::Twelve,
            _ => Svint12BitSize::Twelve,
        };

        Self { bit_size, value }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum Svint12BitSize {
    #[default]
    Three,
    Six,
    Nine,
    Twelve,
}

impl Svint12BitSize {
    pub const FLAG_BITS_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint12BitSize::Three => 3,
            Svint12BitSize::Six => 6,
            Svint12BitSize::Nine => 9,
            Svint12BitSize::Twelve => 12,
        }
    }

    pub fn min_value(&self) -> i16 {
        match self {
            Svint12BitSize::Three => -4,
            Svint12BitSize::Six => -32,
            Svint12BitSize::Nine => -256,
            Svint12BitSize::Twelve => -2_048,
        }
    }

    #[allow(dead_code)]
    pub fn max_value(&self) -> i16 {
        match self {
            Svint12BitSize::Three => 3,
            Svint12BitSize::Six => 31,
            Svint12BitSize::Nine => 255,
            Svint12BitSize::Twelve => 2_047,
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
            Svint12BitSize::Three => 0b00,
            Svint12BitSize::Six => 0b01,
            Svint12BitSize::Nine => 0b10,
            Svint12BitSize::Twelve => 0b11,
        }
    }
}

/// 10.2.5 SVINT13
#[derive(Copy, Clone, Default, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct SVINT13 {
    bit_size: Svint13BitSize,
    pub value: i16,
}

impl VarInt for SVINT13 {
    type BitSize = Svint13BitSize;

    type InnerType = i16;

    fn new(bit_size: Self::BitSize, value: Self::InnerType) -> Self {
        Self { bit_size, value }
    }

    fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    fn flag_bits_size(&self) -> usize {
        Svint13BitSize::FLAG_BITS_SIZE
    }

    fn value(&self) -> Self::InnerType {
        self.value
    }
    fn max_value(&self) -> Self::InnerType {
        self.bit_size.max_value()
    }

    fn min_value(&self) -> Self::InnerType {
        self.bit_size.min_value()
    }
}

impl From<i16> for SVINT13 {
    fn from(value: i16) -> Self {
        let bit_size = match value {
            -16..=15 => Svint13BitSize::Five,
            -64..=63 => Svint13BitSize::Seven,
            -512..=511 => Svint13BitSize::Ten,
            -4_096..=4_095 => Svint13BitSize::Thirteen,
            _ => Svint13BitSize::Thirteen,
        };

        Self { bit_size, value }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub(crate) enum Svint13BitSize {
    #[default]
    Five,
    Seven,
    Ten,
    Thirteen,
}

impl Svint13BitSize {
    pub const FLAG_BITS_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint13BitSize::Five => 5,
            Svint13BitSize::Seven => 7,
            Svint13BitSize::Ten => 10,
            Svint13BitSize::Thirteen => 13,
        }
    }

    pub fn min_value(&self) -> i16 {
        match self {
            Svint13BitSize::Five => -16,
            Svint13BitSize::Seven => -64,
            Svint13BitSize::Ten => -512,
            Svint13BitSize::Thirteen => -4_096,
        }
    }

    #[allow(dead_code)]
    pub fn max_value(&self) -> i16 {
        match self {
            Svint13BitSize::Five => 15,
            Svint13BitSize::Seven => 63,
            Svint13BitSize::Ten => 511,
            Svint13BitSize::Thirteen => 4_095,
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
            Svint13BitSize::Five => 0b00,
            Svint13BitSize::Seven => 0b01,
            Svint13BitSize::Ten => 0b10,
            Svint13BitSize::Thirteen => 0b11,
        }
    }
}

/// 10.2.6 SVINT14
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SVINT14 {
    bit_size: Svint14BitSize,
    pub value: i16,
}

impl VarInt for SVINT14 {
    type BitSize = Svint14BitSize;

    type InnerType = i16;

    fn new(bit_size: Self::BitSize, value: Self::InnerType) -> Self {
        Self { bit_size, value }
    }

    fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    fn flag_bits_size(&self) -> usize {
        Svint14BitSize::FLAG_BITS_SIZE
    }

    fn value(&self) -> Self::InnerType {
        self.value
    }
    fn max_value(&self) -> Self::InnerType {
        self.bit_size.max_value()
    }

    fn min_value(&self) -> Self::InnerType {
        self.bit_size.min_value()
    }
}

impl From<i16> for SVINT14 {
    fn from(value: i16) -> Self {
        let bit_size = match value {
            -8..=7 => Svint14BitSize::Four,
            -64..=63 => Svint14BitSize::Seven,
            -256..=255 => Svint14BitSize::Nine,
            -8_192..=8_191 => Svint14BitSize::Fourteen,
            _ => Svint14BitSize::Fourteen,
        };

        Self { bit_size, value }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum Svint14BitSize {
    #[default]
    Four,
    Seven,
    Nine,
    Fourteen,
}

impl Svint14BitSize {
    pub const FLAG_BITS_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint14BitSize::Four => 4,
            Svint14BitSize::Seven => 7,
            Svint14BitSize::Nine => 9,
            Svint14BitSize::Fourteen => 14,
        }
    }

    pub fn min_value(&self) -> i16 {
        match self {
            Svint14BitSize::Four => -8,
            Svint14BitSize::Seven => -64,
            Svint14BitSize::Nine => -256,
            Svint14BitSize::Fourteen => -8_192,
        }
    }

    #[allow(dead_code)]
    pub fn max_value(&self) -> i16 {
        match self {
            Svint14BitSize::Four => 7,
            Svint14BitSize::Seven => 63,
            Svint14BitSize::Nine => 255,
            Svint14BitSize::Fourteen => 8_191,
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
            Svint14BitSize::Four => 0b00,
            Svint14BitSize::Seven => 0b01,
            Svint14BitSize::Nine => 0b10,
            Svint14BitSize::Fourteen => 0b11,
        }
    }
}

/// 10.2.7 SVINT16
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SVINT16 {
    bit_size: Svint16BitSize,
    pub value: i16,
}

impl VarInt for SVINT16 {
    type BitSize = Svint16BitSize;

    type InnerType = i16;

    fn new(bit_size: Self::BitSize, value: Self::InnerType) -> Self {
        Self { bit_size, value }
    }

    fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    fn flag_bits_size(&self) -> usize {
        Svint16BitSize::FLAG_BITS_SIZE
    }

    fn value(&self) -> Self::InnerType {
        self.value
    }
    fn max_value(&self) -> Self::InnerType {
        self.bit_size.max_value()
    }

    fn min_value(&self) -> Self::InnerType {
        self.bit_size.min_value()
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

        Self { bit_size, value }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub(crate) enum Svint16BitSize {
    #[default]
    Eight,
    Twelve,
    Thirteen,
    Sixteen,
}

impl Svint16BitSize {
    pub const FLAG_BITS_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint16BitSize::Eight => 8,
            Svint16BitSize::Twelve => 12,
            Svint16BitSize::Thirteen => 13,
            Svint16BitSize::Sixteen => 16,
        }
    }

    pub fn min_value(&self) -> i16 {
        match self {
            Svint16BitSize::Eight => -128,
            Svint16BitSize::Twelve => -2_048,
            Svint16BitSize::Thirteen => -4_096,
            Svint16BitSize::Sixteen => i16::MIN,
        }
    }

    #[allow(dead_code)]
    pub fn max_value(&self) -> i16 {
        match self {
            Svint16BitSize::Eight => 127,
            Svint16BitSize::Twelve => 2_047,
            Svint16BitSize::Thirteen => 4_095,
            Svint16BitSize::Sixteen => i16::MAX,
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
            Svint16BitSize::Eight => 0b00,
            Svint16BitSize::Twelve => 0b01,
            Svint16BitSize::Thirteen => 0b10,
            Svint16BitSize::Sixteen => 0b11,
        }
    }
}

/// 10.2.8 SVINT24
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct SVINT24 {
    bit_size: Svint24BitSize,
    pub value: i32,
}

impl VarInt for SVINT24 {
    type BitSize = Svint24BitSize;

    type InnerType = i32;

    fn new(bit_size: Self::BitSize, value: Self::InnerType) -> Self {
        Self { bit_size, value }
    }

    fn bit_size(&self) -> usize {
        self.bit_size.bit_size()
    }

    fn flag_bits_value(&self) -> u8 {
        self.bit_size.into()
    }

    fn flag_bits_size(&self) -> usize {
        Svint24BitSize::FLAG_BITS_SIZE
    }

    fn value(&self) -> Self::InnerType {
        self.value
    }
    fn max_value(&self) -> Self::InnerType {
        self.bit_size.max_value()
    }

    fn min_value(&self) -> Self::InnerType {
        self.bit_size.min_value()
    }
}

impl From<i32> for SVINT24 {
    fn from(value: i32) -> Self {
        let bit_size = match value {
            -32_768..=32_767 => Svint24BitSize::Sixteen,
            -262_144..=262_143 => Svint24BitSize::Nineteen,
            -1_048_576..=1_048_575 => Svint24BitSize::TwentyOne,
            -8_388_608..=8_388_607 => Svint24BitSize::TwentyFour,
            _ => Svint24BitSize::TwentyFour,
        };

        Self { bit_size, value }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub(crate) enum Svint24BitSize {
    #[default]
    Sixteen,
    Nineteen,
    TwentyOne,
    TwentyFour,
}

impl Svint24BitSize {
    pub const FLAG_BITS_SIZE: usize = TWO_BITS;

    pub fn bit_size(&self) -> usize {
        match self {
            Svint24BitSize::Sixteen => 16,
            Svint24BitSize::Nineteen => 19,
            Svint24BitSize::TwentyOne => 21,
            Svint24BitSize::TwentyFour => 24,
        }
    }

    pub fn min_value(&self) -> i32 {
        match self {
            Svint24BitSize::Sixteen => -32_768,
            Svint24BitSize::Nineteen => -262_144,
            Svint24BitSize::TwentyOne => -1_048_576,
            Svint24BitSize::TwentyFour => 8_388_608,
        }
    }

    #[allow(dead_code)]
    pub fn max_value(&self) -> i32 {
        match self {
            Svint24BitSize::Sixteen => 32_767,
            Svint24BitSize::Nineteen => 262_143,
            Svint24BitSize::TwentyOne => 1_048_575,
            Svint24BitSize::TwentyFour => 8_388_607,
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
            Svint24BitSize::Sixteen => 0b00,
            Svint24BitSize::Nineteen => 0b01,
            Svint24BitSize::TwentyOne => 0b10,
            Svint24BitSize::TwentyFour => 0b11,
        }
    }
}

/// 10.3 Custom Floating Point Numbers
///
/// The `CdisFloat` trait models the mantissa and exponent components of a C-DIS float implementation.
/// The actual value is available through the `to_float()` method.
///
/// Because some use cases (such as Variable Parameter Articulated Part record) can be encoded either
/// as a `CdisFloat` or a regular 32-bit float, this struct allows to hold both variants.
/// There are two different constructor methods for this purpose (`new()` and `from_f64` respectively).
///
/// The intended way of using this trait is to implement it on a custom struct.
/// The trait impl for that struct then defines the bit sizes for mantissa and exponent.
pub(crate) trait CdisFloat {
    type Mantissa: FromPrimitive;
    type Exponent: FromPrimitive;
    type InnerFloat;

    const MANTISSA_BITS: usize;
    const EXPONENT_BITS: usize;

    fn new(mantissa: Self::Mantissa, exponent: Self::Exponent) -> Self;
    fn from_float(float: Self::InnerFloat) -> Self;
    fn to_float(&self) -> Self::InnerFloat;
    fn parse(input: BitInput) -> IResult<BitInput, Self>
    where
        Self: Sized;
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize;
}
