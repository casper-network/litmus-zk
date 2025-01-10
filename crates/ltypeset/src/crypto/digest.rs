use crate::primitives::bites::Bytes32;
use lcrypto;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Digest scoped by hashing algo type.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Digest {
    BLAKE2B(Bytes32),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Digest {
    /// Constructor: returns an instance hydrated from a sequence of bytes.
    ///
    /// # Arguments
    ///
    /// * `raw_bytes` - A sequence of bytes.
    ///
    pub fn new(bytes_32: Bytes32) -> Self {
        // N.B. Defaults to BLAKE2B, problematic if other hashing algos are introduced.
        Self::BLAKE2B(bytes_32)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Digest {
    // Returns underlying byte array.
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Digest::BLAKE2B(inner) => inner.as_slice(),
        }
    }

    // Returns inner byte array.
    pub fn inner(&self) -> &Bytes32 {
        match self {
            Digest::BLAKE2B(inner) => inner,
        }
    }

    // Returns length of underlying byte array.
    pub fn len(&self) -> usize {
        match self {
            Digest::BLAKE2B(_) => Bytes32::len(),
        }
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Digest {
    /// Returns a blake2b digest over passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data against which to generate a blake2b digest.
    ///
    pub fn get_blake2b(data: Vec<u8>) -> Self {
        Self::BLAKE2B(Bytes32::new(lcrypto::get_hash_blake2b(data)))
    }

    // Predicate: is inner array all zeros ?
    pub fn is_zero(&self) -> bool {
        self.inner().is_zero()
    }

    /// Verifies digest against passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data against which to verify digest.
    ///
    pub fn verify(&self, data: Vec<u8>) {
        match self {
            Digest::BLAKE2B(_) => {
                assert_eq!(self, &Digest::get_blake2b(data));
            }
        }
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Digest::BLAKE2B(inner) => write!(f, "BLAKE2B:{}", inner),
        }
    }
}

impl From<&str> for Digest {
    fn from(value: &str) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl From<&[u8]> for Digest {
    fn from(value: &[u8]) -> Self {
        Self::new(Bytes32::from(value))
    }
}

impl From<Vec<u8>> for Digest {
    fn from(value: Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<&Vec<u8>> for Digest {
    fn from(value: &Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DigestVistor;

        impl<'de> Visitor<'de> for DigestVistor {
            type Value = Digest;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("supported formats: 64 char hex encoded string | 32 byte array")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Problematic if another hashing algo is introduced.
                Ok(Digest::from(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Problematic if another hashing algo is introduced.
                Ok(Digest::from(v))
            }
        }

        deserializer.deserialize_any(DigestVistor)
    }
}

impl Serialize for Digest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Problematic if another hashing algo is introduced.
        let as_hex = hex::encode(&self.as_slice());

        Ok(serializer.serialize_str(&as_hex).unwrap())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
use rand::Rng;

#[cfg(test)]
impl Digest {
    /// Returns a random `Digest`.
    #[cfg(any(feature = "testing", test))]
    pub fn random() -> Self {
        let g: [u8; 32] = rand::thread_rng().gen();

        Self::from(g.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    const MSG: &[u8] = "أبو يوسف يعقوب بن إسحاق الصبّاح الكندي‎".as_bytes();
    const MSG_DIGEST_BLAKE2B_HEX: &str =
        "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";

    #[test]
    fn test_new_from_str() {
        let _ = Digest::from(MSG_DIGEST_BLAKE2B_HEX);
    }

    #[test]
    fn test_new_from_vec() {
        let as_vec = hex::decode(MSG_DIGEST_BLAKE2B_HEX).unwrap();
        let _ = Digest::from(as_vec);
    }

    #[test]
    fn test_accessor_as_slice() {
        let _ = Digest::from(MSG_DIGEST_BLAKE2B_HEX).as_slice();
    }

    #[test]
    fn test_verify() {
        let digest = Digest::from(MSG_DIGEST_BLAKE2B_HEX);

        assert_eq!(digest.verify(MSG.to_vec()), ());
    }

    #[test]
    #[should_panic]
    fn test_panic_on_verification_failure() {
        let digest = Digest::random();

        digest.verify(MSG.to_vec());
    }
}
