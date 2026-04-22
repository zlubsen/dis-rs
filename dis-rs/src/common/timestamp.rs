use crate::Serialize;
use bytes::BufMut;
use core::{fmt::Display, time::Duration};

/// Number of [`TimeUnits`] in one hour.
pub const TIME_UNITS_PER_HOUR: u32 = 1 << 31;

/// Number of nanoseconds in one hour.
const NANOS_PER_HOUR: u64 = 3_600_000_000_000;

/// Number of nanoseconds in one [`TimeUnits`].
#[allow(clippy::cast_precision_loss)]
const NANOS_PER_TIME_UNIT: f64 = NANOS_PER_HOUR as f64 / TIME_UNITS_PER_HOUR as f64;

/// Maximum number of nanoseconds.
const MAX_NANOS: u64 = NANOS_PER_HOUR - (NANOS_PER_TIME_UNIT.round() as u64);

/// Reference time at which the data contained in the *PDU* was generated.
///
/// Time is represented as [`TimeUnits`] elapsed since the beginning of the current hour in the selected time reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Timestamp {
    /// Time is *relative* to the simulation application issuing the *PDU*.
    ///
    /// A *relative* timestamp should be used when host clocks are not synchronized.
    Relative(TimeUnits),
    /// Time is *absolute* to the simulation time.
    ///
    /// An *absolute* timestamp should be used when host clocks are synchronized.
    Absolute(TimeUnits),
}

impl Timestamp {
    /// Bit indicating a *relative* timestamp.
    pub const RELATIVE_BIT: u32 = 0b0;

    /// Bit indicating an *absolute* timestamp.
    pub const ABSOLUTE_BIT: u32 = 0b1;

    /// Constructs a new `Timestamp` from `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dis_rs::model::{Timestamp, TimeUnits};
    /// let timestamp = Timestamp::new(20);
    /// assert_eq!(timestamp, Timestamp::Relative(TimeUnits::new(10).unwrap()));
    ///
    /// let timestamp = Timestamp::new(21);
    /// assert_eq!(timestamp, Timestamp::Absolute(TimeUnits::new(10).unwrap()));
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: u32) -> Self {
        // SAFETY: right-shifting a u32 by one bit guarantees the value is always less than 21474836478
        #[allow(unsafe_code)]
        let time_units = unsafe { TimeUnits::new_unchecked(value >> 1) };
        let is_relative = (value & 1) == Self::RELATIVE_BIT;

        if is_relative {
            Self::Relative(time_units)
        } else {
            Self::Absolute(time_units)
        }
    }

    /// Returns the [`TimeUnits`] contained by this `Timestamp`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dis_rs::model::{Timestamp, TimeUnits};
    /// let timestamp = Timestamp::new(20);
    /// assert_eq!(timestamp.time_units(), TimeUnits::new(10).unwrap());
    ///
    /// let timestamp = Timestamp::new(21);
    /// assert_eq!(timestamp.time_units(), TimeUnits::new(10).unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub const fn time_units(self) -> TimeUnits {
        match self {
            Self::Absolute(time_units) | Self::Relative(time_units) => time_units,
        }
    }

    /// Returns `true` if this `Timestamp` is *relative*.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dis_rs::model::{Timestamp, TimeUnits};
    /// let timestamp = Timestamp::Relative(TimeUnits::new(10).unwrap());
    /// assert!(timestamp.is_relative());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_relative(self) -> bool {
        matches!(self, Self::Relative(_))
    }

    /// Returns `true` if this `Timestamp` is *absolute*.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dis_rs::model::{Timestamp, TimeUnits};
    /// let timestamp = Timestamp::Absolute(TimeUnits::new(10).unwrap());
    /// assert!(timestamp.is_absolute());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_absolute(self) -> bool {
        matches!(self, Self::Absolute(_))
    }

    /// Converts this `Timestamp` into a [`u32`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dis_rs::model::{Timestamp, TimeUnits};
    /// let timestamp = Timestamp::Relative(TimeUnits::new(10).unwrap());
    /// assert_eq!(timestamp.to_u32(), 20);
    ///
    /// let timestamp = Timestamp::Absolute(TimeUnits::new(10).unwrap());
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

    /// Converts this `Timestamp` into a [`Duration`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::time::Duration;
    /// # use dis_rs::model::{Timestamp, TimeUnits};
    /// let timestamp = Timestamp::Relative(TimeUnits::new(1_073_741_824).unwrap());
    /// assert_eq!(timestamp.to_duration(), Duration::from_mins(30));
    ///
    /// let timestamp = Timestamp::Absolute(TimeUnits::new(1_073_741_824).unwrap());
    /// assert_eq!(timestamp.to_duration(), Duration::from_mins(30));
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_duration(self) -> Duration {
        self.time_units().to_duration()
    }
}

impl Serialize for Timestamp {
    #[inline]
    fn serialize(&self, buf: &mut bytes::BytesMut) -> u16 {
        buf.put_u32(self.to_u32());
        4
    }
}

impl Default for Timestamp {
    #[inline]
    fn default() -> Self {
        Self::Relative(TimeUnits::default())
    }
}

impl From<u32> for Timestamp {
    #[inline]
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<Timestamp> for u32 {
    #[inline]
    fn from(value: Timestamp) -> Self {
        value.to_u32()
    }
}

impl From<Timestamp> for Duration {
    #[inline]
    fn from(value: Timestamp) -> Self {
        value.to_duration()
    }
}

impl PartialEq<u32> for Timestamp {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.to_u32().eq(other)
    }
}

impl PartialEq<Timestamp> for u32 {
    #[inline]
    fn eq(&self, other: &Timestamp) -> bool {
        self.eq(&other.to_u32())
    }
}

impl PartialOrd<u32> for Timestamp {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.to_u32().partial_cmp(other)
    }
}

impl PartialOrd<Timestamp> for u32 {
    #[inline]
    fn partial_cmp(&self, other: &Timestamp) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.to_u32())
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Timestamp;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Timestamp {
        #[inline]
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.to_u32().serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Timestamp {
        #[inline]
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            u32::deserialize(deserializer).map(Self::new)
        }
    }
}

/// Units of time elapsed since the beginning of the current hour.
///
/// `TimeUnits` is guaranteed to be less than `2147483648`.
///
/// A time unit is approximately `1676.38063431` nanoseconds.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeUnits(u32);

impl TimeUnits {
    /// A zero `TimeUnits` representing the start of the hour.
    pub const ZERO: Self = Self(0);

    /// Maximum `TimeUnits` representing one time unit before the start of the next hour.
    pub const MAX: Self = Self(TIME_UNITS_PER_HOUR - 1);

    /// Constructs a new `TimeUnits` from `value` if less than `2147483648`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dis_rs::model::TimeUnits;
    /// let time_units = TimeUnits::new(1_073_741_824);
    /// assert!(matches!(time_units, Some(_)));
    /// assert_eq!(time_units.unwrap().inner(), 1_073_741_824);
    ///
    /// let time_units = TimeUnits::new(2_147_483_647);
    /// assert!(matches!(time_units, Some(_)));
    /// assert_eq!(time_units.unwrap().inner(), 2_147_483_647);
    ///
    /// let time_units = TimeUnits::new(2_147_483_648);
    /// assert!(matches!(time_units, None));
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        if value < TIME_UNITS_PER_HOUR {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Constructs a new `TimeUnits` from `value` without checking if less than `2147483648`.
    ///
    /// # Safety
    ///
    /// `value` must be less than `2147483648`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dis_rs::model::TimeUnits;
    /// let time_units = unsafe { TimeUnits::new_unchecked(1_073_741_824) };
    /// assert_eq!(time_units.inner(), 1_073_741_824);
    ///
    /// let time_units = unsafe { TimeUnits::new_unchecked(2_147_483_647) };
    /// assert_eq!(time_units.inner(), 2_147_483_647);
    /// ```
    #[inline]
    #[must_use]
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    /// Constructs a new `TimeUnits` from `duration` if the total number of nanoseconds
    /// is less than or equal to `3599999998324`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::time::Duration;
    /// # use dis_rs::model::TimeUnits;
    /// let time_units = TimeUnits::from_duration(Duration::from_mins(30));
    /// assert!(matches!(time_units, Some(_)));
    /// assert_eq!(time_units.unwrap().to_duration(), Duration::from_mins(30));
    ///
    /// let time_units = TimeUnits::from_duration(Duration::from_nanos(3_599_999_998_324));
    /// assert!(matches!(time_units, Some(_)));
    /// assert_eq!(time_units.unwrap().to_duration(), Duration::from_nanos(3_599_999_998_324));
    ///
    /// let time_units = TimeUnits::from_duration(Duration::from_nanos(3_599_999_998_325));
    /// assert!(matches!(time_units, None));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_duration(duration: Duration) -> Option<Self> {
        let nanos = duration.as_nanos();

        if nanos <= MAX_NANOS as u128 {
            let time_units = ((nanos as f64) / NANOS_PER_TIME_UNIT).round() as u32;
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
    /// # use core::time::Duration;
    /// # use dis_rs::model::TimeUnits;
    /// let time_units = TimeUnits::new(1_073_741_824).unwrap();
    /// assert_eq!(time_units.inner(), 1_073_741_824);
    ///
    /// let time_units = TimeUnits::from_duration(Duration::from_mins(30)).unwrap();
    /// assert_eq!(time_units.inner(), 1_073_741_824);
    /// ```
    #[inline]
    #[must_use]
    pub const fn inner(self) -> u32 {
        self.0
    }

    /// Returns `true` if this `TimeUnits` is [`Self::ZERO`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dis_rs::model::TimeUnits;
    /// let time_units = TimeUnits::ZERO;
    /// assert!(time_units.is_zero());
    /// assert_eq!(time_units, TimeUnits::new(0).unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == Self::ZERO.0
    }

    /// Returns `true` if this `TimeUnits` is [`Self::MAX`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use dis_rs::model::TimeUnits;
    /// let time_units = TimeUnits::MAX;
    /// assert!(time_units.is_max());
    /// assert_eq!(time_units, TimeUnits::new(2_147_483_647).unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_max(self) -> bool {
        self.0 == Self::MAX.0
    }

    /// Converts this `TimeUnits` into a [`Duration`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::time::Duration;
    /// # use dis_rs::model::TimeUnits;
    /// let time_units = TimeUnits::new(1_073_741_824).unwrap();
    /// assert_eq!(time_units.to_duration(), Duration::from_mins(30));
    ///
    /// let time_units = TimeUnits::from_duration(Duration::from_mins(30)).unwrap();
    /// assert_eq!(time_units.to_duration(), Duration::from_mins(30));
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_duration(self) -> Duration {
        let nanos = ((self.0 as f64) * NANOS_PER_TIME_UNIT).round() as u64;
        Duration::from_nanos(nanos)
    }
}

const _: () = {
    assert!(TimeUnits::MAX.to_duration().as_nanos() == MAX_NANOS as u128);
};

impl Default for TimeUnits {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Display for TimeUnits {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<TimeUnits> for u32 {
    #[inline]
    fn from(value: TimeUnits) -> Self {
        value.inner()
    }
}

impl From<TimeUnits> for Duration {
    #[inline]
    fn from(value: TimeUnits) -> Self {
        value.to_duration()
    }
}

impl PartialEq<u32> for TimeUnits {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<TimeUnits> for u32 {
    #[inline]
    fn eq(&self, other: &TimeUnits) -> bool {
        self.eq(&other.0)
    }
}

impl PartialOrd<u32> for TimeUnits {
    #[inline]
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<TimeUnits> for u32 {
    #[inline]
    fn partial_cmp(&self, other: &TimeUnits) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    use rstest::rstest;
    use std::cmp::Ordering;

    #[rstest]
    #[case(Timestamp::Relative(TimeUnits::ZERO), [0x00, 0x00, 0x00, 0x00])]
    #[case(Timestamp::Relative(TimeUnits::new(152_709_948).unwrap()), [0x12, 0x34, 0x56, 0x78])]
    #[case(Timestamp::Relative(TimeUnits::MAX), [0xff, 0xff, 0xff, 0xfe])]
    #[case(Timestamp::Absolute(TimeUnits::ZERO), [0x00, 0x00, 0x00, 0x01])]
    #[case(Timestamp::Absolute(TimeUnits::new(152_709_948).unwrap()), [0x12, 0x34, 0x56, 0x79])]
    #[case(Timestamp::Absolute(TimeUnits::MAX), [0xff, 0xff, 0xff, 0xff])]
    fn timestamp_parse_and_serialize_roundtrip(
        #[case] timestamp: Timestamp,
        #[case] expected: [u8; std::mem::size_of::<u32>()],
    ) {
        let (slice, timestamp_parse) =
            nom::number::complete::be_u32::<&[u8], nom::error::Error<&[u8]>>(expected.as_slice())
                .map(|(slice, timestamp)| (slice, Timestamp::new(timestamp)))
                .unwrap();
        assert!(slice.is_empty());
        assert_eq!(timestamp_parse, timestamp);

        let mut buf = BytesMut::with_capacity(expected.len());
        timestamp.serialize(&mut buf);
        assert_eq!(buf.len(), expected.len());
        assert_eq!(buf, expected.as_slice());
    }

    #[rstest]
    #[case(Timestamp::Relative(TimeUnits::ZERO), 0)]
    #[case(Timestamp::Relative(TimeUnits::new(123_456_789).unwrap()), 246_913_578)]
    #[case(Timestamp::Relative(TimeUnits::MAX), 4_294_967_294)]
    #[case(Timestamp::Absolute(TimeUnits::ZERO), 1)]
    #[case(Timestamp::Absolute(TimeUnits::new(123_456_789).unwrap()), 246_913_579)]
    #[case(Timestamp::Absolute(TimeUnits::MAX), 4_294_967_295)]
    fn timestamp_u32_roundtrip(#[case] timestamp: Timestamp, #[case] expected: u32) {
        assert_eq!(Timestamp::new(expected), timestamp);
        assert_eq!(Timestamp::from(expected), timestamp);
        assert_eq!(timestamp.to_u32(), expected);
        assert_eq!(u32::from(timestamp), expected);
    }

    #[rstest]
    #[case(Timestamp::Relative(TimeUnits::ZERO), Duration::ZERO)]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), Duration::from_mins(30))]
    #[case(
        Timestamp::Relative(TimeUnits::MAX),
        Duration::from_nanos(3_599_999_998_324)
    )]
    #[case(Timestamp::Absolute(TimeUnits::ZERO), Duration::ZERO)]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), Duration::from_mins(30))]
    #[case(
        Timestamp::Absolute(TimeUnits::MAX),
        Duration::from_nanos(3_599_999_998_324)
    )]
    fn timestamp_duration_consistency(#[case] timestamp: Timestamp, #[case] duration: Duration) {
        assert_eq!(timestamp.to_duration(), duration);
        assert_eq!(Duration::from(timestamp), duration);
    }

    #[rstest]
    #[case(Timestamp::Relative(TimeUnits::ZERO))]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()))]
    #[case(Timestamp::Relative(TimeUnits::MAX))]
    #[case(Timestamp::Absolute(TimeUnits::ZERO))]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()))]
    #[case(Timestamp::Absolute(TimeUnits::MAX))]
    fn timestamp_time_units_consistency(#[case] timestamp: Timestamp) {
        assert_eq!(
            timestamp.to_duration(),
            timestamp.time_units().to_duration()
        );
    }

    #[rstest]
    #[case(Timestamp::Relative(TimeUnits::ZERO))]
    #[case(Timestamp::Relative(TimeUnits::MAX))]
    fn timestamp_is_relative(#[case] timestamp: Timestamp) {
        assert!(timestamp.is_relative());
        assert!(!timestamp.is_absolute());
    }

    #[rstest]
    #[case(Timestamp::Absolute(TimeUnits::ZERO))]
    #[case(Timestamp::Absolute(TimeUnits::MAX))]
    fn timestamp_is_absolute(#[case] timestamp: Timestamp) {
        assert!(!timestamp.is_relative());
        assert!(timestamp.is_absolute());
    }

    #[test]
    fn timestamp_default() {
        assert_eq!(Timestamp::default(), Timestamp::Relative(TimeUnits::ZERO));
    }

    #[rstest]
    #[case(Timestamp::Relative(TimeUnits::ZERO), 0, true)]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_648, true)]
    #[case(Timestamp::Relative(TimeUnits::MAX), 4_294_967_294, true)]
    #[case(Timestamp::Relative(TimeUnits::ZERO), 1, false)]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_649, false)]
    #[case(Timestamp::Relative(TimeUnits::MAX), 4_294_967_295, false)]
    #[case(Timestamp::Absolute(TimeUnits::ZERO), 1, true)]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_649, true)]
    #[case(Timestamp::Absolute(TimeUnits::MAX), 4_294_967_295, true)]
    #[case(Timestamp::Absolute(TimeUnits::ZERO), 0, false)]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_648, false)]
    #[case(Timestamp::Absolute(TimeUnits::MAX), 4_294_967_294, false)]
    fn timestamp_partial_eq(
        #[case] timestamp: Timestamp,
        #[case] value: u32,
        #[case] expected: bool,
    ) {
        assert_eq!(timestamp == value, expected);
        assert_eq!(value == timestamp, expected);
    }

    #[rstest]
    #[case(Timestamp::Relative(TimeUnits::ZERO), 0, Ordering::Equal)]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_648, Ordering::Equal)]
    #[case(Timestamp::Relative(TimeUnits::MAX), 4_294_967_294, Ordering::Equal)]
    #[case(Timestamp::Relative(TimeUnits::ZERO), 4_294_967_294, Ordering::Less)]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_647, Ordering::Greater)]
    #[case(Timestamp::Relative(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_649, Ordering::Less)]
    #[case(Timestamp::Relative(TimeUnits::MAX), 0, Ordering::Greater)]
    #[case(Timestamp::Absolute(TimeUnits::ZERO), 1, Ordering::Equal)]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_649, Ordering::Equal)]
    #[case(Timestamp::Absolute(TimeUnits::MAX), 4_294_967_295, Ordering::Equal)]
    #[case(Timestamp::Absolute(TimeUnits::ZERO), 4_294_967_295, Ordering::Less)]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_648, Ordering::Greater)]
    #[case(Timestamp::Absolute(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap()), 2_147_483_650, Ordering::Less)]
    #[case(Timestamp::Absolute(TimeUnits::MAX), 0, Ordering::Greater)]
    fn timestamp_partial_ord(
        #[case] timestamp: Timestamp,
        #[case] value: u32,
        #[case] expected: Ordering,
    ) {
        let expected = Some(expected);

        assert_eq!(timestamp.partial_cmp(&value), expected);
        assert_eq!(
            value.partial_cmp(&timestamp),
            expected.map(Ordering::reverse)
        );
    }

    #[rstest]
    #[case(Timestamp::Relative(TimeUnits::ZERO), 1, Timestamp::Relative(TimeUnits::new(1).unwrap()))]
    #[case(
        Timestamp::Relative(TimeUnits::ZERO),
        2_147_483_648,
        Timestamp::Relative(TimeUnits::MAX)
    )]
    #[case(Timestamp::Relative(TimeUnits::new(2_147_483_646).unwrap()), 4_294_967_293, Timestamp::Relative(TimeUnits::MAX))]
    #[case(Timestamp::Absolute(TimeUnits::ZERO), 2, Timestamp::Absolute(TimeUnits::new(1).unwrap()))]
    #[case(
        Timestamp::Absolute(TimeUnits::ZERO),
        2_147_483_648,
        Timestamp::Absolute(TimeUnits::MAX)
    )]
    #[case(Timestamp::Absolute(TimeUnits::new(2_147_483_646).unwrap()), 4_294_967_294, Timestamp::Absolute(TimeUnits::MAX))]
    fn timestamp_partial_ord_transitive(
        #[case] a: Timestamp,
        #[case] b: u32,
        #[case] c: Timestamp,
    ) {
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
        assert_eq!(b.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(c.partial_cmp(&b), Some(Ordering::Greater));
        assert_eq!(a.partial_cmp(&c.to_u32()), Some(Ordering::Less));
        assert_eq!(c.partial_cmp(&a.to_u32()), Some(Ordering::Greater));
    }

    #[cfg(feature = "serde")]
    #[rstest]
    #[case(Timestamp::Relative(TimeUnits::ZERO), 0)]
    #[case(Timestamp::Relative(TimeUnits::new(123_456_789).unwrap()), 246_913_578)]
    #[case(Timestamp::Relative(TimeUnits::MAX), 4_294_967_294)]
    #[case(Timestamp::Absolute(TimeUnits::ZERO), 1)]
    #[case(Timestamp::Absolute(TimeUnits::new(123_456_789).unwrap()), 246_913_579)]
    #[case(Timestamp::Absolute(TimeUnits::MAX), 4_294_967_295)]
    fn timestamp_serde_roundtrip(#[case] timestamp: Timestamp, #[case] expected: u32) {
        let expected = serde_json::Value::Number(serde_json::Number::from(expected));

        let timestamp_de = serde_json::from_value::<Timestamp>(expected.clone()).unwrap();
        assert_eq!(timestamp_de, timestamp);

        let timestamp_ser = serde_json::to_value(timestamp).unwrap();
        assert_eq!(timestamp_ser, expected);
    }

    #[rstest]
    #[case(TimeUnits::ZERO, Duration::ZERO)]
    #[case(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap(), Duration::from_mins(30))]
    #[case(TimeUnits::MAX, Duration::from_nanos(3_599_999_998_324))]
    fn time_units_duration_consistency(#[case] time_units: TimeUnits, #[case] duration: Duration) {
        assert_eq!(TimeUnits::from_duration(duration).unwrap(), time_units);
        assert_eq!(Duration::from(time_units), duration);
        assert_eq!(time_units.to_duration(), duration);
    }

    #[rstest]
    #[case(TimeUnits::ZERO, 0)]
    #[case(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap(), TIME_UNITS_PER_HOUR / 2)]
    #[case(TimeUnits::MAX, 2_147_483_647)]
    fn time_units_inner_value(#[case] time_units: TimeUnits, #[case] value: u32) {
        assert_eq!(time_units.inner(), value);
        assert_eq!(u32::from(time_units), value);
    }

    #[rstest]
    #[case(TimeUnits::ZERO.inner())]
    #[case(TIME_UNITS_PER_HOUR / 2)]
    #[case(TimeUnits::MAX.inner())]
    fn time_units_ok(#[case] value: u32) {
        let time_units = TimeUnits::new(value);
        assert!(time_units.is_some());
        assert_eq!(time_units.unwrap().inner(), value);
    }

    #[rstest]
    #[case(TimeUnits::MAX.inner() + 1)]
    #[case(TIME_UNITS_PER_HOUR)]
    #[case(u32::MAX)]
    fn time_units_err(#[case] value: u32) {
        assert!(TimeUnits::new(value).is_none());
    }

    #[rstest]
    fn time_units_is_zero() {
        assert!(TimeUnits::ZERO.is_zero());
        assert!(!TimeUnits::MAX.is_zero());
        assert!(!TimeUnits::new(1).unwrap().is_zero());
    }

    #[rstest]
    fn time_units_is_max() {
        assert!(!TimeUnits::ZERO.is_max());
        assert!(TimeUnits::MAX.is_max());
        assert!(!TimeUnits::new(2_147_483_646).unwrap().is_max());
    }

    #[test]
    fn time_units_default() {
        assert_eq!(TimeUnits::default(), TimeUnits::ZERO);
    }

    #[rstest]
    #[case(TimeUnits::ZERO, "0")]
    #[case(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap(), "1073741824")]
    #[case(TimeUnits::MAX, "2147483647")]
    fn time_units_display(#[case] time_units: TimeUnits, #[case] expected: &str) {
        assert_eq!(time_units.to_string(), expected);
    }

    #[rstest]
    #[case(TimeUnits::ZERO, 0, true)]
    #[case(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap(), 1_073_741_824, true)]
    #[case(TimeUnits::MAX, 2_147_483_647, true)]
    #[case(TimeUnits::ZERO, 1, false)]
    #[case(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap(), 1_073_741_825, false)]
    #[case(TimeUnits::MAX, 2_147_483_646, false)]
    fn time_units_partial_eq(
        #[case] time_units: TimeUnits,
        #[case] value: u32,
        #[case] expected: bool,
    ) {
        assert_eq!(time_units == value, expected);
        assert_eq!(value == time_units, expected);
    }

    #[rstest]
    #[case(TimeUnits::ZERO, 0, Ordering::Equal)]
    #[case(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap(), 1_073_741_824, Ordering::Equal)]
    #[case(TimeUnits::MAX, 2_147_483_647, Ordering::Equal)]
    #[case(TimeUnits::ZERO, 2_147_483_647, Ordering::Less)]
    #[case(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap(), 1_073_741_823, Ordering::Greater)]
    #[case(TimeUnits::new(TIME_UNITS_PER_HOUR / 2).unwrap(), 1_073_741_825, Ordering::Less)]
    #[case(TimeUnits::MAX, 0, Ordering::Greater)]
    fn time_units_partial_ord(
        #[case] time_units: TimeUnits,
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
    #[case(TimeUnits::ZERO, 1, TimeUnits::new(2).unwrap())]
    #[case(TimeUnits::ZERO, 1_073_741_824, TimeUnits::MAX)]
    #[case(TimeUnits::new(2_147_483_645).unwrap(), 2_147_483_646, TimeUnits::MAX)]
    fn time_units_partial_ord_transitive(
        #[case] a: TimeUnits,
        #[case] b: u32,
        #[case] c: TimeUnits,
    ) {
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
        assert_eq!(b.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(c.partial_cmp(&b), Some(Ordering::Greater));
        assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(c.partial_cmp(&a), Some(Ordering::Greater));
    }
}
