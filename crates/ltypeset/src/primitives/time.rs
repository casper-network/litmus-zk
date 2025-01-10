use crate::utils::deconstruct_bytes;
use humantime;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{self, Display, Formatter},
    time::{Duration, SystemTime},
};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// A timestamp type, representing a concrete moment in time.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub struct TimeDiff(Duration);

/// A timestamp type, representing a concrete moment in time.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timestamp(u128);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl TimeDiff {
    pub fn new(ms_since_epoch: Duration) -> Self {
        Self(ms_since_epoch)
    }

    // Returns instance hydrated from current system time.
    pub fn new_from_now() -> Self {
        Self(SystemTime::UNIX_EPOCH.elapsed().unwrap())
    }
}

impl Timestamp {
    pub fn new(ms_since_epoch: u128) -> Self {
        Self(ms_since_epoch)
    }

    // Returns instance hydrated from current system time.
    pub fn new_from_now() -> Self {
        Self(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis())
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl TimeDiff {
    /// The maximum value a time difference can have.
    pub const MAX: TimeDiff = TimeDiff(Duration::MAX);

    /// The minimum value a time difference can have.
    pub const MIN: TimeDiff = TimeDiff(Duration::from_secs(0));

    /// Returns time difference as number of milliseconds since Unix epoch
    pub fn inner(&self) -> Duration {
        self.0
    }
}

impl Timestamp {
    /// The maximum value a timestamp can have.
    pub const MAX: Timestamp = Timestamp(u128::MAX);

    /// The minimum value a timestamp can have.
    pub const MIN: Timestamp = Timestamp(u128::MIN);

    /// Returns time difference as number of milliseconds since Unix epoch
    pub fn inner(&self) -> u128 {
        self.0
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl Display for TimeDiff {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl Display for Timestamp {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        unimplemented!();
    }
}

impl From<u128> for Timestamp {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl From<&str> for Timestamp {
    fn from(value: &str) -> Self {
        Self::from(
            humantime::parse_rfc3339_weak(value)
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        )
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimestampVistor;

        impl<'de> Visitor<'de> for TimestampVistor {
            type Value = Timestamp;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("utf-8 representation of milliseconds since Unix epoch")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let (bytes, remainder) = deconstruct_bytes::<16>(v).unwrap();
                assert_eq!(remainder.len(), 0);
                Ok(Timestamp::from(u128::from_le_bytes(bytes)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp::from(v))
            }
        }

        deserializer.deserialize_any(TimestampVistor)
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.inner().to_le_bytes())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
use rand::Rng;

#[cfg(test)]
impl TimeDiff {
    /// Returns a random `TimeDiff`.
    #[cfg(any(feature = "testing", test))]
    pub fn random() -> Self {
        let as_ms = 1_596_763_000_000 + rand::thread_rng().gen_range(200_000..1_000_000);
        Self::new(Duration::from_millis(as_ms))
    }
}

#[cfg(test)]
impl Timestamp {
    /// Returns a random `Timestamp`.
    #[cfg(any(feature = "testing", test))]
    pub fn random() -> Self {
        Self::new(1_596_763_000_000 + rand::thread_rng().gen_range(200_000..1_000_000))
    }
}
