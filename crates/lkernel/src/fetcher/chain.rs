use super::FetcherBackend;
use ltypeset::chain::{BlockID, BlockWithProofs};
use std::io::Error;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Fetcher {
    ip_address_set: Vec<String>,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Fetcher {
    pub fn new(ip_address_set: Vec<String>) -> Self {
        Self { ip_address_set }
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl FetcherBackend for Fetcher {
    fn get_block_with_proofs(&self, _: BlockID) -> Option<BlockWithProofs> {
        unimplemented!()
    }

    fn init(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
