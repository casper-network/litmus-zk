use super::BlockHash;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Block (v1).
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct Block {
    /// Information pertaining to vm + consensus.
    body: BlockBody,

    /// Digest over block body + header.
    hash: BlockHash,

    /// Block meta data.
    header: BlockHeader,
}

// Block (v1) body.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockBody {}

// Block (v1) body header.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockHeader {
    /// The parent block's hash.
    parent_hash: BlockHash,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Block {
    pub fn new(body: BlockBody, hash: BlockHash, header: BlockHeader) -> Self {
        // TODO: validate inputs.
        Self { body, hash, header }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Block {
    pub fn body(&self) -> &BlockBody {
        unimplemented!()
    }
    pub fn hash(&self) -> &BlockHash {
        &self.hash
    }
    pub fn header(&self) -> &BlockHeader {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Block {
    /// Returns a sequence of bytes for mapping to a block digest.
    pub fn get_bytes_for_hash(&self) -> Vec<u8> {
        unimplemented!("v1 block get_bytes_for_hash");
    }

    /// Returns a sequence of bytes to be signed over when commiting to block finality.
    pub fn get_bytes_for_finality_signature(&self) -> Vec<u8> {
        unimplemented!("v1 block get_bytes_for_finality_signature");
    }
}
