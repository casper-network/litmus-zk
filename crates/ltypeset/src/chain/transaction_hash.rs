use crate::crypto::Digest;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Digest over a transaction's data structure.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub enum TransactionHash {
    /// A version 1 transaction hash.
    V1(TransactionV1Hash),

    /// A version 2 transaction hash.
    V2(TransactionV2Hash),
}

/// Digest over a transaction's data structure.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct TransactionV1Hash(Digest);

/// Digest over a transaction's data structure.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct TransactionV2Hash(Digest);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl TransactionV1Hash {
    pub fn new(digest: Digest) -> Self {
        Self(digest)
    }
}

impl TransactionV2Hash {
    pub fn new(digest: Digest) -> Self {
        Self(digest)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl TransactionV1Hash {
    pub fn inner(&self) -> &Digest {
        &self.0
    }
}

impl TransactionV2Hash {
    pub fn inner(&self) -> &Digest {
        &self.0
    }
}
