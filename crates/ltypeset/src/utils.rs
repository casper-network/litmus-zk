use serde::{Deserialize, Serialize};

/// Codec error types.
#[derive(Copy, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[repr(u8)]
#[non_exhaustive]
pub enum CodecError {
    /// Early end of stream while deserializing.
    EarlyEndOfStream = 0,
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

/// Safely splits slice at given point.
pub(crate) fn safe_split_at(bytes: &[u8], n: usize) -> Result<(&[u8], &[u8]), CodecError> {
    if n > bytes.len() {
        Err(CodecError::EarlyEndOfStream)
    } else {
        Ok(bytes.split_at(n))
    }
}
