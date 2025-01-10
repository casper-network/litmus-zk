use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticVersion {
    /// Major version.
    pub major: u32,

    /// Minor version.
    pub minor: u32,

    /// Patch version.
    pub patch: u32,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl SemanticVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        // TODO: validate inputs
        Self {
            major,
            minor,
            patch,
        }
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SEM-VAR::{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for SemanticVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SemanticVersionVistor;

        impl<'de> Visitor<'de> for SemanticVersionVistor {
            type Value = SemanticVersion;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("supported formats: 64 char hex encoded string | 32 byte array")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let tokens: Vec<&str> = v.split('.').collect();
                if tokens.len() != 3 {
                    panic!("SemanticVersion deserialization error.")
                }

                Ok(SemanticVersion {
                    major: tokens[0].parse().unwrap(),
                    minor: tokens[1].parse().unwrap(),
                    patch: tokens[2].parse().unwrap(),
                })
            }
        }

        deserializer.deserialize_any(SemanticVersionVistor)
    }
}

impl Serialize for SemanticVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}.{}.{}", self.major, self.minor, self.patch);

        Ok(serializer.serialize_str(&s).unwrap())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
use rand::Rng;
