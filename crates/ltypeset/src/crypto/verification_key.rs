use crate::primitives::bites::{Bytes32, Bytes33};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const TAG_ED25519: u8 = 1;
const TAG_SECP256K1: u8 = 2;
const VKEY_SIZE_ED25519: usize = 32;
const VKEY_SIZE_RANGE: std::ops::Range<usize> = 33..35;
const VKEY_SIZE_SECP256K1: usize = 33;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// A wrapped verification key counterpart of an assymetric signing key.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerificationKey {
    ED25519(Bytes32),
    SECP256K1(Bytes33),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl VerificationKey {
    /// Constructor: returns an instance hydrated from a sequence of bytes.
    ///
    /// # Arguments
    ///
    /// * `raw_bytes` - A sequence of bytes.
    ///
    pub fn new(raw_bytes: &[u8]) -> Self {
        assert!(
            VKEY_SIZE_RANGE.contains(&raw_bytes.len()),
            "Invalid verification key length"
        );
        match raw_bytes[0] {
            TAG_ED25519 => {
                assert!(
                    raw_bytes.len() == VKEY_SIZE_ED25519 + 1,
                    "Invalid verification key length"
                );
                VerificationKey::ED25519(Bytes32::from(raw_bytes[1..33].to_vec()))
            }
            TAG_SECP256K1 => {
                assert!(
                    raw_bytes.len() == VKEY_SIZE_SECP256K1 + 1,
                    "Invalid verification key length"
                );
                VerificationKey::SECP256K1(Bytes33::from(raw_bytes[1..34].to_vec()))
            }
            _ => panic!("Unsupported signature key type prefix"),
        }
    }

    /// Constructor: returns a new ed25519 verification key.
    ///
    /// # Arguments
    ///
    /// * `sig` - Verification key issued by an ed25519 algorithm.
    ///
    pub fn new_ed25519(vk: Bytes32) -> Self {
        VerificationKey::ED25519(vk)
    }

    /// Constructor: returns a new secp256k1 verification key.
    ///
    /// # Arguments
    ///
    /// * `sig` - Verification key issued by an secp256k1 algorithm.
    ///
    pub fn new_secp256k1(vk: Bytes33) -> Self {
        VerificationKey::SECP256K1(vk)
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl VerificationKey {
    // Returns underlying byte array.
    pub fn as_slice(&self) -> &[u8] {
        match self {
            VerificationKey::ED25519(inner) => inner.as_slice(),
            VerificationKey::SECP256K1(inner) => inner.as_slice(),
        }
    }

    // Returns underlying byte array prefixed with algorithm type tag.
    pub fn as_slice_with_tag(&self) -> Vec<u8> {
        let mut f = Vec::from(self.as_slice());
        f.insert(0, self.get_tag());

        f
    }

    // Returns algorithm type tag.
    pub fn get_tag(&self) -> u8 {
        match self {
            VerificationKey::ED25519(_) => TAG_ED25519,
            VerificationKey::SECP256K1(_) => TAG_SECP256K1,
        }
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl fmt::Display for VerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationKey::ED25519(inner) => write!(f, "VKEY:ED25519:{}", inner),
            VerificationKey::SECP256K1(inner) => write!(f, "VKEY:SECP256K1:{}", inner),
        }
    }
}

impl From<&str> for VerificationKey {
    fn from(value: &str) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl From<&[u8]> for VerificationKey {
    fn from(value: &[u8]) -> Self {
        Self::new(&value)
    }
}

impl From<Vec<u8>> for VerificationKey {
    fn from(value: Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<&Vec<u8>> for VerificationKey {
    fn from(value: &Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for VerificationKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VerificationKeyVistor;

        impl<'de> Visitor<'de> for VerificationKeyVistor {
            type Value = VerificationKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("either a 64 char hex encoded string or a 32 byte array")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(VerificationKey::from(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(VerificationKey::from(v))
            }
        }

        deserializer.deserialize_any(VerificationKeyVistor)
    }
}

impl Serialize for VerificationKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let as_hex = hex::encode(self.as_slice_with_tag());

        Ok(serializer.serialize_str(&as_hex).unwrap())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const VKEY_ED25519_TAGGED_HEX: &str =
        "01764f83295812c03354e0cd64718a7e50b452696799dc9d6e446338d668f3b2d9";
    const VKEY_SECP256K1_TAGGED_HEX: &str =
        "0203eed4eb0b40b3131679c365e3a23780eabfeaeb01776b0f908223ad1d4bd06f0d";
    const VKEY_SET: [&str; 2] = [VKEY_ED25519_TAGGED_HEX, VKEY_SECP256K1_TAGGED_HEX];

    #[test]
    fn test_new_from_str() {
        for vkey in VKEY_SET {
            let _ = VerificationKey::from(vkey);
        }
    }

    #[test]
    fn test_new_from_vec() {
        for vkey in VKEY_SET {
            let as_vec = hex::decode(vkey).unwrap();
            let _ = VerificationKey::from(as_vec);
        }
    }

    #[test]
    fn test_accessor_get_tag() {
        for vkey in VKEY_SET {
            match vkey.as_bytes()[0] {
                TAG_ED25519 => assert_eq!(VerificationKey::from(vkey).get_tag(), TAG_ED25519),
                TAG_SECP256K1 => assert_eq!(VerificationKey::from(vkey).get_tag(), TAG_SECP256K1),
                _ => {}
            }
        }
    }

    #[test]
    fn test_accessor_as_slice() {
        for vkey in VKEY_SET {
            let vkey1 = VerificationKey::from(vkey).as_slice().to_owned();
            let vkey2 = VerificationKey::from(vkey).as_slice_with_tag();
            assert_eq!(vkey1.len() + 1, vkey2.len());
        }
    }

    #[test]
    #[should_panic]
    fn test_panic_if_tag_is_invalid() {
        for vkey in VKEY_SET {
            let vkey = format!("99{}", &vkey[..vkey.len() - 1]);
            let _ = VerificationKey::from(vkey.as_str());
        }
    }

    #[test]
    #[should_panic]
    fn test_panic_if_length_is_invalid_1() {
        for vkey in VKEY_SET {
            let vkey = &vkey[..vkey.len() - 1];
            let _ = VerificationKey::from(vkey);
        }
    }

    #[test]
    #[should_panic]
    fn test_panic_if_length_is_invalid_2() {
        for vkey in VKEY_SET {
            let vkey = format!("{}d9", vkey);
            let _ = VerificationKey::from(vkey.as_str());
        }
    }
}
