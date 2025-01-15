use crate::codecs::binary::Encode;
use ltypeset::chain::{Block, ChainNameDigest};

/// Returns a set of bytes for computing a block digest.
///
/// # Arguments
///
/// * `block` - Block over which a message will be computed for subsequent mapping to a digest.
///
pub fn get_digest_bytes_for_block(block: &Block) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    match block {
        Block::V1(_) => {
            unimplemented!()
        }
        Block::V2(inner) => {
            let header = inner.header();
            for encoded in [
                header.parent_hash().encode(),
                header.state_root_hash().encode(),
                header.body_hash().encode(),
                header.random_bit().encode(),
                header.accumulated_seed().encode(),
                header.era_end().encode(),
                header.timestamp().encode(),
                header.era_id().encode(),
                header.height().encode(),
                header.protocol_version().encode(),
                header.proposer().encode(),
                header.current_gas_price().encode(),
                header.last_switch_block_hash().encode(),
            ] {
                result.extend_from_slice(encoded.unwrap().as_slice());
            }
        }
    }

    result
}

// self.parent_hash.write_bytes(writer)?;
// self.state_root_hash.write_bytes(writer)?;
// self.body_hash.write_bytes(writer)?;
// self.random_bit.write_bytes(writer)?;
// self.accumulated_seed.write_bytes(writer)?;
// self.era_end.write_bytes(writer)?;
// self.timestamp.write_bytes(writer)?;
// self.era_id.write_bytes(writer)?;
// self.height.write_bytes(writer)?;
// self.protocol_version.write_bytes(writer)?;
// self.proposer.write_bytes(writer)?;
// self.current_gas_price.write_bytes(writer)?;
// self.last_switch_block_hash.write_bytes(writer)

/// Returns a set of bytes for computing a block finality signature.
///
/// # Arguments
///
/// * `block` - Block over which a message will be computed for subsequent mapping to a finality signature.
/// * `chain_name_digest` - Digest of chain name associated with network block production.
///
pub fn get_signature_bytes_for_block_finality(
    block: &Block,
    chain_name_digest: &ChainNameDigest,
) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for encoded in match block {
        Block::V1(_) => {
            unimplemented!()
        }
        Block::V2(inner) => [
            inner.hash().encode(),
            inner.header().height().encode(),
            inner.header().era_id().encode(),
            chain_name_digest.encode(),
        ],
    } {
        result.extend_from_slice(encoded.unwrap().as_slice());
    }

    result
}
