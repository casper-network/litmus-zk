use super::constants;
use core::fmt::{Debug, Display};
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Codec error types.
#[derive(Copy, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[repr(u8)]
#[non_exhaustive]
pub enum CodecError {
    /// Early end of stream while deserializing.
    EarlyEndOfStream = 0,
    /// Formatting error while deserializing.
    Formatting,
    /// Not all input bytes were consumed in [`deserialize`].
    LeftOverBytes,
    /// Out of memory error.
    OutOfMemory,
    /// No serialized representation is available for a value.
    NotRepresentable,
    /// Exceeded a recursion depth limit.
    ExceededRecursionDepth,
}

/// Trait implemented by types decodeable from a `Vec<Byte>`.
pub trait Decode: Sized {
    /// Decodes slice into instance of `Self`.
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError>;

    /// Decodes `Vec<u8>` into instance of `Self`.
    fn from_vec(bstream: Vec<u8>) -> Result<(Self, Vec<u8>), CodecError> {
        Self::decode(bstream.as_slice()).map(|(x, remainder)| (x, Vec::from(remainder)))
    }
}

/// Trait implemented by types encodeable as a `Vec<Byte>`.
pub trait Encode {
    /// Encodes `&self` as a `Vec<Byte>`.
    fn encode(&self) -> Result<Vec<u8>, CodecError> {
        let encoded_length = self.get_encoded_size();
        if encoded_length > u32::MAX as usize {
            return Err(CodecError::OutOfMemory);
        }
        let mut result = Vec::<u8>::with_capacity(encoded_length);
        self.write_encoded(&mut result)?;
        Ok(result)
    }

    /// Consumes `self` and encodes accordingly.
    fn into_bytes(self) -> Result<Vec<u8>, CodecError>
    where
        Self: Sized,
    {
        self.encode()
    }

    /// Returns size of `Vec<u8>` returned from call to `encode()`.
    fn get_encoded_size(&self) -> usize;

    /// Writes `&self` into a mutable `writer`.
    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError>;
}

/// Returns a `Vec<Byte>` initialized with sufficient capacity to hold `to_be_serialized` after
/// serialization, or an error if the capacity would exceed `u32::MAX`.
pub(crate) fn allocate_buffer<T: Encode + Sized>(
    to_be_serialized: &T,
) -> Result<Vec<u8>, CodecError> {
    let serialized_length = to_be_serialized.get_encoded_size();
    if serialized_length > u32::MAX as usize {
        return Err(CodecError::OutOfMemory);
    }
    Ok(Vec::with_capacity(serialized_length))
}

/// Asserts codec roundtrip over an instance of an entity of type T.
#[cfg(test)]
pub(crate) fn assert_codec<T>(entity: &T)
where
    T: Decode + Encode + Debug + Display + PartialEq,
{
    let encoded = T::encode(&entity).unwrap();

    let size = T::get_encoded_size(&entity);
    assert_eq!(size, encoded.len(), "Size mismatch");

    let mut written_bytes = vec![];
    entity.write_encoded(&mut written_bytes).unwrap();
    assert_eq!(encoded, written_bytes);

    let (decoded, bstream) = T::decode(&encoded).unwrap();
    assert_eq!(entity, &decoded);
    assert_eq!(bstream.len(), 0);
}

/// Deconstructs a byte sequence into left & right sequences at a certain index.
pub(crate) fn deconstruct_bytes<const N: usize>(
    bytes: &[u8],
) -> Result<([u8; N], &[u8]), CodecError> {
    let mut result = [0u8; N];
    let (bytes, remainder) = safe_split_at(bytes, N)?;
    result.copy_from_slice(bytes);

    Ok((result, remainder))
}

/// Serializes a slice of bytes with a length prefix.
///
/// This function is serializing a slice of bytes with an addition of a 4 byte length prefix.
///
/// For safety you should prefer to use [`vec_u8_to_bytes`]. For efficiency reasons you should also
/// avoid using serializing Vec<u8>.
pub(crate) fn encode_byte_slice(bytes: &[u8]) -> Result<Vec<u8>, CodecError> {
    let serialized_length = get_encoded_size_of_byte_slice(bytes);
    let mut vec = Vec::with_capacity(serialized_length);
    let length_prefix: u32 = bytes
        .len()
        .try_into()
        .map_err(|_| CodecError::NotRepresentable)?;
    let length_prefix_bytes = length_prefix.to_le_bytes();
    vec.extend_from_slice(&length_prefix_bytes);
    vec.extend_from_slice(bytes);
    Ok(vec)
}

/// Returns serialized length of serialized slice of bytes.
///
/// This function adds a length prefix in the beginning.
pub(crate) fn get_encoded_size_of_byte_slice(bytes: &[u8]) -> usize {
    constants::ENCODED_SIZE_U32 + bytes.len()
}

pub(crate) fn get_encoded_size_of_byte_vec(vec: &Vec<u8>) -> usize {
    get_encoded_size_of_byte_slice(vec.as_slice())
}

/// Safely splits slice at given point.
pub(crate) fn safe_split_at(bytes: &[u8], n: usize) -> Result<(&[u8], &[u8]), CodecError> {
    if n > bytes.len() {
        Err(CodecError::EarlyEndOfStream)
    } else {
        Ok(bytes.split_at(n))
    }
}

/// Returns a `Vec<u8>` initialized with sufficient capacity to hold `to_be_serialized` after
/// serialization.
pub(crate) fn unchecked_allocate_buffer<T: Encode>(to_be_serialized: &T) -> Vec<u8> {
    let serialized_length = to_be_serialized.get_encoded_size();
    Vec::with_capacity(serialized_length)
}

pub(crate) fn write_byte_slice(bytes: &[u8], writer: &mut Vec<u8>) -> Result<(), CodecError> {
    let length_32: u32 = bytes
        .len()
        .try_into()
        .map_err(|_| CodecError::NotRepresentable)?;
    writer.extend_from_slice(&length_32.to_le_bytes());
    writer.extend_from_slice(bytes);
    Ok(())
}
