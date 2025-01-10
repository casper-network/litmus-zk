/// Returns a blake2b digest over passed data.
///
/// # Arguments
///
/// * `data` - Data against which to generate a blake2b digest.
///
pub fn get_hash_blake2b(data: Vec<u8>) -> [u8; 32] {
    use blake2::{
        digest::{Update, VariableOutput},
        Blake2bVar,
    };

    let mut hasher = Blake2bVar::new(32).unwrap();
    hasher.update(&data);
    let mut buffer = [0_u8; 32];
    hasher.finalize_variable(&mut buffer).unwrap();

    buffer
}

/// Verifies ed25519 signature against arbitrary data.
///
/// # Arguments
///
/// * `sig` - Signature over data to be verified.
/// * `vkey` - Verification key counterpart to signing key.
/// * `msg` - Data over which signature was issued.
///
pub fn verify_signature_ed25519(sig: &[u8; 64], vkey: &[u8; 32], msg: &[u8]) {
    use ed25519_consensus::{Signature, VerificationKey};

    let sig = Signature::try_from(sig.as_slice()).unwrap();
    let vkey = VerificationKey::try_from(vkey.as_slice()).unwrap();
    match vkey.verify(&sig, &msg) {
        Result::Ok(_) => {}
        Result::Err(_) => panic!("ED25519 signature verification failure"),
    }
}

/// Verifies secp256k1 signature against arbitrary data.
///
/// # Arguments
///
/// * `sig` - Signature over data to be verified.
/// * `vkey` - Verification key counterpart to signing key.
/// * `msg` - Data over which signature was issued.
///
pub fn verify_signature_secp256k1(sig: &[u8; 64], vkey: &[u8; 33], msg: &[u8]) {
    use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

    let msg = Message::from_digest_slice(msg).unwrap();
    let pbk = PublicKey::from_slice(vkey.as_slice()).unwrap();
    let sig = Signature::from_compact(&sig.as_slice()).unwrap();
    match Secp256k1::new().verify_ecdsa(&msg, &sig, &pbk) {
        Result::Ok(_) => {}
        Result::Err(_) => panic!("SECP256K1 signature verification failure"),
    }
}
