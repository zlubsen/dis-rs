use crate::writing::{write_value_unsigned, BitBuffer, SerializeCdis};
use core::{fmt::Display, time::Duration};
use dis_rs::timestamp::{TimeUnits, Timestamp, TIME_UNITS_PER_HOUR};

/// Number of [`CdisTimeUnits`] in one hour.
pub const CDIS_TIME_UNITS_PER_HOUR: u32 = 1 << 25;

/// Number of nanoseconds in one hour.
const NANOS_PER_HOUR: u64 = 3_600_000_000_000;

/// Number of nanoseconds in one [`CdisTimeUnits`].
#[allow(clippy::cast_precision_loss)]
const NANOS_PER_CDIS_TIME_UNIT: f64 = NANOS_PER_HOUR as f64 / CDIS_TIME_UNITS_PER_HOUR as f64;

/// Maximum number of nanoseconds.
const MAX_NANOS: u64 = NANOS_PER_HOUR - (NANOS_PER_CDIS_TIME_UNIT.round() as u64);

/// Number of [`TimeUnits`] in one [`CdisTimeUnits`].
const TIME_UNITS_PER_CDIS_TIME_UNIT: u32 = TIME_UNITS_PER_HOUR / CDIS_TIME_UNITS_PER_HOUR;

/// Reference time at which the data contained in the *PDU* was generated.
///
/// Time is represented as [`CdisTimeUnits`] elapsed since the beginning of the current hour in the selected time reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CdisTimestamp {
    /// Time is *relative* to the simulation application issuing the *PDU*.
    ///
    /// A *relative* timestamp should be used when host clocks are not synchronized.
    Relative(CdisTimeUnits),
    /// Time is *absolute* to the simulation time.
    ///
    /// An *absolute* timestamp should be used when host clocks are synchronized.
    Absolute(CdisTimeUnits),
}

impl CdisTimestamp {
    /// Size of `CdisTimestamp` in bits.
    pub const BITS: usize = CdisTimeUnits::BITS + 1;

    /// Bit indicating a *relative* timestamp.
    pub const RELATIVE_BIT: u32 = 0b0;

    /// Bit indicating an *absolute* timestamp.
    pub const ABSOLUTE_BIT: u32 = 0b1;

    /// Constructs a new `CdisTimestamp` from `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::{CdisTimestamp, CdisTimeUnits};
    /// let timestamp = CdisTimestamp::new(20);
    /// assert_eq!(timestamp, CdisTimestamp::Relative(CdisTimeUnits::new(10).unwrap()));
    ///
    /// let timestamp = CdisTimestamp::new(21);
    /// assert_eq!(timestamp, CdisTimestamp::Absolute(CdisTimeUnits::new(10).unwrap()));
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: u32) -> Self {
        // SAFETY: right-shifting a u32 by one bit and masking guarantees the value is always less than 33554432
        #[allow(unsafe_code)]
        let time_units =
            unsafe { CdisTimeUnits::new_unchecked((value >> 1) & CdisTimeUnits::MASK) };
        let is_relative = (value & 1) == Self::RELATIVE_BIT;

        if is_relative {
            Self::Relative(time_units)
        } else {
            Self::Absolute(time_units)
        }
    }

    /// Returns the [`CdisTimeUnits`] contained by this `CdisTimestamp`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::{CdisTimestamp, CdisTimeUnits};
    /// let timestamp = CdisTimestamp::new(20);
    /// assert_eq!(timestamp.time_units(), CdisTimeUnits::new(10).unwrap());
    ///
    /// let timestamp = CdisTimestamp::new(21);
    /// assert_eq!(timestamp.time_units(), CdisTimeUnits::new(10).unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub const fn time_units(self) -> CdisTimeUnits {
        match self {
            Self::Absolute(time_units) | Self::Relative(time_units) => time_units,
        }
    }

    /// Returns `true` if this `CdisTimestamp` is *relative*.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::{CdisTimestamp, CdisTimeUnits};
    /// let timestamp = CdisTimestamp::Relative(CdisTimeUnits::new(10).unwrap());
    /// assert!(timestamp.is_relative());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_relative(self) -> bool {
        matches!(self, Self::Relative(_))
    }

    /// Returns `true` if this `CdisTimestamp` is *absolute*.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::{CdisTimestamp, CdisTimeUnits};
    /// let timestamp = CdisTimestamp::Absolute(CdisTimeUnits::new(10).unwrap());
    /// assert!(timestamp.is_absolute());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_absolute(self) -> bool {
        matches!(self, Self::Absolute(_))
    }

    /// Converts this `CdisTimestamp` into a [`u32`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::{CdisTimestamp, CdisTimeUnits};
    /// let timestamp = CdisTimestamp::Relative(CdisTimeUnits::new(10).unwrap());
    /// assert_eq!(timestamp.to_u32(), 20);
    ///
    /// let timestamp = CdisTimestamp::Absolute(CdisTimeUnits::new(10).unwrap());
    /// assert_eq!(timestamp.to_u32(), 21);
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        let time_units = self.time_units().inner();
        let bit = match self {
            Self::Relative(_) => Self::RELATIVE_BIT,
            Self::Absolute(_) => Self::ABSOLUTE_BIT,
        };

        (time_units << 1) | bit
    }

    /// Converts this `CdisTimestamp` into a [`Duration`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::{CdisTimestamp, CdisTimeUnits};
    /// # use core::time::Duration;
    /// let timestamp = CdisTimestamp::Relative(CdisTimeUnits::new(16_777_216).unwrap());
    /// assert_eq!(timestamp.to_duration(), Duration::from_mins(30));
    ///
    /// let timestamp = CdisTimestamp::Absolute(CdisTimeUnits::new(16_777_216).unwrap());
    /// assert_eq!(timestamp.to_duration(), Duration::from_mins(30));
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_duration(self) -> Duration {
        self.time_units().to_duration()
    }
}

impl SerializeCdis for CdisTimestamp {
    #[inline]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        write_value_unsigned(buf, cursor, Self::BITS, self.to_u32())
    }
}

impl Default for CdisTimestamp {
    #[inline]
    fn default() -> Self {
        Self::Relative(CdisTimeUnits::default())
    }
}

impl From<Timestamp> for CdisTimestamp {
    fn from(value: Timestamp) -> Self {
        match value {
            Timestamp::Relative(time_units) => Self::Relative(time_units.into()),
            Timestamp::Absolute(time_units) => Self::Absolute(time_units.into()),
        }
    }
}

impl From<CdisTimestamp> for Timestamp {
    fn from(value: CdisTimestamp) -> Self {
        match value {
            CdisTimestamp::Relative(time_units) => Self::Relative(time_units.into()),
            CdisTimestamp::Absolute(time_units) => Self::Absolute(time_units.into()),
        }
    }
}

impl From<u32> for CdisTimestamp {
    #[inline]
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<CdisTimestamp> for u32 {
    #[inline]
    fn from(value: CdisTimestamp) -> Self {
        value.to_u32()
    }
}

impl From<CdisTimestamp> for Duration {
    #[inline]
    fn from(value: CdisTimestamp) -> Self {
        value.to_duration()
    }
}

impl PartialEq<u32> for CdisTimestamp {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.to_u32().eq(other)
    }
}

impl PartialEq<CdisTimestamp> for u32 {
    #[inline]
    fn eq(&self, other: &CdisTimestamp) -> bool {
        self.eq(&other.to_u32())
    }
}

/// Units of time elapsed since the beginning of the current hour.
///
/// `CdisTimeUnits` is guaranteed to be less than `33554432`.
///
/// A time unit is approximately `107288.36059570` nanoseconds.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CdisTimeUnits(u32);

impl CdisTimeUnits {
    /// Size of `CdisTimeUnits` in bits.
    pub const BITS: usize = 25;

    /// Bitmask of `CdisTimeUnits`.
    pub const MASK: u32 = CDIS_TIME_UNITS_PER_HOUR - 1;

    /// A zero `CdisTimeUnits` representing the start of the hour.
    pub const ZERO: Self = Self(0);

    /// Maximum `CdisTimeUnits` representing one time unit before the start of the next hour.
    pub const MAX: Self = Self(CDIS_TIME_UNITS_PER_HOUR - 1);

    /// Constructs a new `CdisTimeUnits` from `value` if less than `33554432`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::CdisTimeUnits;
    /// let time_units = CdisTimeUnits::new(16_777_216);
    /// assert!(matches!(time_units, Some(_)));
    /// assert_eq!(time_units.unwrap().inner(), 16_777_216);
    ///
    /// let time_units = CdisTimeUnits::new(33_554_431);
    /// assert!(matches!(time_units, Some(_)));
    /// assert_eq!(time_units.unwrap().inner(), 33_554_431);
    ///
    /// let time_units = CdisTimeUnits::new(33_554_432);
    /// assert!(matches!(time_units, None));
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        if value < CDIS_TIME_UNITS_PER_HOUR {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Constructs a new `CdisTimeUnits` from `value` without checking if less than `33554432`.
    ///
    /// # Safety
    ///
    /// `value` must be less than `33554432`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::CdisTimeUnits;
    /// let time_units = unsafe { CdisTimeUnits::new_unchecked(16_777_216) };
    /// assert_eq!(time_units.inner(), 16_777_216);
    ///
    /// let time_units = unsafe { CdisTimeUnits::new_unchecked(33_554_431) };
    /// assert_eq!(time_units.inner(), 33_554_431);
    /// ```
    #[inline]
    #[must_use]
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    /// Constructs a new `CdisTimeUnits` from `duration` if the total number of nanoseconds
    /// is less than or equal to `3599999892712`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::CdisTimeUnits;
    /// # use core::time::Duration;
    /// let time_units = CdisTimeUnits::from_duration(Duration::from_mins(30));
    /// assert!(matches!(time_units, Some(_)));
    /// assert_eq!(time_units.unwrap().to_duration(), Duration::from_mins(30));
    ///
    /// let time_units = CdisTimeUnits::from_duration(Duration::from_nanos(3_599_999_892_712));
    /// assert!(matches!(time_units, Some(_)));
    /// assert_eq!(time_units.unwrap().to_duration(), Duration::from_nanos(3_599_999_892_712));
    ///
    /// let time_units = CdisTimeUnits::from_duration(Duration::from_nanos(3_599_999_892_713));
    /// assert!(matches!(time_units, None));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_duration(duration: Duration) -> Option<Self> {
        let nanos = duration.as_nanos();

        if nanos <= MAX_NANOS as u128 {
            let time_units = ((nanos as f64) / NANOS_PER_CDIS_TIME_UNIT).round() as u32;
            Some(Self(time_units))
        } else {
            None
        }
    }

    /// Returns the inner [`u32`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::CdisTimeUnits;
    /// # use core::time::Duration;
    /// let time_units = CdisTimeUnits::new(16_777_216).unwrap();
    /// assert_eq!(time_units.inner(), 16_777_216);
    ///
    /// let time_units = CdisTimeUnits::from_duration(Duration::from_mins(30)).unwrap();
    /// assert_eq!(time_units.inner(), 16_777_216);
    /// ```
    #[inline]
    #[must_use]
    pub const fn inner(self) -> u32 {
        self.0
    }

    /// Returns `true` if this `CdisTimeUnits` is [`Self::ZERO`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::CdisTimeUnits;
    /// let time_units = CdisTimeUnits::ZERO;
    /// assert!(time_units.is_zero());
    /// assert_eq!(time_units, CdisTimeUnits::new(0).unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == Self::ZERO.0
    }

    /// Returns `true` if this `CdisTimeUnits` is [`Self::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::CdisTimeUnits;
    /// let time_units = CdisTimeUnits::MAX;
    /// assert!(time_units.is_max());
    /// assert_eq!(time_units, CdisTimeUnits::new(33_554_431).unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_max(self) -> bool {
        self.0 == Self::MAX.0
    }

    /// Converts this `CdisTimeUnits` into a [`Duration`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use cdis_assemble::records::timestamp::CdisTimeUnits;
    /// # use core::time::Duration;
    /// let time_units = CdisTimeUnits::new(16_777_216).unwrap();
    /// assert_eq!(time_units.to_duration(), Duration::from_mins(30));
    ///
    /// let time_units = CdisTimeUnits::from_duration(Duration::from_mins(30)).unwrap();
    /// assert_eq!(time_units.to_duration(), Duration::from_mins(30));
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_duration(self) -> Duration {
        let nanos = ((self.0 as f64) * NANOS_PER_CDIS_TIME_UNIT).round() as u64;
        Duration::from_nanos(nanos)
    }
}

const _: () = {
    assert!(CdisTimeUnits::BITS == CdisTimeUnits::MASK.count_ones() as usize);
    assert!(CdisTimeUnits::MAX.to_duration().as_nanos() == MAX_NANOS as u128);
};

impl Default for CdisTimeUnits {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Display for CdisTimeUnits {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<TimeUnits> for CdisTimeUnits {
    #[inline]
    fn from(value: TimeUnits) -> Self {
        // SAFETY: 2147483647 / 64 < 33554432
        //         => 33554431 < 33554432
        #[allow(unsafe_code)]
        unsafe {
            Self::new_unchecked(value.inner() / TIME_UNITS_PER_CDIS_TIME_UNIT)
        }
    }
}

impl From<CdisTimeUnits> for TimeUnits {
    #[inline]
    fn from(value: CdisTimeUnits) -> Self {
        // SAFETY: 33554431 * 64 < 2147483648
        //         => 2147483584 < 2147483648
        #[allow(unsafe_code)]
        unsafe {
            Self::new_unchecked(value.0 * TIME_UNITS_PER_CDIS_TIME_UNIT)
        }
    }
}

impl From<CdisTimeUnits> for u32 {
    #[inline]
    fn from(value: CdisTimeUnits) -> Self {
        value.inner()
    }
}

impl From<CdisTimeUnits> for Duration {
    #[inline]
    fn from(value: CdisTimeUnits) -> Self {
        value.to_duration()
    }
}

impl PartialEq<u32> for CdisTimeUnits {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<CdisTimeUnits> for u32 {
    #[inline]
    fn eq(&self, other: &CdisTimeUnits) -> bool {
        self.eq(&other.0)
    }
}

impl PartialOrd<u32> for CdisTimeUnits {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<CdisTimeUnits> for u32 {
    #[inline]
    fn partial_cmp(&self, other: &CdisTimeUnits) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::cmp::Ordering;

    #[rstest]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::ZERO), [0x00, 0x00, 0x00, 0x00])]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(2_386_093).unwrap()), [0x12 ,0x34, 0x56, 0b1000_0000])]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::MAX), [0xff, 0xff, 0xff, 0b1000_0000])]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::ZERO), [0x00, 0x00, 0x00, 0b0100_0000])]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(2_386_093).unwrap()), [0x12, 0x34, 0x56, 0b1100_0000])]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::MAX), [0xff, 0xff, 0xff, 0b1100_0000])]
    fn timestamp_parse_and_serialize_roundtrip(
        #[case] timestamp: CdisTimestamp,
        #[case] expected: [u8; std::mem::size_of::<u32>()],
    ) {
        let (input, timestamp_parse) =
            nom::bits::complete::take::<&[u8], u32, usize, nom::error::Error<(&[u8], usize)>>(
                CdisTimestamp::BITS,
            )((expected.as_slice(), 0))
            .map(|(input, timestamp)| (input, CdisTimestamp::new(timestamp)))
            .unwrap();
        assert_eq!(input.0, &[expected[3]]);
        assert_eq!(input.1, 2);
        assert_eq!(timestamp_parse, timestamp);

        let mut buf = BitBuffer::ZERO;
        let cursor = timestamp.serialize(&mut buf, 0);
        assert_eq!(cursor, CdisTimestamp::BITS);
        assert_eq!(&buf.as_raw_slice()[..expected.len()], expected.as_slice());
    }

    #[rstest]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::ZERO), [0x00, 0x00, 0x00, 0b0011_1111])]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(2_386_093).unwrap()), [0x12 ,0x34, 0x56, 0b1011_1111])]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::MAX), [0xff, 0xff, 0xff, 0b1011_1111])]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::ZERO), [0x00, 0x00, 0x00, 0b0111_1111])]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(2_386_093).unwrap()), [0x12, 0x34, 0x56, 0b1111_1111])]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::MAX), [0xff, 0xff, 0xff, 0b1111_1111])]
    fn timestamp_parse_and_serialize_roundtrip_truncation(
        #[case] timestamp: CdisTimestamp,
        #[case] expected: [u8; std::mem::size_of::<u32>()],
    ) {
        let mut expected_truncation = expected;
        expected_truncation[3] &= 0b1100_0000;

        let (input, timestamp_parse) =
            nom::bits::complete::take::<&[u8], u32, usize, nom::error::Error<(&[u8], usize)>>(
                CdisTimestamp::BITS,
            )((expected.as_slice(), 0))
            .map(|(input, timestamp)| (input, CdisTimestamp::new(timestamp)))
            .unwrap();
        assert_eq!(input.0, &[expected[3]]);
        assert_eq!(input.1, 2);
        assert_eq!(timestamp_parse, timestamp);

        let mut buf = BitBuffer::ZERO;
        let cursor = timestamp.serialize(&mut buf, 0);
        assert_eq!(cursor, CdisTimestamp::BITS);
        assert_eq!(
            &buf.as_raw_slice()[..expected.len()],
            expected_truncation.as_slice()
        );
    }

    #[rstest]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::ZERO), 0)]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(16_777_216).unwrap()), 33_554_432)]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::MAX), 67_108_862)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::ZERO), 1)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(16_777_216).unwrap()), 33_554_433)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::MAX), 67_108_863)]
    fn timestamp_u32_roundtrip(#[case] timestamp: CdisTimestamp, #[case] expected: u32) {
        assert_eq!(CdisTimestamp::new(expected), timestamp);
        assert_eq!(CdisTimestamp::from(expected), timestamp);
        assert_eq!(timestamp.to_u32(), expected);
        assert_eq!(u32::from(timestamp), expected);
    }

    #[rstest]
    #[case(
        Timestamp::Relative(TimeUnits::ZERO),
        CdisTimestamp::Relative(CdisTimeUnits::ZERO)
    )]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT).unwrap()), CdisTimestamp::Relative(CdisTimeUnits::new(1).unwrap()))]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT + 1).unwrap()), CdisTimestamp::Relative(CdisTimeUnits::new(1).unwrap()))]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT + (TIME_UNITS_PER_CDIS_TIME_UNIT - 1)).unwrap()), CdisTimestamp::Relative(CdisTimeUnits::new(1).unwrap()))]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT * 2).unwrap()), CdisTimestamp::Relative(CdisTimeUnits::new(2).unwrap()))]
    #[case(Timestamp::Relative(TimeUnits::new((TIME_UNITS_PER_HOUR / 2) + 1).unwrap()), CdisTimestamp::Relative(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()))]
    #[case(
        Timestamp::Relative(TimeUnits::MAX),
        CdisTimestamp::Relative(CdisTimeUnits::MAX)
    )]
    #[case(
        Timestamp::Absolute(TimeUnits::ZERO),
        CdisTimestamp::Absolute(CdisTimeUnits::ZERO)
    )]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT).unwrap()), CdisTimestamp::Absolute(CdisTimeUnits::new(1).unwrap()))]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT + 1).unwrap()), CdisTimestamp::Absolute(CdisTimeUnits::new(1).unwrap()))]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT + (TIME_UNITS_PER_CDIS_TIME_UNIT - 1)).unwrap()), CdisTimestamp::Absolute(CdisTimeUnits::new(1).unwrap()))]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT * 2).unwrap()), CdisTimestamp::Absolute(CdisTimeUnits::new(2).unwrap()))]
    #[case(Timestamp::Absolute(TimeUnits::new((TIME_UNITS_PER_HOUR / 2) + 1).unwrap()), CdisTimestamp::Absolute(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()))]
    #[case(
        Timestamp::Absolute(TimeUnits::MAX),
        CdisTimestamp::Absolute(CdisTimeUnits::MAX)
    )]
    fn dis_to_cdis_timestamp(
        #[case] dis_timestamp: Timestamp,
        #[case] cdis_timestamp: CdisTimestamp,
    ) {
        assert_eq!(CdisTimestamp::from(dis_timestamp), cdis_timestamp);
    }

    #[rstest]
    #[case(
        CdisTimestamp::Relative(CdisTimeUnits::ZERO),
        Timestamp::Relative(TimeUnits::ZERO)
    )]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(1).unwrap()), Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT).unwrap()))]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(2).unwrap()), Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT * 2).unwrap()))]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()), Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()))]
    #[case(
        CdisTimestamp::Relative(CdisTimeUnits::MAX),
        Timestamp::Relative(TimeUnits::new(2_147_483_584).unwrap())
    )]
    #[case(
        CdisTimestamp::Absolute(CdisTimeUnits::ZERO),
        Timestamp::Absolute(TimeUnits::ZERO)
    )]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(1).unwrap()), Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT).unwrap()))]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(2).unwrap()), Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT * 2).unwrap()))]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()), Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()))]
    #[case(
        CdisTimestamp::Absolute(CdisTimeUnits::MAX),
        Timestamp::Absolute(TimeUnits::new(2_147_483_584).unwrap())
    )]
    fn cdis_to_dis_timestamp(
        #[case] cdis_timestamp: CdisTimestamp,
        #[case] dis_timestamp: Timestamp,
    ) {
        assert_eq!(Timestamp::from(cdis_timestamp), dis_timestamp);
    }

    #[rstest]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::ZERO), Duration::ZERO)]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()), Duration::from_mins(30))]
    #[case(
        CdisTimestamp::Relative(CdisTimeUnits::MAX),
        Duration::from_nanos(3_599_999_892_712)
    )]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::ZERO), Duration::ZERO)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()), Duration::from_mins(30))]
    #[case(
        CdisTimestamp::Absolute(CdisTimeUnits::MAX),
        Duration::from_nanos(3_599_999_892_712)
    )]
    fn timestamp_duration_consistency(
        #[case] timestamp: CdisTimestamp,
        #[case] duration: Duration,
    ) {
        assert_eq!(timestamp.to_duration(), duration);
        assert_eq!(Duration::from(timestamp), duration);
    }

    #[rstest]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::ZERO))]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()))]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::MAX))]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::ZERO))]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()))]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::MAX))]
    fn timestamp_time_units_consistency(#[case] timestamp: CdisTimestamp) {
        assert_eq!(
            timestamp.to_duration(),
            timestamp.time_units().to_duration()
        );
    }

    #[rstest]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::ZERO))]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::MAX))]
    fn timestamp_is_relative(#[case] timestamp: CdisTimestamp) {
        assert!(timestamp.is_relative());
        assert!(!timestamp.is_absolute());
    }

    #[rstest]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::ZERO))]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::MAX))]
    fn timestamp_is_absolute(#[case] timestamp: CdisTimestamp) {
        assert!(!timestamp.is_relative());
        assert!(timestamp.is_absolute());
    }

    #[test]
    fn timestamp_default() {
        assert_eq!(
            CdisTimestamp::default(),
            CdisTimestamp::Relative(CdisTimeUnits::ZERO)
        );
    }

    #[rstest]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::ZERO), 0, true)]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()), 33_554_432, true)]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::MAX), 67_108_862, true)]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::ZERO), 1, false)]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()), 33_554_433, false)]
    #[case(CdisTimestamp::Relative(CdisTimeUnits::MAX), 67_108_863, false)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::ZERO), 1, true)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()), 33_554_433, true)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::MAX), 67_108_863, true)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::ZERO), 0, false)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap()), 33_554_432, false)]
    #[case(CdisTimestamp::Absolute(CdisTimeUnits::MAX), 67_108_862, false)]
    fn timestamp_partial_eq(
        #[case] timestamp: CdisTimestamp,
        #[case] value: u32,
        #[case] expected: bool,
    ) {
        assert_eq!(timestamp == value, expected);
        assert_eq!(value == timestamp, expected);
    }

    #[rstest]
    #[case(TimeUnits::ZERO, CdisTimeUnits::ZERO)]
    #[case(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT).unwrap(), CdisTimeUnits::new(1).unwrap())]
    #[case(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT + 1).unwrap(), CdisTimeUnits::new(1).unwrap())]
    #[case(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT + (TIME_UNITS_PER_CDIS_TIME_UNIT - 1)).unwrap(), CdisTimeUnits::new(1).unwrap())]
    #[case(TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT * 2).unwrap(), CdisTimeUnits::new(2).unwrap())]
    #[case(TimeUnits::new((TIME_UNITS_PER_HOUR / 2) + 1).unwrap(), CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap())]
    #[case(TimeUnits::MAX, CdisTimeUnits::MAX)]
    fn dis_to_cdis_time_units(
        #[case] dis_time_units: TimeUnits,
        #[case] cdis_time_units: CdisTimeUnits,
    ) {
        assert_eq!(CdisTimeUnits::from(dis_time_units), cdis_time_units);
    }

    #[rstest]
    #[case(CdisTimeUnits::ZERO, TimeUnits::ZERO)]
    #[case(CdisTimeUnits::new(1).unwrap(), TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT).unwrap())]
    #[case(CdisTimeUnits::new(2).unwrap(), TimeUnits::new(TIME_UNITS_PER_CDIS_TIME_UNIT * 2).unwrap())]
    #[case(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap(), TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap())]
    #[case(CdisTimeUnits::MAX, TimeUnits::new(2_147_483_584).unwrap())]
    fn cdis_to_dis_time_units(
        #[case] cdis_time_units: CdisTimeUnits,
        #[case] dis_time_units: TimeUnits,
    ) {
        assert_eq!(TimeUnits::from(cdis_time_units), dis_time_units);
    }

    #[rstest]
    #[case(CdisTimeUnits::ZERO, Duration::ZERO)]
    #[case(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap(), Duration::from_mins(30))]
    #[case(CdisTimeUnits::MAX, Duration::from_nanos(3_599_999_892_712))]
    fn time_units_duration_consistency(
        #[case] time_units: CdisTimeUnits,
        #[case] duration: Duration,
    ) {
        assert_eq!(CdisTimeUnits::from_duration(duration).unwrap(), time_units);
        assert_eq!(Duration::from(time_units), duration);
        assert_eq!(time_units.to_duration(), duration);
    }

    #[rstest]
    #[case(CdisTimeUnits::ZERO, 0)]
    #[case(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap(), CDIS_TIME_UNITS_PER_HOUR / 2)]
    #[case(CdisTimeUnits::MAX, 33_554_431)]
    fn time_units_inner_value(#[case] time_units: CdisTimeUnits, #[case] value: u32) {
        assert_eq!(time_units.inner(), value);
        assert_eq!(u32::from(time_units), value);
    }

    #[rstest]
    #[case(CdisTimeUnits::ZERO.inner())]
    #[case(CDIS_TIME_UNITS_PER_HOUR / 2)]
    #[case(CdisTimeUnits::MAX.inner())]
    fn time_units_ok(#[case] value: u32) {
        let time_units = CdisTimeUnits::new(value);
        assert!(time_units.is_some());
        assert_eq!(time_units.unwrap().inner(), value);
    }

    #[rstest]
    #[case(CdisTimeUnits::MAX.inner() + 1)]
    #[case(CDIS_TIME_UNITS_PER_HOUR)]
    #[case(u32::MAX)]
    fn time_units_err(#[case] value: u32) {
        assert!(CdisTimeUnits::new(value).is_none());
    }

    #[rstest]
    fn time_units_is_zero() {
        assert!(CdisTimeUnits::ZERO.is_zero());
        assert!(!CdisTimeUnits::MAX.is_zero());
        assert!(!CdisTimeUnits::new(1).unwrap().is_zero());
    }

    #[rstest]
    fn time_units_is_max() {
        assert!(!CdisTimeUnits::ZERO.is_max());
        assert!(CdisTimeUnits::MAX.is_max());
        assert!(!CdisTimeUnits::new(33_554_430).unwrap().is_max());
    }

    #[test]
    fn time_units_default() {
        assert_eq!(CdisTimeUnits::default(), CdisTimeUnits::ZERO);
    }

    #[rstest]
    #[case(CdisTimeUnits::ZERO, "0")]
    #[case(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap(), "16777216")]
    #[case(CdisTimeUnits::MAX, "33554431")]
    fn time_units_display(#[case] time_units: CdisTimeUnits, #[case] expected: &str) {
        assert_eq!(time_units.to_string(), expected);
    }

    #[rstest]
    #[case(CdisTimeUnits::ZERO, 0, true)]
    #[case(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap(), 16_777_216, true)]
    #[case(CdisTimeUnits::MAX, 33_554_431, true)]
    #[case(CdisTimeUnits::ZERO, 1, false)]
    #[case(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap(), 16_777_217, false)]
    #[case(CdisTimeUnits::MAX, 33_554_432, false)]
    fn time_units_partial_eq(
        #[case] time_units: CdisTimeUnits,
        #[case] value: u32,
        #[case] expected: bool,
    ) {
        assert_eq!(time_units == value, expected);
        assert_eq!(value == time_units, expected);
    }

    #[rstest]
    #[case(CdisTimeUnits::ZERO, 0, Ordering::Equal)]
    #[case(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap(), 16_777_216, Ordering::Equal)]
    #[case(CdisTimeUnits::MAX, 33_554_431, Ordering::Equal)]
    #[case(CdisTimeUnits::ZERO, 33_554_431, Ordering::Less)]
    #[case(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap(), 16_777_215, Ordering::Greater)]
    #[case(CdisTimeUnits::new(CDIS_TIME_UNITS_PER_HOUR / 2).unwrap(), 16_777_217, Ordering::Less)]
    #[case(CdisTimeUnits::MAX, 0, Ordering::Greater)]
    fn time_units_partial_ord(
        #[case] time_units: CdisTimeUnits,
        #[case] value: u32,
        #[case] expected: Ordering,
    ) {
        let expected = Some(expected);

        assert_eq!(time_units.partial_cmp(&value), expected);
        assert_eq!(
            value.partial_cmp(&time_units),
            expected.map(Ordering::reverse)
        );
    }

    #[rstest]
    #[case(CdisTimeUnits::ZERO, 1, CdisTimeUnits::new(2).unwrap())]
    #[case(CdisTimeUnits::ZERO, 16_777_216, CdisTimeUnits::MAX)]
    #[case(CdisTimeUnits::new(33_554_429).unwrap(), 33_554_430, CdisTimeUnits::MAX)]
    fn time_units_partial_ord_transitive(
        #[case] a: CdisTimeUnits,
        #[case] b: u32,
        #[case] c: CdisTimeUnits,
    ) {
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
        assert_eq!(b.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(c.partial_cmp(&b), Some(Ordering::Greater));
        assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(c.partial_cmp(&a), Some(Ordering::Greater));
    }
}
