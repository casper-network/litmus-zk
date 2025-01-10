use crate::fixtures::types::{
    Fixtures, WrappedBlockV1WithProofs, WrappedBlockV2WithProofs, WrappedDigest, WrappedSignature,
};
use sp1_sdk::SP1Stdin;

const VERIFICATION_TYPE_DIGEST: u8 = 0;
const VERIFICATION_TYPE_SIGNATURE: u8 = 1;
const VERIFICATION_TYPE_BLOCK_V1_WITH_PROOFS: u8 = 10;
const VERIFICATION_TYPE_BLOCK_V2_WITH_PROOFS: u8 = 11;

impl From<Fixtures> for Vec<SP1Stdin> {
    fn from(value: Fixtures) -> Self {
        let mut result: Vec<SP1Stdin> = Vec::new();
        for item in value.set_of_digests {
            result.push(SP1Stdin::try_from(&item).unwrap());
        }
        for item in value.set_of_signatures {
            result.push(SP1Stdin::try_from(&item).unwrap());
        }

        result
    }
}

impl From<&WrappedDigest> for SP1Stdin {
    fn from(value: &WrappedDigest) -> Self {
        let mut vm_stdin = Self::new();
        vm_stdin.write(&VERIFICATION_TYPE_DIGEST);
        vm_stdin.write_vec(serde_cbor::to_vec(value.inner()).unwrap());
        vm_stdin.write_vec(value.msg());

        vm_stdin
    }
}

impl From<&WrappedSignature> for SP1Stdin {
    fn from(value: &WrappedSignature) -> Self {
        let mut vm_stdin = Self::new();
        vm_stdin.write(&VERIFICATION_TYPE_SIGNATURE);
        vm_stdin.write_vec(serde_cbor::to_vec(value.sig()).unwrap());
        vm_stdin.write_vec(serde_cbor::to_vec(value.vkey()).unwrap());
        vm_stdin.write_vec(serde_cbor::to_vec(value.msg()).unwrap());

        vm_stdin
    }
}

impl From<&WrappedBlockV1WithProofs> for SP1Stdin {
    fn from(value: &WrappedBlockV1WithProofs) -> Self {
        let mut vm_stdin = Self::new();
        vm_stdin.write(&VERIFICATION_TYPE_BLOCK_V1_WITH_PROOFS);
        vm_stdin.write_vec(serde_cbor::to_vec(value.inner()).unwrap());

        vm_stdin
    }
}

impl From<&WrappedBlockV2WithProofs> for SP1Stdin {
    fn from(value: &WrappedBlockV2WithProofs) -> Self {
        let mut vm_stdin = Self::new();
        vm_stdin.write(&VERIFICATION_TYPE_BLOCK_V2_WITH_PROOFS);
        vm_stdin.write_vec(serde_cbor::to_vec(value.inner()).unwrap());
        vm_stdin.write_vec(serde_cbor::to_vec(value.chain_name_digest()).unwrap());

        vm_stdin
    }
}
