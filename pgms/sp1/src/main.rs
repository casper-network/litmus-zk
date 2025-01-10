#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg(target_os = "zkvm")]
sp1_zkvm::entrypoint!(main);

mod chain;
mod crypto;

// Supported verification types.
const VERIFICATION_TYPE_DIGEST: u8 = 0;
const VERIFICATION_TYPE_SIGNATURE: u8 = 1;
const VERIFICATION_TYPE_BLOCK_V1_WITH_PROOFS: u8 = 10;
const VERIFICATION_TYPE_BLOCK_V2_WITH_PROOFS: u8 = 11;

/// Program entry point - wrapped by sp1 for execution within zk-vm.
///
/// N.B. Arguments are parsed from SP1 ZK-VM i/o buffer:
pub fn main() {
    let verification_type_tag = sp1_zkvm::io::read::<u8>();
    match verification_type_tag {
        VERIFICATION_TYPE_BLOCK_V1_WITH_PROOFS => {
            chain::verify_block_v1_with_proofs(sp1_zkvm::io::read_vec())
        }
        VERIFICATION_TYPE_BLOCK_V2_WITH_PROOFS => {
            chain::verify_block_v2_with_proofs(sp1_zkvm::io::read_vec(), sp1_zkvm::io::read_vec())
        }
        VERIFICATION_TYPE_DIGEST => {
            crypto::verify_digest(sp1_zkvm::io::read_vec(), sp1_zkvm::io::read_vec())
        }
        VERIFICATION_TYPE_DIGEST_SIGNATURE => {
            crypto::verify_digest_signature(
                sp1_zkvm::io::read_vec(),
                sp1_zkvm::io::read_vec(),
                sp1_zkvm::io::read_vec(),
            );
        }
        _ => {
            panic!("Unsupported verification type")
        }
    }
}
