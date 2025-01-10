use super::kernel::config::{Config, FetcherConfig};
pub use chain::Fetcher as ChainFetcher;
pub use fsys::Fetcher as FileSystemFetcher;
use ltypeset::chain::{BlockID, BlockWithProofs};
use std::io::Error;

pub mod chain;
pub mod fsys;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub enum Fetcher {
    Chain(ChainFetcher),
    FileSystem(FileSystemFetcher),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Fetcher {
    pub fn new(config: Config) -> Self {
        match config.fetcher() {
            FetcherConfig::Chain { ip_address_set } => {
                Self::Chain(ChainFetcher::new(ip_address_set.to_owned()))
            }
            FetcherConfig::FileSystem { path_to_root } => {
                use camino::Utf8PathBuf;

                Self::FileSystem(FileSystemFetcher::new(
                    Utf8PathBuf::from(path_to_root).as_path(),
                ))
            }
        }
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

/// Set of backend functions that each block fetcher must implement.
pub trait FetcherBackend {
    /// Retrieves a block by an identifier.
    ///
    /// # Arguments
    ///
    /// * `block_id` - Identifier of a block for which to issue a query.
    ///
    fn get_block_with_proofs(&self, block_id: BlockID) -> Option<BlockWithProofs>;

    /// Fetcher initializer.
    fn init(&self) -> Result<(), Error>;
}

impl FetcherBackend for Fetcher {
    fn get_block_with_proofs(&self, block_id: BlockID) -> Option<BlockWithProofs> {
        match self {
            Self::Chain(inner) => inner.get_block_with_proofs(block_id),
            Self::FileSystem(inner) => inner.get_block_with_proofs(block_id),
        }
    }

    fn init(&self) -> Result<(), Error> {
        match self {
            Self::Chain(inner) => inner.init(),
            Self::FileSystem(inner) => inner.init(),
        }
    }
}
