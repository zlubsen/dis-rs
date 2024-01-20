pub const PDU_HEADER_LEN_BYTES: u16 = 12;
pub const VARIABLE_PARAMETER_RECORD_LENGTH : u16 = 16;
pub const ONE_BYTE_IN_BITS: usize = 8;
pub const NO_REMAINDER: usize = 0;
pub const ZERO_OCTETS: usize = 0;
pub const FOUR_OCTETS: usize = 4;
#[allow(dead_code)]
pub const SIX_OCTETS: usize = 6;
pub const EIGHT_OCTETS: usize = 8;
pub const LEAST_SIGNIFICANT_BIT : u32 = 0x001;
pub const NANOSECONDS_PER_HOUR: u32 = 3600 * 1e6 as u32;
pub const TIME_UNITS_PER_HOUR: u32 = 2^31 - 1;
pub const NANOSECONDS_PER_TIME_UNIT: f32 = NANOSECONDS_PER_HOUR as f32 / TIME_UNITS_PER_HOUR as f32;