use crate::binary::utils::{CodecError, Decode, Encode};
use ltypeset::{chain::ChainNameDigest, crypto::Digest};

// ------------------------------------------------------------------------
// Codec: ChainNameDigest.
// ------------------------------------------------------------------------

impl Decode for ChainNameDigest {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bstream) = Digest::decode(bstream).unwrap();

        Ok((ChainNameDigest::new(inner), bstream))
    }
}

impl Encode for ChainNameDigest {
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
