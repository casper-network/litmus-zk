use ltypeset::chain::{BlockWithProofs, ChainNameDigest};
use lverifiers;

pub fn verify_block_v1_with_proofs(encoded_block_with_proofs: Vec<u8>) {
    unimplemented!("verify_block_v1_with_proofs");
}

pub fn verify_block_v2_with_proofs(
    encoded_block_with_proofs: Vec<u8>,
    encoded_chain_name_digest: Vec<u8>,
) {
    let block_with_proofs: BlockWithProofs =
        serde_cbor::from_slice(&encoded_block_with_proofs).unwrap();
    let chain_name_digest: ChainNameDigest =
        serde_cbor::from_slice(&encoded_chain_name_digest).unwrap();

    lverifiers::verify_block_v2_with_proofs(block_with_proofs, chain_name_digest, None);
}
