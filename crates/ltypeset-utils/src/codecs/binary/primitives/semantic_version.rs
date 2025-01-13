use super::super::constants;
use super::super::utils::{CodecError, Decode, Encode};
use ltypeset::primitives::semantic_version::SemanticVersion;

// ------------------------------------------------------------------------
// Codec: SemanticVersion.
// ------------------------------------------------------------------------

const ENCODED_SIZE: usize = constants::ENCODED_SIZE_U32 * 3;

impl Decode for SemanticVersion {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (major, bstream) = u32::decode(&bstream).unwrap();
        let (minor, bstream) = u32::decode(&bstream).unwrap();
        let (patch, bstream) = u32::decode(&bstream).unwrap();

        Ok((SemanticVersion::new(major, minor, patch), bstream))
    }
}

impl Encode for SemanticVersion {
    fn get_encoded_size(&self) -> usize {
        ENCODED_SIZE
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.major.write_encoded(writer).unwrap();
        self.minor.write_encoded(writer).unwrap();
        self.patch.write_encoded(writer).unwrap();
        Ok(())
    }
}
