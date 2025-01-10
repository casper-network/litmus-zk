use ltypeset::chain::{Block, BlockHash, BlockWithProofs, ChainNameDigest, EraConsensusInfo};

/// Verifies a version two block.
///
/// # Arguments
///
/// * `entity` - Block to be verified.
/// * `chain_name_digest` - Digest over name of a blockchain.
/// * `era_consensus_info` - Information pulled from a previous era necessary to tally finality signatures.
///
pub fn verify_block_v2_with_proofs(
    block_with_proofs: BlockWithProofs,
    chain_name_digest: ChainNameDigest,
    era_consensus_info: Option<EraConsensusInfo>,
) {
    // Destructure inner block.
    let block = match block_with_proofs.block() {
        Block::V2(inner) => inner,
        _ => panic!("Invalid block version."),
    };

    // BL-001: Verify that recomputed block hash is equal to actual block hash.
    assert_eq!(block.hash(), &BlockHash::from(block.get_bytes_for_digest()));

    // BL-002: Verify that switch block is not from a previous era.

    // BL-003: Verify that block signatory is an era signatory.

    // BL-004: Verify that each finality signature is valid.
    let msg = block.get_bytes_for_finality_signature(&chain_name_digest);
    for proof in block_with_proofs.proofs() {
        proof.signature().verify(proof.verification_key(), &msg);
    }

    // BL-005: Verify that weight of finality signatures is sufficient.
}
