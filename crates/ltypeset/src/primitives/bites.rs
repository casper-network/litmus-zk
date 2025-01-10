use serde::{Deserialize, Serialize};
use std::fmt;

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const SIZE_32: usize = 32;
const SIZE_33: usize = 33;
const SIZE_64: usize = 64;
const SIZE_65: usize = 65;

const ZERO_ARRAY_32: [u8; SIZE_32] = [0_u8; SIZE_32];
const ZERO_ARRAY_33: [u8; SIZE_33] = [0_u8; SIZE_33];
const ZERO_ARRAY_64: [u8; SIZE_64] = [0_u8; SIZE_64];
const ZERO_ARRAY_65: [u8; SIZE_65] = [0_u8; SIZE_65];

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Byte array with fixed size of N.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Bytes<const N: usize> {
    #[serde(with = "serde_bytes")]
    data: [u8; N],
}

// Byte array of size 32 - typically a digest | public key.
pub type Bytes32 = Bytes<SIZE_32>;

// Byte array of size 33 - typically a public key.
pub type Bytes33 = Bytes<SIZE_33>;

// Byte array of size 64 - typically a signature.
pub type Bytes64 = Bytes<SIZE_64>;

// Byte array of size 65 - typically a signature prefixed by it's algo type.
pub type Bytes65 = Bytes<SIZE_65>;

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl<const N: usize> Bytes<N> {
    pub fn new(data: [u8; N]) -> Self {
        Self { data }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl<const N: usize> Bytes<N> {
    // Returns reference to underlying byte array.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    // Returns reference to underlying byte array mutably.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    // Returns underlying byte array.
    pub fn inner(&self) -> [u8; N] {
        self.data
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl<const N: usize> Bytes<N> {
    // Predicate: is inner array all zeros ?
    pub fn is_zero(&self) -> bool {
        match N {
            SIZE_32 => self.as_slice() == ZERO_ARRAY_32,
            SIZE_33 => self.as_slice() == ZERO_ARRAY_33,
            SIZE_64 => self.as_slice() == ZERO_ARRAY_64,
            SIZE_65 => self.as_slice() == ZERO_ARRAY_65,
            _ => panic!("Unsupported byte array length"),
        }
    }

    // Returns length of underlying byte array.
    pub fn len() -> usize {
        N
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl<const N: usize> Default for Bytes<N> {
    fn default() -> Self {
        Self::new([0_u8; N])
    }
}

impl<const N: usize> fmt::Display for Bytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}..{})",
            hex::encode(&self.data[0..2]),
            hex::encode(&self.data[&self.data.len() - 2..])
        )
    }
}

impl<const N: usize> AsRef<Bytes<N>> for Bytes<N> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<const N: usize> From<&[u8]> for Bytes<N> {
    fn from(value: &[u8]) -> Self {
        Self::new(value[0..N].try_into().unwrap())
    }
}

impl<const N: usize> From<Vec<u8>> for Bytes<N> {
    fn from(value: Vec<u8>) -> Self {
        Bytes::from(&value)
    }
}

impl<const N: usize> From<&Vec<u8>> for Bytes<N> {
    fn from(value: &Vec<u8>) -> Self {
        Bytes::from(value.as_slice())
    }
}

impl<const N: usize> From<&str> for Bytes<N> {
    fn from(value: &str) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl<const N: usize> From<String> for Bytes<N> {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl<const N: usize> From<&String> for Bytes<N> {
    fn from(value: &String) -> Self {
        Self::from(value.to_owned())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
use rand::Rng;

#[cfg(test)]
pub fn new_from_random<const N: usize>() -> Bytes<N> {
    let mut rng = rand::thread_rng();
    let mut buffer = vec![0u8; N];
    rng.fill(&mut buffer[..]);

    Bytes::<N>::from(buffer)
}

#[cfg(test)]
impl Bytes32 {
    pub fn new_from_arb() -> impl Strategy<Value = Self> {
        any::<[u8; SIZE_32]>().prop_map(Self::new)
    }

    fn new_from_random() -> Bytes32 {
        new_from_random::<SIZE_32>()
    }
}

#[cfg(test)]
impl Bytes33 {
    pub fn new_from_arb() -> impl Strategy<Value = Self> {
        any::<[u8; SIZE_33]>().prop_map(Self::new)
    }

    fn new_from_random() -> Bytes33 {
        new_from_random::<SIZE_33>()
    }
}

#[cfg(test)]
impl Bytes64 {
    pub fn new_from_arb() -> impl Strategy<Value = Self> {
        any::<[u8; SIZE_64]>().prop_map(Self::new)
    }

    fn new_from_random() -> Bytes64 {
        new_from_random::<SIZE_64>()
    }
}

#[cfg(test)]
impl Bytes65 {
    pub fn new_from_arb() -> impl Strategy<Value = Self> {
        any::<[u8; SIZE_65]>().prop_map(Self::new)
    }

    fn new_from_random() -> Bytes65 {
        new_from_random::<SIZE_65>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_random_32() {
        for _ in 0..10 {
            let _ = Bytes32::new_from_random();
        }
    }

    #[test]
    fn new_from_random_33() {
        for _ in 0..10 {
            let _ = Bytes33::new_from_random();
        }
    }

    #[test]
    fn new_from_random_64() {
        for _ in 0..10 {
            let _ = Bytes64::new_from_random();
        }
    }

    #[test]
    fn new_from_random_65() {
        for _ in 0..10 {
            let _ = Bytes65::new_from_random();
        }
    }

    proptest! {
        #[test]
        fn new_from_arb_32(_ in Bytes32::new_from_arb()) {
            ()
        }

        #[test]
        fn new_from_arb_33(_ in Bytes33::new_from_arb()) {
            ()
        }

        #[test]
        fn new_from_arb_64(_ in Bytes64::new_from_arb()) {
            ()
        }

        #[test]
        fn new_from_arb_65(_ in Bytes65::new_from_arb()) {
            ()
        }
    }
}
