use lcodecs::binary::Encode;
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
            for encoded in [
                inner.header().parent_hash().encode(),
                inner.header().state_root_hash().encode(),
                inner.header().body_hash().encode(),
                inner.header().random_bit().encode(),
                inner.header().accumulated_seed().encode(),
                // inner.header().era_end().encode(),
                inner.header().timestamp().encode(),
                inner.header().era_id().encode(),
                inner.header().height().encode(),
                // inner.header().protocol_version().encode(),
                inner.header().proposer().encode(),
                inner.header().current_gas_price().encode(),
                // inner.header().last_switch_block_hash().encode(),
            ] {
                result.extend_from_slice(encoded.unwrap().as_slice());
            }
        }
    }

    result
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
