use super::super::utils::{CodecError, Decode, Encode};
use ltypeset::primitives::time::Timestamp;

// ------------------------------------------------------------------------
// Codec: Digest.
// ------------------------------------------------------------------------

impl Decode for Timestamp {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bstream) = u128::decode(&bstream).unwrap();

        Ok((Self::new(inner), &bstream))
    }
}

impl Encode for Timestamp {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_encoded(writer).unwrap();
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
mod arbs {
    use super::*;

    #[cfg(test)]
    pub fn timestamp() -> impl Strategy<Value = Timestamp> {
        any::<u128>().prop_map(Timestamp::new)
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::binary::utils::assert_codec;

    proptest! {
        #[test]
        fn codec_timestamp(timestamp in arbs::timestamp()) {
            assert_codec(&timestamp);
        }
    }
}
