use dis_rs::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, AttachedPartDetachedIndicator, AttachedParts, ChangeIndicator, EntityAssociationAssociationStatus, EntityAssociationGroupMemberType, EntityAssociationPhysicalAssociationType, EntityAssociationPhysicalConnectionType, PduType, SeparationPreEntityIndicator, SeparationReasonForSeparation, StationName};
use dis_rs::model::{DatumSpecification, DisTimeStamp, EventId, FixedDatum, Location, PduStatus, SimulationAddress, VariableDatum};
use dis_rs::model::TimeStamp;
use crate::constants::{CDIS_NANOSECONDS_PER_TIME_UNIT, CDIS_TIME_UNITS_PER_HOUR, DIS_TIME_UNITS_PER_HOUR, EIGHT_BITS, FIFTEEN_BITS, FIVE_BITS, FOUR_BITS, FOURTEEN_BITS, LEAST_SIGNIFICANT_BIT, ONE_BIT, SIXTY_FOUR_BITS, THIRTY_NINE_BITS, THIRTY_TWO_BITS, THREE_BITS};
use crate::records::model::CdisProtocolVersion::{Reserved, SISO_023_2023, StandardDis};
use crate::types::model::{CdisFloat, CdisFloatBase, SVINT12, SVINT14, SVINT16, SVINT24, UVINT16, UVINT8, VarInt};

use num_traits::FromPrimitive;

pub(crate) trait CdisRecord {
    fn record_length(&self) -> usize;
}

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

impl CdisRecord for CdisHeader {
    fn record_length(&self) -> usize {
        const ALWAYS_PRESENT_FIELDS_LENGTH : usize = 58;
        ALWAYS_PRESENT_FIELDS_LENGTH + self.exercise_id.record_length()
    }
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
/// The `DisTimeStamp` stores both the units past the hour, and a conversion to
/// nanoseconds past the hour.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CdisTimeStamp {
    Absolute { units_past_the_hour: u32, nanoseconds_past_the_hour: u32 },
    Relative { units_past_the_hour: u32, nanoseconds_past_the_hour: u32 },
}

impl CdisTimeStamp {
    pub fn new_absolute_from_secs(seconds_past_the_hour: u32) -> Self {
        let nanoseconds_past_the_hour = CdisTimeStamp::seconds_to_nanoseconds(seconds_past_the_hour);
        let units_past_the_hour = CdisTimeStamp::nanoseconds_to_cdis_time_units(nanoseconds_past_the_hour);
        Self::Absolute {
            units_past_the_hour,
            nanoseconds_past_the_hour
        }
    }

    pub fn new_relative_from_secs(seconds_past_the_hour: u32) -> Self {
        let nanoseconds_past_the_hour = CdisTimeStamp::seconds_to_nanoseconds(seconds_past_the_hour);
        let units_past_the_hour = CdisTimeStamp::nanoseconds_to_cdis_time_units(nanoseconds_past_the_hour);
        Self::Relative {
            units_past_the_hour,
            nanoseconds_past_the_hour
        }
    }

    pub fn new_absolute_from_units(units_past_the_hour: u32) -> Self {
        Self::Absolute {
            units_past_the_hour,
            nanoseconds_past_the_hour: Self::cdis_time_units_to_nanoseconds(units_past_the_hour),
        }
    }

    pub fn new_relative_from_units(units_past_the_hour: u32) -> Self {
        Self::Relative {
            units_past_the_hour,
            nanoseconds_past_the_hour: Self::cdis_time_units_to_nanoseconds(units_past_the_hour),
        }
    }

    /// Helper function to convert seconds to nanoseconds
    fn seconds_to_nanoseconds(seconds: u32) -> u32 {
        seconds * 1_000_000
    }

    /// Helper function to convert nanoseconds pas the hour to DIS Time Units past the hour.
    fn nanoseconds_to_cdis_time_units(nanoseconds_past_the_hour: u32) -> u32 {
        (nanoseconds_past_the_hour as f32 / CDIS_NANOSECONDS_PER_TIME_UNIT) as u32
    }

    fn cdis_time_units_to_nanoseconds(cdis_time_units: u32) -> u32 {
        (cdis_time_units as f32 * CDIS_NANOSECONDS_PER_TIME_UNIT) as u32
    }
}

impl Default for CdisTimeStamp {
    fn default() -> Self {
        CdisTimeStamp::new_relative_from_secs(0)
    }
}

impl From<u32> for CdisTimeStamp {
    fn from(value: u32) -> Self {
        let is_absolute_timestamp = (value & LEAST_SIGNIFICANT_BIT) == LEAST_SIGNIFICANT_BIT;
        let units_past_the_hour = value >> 1;
        let nanoseconds_past_the_hour = (units_past_the_hour as f32 * CDIS_NANOSECONDS_PER_TIME_UNIT) as u32;

        if is_absolute_timestamp {
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

impl From<CdisTimeStamp> for TimeStamp {
    fn from(value: CdisTimeStamp) -> Self {
        let raw_timestamp = match value {
            CdisTimeStamp::Absolute { units_past_the_hour, nanoseconds_past_the_hour: _nanoseconds_past_the_hour } => {
                (units_past_the_hour << 1) | LEAST_SIGNIFICANT_BIT
            }
            CdisTimeStamp::Relative { units_past_the_hour, nanoseconds_past_the_hour: _nanoseconds_past_the_hour } => {
                units_past_the_hour << 1
            }
        };

        Self { raw_timestamp }
    }
}

impl From<DisTimeStamp> for CdisTimeStamp {
    fn from(value: DisTimeStamp) -> Self {
        let dis_to_cdis_time_units = CDIS_TIME_UNITS_PER_HOUR as f32 / DIS_TIME_UNITS_PER_HOUR as f32;
        match value {
            DisTimeStamp::Absolute { units_past_the_hour, nanoseconds_past_the_hour: _nanoseconds_past_the_hour } => {
                let units_past_the_hour = units_past_the_hour as f32 * dis_to_cdis_time_units;
                CdisTimeStamp::new_absolute_from_units(units_past_the_hour.round() as u32)
            }
            DisTimeStamp::Relative { units_past_the_hour, nanoseconds_past_the_hour: _nanoseconds_past_the_hour } => {
                let units_past_the_hour = units_past_the_hour as f32 * dis_to_cdis_time_units;
                CdisTimeStamp::new_relative_from_units(units_past_the_hour.round() as u32)
            }
        }
    }
}

impl From<CdisTimeStamp> for DisTimeStamp {
    fn from(value: CdisTimeStamp) -> Self {
        let cdis_to_dis_time_units = DIS_TIME_UNITS_PER_HOUR as f32 / CDIS_TIME_UNITS_PER_HOUR as f32;
        match value {
            CdisTimeStamp::Absolute { units_past_the_hour, nanoseconds_past_the_hour: _nanoseconds_past_the_hour } => {
                let units_past_the_hour = units_past_the_hour as f32 * cdis_to_dis_time_units;
                DisTimeStamp::new_absolute_from_units(units_past_the_hour.round() as u32)
            }
            CdisTimeStamp::Relative { units_past_the_hour, nanoseconds_past_the_hour: _nanoseconds_past_the_hour } => {
                let units_past_the_hour = units_past_the_hour as f32 * cdis_to_dis_time_units;
                DisTimeStamp::new_relative_from_units(units_past_the_hour.round() as u32)
            }
        }
    }
}

pub fn dis_to_cdis_u32_timestamp(dis_u32: u32) -> u32 {
    TimeStamp::from(CdisTimeStamp::from(DisTimeStamp::from(dis_u32))).raw_timestamp
}

pub fn cdis_to_dis_u32_timestamp(cdis_u32: u32) -> u32 {
    TimeStamp::from(DisTimeStamp::from(CdisTimeStamp::from(TimeStamp::from(cdis_u32)))).raw_timestamp
}

/// 11.1 Angular Velocity
/// Scale = (2^11 - 1) / (4 * pi)
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct AngularVelocity {
    pub x: SVINT12,
    pub y: SVINT12,
    pub z: SVINT12,
}

impl AngularVelocity {
    pub fn new(x: SVINT12, y: SVINT12, z: SVINT12) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

impl CdisRecord for AngularVelocity {
    fn record_length(&self) -> usize {
        self.x.record_length()
            + self.y.record_length()
            + self.z.record_length()
    }
}

/// 11.1 Linear Acceleration
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LinearAcceleration {
    pub x: SVINT14,
    pub y: SVINT14,
    pub z: SVINT14,
}

impl LinearAcceleration {
    pub fn new(x: SVINT14, y: SVINT14, z: SVINT14) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

impl CdisRecord for LinearAcceleration {
    fn record_length(&self) -> usize {
        self.x.record_length()
            + self.y.record_length()
            + self.z.record_length()
    }
}

/// 11.6 Datum Specification Record
impl CdisRecord for DatumSpecification {
    fn record_length(&self) -> usize {
        UVINT8::from(u8::from_usize(self.fixed_datum_records.len()).unwrap_or(u8::MAX)).record_length()
        + UVINT8::from(u8::from_usize(self.variable_datum_records.len()).unwrap_or(u8::MAX)).record_length()
        + self.fixed_datum_records.iter().map(|datum| datum.record_length() ).sum::<usize>()
        + self.variable_datum_records.iter().map(|datum| datum.record_length() ).sum::<usize>()
    }
}

/// DIS v7 6.2.37
impl CdisRecord for FixedDatum {
    fn record_length(&self) -> usize {
        SIXTY_FOUR_BITS
    }
}

/// DIS v7 6.2.93
impl CdisRecord for VariableDatum {
    fn record_length(&self) -> usize {
        THIRTY_TWO_BITS + FOURTEEN_BITS
            + self.datum_value.len() * EIGHT_BITS
    }
}

/// 11.10 Entity Coordinate Vector
#[derive(Copy, Clone, Debug, Default, PartialEq)]
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

impl CdisRecord for EntityCoordinateVector {
    fn record_length(&self) -> usize {
        self.x.record_length()
            + self.y.record_length()
            + self.z.record_length()
    }
}

/// 11.11 Entity Identifier Record
#[derive(Copy, Clone, Default, Debug, PartialEq)]
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

impl From<&EntityId> for dis_rs::model::EntityId {
    fn from(value: &EntityId) -> Self {
        Self {
            simulation_address: SimulationAddress::new(
                value.site.value, value.application.value),
            entity_id: value.entity.value,
        }
    }
}

/// Convert (and thus encode) a dis-rs EventId to cdis-assemble EntityId,
/// because the cdis library does not model the EventId record explicitly.
impl From<&EventId> for EntityId {
    fn from(value: &EventId) -> Self {
        Self::new(
            UVINT16::from(value.simulation_address.site_id),
            UVINT16::from(value.simulation_address.application_id),
            UVINT16::from(value.event_id),
        )
    }
}

/// Convert (and thus decode) a cdis-assemble EntityId to dis-rs EventId,
/// because the cdis library does not model the EventId record explicitly.
impl From<&EntityId> for EventId {
    fn from(value: &EntityId) -> Self {
        Self::new(
            SimulationAddress::new(value.site.value, value.application.value),
            value.entity.value
        )
    }
}

impl CdisRecord for EntityId {
    fn record_length(&self) -> usize {
        self.site.record_length()
            + self.application.record_length()
            + self.entity.record_length()
    }
}

/// 11.12 Entity Type
#[derive(Copy, Clone, Debug, Default, PartialEq)]
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

impl CdisRecord for EntityType {
    fn record_length(&self) -> usize {
        const ALWAYS_PRESENT_FIELDS_LENGTH: usize = 17;
        ALWAYS_PRESENT_FIELDS_LENGTH
            + self.category.record_length()
            + self.subcategory.record_length()
            + self.specific.record_length()
            + self.extra.record_length()
    }
}

/// 11.19 Linear Velocity
#[derive(Copy, Clone, Debug, Default, PartialEq)]
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

impl CdisRecord for LinearVelocity {
    fn record_length(&self) -> usize {
        self.x.record_length()
            + self.y.record_length()
            + self.z.record_length()
    }
}

/// 11.22 Orientation
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Orientation {
    pub psi: i16,
    pub theta: i16,
    pub phi: i16,
}

impl Orientation {
    pub fn new(psi: i16, theta: i16, phi: i16) -> Self {
        Self {
            psi,
            theta,
            phi,
        }
    }
}

impl CdisRecord for Orientation {
    fn record_length(&self) -> usize {
        THIRTY_NINE_BITS
    }
}

/// 11.25 Units
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum UnitsDekameters {
    Centimeter,
    #[default]
    Dekameter,
}

impl CdisRecord for UnitsDekameters {
    fn record_length(&self) -> usize {
        1
    }
}

impl From<u8> for UnitsDekameters {
    fn from(value: u8) -> Self {
        match value {
            0 => UnitsDekameters::Centimeter,
            _ => UnitsDekameters::Dekameter,
        }
    }
}

impl From<UnitsDekameters> for u8 {
    fn from(value: UnitsDekameters) -> Self {
        match value {
            UnitsDekameters::Centimeter => 0,
            UnitsDekameters::Dekameter => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum UnitsMeters {
    #[default]
    Centimeter,
    Meter,
}

impl From<u8> for UnitsMeters {
    fn from(value: u8) -> Self {
        match value {
            0 => UnitsMeters::Centimeter,
            _ => UnitsMeters::Meter,
        }
    }
}

impl From<UnitsMeters> for u8 {
    fn from(value: UnitsMeters) -> Self {
        match value {
            UnitsMeters::Centimeter => 0,
            UnitsMeters::Meter => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum UnitsMass {
    #[default]
    Grams,
    Kilograms,
}

impl From<u8> for UnitsMass {
    fn from(value: u8) -> Self {
        match value {
            0 => UnitsMass::Grams,
            _ => UnitsMass::Kilograms,
        }
    }
}

impl From<UnitsMass> for u8 {
    fn from(value: UnitsMass) -> Self {
        match value {
            UnitsMass::Grams => 0,
            UnitsMass::Kilograms => 1,
        }
    }
}

/// 11.26 Valid Entity State Marking Characters
#[derive(Clone, Debug, PartialEq)]
pub struct CdisEntityMarking {
    pub(crate) char_encoding: CdisMarkingCharEncoding,
    pub marking: String,
}

impl CdisEntityMarking {
    pub fn new(marking: String) -> Self {
        const MAX_MARKING_LENGTH: usize = 11;
        let marking = if marking.len() > MAX_MARKING_LENGTH {
            let mut marking = marking;
            marking.truncate(MAX_MARKING_LENGTH);
            marking
        } else { marking };

        Self {
            char_encoding: Self::check_char_encoding(&marking),
            marking,
        }
    }

    fn check_char_encoding(marking: &str) -> CdisMarkingCharEncoding {
        const LEAST_USED_CHARS_MORSE: [char; 5] = ['J', 'K', 'Q', 'X', 'Z'];
        let has_only_ascii_alphanumeric = marking.chars()
            .filter(|&char| char != '\0')       // filter the NUL control character, as it is allowed
            .all(|char| char.is_ascii_alphanumeric());              // only ASCII alphanumeric characters fit in C-DIS 5-bit encoding
        let contains_least_used_char_morse = marking.chars()
            .any(|char| LEAST_USED_CHARS_MORSE.contains(&char)); // and it should not contain the five least used characters

        if has_only_ascii_alphanumeric & !contains_least_used_char_morse {
            CdisMarkingCharEncoding::FiveBit
        } else {
            CdisMarkingCharEncoding::SixBit
        }
    }
}

impl Default for CdisEntityMarking {
    fn default() -> Self {
        Self {
            char_encoding: CdisMarkingCharEncoding::FiveBit,
            marking: "NONAME".to_string(),
        }
    }
}

impl CdisRecord for CdisEntityMarking {
    fn record_length(&self) -> usize {
        const ALWAYS_PRESENT_FIELDS_LENGTH: usize = FIVE_BITS;
        ALWAYS_PRESENT_FIELDS_LENGTH
            + (self.marking.len() * self.char_encoding.bit_size())
    }
}

impl From<(&[u8], CdisMarkingCharEncoding)> for CdisEntityMarking {
    fn from((chars, encoding): (&[u8], CdisMarkingCharEncoding)) -> Self {
        let mut marking = String::with_capacity(11);
        chars.iter()
            .map(|code| encoding.char_from_code(*code) )
            .for_each(|ch| marking.push(ch) );
        Self {
            char_encoding: encoding,
            marking,
        }
    }
}

impl From<&str> for CdisEntityMarking {
    fn from(value: &str) -> Self {
        CdisEntityMarking::new(value.into())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CdisMarkingCharEncoding {
    FiveBit,
    SixBit,
}

impl CdisMarkingCharEncoding {
    pub fn new(char_bit_size: u8) -> Self {
        if char_bit_size == 0 {
            Self::FiveBit
        } else {
            Self::SixBit
        }
    }

    pub fn bit_size(&self) -> usize {
        match self {
            CdisMarkingCharEncoding::FiveBit => { 5 }
            CdisMarkingCharEncoding::SixBit => { 6 }
        }
    }

    pub fn encoding(&self) -> u8 {
        match self {
            CdisMarkingCharEncoding::FiveBit => { 0 }
            CdisMarkingCharEncoding::SixBit => { 1 }
        }
    }

    #[allow(clippy::wildcard_in_or_patterns)]
    pub fn char_from_code(&self, code: u8) -> char {
        match self {
            CdisMarkingCharEncoding::FiveBit => {
                match code {
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
                }
            }
            CdisMarkingCharEncoding::SixBit => {
                match code {
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
                }
            }
        }
    }

    #[allow(clippy::wildcard_in_or_patterns)]
    pub fn u8_from_char(&self, c: char) -> u8 {
        match self {
            CdisMarkingCharEncoding::FiveBit => {
                match c {
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
            CdisMarkingCharEncoding::SixBit => {
                match c {
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
    }
}

/// 11.27 World Coordinates Record
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct WorldCoordinates {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude_msl: SVINT24,
}

impl WorldCoordinates {
    pub fn new(latitude: f32, longitude: f32, altitude_msl: SVINT24) -> Self {
        Self {
            latitude,
            longitude,
            altitude_msl,
        }
    }
}

impl CdisRecord for WorldCoordinates {
    fn record_length(&self) -> usize {
        const CONST_BIT_SIZE: usize = 63; // lat + lon
        CONST_BIT_SIZE + self.altitude_msl.record_length()
    }
}

impl From<WorldCoordinates> for Location {
    /// Applies Geo to ECEF conversion
    ///
    /// Adapted from https://danceswithcode.net/engineeringnotes/geodetic_to_ecef/geodetic_to_ecef.html
    fn from(value: WorldCoordinates) -> Self {
        // TODO account for the scaling of lat
        // TODO account for the scaling of lon
        // TODO use of the Units flag - correct calculation of Altitude MSL
        let (ecef_x, ecef_y, ecef_z) = dis_rs::utils::geodetic_lla_to_ecef(
            value.latitude as f64,
            value.longitude as f64,
            value.altitude_msl.value as f64);

        Self {
            x_coordinate: ecef_x,
            y_coordinate: ecef_y,
            z_coordinate: ecef_z,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ParameterValueFloat {
    base: CdisFloatBase,
}

impl CdisFloat for ParameterValueFloat {
    const MANTISSA_BITS: usize = FIFTEEN_BITS;
    const EXPONENT_BITS: usize = THREE_BITS;

    fn new(mantissa: i32, exponent: i8) -> Self {
        Self {
            base: CdisFloatBase {
                mantissa,
                exponent,
                regular_float: None,
            }
        }
    }

    fn from_f64(regular_float: f64) -> Self {
        Self {
            base: CdisFloatBase {
                mantissa: 0,
                exponent: 0,
                regular_float: Some(regular_float),
            }
        }
    }

    fn mantissa(&self) -> i32 {
        self.base.mantissa
    }

    fn exponent(&self) -> i8 {
        self.base.exponent
    }

    fn regular_float(&self) -> Option<f64> {
        self.base.regular_float
    }
}

/// 12 Variable Parameter Records
#[derive(Clone, Debug, PartialEq)]
pub enum CdisVariableParameter {
    ArticulatedPart(CdisArticulatedPartVP),
    AttachedPart(CdisAttachedPartVP),
    EntitySeparation(CdisEntitySeparationVP),
    EntityType(CdisEntityTypeVP),
    EntityAssociation(CdisEntityAssociationVP),
    Unspecified,
}

impl CdisRecord for CdisVariableParameter {
    fn record_length(&self) -> usize {
        // TODO currently always compresses Variable Parameters; how to decide how to encode?
        FOUR_BITS + match self {
            CdisVariableParameter::ArticulatedPart(vp) => { vp.record_length() }
            CdisVariableParameter::AttachedPart(vp) => { vp.record_length() }
            CdisVariableParameter::EntitySeparation(vp) => { vp.record_length() }
            CdisVariableParameter::EntityType(vp) => { vp.record_length() }
            CdisVariableParameter::EntityAssociation(vp) => { vp.record_length() }
            CdisVariableParameter::Unspecified => { 0 }
        }
    }
}

/// 12.1 Articulated Part Variable Parameter (VP) Record
#[derive(Clone, Debug, PartialEq)]
pub struct CdisArticulatedPartVP {
    pub change_indicator: u8,
    pub attachment_id: u16,
    pub type_class: ArticulatedPartsTypeClass,
    pub type_metric: ArticulatedPartsTypeMetric,
    pub parameter_value: ParameterValueFloat,
}

impl CdisRecord for CdisArticulatedPartVP {
    fn record_length(&self) -> usize {
        const CONST_BIT_SIZE: usize = 50;
        CONST_BIT_SIZE
    }
}

/// 12.2 Attached Part VP Record
#[derive(Clone, Debug, PartialEq)]
pub struct CdisAttachedPartVP {
    pub detached_indicator: AttachedPartDetachedIndicator,
    pub attachment_id: u16,
    pub parameter_type: AttachedParts,
    pub attached_part_type: EntityType,
}

impl CdisRecord for CdisAttachedPartVP {
    fn record_length(&self) -> usize {
        const CONST_BIT_SIZE: usize = 22;
        CONST_BIT_SIZE
            + self.attached_part_type.record_length()
    }
}

/// 12.3 Entity Separation VP Record
#[derive(Clone, Debug, PartialEq)]
pub struct CdisEntitySeparationVP {
    pub reason_for_separation: SeparationReasonForSeparation,
    pub pre_entity_indicator: SeparationPreEntityIndicator,
    pub parent_entity_id: EntityId,
    pub station_name: StationName,
    pub station_number: u16,
}

impl CdisRecord for CdisEntitySeparationVP {
    fn record_length(&self) -> usize {
        const CONST_BIT_SIZE: usize = 24;
        CONST_BIT_SIZE
            + self.parent_entity_id.record_length()
    }
}

/// 12.4 Entity Type VP Record
#[derive(Clone, Debug, PartialEq)]
pub struct CdisEntityTypeVP {
    pub change_indicator: ChangeIndicator,
    pub attached_part_type: EntityType,
}

impl CdisRecord for CdisEntityTypeVP {
    fn record_length(&self) -> usize {
        ONE_BIT
            + self.attached_part_type.record_length()
    }
}

/// 12.5 Entity Association VP Record
#[derive(Clone, Debug, PartialEq)]
pub struct CdisEntityAssociationVP {
    pub change_indicator: ChangeIndicator,
    pub association_status: EntityAssociationAssociationStatus,
    pub association_type: EntityAssociationPhysicalAssociationType,
    pub entity_id: EntityId,
    pub own_station_location: StationName,
    pub physical_connection_type: EntityAssociationPhysicalConnectionType,
    pub group_member_type: EntityAssociationGroupMemberType,
    pub group_number: u16,
}

impl CdisRecord for CdisEntityAssociationVP {
    fn record_length(&self) -> usize {
        const CONST_BIT_SIZE: usize = 44;
        CONST_BIT_SIZE
            + self.entity_id.record_length()
    }
}

#[cfg(test)]
mod tests {
    use crate::records::model::{CdisEntityMarking, CdisMarkingCharEncoding};

    #[test]
    fn cdis_char_encodings_five_bits() {
        assert_eq!(0, CdisMarkingCharEncoding::FiveBit.u8_from_char('\0'));
        assert_eq!(1, CdisMarkingCharEncoding::FiveBit.u8_from_char('A'));
        assert_eq!(31, CdisMarkingCharEncoding::FiveBit.u8_from_char('9'));
        assert_eq!(63, CdisMarkingCharEncoding::FiveBit.u8_from_char('*'));
        assert_eq!(63, CdisMarkingCharEncoding::FiveBit.u8_from_char('a'));
        assert_eq!(63, CdisMarkingCharEncoding::FiveBit.u8_from_char('['));

        assert_eq!('\0', CdisMarkingCharEncoding::FiveBit.char_from_code(0));
        assert_eq!('A', CdisMarkingCharEncoding::FiveBit.char_from_code(1));
        assert_eq!('9', CdisMarkingCharEncoding::FiveBit.char_from_code(31));
        assert_eq!('*', CdisMarkingCharEncoding::FiveBit.char_from_code(63));
        assert_eq!('L', CdisMarkingCharEncoding::FiveBit.char_from_code(10));
    }

    #[test]
    fn cdis_char_encodings_six_bits() {
        assert_eq!(0, CdisMarkingCharEncoding::SixBit.u8_from_char('\0'));
        assert_eq!(1, CdisMarkingCharEncoding::SixBit.u8_from_char('A'));
        assert_eq!(63, CdisMarkingCharEncoding::SixBit.u8_from_char('a'));
        assert_eq!(31, CdisMarkingCharEncoding::SixBit.u8_from_char('1'));
        assert_eq!(62, CdisMarkingCharEncoding::SixBit.u8_from_char('$'));
        assert_eq!(63, CdisMarkingCharEncoding::SixBit.u8_from_char('*'));
        assert_eq!(41, CdisMarkingCharEncoding::SixBit.u8_from_char('['));

        assert_eq!('\0', CdisMarkingCharEncoding::SixBit.char_from_code(0));
        assert_eq!('A', CdisMarkingCharEncoding::SixBit.char_from_code(1));
        assert_eq!('1', CdisMarkingCharEncoding::SixBit.char_from_code(31));
        assert_eq!('\\', CdisMarkingCharEncoding::SixBit.char_from_code(58));
        assert_eq!('*', CdisMarkingCharEncoding::SixBit.char_from_code(63));
    }

    #[test]
    fn cdis_marking_from_string_five_bits() {
        let input = "ABCDE";
        let actual = CdisEntityMarking::from(input);

        assert_eq!(String::from(input), actual.marking);
        assert_eq!(CdisMarkingCharEncoding::FiveBit, actual.char_encoding);
    }

    #[test]
    fn cdis_marking_from_string_six_bits() {
        let input = "ABCDEJ";
        let actual = CdisEntityMarking::from(input);

        assert_eq!(String::from(input), actual.marking);
        assert_eq!(CdisMarkingCharEncoding::SixBit, actual.char_encoding);
    }

    #[test]
    fn cdis_marking_from_string_truncate() {
        let input = "ABCDEFGHIJKL";
        let actual = CdisEntityMarking::from(input);

        assert_eq!(11, actual.marking.len());
        assert_eq!(&String::from(input)[..11], actual.marking.as_str());
        assert_eq!(CdisMarkingCharEncoding::SixBit, actual.char_encoding);
    }

    #[test]
    fn cdis_marking_from_vec_u8_five_bit_codes() {
        let input: [u8; 5] = [1,2,3,4,5];
        let actual = CdisEntityMarking::from((&input[..], CdisMarkingCharEncoding::FiveBit));

        assert_eq!(String::from("ABCDE"), actual.marking.as_str());
        assert_eq!(CdisMarkingCharEncoding::FiveBit, actual.char_encoding);
    }

    #[test]
    fn cdis_marking_from_vec_u8_six_bit_codes() {
        let input: [u8; 5] = [10,11,12,13,14];
        let actual = CdisEntityMarking::from((&input[..], CdisMarkingCharEncoding::SixBit));

        assert_eq!(String::from("JKLMN"), actual.marking.as_str());
        assert_eq!(CdisMarkingCharEncoding::SixBit, actual.char_encoding);
    }
}
