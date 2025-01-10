use super::super::utils::{CodecError, Decode, Encode};
use ltypeset::chain::{EraEndV2, EraId};
use std::{collections::BTreeMap, vec::Vec};

// ------------------------------------------------------------------------
// Codec: EraEndV2.
// ------------------------------------------------------------------------

impl Decode for EraEndV2 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (equivocators, bytes) = Vec::decode(bytes).unwrap();
        let (inactive_validators, bytes) = Vec::decode(bytes).unwrap();
        let (next_era_validator_weights, bytes) = BTreeMap::decode(bytes).unwrap();
        let (rewards, bytes) = BTreeMap::decode(bytes).unwrap();
        let (next_era_gas_price, bytes) = u8::decode(bytes).unwrap();
        Ok((
            EraEndV2::new(
                equivocators,
                inactive_validators,
                next_era_validator_weights,
                rewards,
                next_era_gas_price,
            ),
            bytes,
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

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.equivocators().write_bytes(writer).unwrap();
        self.inactive_validators().write_bytes(writer).unwrap();
        self.next_era_validator_weights()
            .write_bytes(writer)
            .unwrap();
        self.rewards().write_bytes(writer).unwrap();
        self.next_era_gas_price().write_bytes(writer).unwrap();
        Ok(())
    }
}

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
