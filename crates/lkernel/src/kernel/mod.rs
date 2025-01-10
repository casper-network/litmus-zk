pub(super) mod config;

use super::fetcher::FetcherBackend;
pub use super::{cache::Cache, fetcher::Fetcher, prover::Prover};
use camino::Utf8Path;
use ltypeset::chain::{BlockHash, BlockID, BlockWithProofs, ChainNameDigest};
pub use {config::Config, config::FetcherConfig};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct Kernel {
    config: Config,
    cache: Cache,
    fetcher: Fetcher,
    prover: Prover,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Kernel {
    pub fn new(path_to_config_toml: &Utf8Path) -> Self {
        let config = Config::new(&path_to_config_toml);
        let cache = Cache::new(config.clone());
        let fetcher = Fetcher::new(config.clone());
        let prover = Prover::new(config.clone());

        Self {
            cache,
            config,
            fetcher,
            prover,
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Kernel {
    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn fetcher(&self) -> &Fetcher {
        &self.fetcher
    }

    pub fn prover(&self) -> &Prover {
        &self.prover
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Kernel {
    /// Initialises kernel components.
    pub fn init(&self) {
        self.fetcher.init().unwrap();
    }

    /// Returns block with associated proofs.
    pub fn get_block_with_proofs(&self, block_hash: Option<BlockHash>) -> Option<BlockWithProofs> {
        // If requested block hash is unspecified then set from config.
        let block_hash = match block_hash {
            Option::Some(inner) => inner,
            Option::None => self.config.trusted_block_hash,
        };

        self.fetcher
            .get_block_with_proofs(BlockID::from(block_hash))
    }

    /// Returns digest over associated chain name.
    pub fn get_chain_name_digest(&self) -> ChainNameDigest {
        self.config.get_chain_name_digest()
    }
}
