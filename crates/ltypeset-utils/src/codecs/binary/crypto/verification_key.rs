use super::super::{
    constants,
    utils::{CodecError, Decode, Encode},
};
use ltypeset::{
    crypto::VerificationKey,
    primitives::bites::{Bytes32, Bytes33},
};

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const TAG_ED25519: u8 = 1;
const TAG_SECP256K1: u8 = 2;

// ------------------------------------------------------------------------
// Codec: SemanticVersion.
// ------------------------------------------------------------------------

impl Decode for VerificationKey {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (vkey_tag, bstream) = u8::decode(bstream).unwrap();
        let (vkey, bstream) = match vkey_tag {
            TAG_ED25519 => {
                let (vk, bstream) = Bytes32::decode(bstream).unwrap();
                (VerificationKey::new_ed25519(vk), bstream)
            }
            TAG_SECP256K1 => {
                let (vk, bstream) = Bytes33::decode(bstream).unwrap();
                (VerificationKey::new_secp256k1(vk), bstream)
            }
            _ => panic!("Invalid verification type tag"),
        };

        Ok((vkey, bstream))
    }
}

impl Encode for VerificationKey {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U8
            + match self {
                VerificationKey::ED25519(_) => Bytes32::len(),
                VerificationKey::SECP256K1(_) => Bytes33::len(),
            }
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        match self {
            VerificationKey::ED25519(inner) => {
                writer.push(TAG_ED25519);
                inner.write_encoded(writer).unwrap();
            }
            VerificationKey::SECP256K1(inner) => {
                writer.push(TAG_SECP256K1);
                inner.write_encoded(writer).unwrap();
            }
        }

        Ok(())
    }
}
