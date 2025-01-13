use super::super::utils::{safe_split_at, CodecError, Decode, Encode};
use ltypeset::primitives::bites::{Bytes32, Bytes33, Bytes64, Bytes65};

// ------------------------------------------------------------------------
// Codec: [Byte; N].
// ------------------------------------------------------------------------

impl<const N: usize> Decode for [u8; N] {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, bstream) = safe_split_at(bstream, N)?;
        let ptr = bytes.as_ptr() as *const [u8; N];
        let result = unsafe { *ptr };
        Ok((result, bstream))
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn get_encoded_size(&self) -> usize {
        N
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: Bytes32.
// ------------------------------------------------------------------------

impl Decode for Bytes32 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::decode(bstream).map(|(arr, bstream)| (Self::new(arr), bstream))
    }
}

impl Encode for Bytes32 {
    fn get_encoded_size(&self) -> usize {
        Bytes32::len()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: Bytes33.
// ------------------------------------------------------------------------

impl Decode for Bytes33 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::decode(bstream).map(|(arr, bstream)| (Self::new(arr), bstream))
    }
}

impl Encode for Bytes33 {
    fn get_encoded_size(&self) -> usize {
        Bytes33::len()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: Bytes64.
// ------------------------------------------------------------------------

impl Decode for Bytes64 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::decode(bstream).map(|(arr, bstream)| (Self::new(arr), bstream))
    }
}

impl Encode for Bytes64 {
    fn get_encoded_size(&self) -> usize {
        Bytes64::len()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: Bytes65.
// ------------------------------------------------------------------------

impl Decode for Bytes65 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Decode::decode(bstream).map(|(arr, bstream)| (Self::new(arr), bstream))
    }
}

impl Encode for Bytes65 {
    fn get_encoded_size(&self) -> usize {
        Bytes65::len()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(self.as_slice());
        Ok(())
    }
}
