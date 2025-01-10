use crate::crypto::Digest;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Digest over a chain's name.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct ChainNameDigest(Digest);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl ChainNameDigest {
    pub fn new(digest: Digest) -> Self {
        Self(digest)
    }

    // Instantiates an instance based upon a network's name as defined in a chain specification.
    pub fn new_from_chain_name(name: &str) -> Self {
        let inner = Digest::get_blake2b(name.as_bytes().to_vec());

        Self::new(inner)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl ChainNameDigest {
    pub fn inner(&self) -> &Digest {
        &self.0
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl From<&str> for ChainNameDigest {
    fn from(value: &str) -> Self {
        Self::new_from_chain_name(value)
    }
}
