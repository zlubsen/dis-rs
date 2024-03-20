/// 10.2.1 UVINT8
pub struct UVINT8 {
    bit_size: Uvint8BitSize,
    pub value: u8,
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

pub(crate) enum Uvint8BitSize {
    Four,
    Eight,
}

impl From<u8> for Uvint8BitSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Uvint8BitSize::Four,
            _ => Uvint8BitSize::Eight,
        }
    }
}

/// 10.2.2 UVINT16
pub struct UVINT16 {
    bit_size: Uvint16BitSize,
    pub value: u16,
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

pub(crate) enum Uvint16BitSize {
    Eight,
    Eleven,
    Fourteen,
    Sixteen,
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

/// 10.2.3 UVINT32
pub struct UVINT32 {
    bit_size: Uvint32BitSize,
    pub value: u32,
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

pub(crate) enum Uvint32BitSize {
    Eight,
    Fifteen,
    Eighteen,
    ThirtyTwo,
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

/// 10.2.4 SVINT12
pub struct SVINT12 {
    bit_size: Svint12BitSize,
    pub value: i16,
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

pub(crate) enum Svint12BitSize {
    Three,
    Six,
    Nine,
    Twelve,
}

impl From<i16> for Svint12BitSize {
    fn from(value: i16) -> Self {
        match value {
            0 => Svint12BitSize::Three,
            1 => Svint12BitSize::Six,
            2 => Svint12BitSize::Nine,
            _ => Svint12BitSize::Twelve,
        }
    }
}

/// 10.2.5 SVINT13
pub struct SVINT13 {
    bit_size: Svint13BitSize,
    pub value: i16,
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

pub(crate) enum Svint13BitSize {
    Five,
    Seven,
    Ten,
    Thirteen,
}

impl From<i16> for Svint13BitSize {
    fn from(value: i16) -> Self {
        match value {
            0 => Svint13BitSize::Five,
            1 => Svint13BitSize::Seven,
            2 => Svint13BitSize::Ten,
            _ => Svint13BitSize::Thirteen,
        }
    }
}

/// 10.2.6 SVINT14
pub struct SVINT14 {
    bit_size: Svint14BitSize,
    pub value: i16,
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

pub(crate) enum Svint14BitSize {
    Four,
    Seven,
    Nine,
    Fourteen,
}

impl From<i16> for Svint14BitSize {
    fn from(value: i16) -> Self {
        match value {
            0 => Svint14BitSize::Four,
            1 => Svint14BitSize::Seven,
            2 => Svint14BitSize::Nine,
            _ => Svint14BitSize::Fourteen,
        }
    }
}

/// 10.2.7 SVINT16
pub struct SVINT16 {
    bit_size: Svint16BitSize,
    pub value: i16,
}

impl From<i16> for SVINT16 {
    fn from(value: i16) -> Self {
        let bit_size = match value {
            -128..=127 => Svint16BitSize::Eight,
            -2_048..=2_047 => Svint16BitSize::Twelve,
            -4_096..=4_095 => Svint16BitSize::Thirteen,
            -32_768..=32_767 => Svint16BitSize::Sixteen,
            _ => { Svint16BitSize::Sixteen }
        };

        Self {
            bit_size,
            value
        }
    }
}

pub(crate) enum Svint16BitSize {
    Eight,
    Twelve,
    Thirteen,
    Sixteen,
}

impl From<i16> for Svint16BitSize {
    fn from(value: i16) -> Self {
        match value {
            0 => Svint16BitSize::Eight,
            1 => Svint16BitSize::Twelve,
            2 => Svint16BitSize::Thirteen,
            _ => Svint16BitSize::Sixteen,
        }
    }
}

/// 10.2.8 SVINT24
pub struct SVINT24 {
    bit_size: Svint24BitSize,
    pub value: i32,
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

pub(crate) enum Svint24BitSize {
    Sixteen,
    Nineteen,
    TwentyOne,
    TwentyFour,
}

impl From<i32> for Svint24BitSize {
    fn from(value: i32) -> Self {
        match value {
            0 => Svint24BitSize::Sixteen,
            1 => Svint24BitSize::Nineteen,
            2 => Svint24BitSize::TwentyOne,
            _ => Svint24BitSize::TwentyFour,
        }
    }
}

pub struct Float {
    value: f32,
    mantissa: u16,
    exponent: i16,
}
