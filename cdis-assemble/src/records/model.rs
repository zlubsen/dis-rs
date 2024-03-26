use dis_rs::enumerations::PduType;
use dis_rs::model::PduStatus;
use dis_rs::model::{TimeStamp};
use crate::constants::{CDIS_NANOSECONDS_PER_TIME_UNIT, LEAST_SIGNIFICANT_BIT};
use crate::records::model::CdisProtocolVersion::{Reserved, SISO_023_2023, StandardDis};
use crate::types::model::{SVINT12, SVINT16, UVINT16, UVINT8};

/// 13.1 C-DIS PDU Header
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CdisHeader {
    pub protocol_version: CdisProtocolVersion,
    pub exercise_id: UVINT8,
    pub pdu_type: PduType,
    pub timestamp: TimeStamp,
    pub length: u16,
    pub pdu_status: PduStatus,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CdisProtocolVersion {
    StandardDis,
    SISO_023_2023,
    Reserved(u8),
}

impl From<u8> for CdisProtocolVersion {
    fn from(value: u8) -> Self {
        match value {
            0 => StandardDis,
            1 => SISO_023_2023,
            reserved => Reserved(reserved),
        }
    }
}

impl From<CdisProtocolVersion> for u8 {
    fn from(value: CdisProtocolVersion) -> Self {
        match value {
            StandardDis => { 0 }
            SISO_023_2023 => { 1 }
            Reserved(reserved) => { reserved }
        }
    }
}

/// A timestamp type that models the timestamp mechanism as described in the
/// DIS standard (section 6.2.88 Timestamp). This timestamp interprets a u32 value
/// as an Absolute or a Relative timestamp based on the Least Significant Bit.
/// The remaining (upper) bits represent the units of time passed since the
/// beginning of the current hour in the selected time reference.
/// The `DisTimeStamp` stores both the units past the hour, as well as a conversion to
/// nanoseconds past the hour.
pub enum CdisTimeStamp {
    Absolute { units_past_the_hour: u32, nanoseconds_past_the_hour: u32 },
    Relative { units_past_the_hour: u32, nanoseconds_past_the_hour: u32 },
}

impl CdisTimeStamp {
    pub fn new_absolute_from_secs(seconds_past_the_hour: u32) -> Self {
        let nanoseconds_past_the_hour = CdisTimeStamp::seconds_to_nanoseconds(seconds_past_the_hour);
        let units_past_the_hour = CdisTimeStamp::nanoseconds_to_dis_time_units(nanoseconds_past_the_hour);
        Self::Absolute {
            units_past_the_hour,
            nanoseconds_past_the_hour
        }
    }

    pub fn new_relative_from_secs(seconds_past_the_hour: u32) -> Self {
        let nanoseconds_past_the_hour = CdisTimeStamp::seconds_to_nanoseconds(seconds_past_the_hour);
        let units_past_the_hour = CdisTimeStamp::nanoseconds_to_dis_time_units(nanoseconds_past_the_hour);
        Self::Relative {
            units_past_the_hour,
            nanoseconds_past_the_hour
        }
    }

    /// Helper function to convert seconds to nanoseconds
    fn seconds_to_nanoseconds(seconds: u32) -> u32 {
        seconds * 1_000_000
    }

    /// Helper function to convert nanoseconds pas the hour to DIS Time Units past the hour.
    fn nanoseconds_to_dis_time_units(nanoseconds_past_the_hour: u32) -> u32 {
        (nanoseconds_past_the_hour as f32 / CDIS_NANOSECONDS_PER_TIME_UNIT) as u32
    }
}

impl From<u32> for CdisTimeStamp {
    fn from(value: u32) -> Self {
        let absolute_bit = (value & LEAST_SIGNIFICANT_BIT) == LEAST_SIGNIFICANT_BIT;
        let units_past_the_hour = value >> 1;
        let nanoseconds_past_the_hour = (units_past_the_hour as f32 * CDIS_NANOSECONDS_PER_TIME_UNIT) as u32;

        if absolute_bit {
            Self::Absolute { units_past_the_hour, nanoseconds_past_the_hour }
        } else {
            Self::Relative { units_past_the_hour, nanoseconds_past_the_hour }
        }
    }
}

impl From<TimeStamp> for CdisTimeStamp {
    fn from(value: TimeStamp) -> Self {
        CdisTimeStamp::from(value.raw_timestamp)
    }
}

/// 11.1 Angular Velocity
/// Scale = (2^11 - 1) / (4 * pi)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AngularVelocity {
    pub x: SVINT12,
    pub y: SVINT12,
    pub z: SVINT12,
}

impl AngularVelocity {
    pub const SCALE: f32 = (2^11 - 1) as f32 / (4.0f32 * std::f32::consts::PI);
    pub fn new(x: SVINT12, y: SVINT12, z: SVINT12) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

/// 11.10 Entity Coordinate Vector
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityCoordinateVector {
    pub x: SVINT16,
    pub y: SVINT16,
    pub z: SVINT16,
}

impl EntityCoordinateVector {
    pub fn new(x: SVINT16, y: SVINT16, z: SVINT16) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

/// 11.11 Entity Identifier Record
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityId {
    pub site: UVINT16,
    pub application: UVINT16,
    pub entity: UVINT16,
}
impl EntityId {
    pub fn new(site: UVINT16, application: UVINT16, entity: UVINT16) -> Self {
        Self {
            site,
            application,
            entity,
        }
    }
}

/// 11.12 Entity Type
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityType {
    pub kind: u8,
    pub domain: u8,
    pub country: u16,
    pub category: UVINT8,
    pub subcategory: UVINT8,
    pub specific: UVINT8,
    pub extra: UVINT8,
}

impl EntityType {
    pub fn new(kind: u8, domain: u8, country: u16, category: UVINT8, subcategory: UVINT8, specific: UVINT8, extra: UVINT8) -> Self {
        Self {
            kind,
            domain,
            country,
            category,
            subcategory,
            specific,
            extra,
        }
    }
}

/// 11.19 Linear Velocity
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LinearVelocity {
    pub x: SVINT16,
    pub y: SVINT16,
    pub z: SVINT16,
}

impl LinearVelocity {
    pub fn new(x: SVINT16, y: SVINT16, z: SVINT16) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

/// 11.22 Orientation
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Orientation {
    pub psi: u16,
    pub theta: u16,
    pub phi: u16,
}

impl Orientation {
    pub fn new(psi: u16, theta: u16, phi: u16) -> Self {
        Self {
            psi,
            theta,
            phi,
        }
    }
}

/// 11.25 Units
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Units {
    Centimeters,
    Meters,
}

impl From<u8> for Units {
    fn from(value: u8) -> Self {
        match value {
            0 => Units::Centimeters,
            _ => Units::Meters,
        }
    }
}

impl From<Units> for u8 {
    fn from(value: Units) -> Self {
        match value {
            Units::Centimeters => 0,
            Units::Meters => 1,
        }
    }
}

/// 11.26 Valid Entity State Marking Characters
pub struct CdisEntityMarking {
    five_bit_char_size: bool,
    pub marking: String,
}

impl CdisEntityMarking {
    pub fn new(marking: String) -> Self {
        Self {
            five_bit_char_size: Self::can_fit_in_five_bit_chars(&marking),
            marking,
        }
    }

    fn can_fit_in_five_bit_chars(marking: &str) -> bool {
        const LEAST_USED_CHARS_MORSE: [char; 5] = ['J', 'K', 'Q', 'X', 'Z'];
        let has_only_ascii_alphanumeric = marking.chars()
            .filter(|char|char != '\0')// filter the NUL control character, as it is allowed
            .any(|char| !char.is_ascii_alphanumeric());     // only ASCII alphanumeric characters fit in C-DIS 5-bit encoding
        let contains_least_used_char_morse = marking.chars()
            .any(|char| LEAST_USED_CHARS_MORSE.contains(&char)); // and it should not contain the five least used characters

        has_only_ascii_alphanumeric & !contains_least_used_char_morse
    }

    // strip content of `marking` from unsupported characters, as defined in Table 38
    fn sanitize_marking(six_bit_char_size: bool, marking: String) -> String {
        let marking = match six_bit_char_size {
            true => Self::sanitize_six_bit_chars(marking),
            false => Self::sanitize_five_bit_chars(marking),
        };

        marking
    }

    fn sanitize_six_bit_chars(marking: String) -> String {
        let marking = marking.chars().filter(|char| char.is_ascii()).map(|char| {
            u8::from(CdisCharacterSixBit(char))
        }).collect();
        let marking = String::from_utf8_lossy(marking);
        marking.into_string()
    }

    fn sanitize_five_bit_chars(marking: String) -> String {
        const ASTERISK_ASCII_CODE: u8 = 63;
        let marking = marking.chars()
            .filter(|char| char.is_ascii())
            .map(|char| u8::from(char) )
            .map(|&code| if code <= 43u8 { code } else { ASTERISK_ASCII_CODE })
            .collect();
        let marking = String::from_utf8_lossy(marking);
        marking.into_string()
    }
}

pub struct CdisCharacterSixBit(char);
pub struct CdisCharacterFiveBit(char);

impl From<u8> for CdisCharacterSixBit {
    fn from(value: u8) -> Self {
        Self(match value {
            0 => '\0',
            1 => 'A',
            2 => 'B',
            3 => 'C',
            4 => 'D',
            5 => 'E',
            6 => 'F',
            7 => 'G',
            8 => 'H',
            9 => 'I',
            10 => 'J',
            11 => 'K',
            12 => 'L',
            13 => 'M',
            14 => 'N',
            15 => 'O',
            16 => 'P',
            17 => 'Q',
            18 => 'R',
            19 => 'S',
            20 => 'T',
            21 => 'U',
            22 => 'V',
            23 => 'W',
            24 => 'X',
            25 => 'Y',
            26 => 'Z',
            27 => '.',
            28 => '?',
            29 => '!',
            30 => '0',
            31 => '1',
            32 => '2',
            33 => '3',
            34 => '4',
            35 => '5',
            36 => '6',
            37 => '7',
            38 => '8',
            39 => '9',
            40 => ' ',
            41 => '[',
            42 => ']',
            43 => '(',
            44 => ')',
            45 => '{',
            46 => '}',
            47 => '+',
            48 => '-',
            49 => '_',
            50 => '@',
            51 => '&',
            52 => '"',
            53 => '\'',
            54 => ':',
            55 => ';',
            56 => ',',
            57 => '~',
            58 => '\\',
            59 => '/',
            60 => '%',
            61 => '#',
            62 => '$',
            63 | _ => '*',
        })
    }
}

impl From<CdisCharacterSixBit> for u8 {
    fn from(value: CdisCharacterSixBit) -> Self {
        match value.0 {
            '\0' => 0,
            'A' => 1,
            'B' => 2,
            'C' => 3,
            'D' => 4,
            'E' => 5,
            'F' => 6,
            'G' => 7,
            'H' => 8,
            'I' => 9,
            'J' => 10,
            'K' => 11,
            'L' => 12,
            'M' => 13,
            'N' => 14,
            'O' => 15,
            'P' => 16,
            'Q' => 17,
            'R' => 18,
            'S' => 19,
            'T' => 20,
            'U' => 21,
            'V' => 22,
            'W' => 23,
            'X' => 24,
            'Y' => 25,
            'Z' => 26,
            '.' => 27,
            '?' => 28,
            '!' => 29,
            '0' => 30,
            '1' => 31,
            '2' => 32,
            '3' => 33,
            '4' => 34,
            '5' => 35,
            '6' => 36,
            '7' => 37,
            '8' => 38,
            '9' => 39,
            ' ' => 40,
            '[' => 41,
            ']' => 42,
            '(' => 43,
            ')' => 44,
            '{' => 45,
            '}' => 46,
            '+' => 47,
            '-' => 48,
            '_' => 49,
            '@' => 50,
            '&' => 51,
            '"' => 52,
            '\'' => 53,
            ':' => 54,
            ';' => 55,
            ',' => 56,
            '~' => 57,
            '\\' => 58,
            '/' => 59,
            '%' => 60,
            '#' => 61,
            '$' => 62,
            '*' | _ => 63,
        }
    }
}

impl From<u8> for CdisCharacterFiveBit {
    fn from(value: u8) -> Self {
        Self(match value {
            0 => '\0',
            1 => 'A',
            2 => 'B',
            3 => 'C',
            4 => 'D',
            5 => 'E',
            6 => 'F',
            7 => 'G',
            8 => 'H',
            9 => 'I',
            10 => 'L',
            11 => 'M',
            12 => 'N',
            13 => 'O',
            14 => 'P',
            15 => 'R',
            16 => 'S',
            17 => 'T',
            18 => 'U',
            19 => 'V',
            20 => 'W',
            21 => 'Y',
            22 => '0',
            23 => '1',
            24 => '2',
            25 => '3',
            26 => '4',
            27 => '5',
            28 => '6',
            29 => '7',
            30 => '8',
            31 => '9',
            _ => '*',
        })
    }
}

impl From<CdisCharacterFiveBit> for u8 {
    fn from(value: CdisCharacterFiveBit) -> Self {
        match value.0 {
            '\0' => 0,
            'A' => 1,
            'B' => 2,
            'C' => 3,
            'D' => 4,
            'E' => 5,
            'F' => 6,
            'G' => 7,
            'H' => 8,
            'I' => 9,
            'L' => 10,
            'M' => 11,
            'N' => 12,
            'O' => 13,
            'P' => 14,
            'R' => 15,
            'S' => 16,
            'T' => 17,
            'U' => 18,
            'V' => 19,
            'W' => 20,
            'X' => 21,
            '0' => 22,
            '1' => 23,
            '2' => 24,
            '3' => 25,
            '4' => 26,
            '5' => 27,
            '6' => 28,
            '7' => 29,
            '8' => 30,
            '9' => 31,
            '*' | _ => 63,
        }
    }
}