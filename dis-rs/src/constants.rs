pub const PDU_HEADER_LEN_BYTES: u16 = 12;
pub const VARIABLE_PARAMETER_RECORD_LENGTH : u16 = 16;

pub const ONE_BYTE_IN_BITS: usize = 8;
pub const NO_REMAINDER: usize = 0;
pub const ZERO_OCTETS: usize = 0;
pub const ONE_OCTET: usize = 1;
pub const TWO_OCTETS: usize = 2;
pub const THREE_OCTETS: usize = 3;
pub const FOUR_OCTETS: usize = 4;
#[allow(dead_code)]
pub const SIX_OCTETS: usize = 6;
pub const EIGHT_OCTETS: usize = 8;
pub const TWELVE_OCTETS: usize = 12;
pub const FIFTEEN_OCTETS: usize = 15;
pub const TWENTY_OCTETS: usize = 20;
pub const THIRTY_TWO_OCTETS: usize = 32;
pub const LEAST_SIGNIFICANT_BIT : u32 = 0x001;
pub const FIVE_LEAST_SIGNIFICANT_BITS : u32 = 0x1f;
pub const NANOSECONDS_PER_HOUR: u32 = 3600 * 1e6 as u32;
pub const TIME_UNITS_PER_HOUR: u32 = (2^31) - 1;
pub const NANOSECONDS_PER_TIME_UNIT: f32 = NANOSECONDS_PER_HOUR as f32 / TIME_UNITS_PER_HOUR as f32;

pub const BIT_0_IN_BYTE: u8 = 0x80;
pub const BIT_1_IN_BYTE: u8 = 0x40;
pub const BIT_2_IN_BYTE: u8 = 0x20;
pub const BIT_3_IN_BYTE: u8 = 0x10;
pub const BIT_4_IN_BYTE: u8 = 0x08;
pub const BIT_5_IN_BYTE: u8 = 0x04;
pub const BIT_6_IN_BYTE: u8 = 0x02;
pub const BIT_7_IN_BYTE: u8 = 0x01;
pub const BITS_2_3_IN_BYTE: u8 = 0x30;
pub const BITS_5_6_IN_BYTE: u8 = 0x06;


