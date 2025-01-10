use ltypeset::{
    chain::{BlockHash, BlockWithProofs, ChainNameDigest},
    crypto::{Digest, Signature, VerificationKey},
};
use serde::{Deserialize, Serialize};

// Digest information required for verification.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DigestFixture {
    // Hashing algorithm used to compute digest.
    pub algo: String,

    // Hex representation of data over which digest was computed.
    pub msg: String,

    // Computed digest.
    #[serde(with = "hex::serde")]
    pub digest: Vec<u8>,
}

// Signature information.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignatureFixture {
    // ECC algorithm used to compute digest.
    pub algo: String,

    // Hex representation of data over which signature was computed.
    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,

    // Binary representation of ECC verification key.
    #[serde(with = "hex::serde")]
    pub pbk: Vec<u8>,

    // Binary representation of ECC signing key.
    #[serde(with = "hex::serde")]
    pub pvk: Vec<u8>,

    // Binary representation of computed signature.
    #[serde(with = "hex::serde")]
    pub sig: Vec<u8>,

    // Binary representation of verification key with algo type prefix.
    #[serde(with = "hex::serde")]
    pub tagged_pbk: Vec<u8>,

    // Binary representation of computed signature with algo type prefix.
    #[serde(with = "hex::serde")]
    pub tagged_sig: Vec<u8>,
}

// Fixtures for use in cryptography tests.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CryptoFixtures {
    pub digests: Vec<DigestFixture>,
    pub signatures: Vec<SignatureFixture>,
}

// V1 block with associated proof set.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedBlockV1WithProofs(pub BlockWithProofs);

impl WrappedBlockV1WithProofs {
    // Block and set of associated finality signatures.
    pub(crate) fn inner(&self) -> &BlockWithProofs {
        &self.0
    }
}

// Wrapped V2 block with associated proof set.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedBlockV2WithProofs(pub BlockWithProofs, pub ChainNameDigest);

impl WrappedBlockV2WithProofs {
    // Name of chain associated with block.
    pub(crate) fn chain_name_digest(&self) -> &ChainNameDigest {
        &self.1
    }

    // Block and set of associated finality signatures.
    pub(crate) fn inner(&self) -> &BlockWithProofs {
        &self.0
    }
}

// Wrapped cryptographic digest.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedDigest(pub Digest, pub Vec<u8>);

impl WrappedDigest {
    // Computed digest.
    pub(crate) fn inner(&self) -> &Digest {
        &self.0
    }

    // Data over which digest was computed.
    pub(crate) fn msg(&self) -> Vec<u8> {
        self.1.to_owned()
    }
}

// Wrapped cryptographic signature.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WrappedSignature(pub Signature, pub VerificationKey, pub Digest);

impl WrappedSignature {
    // Data over which signature was computed.
    pub(crate) fn msg(&self) -> &Digest {
        &self.2
    }

    // Computed signature.
    pub(crate) fn sig(&self) -> &Signature {
        &self.0
    }

    // Signature verification key.
    pub(crate) fn vkey(&self) -> &VerificationKey {
        &self.1
    }
}

// Fixtures for tests.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fixtures {
    pub set_of_blocks_with_proofs: Vec<WrappedBlockV2WithProofs>,
    pub set_of_digests: Vec<WrappedDigest>,
    pub set_of_signatures: Vec<WrappedSignature>,
    pub trusted_block_hash: BlockHash,
}
