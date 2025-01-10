use crate::binary::utils::{CodecError, Decode, Encode};
use ltypeset::chain::ValidatorWeight;

// ------------------------------------------------------------------------
// Codec: Motes.
// ------------------------------------------------------------------------

impl Decode for ValidatorWeight {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!()
    }
}

impl Encode for ValidatorWeight {
    fn get_encoded_size(&self) -> usize {
        unimplemented!()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!()
    }
}
