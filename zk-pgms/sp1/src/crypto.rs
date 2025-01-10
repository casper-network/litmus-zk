use ltypeset::crypto::{Digest, Signature, VerificationKey};

/// Verifies a digest over a byte vector.
///
/// # Arguments
///
/// * `encoded_vkey` - A cbor encoded digest.
/// * `msg` - Message over which digest was claimed to have been computed.
///
pub fn verify_digest(encoded_vkey: Vec<u8>, msg: Vec<u8>) {
    let digest: Digest = serde_cbor::from_slice(&encoded_vkey).unwrap();

    digest.verify(msg);
}

/// Verifies a signature over a digest.
///
/// # Arguments
///
/// * `encoded_digest` - A cbor encoded digest.
/// * `encoded_sig` - A cbor encoded signature.
/// * `encoded_vkey` - A cbor encoded verification key.
///
pub fn verify_digest_signature(
    encoded_sig: Vec<u8>,
    encoded_vkey: Vec<u8>,
    encoded_digest: Vec<u8>,
) {
    let digest: Digest = serde_cbor::from_slice(&encoded_digest).unwrap();
    let sig: Signature = serde_cbor::from_slice(&encoded_sig).unwrap();
    let vkey: VerificationKey = serde_cbor::from_slice(&encoded_vkey).unwrap();

    sig.verify_digest(&vkey, &digest);
}
