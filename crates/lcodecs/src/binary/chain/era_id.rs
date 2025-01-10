use crate::binary::utils::{CodecError, Decode, Encode};
use ltypeset::chain::EraId;

// ------------------------------------------------------------------------
// Codec: EraId.
// ------------------------------------------------------------------------

impl Decode for EraId {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bytes) = u64::decode(&bytes).unwrap();

        Ok((Self::new(inner), &bytes))
    }
}

impl Encode for EraId {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_bytes(writer).unwrap();
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
    pub(super) fn era_id() -> impl Strategy<Value = EraId> {
        any::<u64>().prop_map(EraId::new)
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::binary::utils::assert_codec;

    proptest! {
        #[test]
        fn codec_era_id(era_id in arbs::era_id()) {
            assert_codec(&era_id);
        }
    }
}
