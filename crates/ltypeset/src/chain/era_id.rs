use serde::{Deserialize, Serialize};
use std::fmt;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// An era represents a set of consensus rounds.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct EraId(u64);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl EraId {
    /// Factory: new [`EraId`] instance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl EraId {
    // Maximum possible value an [`EraId`] can hold.
    pub const MAX: EraId = EraId(u64::MAX);

    // Minimum possible value an [`EraId`] can hold.
    pub const MIN: EraId = EraId(u64::MIN);

    // Wrapped value.
    pub fn inner(&self) -> u64 {
        self.0
    }

    /// Returns whether this is era 0.
    pub fn is_genesis(&self) -> bool {
        self.0 == 0
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl fmt::Display for EraId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERA-ID::{}", self.inner())
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
impl EraId {
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
    fn test_is_genesis() {
        assert_eq!(EraId::MIN.is_genesis(), true);
        assert_eq!(EraId::MAX.is_genesis(), false);
    }

    #[test]
    fn new_from_random() {
        for _ in 0..10 {
            EraId::new_from_random();
        }
    }

    proptest! {
        #[test]
        fn new_from_arb(_ in EraId::new_from_arb()) {
            ()
        }
    }
}
