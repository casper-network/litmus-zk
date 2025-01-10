extern crate alloc;

use crate::{
    chain::{
        BlockHash, BlockHeight, ChainNameDigest, EraEndV2, EraId, ProtocolVersion,
        TransactionV2Hash,
    },
    crypto::{Digest, PublicKey},
    primitives::time::Timestamp,
};
use alloc::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Block (v2).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct Block {
    /// Information pertaining to vm + consensus.
    header: BlockHeader,

    /// Digest over block body + header.
    hash: BlockHash,

    /// Block meta data.
    body: BlockBody,
}

// Block (v2) body.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockBody {
    /// List of identifiers for finality signatures for a particular past block.
    rewarded_signatures: Vec<Vec<u8>>,

    /// Map of transactions mapping categories to a list of transaction hashes.
    transactions: BTreeMap<u8, Vec<TransactionV2Hash>>,
}

// Block (v2) header.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockHeader {
    /// A seed needed for initializing a future era.
    accumulated_seed: Digest,

    /// Digest over block's body.
    body_hash: Digest,

    /// Era specific gas price.
    current_gas_price: u8,

    /// `EraEnd` report (if it is a switch block).
    era_end: Option<EraEndV2>,

    /// ID of era at point in chain time when block was created.
    era_id: EraId,

    /// Height of this block, i.e. the number of ancestors.
    height: BlockHeight,

    /// Most recent switch block hash.
    last_switch_block_hash: Option<BlockHash>,

    /// Parent block hash.
    parent_hash: BlockHash,

    /// Public key of validator granted protocol permission to propose block.
    proposer: PublicKey,

    /// Protocol version of network at point of block creation.
    protocol_version: ProtocolVersion,

    /// A random bit needed for initializing a future era.
    random_bit: bool,

    /// Root hash of global state after deploys in this block have been executed.
    state_root_hash: Digest,

    /// Timestamp at which block was created.
    timestamp: Timestamp,
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

impl BlockBody {
    pub fn new(
        rewarded_signatures: Vec<Vec<u8>>,
        transactions: BTreeMap<u8, Vec<TransactionV2Hash>>,
    ) -> Self {
        // TODO: validate inputs.
        Self {
            rewarded_signatures,
            transactions,
        }
    }
}

impl BlockHeader {
    pub fn new(
        accumulated_seed: Digest,
        body_hash: Digest,
        current_gas_price: u8,
        era_end: Option<EraEndV2>,
        era_id: EraId,
        height: BlockHeight,
        last_switch_block_hash: Option<BlockHash>,
        parent_hash: BlockHash,
        proposer: PublicKey,
        protocol_version: ProtocolVersion,
        random_bit: bool,
        state_root_hash: Digest,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            accumulated_seed,
            body_hash,
            current_gas_price,
            era_end,
            era_id,
            height,
            last_switch_block_hash,
            parent_hash,
            proposer,
            protocol_version,
            random_bit,
            state_root_hash,
            timestamp,
        }
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
        &self.header
    }
}

impl BlockBody {
    pub fn rewarded_signatures(&self) -> &Vec<Vec<u8>> {
        &self.rewarded_signatures
    }

    pub fn transactions(&self) -> &BTreeMap<u8, Vec<TransactionV2Hash>> {
        &self.transactions
    }
}

impl BlockHeader {
    pub fn accumulated_seed(&self) -> &Digest {
        &self.accumulated_seed
    }

    pub fn body_hash(&self) -> &Digest {
        &self.body_hash
    }

    pub fn current_gas_price(&self) -> &u8 {
        &self.current_gas_price
    }

    pub fn era_end(&self) -> &Option<EraEndV2> {
        &self.era_end
    }

    pub fn era_id(&self) -> &EraId {
        &self.era_id
    }

    pub fn height(&self) -> &BlockHeight {
        &self.height
    }

    pub fn last_switch_block_hash(&self) -> &Option<BlockHash> {
        &self.last_switch_block_hash
    }

    pub fn parent_hash(&self) -> &BlockHash {
        &self.parent_hash
    }

    pub fn proposer(&self) -> &PublicKey {
        &self.proposer
    }

    pub fn protocol_version(&self) -> &ProtocolVersion {
        &self.protocol_version
    }

    pub fn random_bit(&self) -> &bool {
        &self.random_bit
    }

    pub fn state_root_hash(&self) -> &Digest {
        &self.state_root_hash
    }

    pub fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Block {
    /// Returns a sequence of bytes for mapping to a block digest.
    pub fn get_bytes_for_digest(&self) -> Vec<u8> {
        self.header().get_bytes_for_digest()
    }

    /// Returns a sequence of bytes to be signed over when commiting to block finality.
    pub fn get_bytes_for_finality_signature(&self, chain_name_digest: &ChainNameDigest) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(self.hash().inner().as_slice());
        result.extend_from_slice(self.header().height().inner().to_le_bytes().as_slice());
        result.extend_from_slice(self.header().era_id().inner().to_le_bytes().as_slice());
        result.extend_from_slice(chain_name_digest.inner().as_slice());

        result
    }
}

impl BlockBody {
    /// Returns a sequence of bytes for mapping to a block digest.
    pub fn get_bytes_for_digest(&self) -> Vec<u8> {
        unimplemented!()
    }
}

impl BlockHeader {
    /// Returns a sequence of bytes for mapping to a block digest.
    pub fn get_bytes_for_digest(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(self.parent_hash().inner().as_slice());
        result.extend_from_slice(self.state_root_hash().as_slice());
        result.extend_from_slice(self.body_hash().as_slice());
        result.extend_from_slice(
            u8::from(self.random_bit().to_owned())
                .to_le_bytes()
                .as_slice(),
        );
        result.extend_from_slice(self.accumulated_seed().as_slice());
        // result.extend_from_slice(self.era_end().hash(state););
        result.extend_from_slice(self.timestamp().inner().to_le_bytes().as_slice());
        result.extend_from_slice(self.era_id().inner().to_le_bytes().as_slice());
        result.extend_from_slice(self.height().inner().to_le_bytes().as_slice());
        // result.extend_from_slice(self.protocol_version());
        result.extend_from_slice(self.proposer().as_slice());
        result.extend_from_slice(self.current_gas_price().to_le_bytes().as_slice());
        // result.extend_from_slice(self.last_switch_block_hash().unwrap().inner().as_slice());

        result
    }
}
