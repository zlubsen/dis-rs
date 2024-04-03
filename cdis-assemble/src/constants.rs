pub const ONE_BIT: usize = 1;
pub const TWO_BITS: usize = 2;
pub const THREE_BITS: usize = 3;
pub const FOUR_BITS: usize = 4;
pub const FIVE_BITS: usize = 5;
pub const SIX_BITS: usize = 6;
pub const SEVEN_BITS: usize = 7;
pub const EIGHT_BITS: usize = 8;
pub const NINE_BITS: usize = 9;
pub const TEN_BITS: usize = 10;
pub const ELEVEN_BITS: usize = 11;
pub const TWELVE_BITS: usize = 12;
pub const THIRTEEN_BITS: usize = 13;
pub const FOURTEEN_BITS: usize = 14;
pub const FIFTEEN_BITS: usize = 15;
pub const SIXTEEN_BITS: usize = 16;
pub const TWENTY_SIX_BITS: usize = 26;
pub const THIRTY_BITS: usize = 30;

pub const THIRTY_ONE_BITS: usize = 31;
pub const THIRTY_TWO_BITS: usize = 32;
pub const THIRTY_NINE_BITS: usize = 39;
pub const HUNDRED_TWENTY_BITS: usize = 120;
pub const LEAST_SIGNIFICANT_BIT : u32 = 0x001;
pub const NANOSECONDS_PER_HOUR: u32 = 3600 * 1e6 as u32;
pub const CDIS_TIME_UNITS_PER_HOUR: u32 = (2^25) - 1;
pub const CDIS_NANOSECONDS_PER_TIME_UNIT: f32 = NANOSECONDS_PER_HOUR as f32 / CDIS_TIME_UNITS_PER_HOUR as f32;
pub const DIS_TIME_UNITS_PER_HOUR: u32 = (2^31) - 1;
pub const DIS_NANOSECONDS_PER_TIME_UNIT: f32 = NANOSECONDS_PER_HOUR as f32 / DIS_TIME_UNITS_PER_HOUR as f32;
pub const MTU_BYTES: usize = 1500;
pub const MTU_BITS: usize = MTU_BYTES * EIGHT_BITS;
pub const DECIMETERS_TO_METERS: f32 = 10f32;

pub struct EcefToGeoConstants;

impl EcefToGeoConstants {
    pub const WGS_84_SEMI_MAJOR_AXIS: f32 = 6378137.0;  //WGS-84 semi-major axis
    pub const E2: f32 = 6.6943799901377997e-3;          // WGS-84 first eccentricity squared
    pub const A1: f32 = 4.2697672707157535e+4;          //a1 = a*e2
    pub const A2: f32 = 1.8230912546075455e+9;          //a2 = a1*a1
    pub const A3: f32 = 1.4291722289812413e+2;          //a3 = a1*e2/2
    pub const A4: f32 = 4.5577281365188637e+9;          //a4 = 2.5*a2
    pub const A5: f32 = 4.2840589930055659e+4;          //a5 = a1+a3
    pub const A6: f32 = 9.9330562000986220e-1;          //a6 = 1-e2
}