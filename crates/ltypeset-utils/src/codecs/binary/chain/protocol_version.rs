use super::super::utils::{CodecError, Decode, Encode};
use ltypeset::{chain::ProtocolVersion, primitives::SemanticVersion};

// ------------------------------------------------------------------------
// Codec: ProtocolVersion.
// ------------------------------------------------------------------------

impl Decode for ProtocolVersion {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bstream) = SemanticVersion::decode(&bstream).unwrap();

        Ok((Self::new(inner), &bstream))
    }
}

impl Encode for ProtocolVersion {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_encoded(writer).unwrap();
        Ok(())
    }
}
