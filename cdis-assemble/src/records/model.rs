use dis_rs::enumerations::PduType;
use dis_rs::model::PduStatus;
use dis_rs::model::{TimeStamp};
use crate::constants::{CDIS_NANOSECONDS_PER_TIME_UNIT, LEAST_SIGNIFICANT_BIT};
use crate::records::model::CdisProtocolVersion::{Reserved, SISO_023_2023, StandardDis};
use crate::types::model::UVINT8;

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
