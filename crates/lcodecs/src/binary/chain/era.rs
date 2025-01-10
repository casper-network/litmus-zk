use super::super::utils::{CodecError, Decode, Encode};
use ltypeset::chain::{EraEndV2, EraId};
use std::{collections::BTreeMap, vec::Vec};

// ------------------------------------------------------------------------
// Codec: EraEndV2.
// ------------------------------------------------------------------------

impl Decode for EraEndV2 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (equivocators, bstream) = Vec::decode(bstream).unwrap();
        let (inactive_validators, bstream) = Vec::decode(bstream).unwrap();
        let (next_era_validator_weights, bstream) = Vec::decode(bstream).unwrap();
        let (rewards, bstream) = BTreeMap::decode(bstream).unwrap();
        let (next_era_gas_price, bstream) = u8::decode(bstream).unwrap();
        Ok((
            EraEndV2::new(
                equivocators,
                inactive_validators,
                next_era_validator_weights,
                rewards,
                next_era_gas_price,
            ),
            bstream,
        ))
    }
}

impl Encode for EraEndV2 {
    fn get_encoded_size(&self) -> usize {
        self.equivocators().get_encoded_size()
            + self.inactive_validators().get_encoded_size()
            + self.next_era_validator_weights().get_encoded_size()
            + self.rewards().get_encoded_size()
            + self.next_era_gas_price().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.equivocators().write_encoded(writer).unwrap();
        self.inactive_validators().write_encoded(writer).unwrap();
        self.next_era_validator_weights()
            .write_encoded(writer)
            .unwrap();
        self.rewards().write_encoded(writer).unwrap();
        self.next_era_gas_price().write_encoded(writer).unwrap();
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: EraId.
// ------------------------------------------------------------------------

impl Decode for EraId {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bstream) = u64::decode(&bstream).unwrap();

        Ok((Self::new(inner), &bstream))
    }
}

impl Encode for EraId {
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
