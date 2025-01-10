use super::super::constants;
use super::super::utils::{deconstruct_bytes, CodecError, Decode, Encode};

// ------------------------------------------------------------------------
// Codec: i32.
// ------------------------------------------------------------------------

impl Decode for i32 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, bstream) = deconstruct_bytes::<4>(&bstream).unwrap();

        Ok((<i32>::from_le_bytes(bytes), bstream))
    }
}

impl Encode for i32 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_I32
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: i64.
// ------------------------------------------------------------------------

impl Decode for i64 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, bstream) = deconstruct_bytes::<8>(&bstream).unwrap();

        Ok((<i64>::from_le_bytes(bytes), bstream))
    }
}

impl Encode for i64 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_I64
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: u8.
// ------------------------------------------------------------------------

impl Decode for u8 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        match bstream.split_first() {
            None => Err(CodecError::EarlyEndOfStream),
            Some((byte, bstream)) => Ok((*byte, bstream)),
        }
    }
}

impl Encode for u8 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U8
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.push(*self);
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: u16.
// ------------------------------------------------------------------------

impl Decode for u16 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, bstream) = deconstruct_bytes::<2>(&bstream).unwrap();

        Ok((<u16>::from_le_bytes(bytes), bstream))
    }
}

impl Encode for u16 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U16
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: u32.
// ------------------------------------------------------------------------

impl Decode for u32 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, bstream) = deconstruct_bytes::<4>(&bstream).unwrap();

        Ok((<u32>::from_le_bytes(bytes), bstream))
    }
}

impl Encode for u32 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U32
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: u64.
// ------------------------------------------------------------------------

impl Decode for u64 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, bstream) = deconstruct_bytes::<8>(&bstream).unwrap();

        Ok((<u64>::from_le_bytes(bytes), bstream))
    }
}

impl Encode for u64 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U64
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: u128.
// ------------------------------------------------------------------------

impl Decode for u128 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (bytes, bstream) = deconstruct_bytes::<16>(&bstream).unwrap();

        Ok((<u128>::from_le_bytes(bytes), bstream))
    }
}

impl Encode for u128 {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U128
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod proptests {
    use crate::binary::utils::assert_codec;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_i32(u in any::<i32>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_i64(u in any::<i64>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u8(u in any::<u8>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u16(u in any::<u16>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u32(u in any::<u32>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u64(u in any::<u64>()) {
            assert_codec(&u);
        }

        #[test]
        fn test_u128(u in any::<u128>()) {
            assert_codec(&u);
        }
    }
}
