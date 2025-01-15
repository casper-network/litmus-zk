use crate::crypto::Digest;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Digest over a chain's block data structure.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockHash(Digest);

/// Monotonically increasing chain height.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockHeight(u64);

/// Unique block identifier scoped by chain.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub enum BlockID {
    /// Digest over a chain's block data structure.
    BlockHash(BlockHash),

    /// Monotonically increasing chain height.
    BlockHeight(BlockHeight),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl BlockHash {
    /// Constructor: new [`BlockHash`] instance.
    pub fn new(digest: Digest) -> Self {
        Self(digest)
    }
}

impl BlockHeight {
    /// Constructor: new [`BlockHeight`] instance.
    pub const fn new(height: u64) -> Self {
        Self(height)
    }
}

impl BlockID {
    /// Constructor: new [`BlockID`] instance.
    pub fn new_from_hash(identifier: BlockHash) -> Self {
        Self::BlockHash(identifier)
    }

    /// Constructor: new [`BlockID`] instance.
    pub fn new_from_height(height: u64) -> Self {
        Self::BlockHeight(BlockHeight::new(height))
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl BlockHash {
    pub fn inner(&self) -> &Digest {
        &self.0
    }
}

impl BlockHeight {
    pub fn inner(&self) -> u64 {
        self.0
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl BlockHash {
    pub fn is_genesis(&self) -> bool {
        self.inner().is_zero()
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BLOCK-HASH:{}", self.inner())
    }
}

impl fmt::Display for BlockHeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl From<&str> for BlockHeight {
    fn from(value: &str) -> Self {
        Self::new(value.parse().unwrap())
    }
}

impl From<&str> for BlockHash {
    fn from(value: &str) -> Self {
        Self::new(Digest::from(value))
    }
}

impl From<&str> for BlockID {
    fn from(value: &str) -> Self {
        Self::BlockHash(BlockHash::from(value))
    }
}

impl From<&[u8]> for BlockHash {
    fn from(value: &[u8]) -> Self {
        Self::new(Digest::from(value))
    }
}

impl From<Vec<u8>> for BlockHash {
    fn from(value: Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<&Vec<u8>> for BlockHash {
    fn from(value: &Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<BlockHash> for BlockID {
    fn from(value: BlockHash) -> Self {
        Self::new_from_hash(value)
    }
}

impl From<BlockHeight> for BlockID {
    fn from(value: BlockHeight) -> Self {
        Self::new_from_height(value.inner())
    }
}

impl From<Digest> for BlockHash {
    fn from(value: Digest) -> Self {
        BlockHash::new(value)
    }
}

impl From<Digest> for BlockID {
    fn from(value: Digest) -> Self {
        Self::from(BlockHash::from(value))
    }
}

impl From<u64> for BlockHeight {
    fn from(value: u64) -> Self {
        BlockHeight::new(value)
    }
}

impl From<u64> for BlockID {
    fn from(value: u64) -> Self {
        Self::from(BlockHeight::from(value))
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    mod data {
        pub const BLOCK_HASH_HEX: &str =
            "b3f473058b6065c1d08a400dbc76054e491513a8b102c1709561730294c5a88e";
        pub const BLOCK_HEIGHT: u64 = u64::MAX;
    }

    mod utils {
        use super::super::*;
        use rand::Rng;

        impl BlockHash {
            pub fn new_from_random() -> Self {
                Self::new(Digest::new_from_random())
            }
        }

        impl BlockHeight {
            pub fn new_from_random() -> Self {
                Self::new(rand::thread_rng().gen())
            }
        }

        impl BlockID {
            pub fn new_from_random_hash() -> Self {
                Self::BlockHash(BlockHash::new_from_random())
            }

            pub fn new_from_random_height() -> Self {
                Self::BlockHeight(BlockHeight::new_from_random())
            }
        }
    }

    #[test]
    fn test_new_block_hash() {
        let _ = BlockHash::from(data::BLOCK_HASH_HEX);
        let _ = BlockHash::new_from_random();
    }

    #[test]
    fn test_new_block_height() {
        let _ = BlockHeight::from(data::BLOCK_HEIGHT);
        let _ = BlockHeight::new_from_random();
    }

    #[test]
    fn test_new_block_id() {
        let _ = BlockID::from(data::BLOCK_HASH_HEX);
        let _ = BlockID::new_from_random_hash();
        let _ = BlockID::from(data::BLOCK_HEIGHT);
        let _ = BlockID::new_from_random_height();
    }
}
