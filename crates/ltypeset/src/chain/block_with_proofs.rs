use super::{Block, BlockSignature};
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Block with associated set of proofs.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockWithProofs {
    block: Block,
    proofs: Vec<BlockSignature>,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl BlockWithProofs {
    pub fn new(block: Block, proofs: Vec<BlockSignature>) -> Self {
        // TODO: validate inputs.
        Self { block, proofs }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl BlockWithProofs {
    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn proofs(&self) -> &Vec<BlockSignature> {
        &self.proofs
    }
}
