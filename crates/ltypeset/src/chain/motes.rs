use core::ops::Add;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Base unit of system economic security mechanism.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Motes(u64);

/// Constants.
impl Motes {
    /// Maximum possible value.
    pub const MIN: Motes = Motes(u64::MIN);
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Motes {
    /// Factory: new [`Motes`] instance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Motes {
    pub fn inner(&self) -> u64 {
        self.0
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl fmt::Display for Motes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MOTES::{}", self.inner())
    }
}

impl Add for Motes {
    type Output = Motes;

    fn add(self, rhs: Self) -> Self::Output {
        Motes::new(self.inner() + rhs.inner())
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Motes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MotesVistor;

        impl<'de> Visitor<'de> for MotesVistor {
            type Value = Motes;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("supported formats: 64 char hex encoded string | 32 byte array")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Motes(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_u64(v.parse().unwrap())
            }
        }

        deserializer.deserialize_any(MotesVistor)
    }
}

impl Serialize for Motes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_u64(self.inner()).unwrap())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
use rand::Rng;

#[cfg(test)]
impl Motes {
    pub fn new_from_arb() -> impl Strategy<Value = Self> {
        any::<u64>().prop_map(Self::new)
    }

    pub fn new_from_random() -> Self {
        Self::new(rand::thread_rng().gen())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_random() {
        for _ in 0..10 {
            Motes::new_from_random();
        }
    }

    proptest! {
        #[test]
        fn new_from_arb(_ in Motes::new_from_arb()) {
            ()
        }
    }
}
