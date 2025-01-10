use crate::binary::utils::{CodecError, Decode, Encode};
use ltypeset::chain::Motes;

// ------------------------------------------------------------------------
// Codec: Motes.
// ------------------------------------------------------------------------

impl Decode for Motes {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bstream) = u64::decode(&bstream).unwrap();

        Ok((Self::new(inner), &bstream))
    }
}

impl Encode for Motes {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_encoded(writer).unwrap();
        Ok(())
    }
}
