/// Encoded size: `bool`.
pub(super) const ENCODED_SIZE_BOOL: usize = 1;

/// The number of bytes in a serialized `i32`.
pub(super) const ENCODED_SIZE_I32: usize = core::mem::size_of::<i32>();

/// The number of bytes in a serialized `i64`.
pub(super) const ENCODED_SIZE_I64: usize = core::mem::size_of::<i64>();

/// Encoded size: `unit`.
pub(super) const ENCODED_SIZE_UNIT: usize = 0;

/// Encoded size: `u8`.
pub(super) const ENCODED_SIZE_U8: usize = core::mem::size_of::<u8>();

/// Encoded size: `u16`.
pub(super) const ENCODED_SIZE_U16: usize = core::mem::size_of::<u16>();

/// Encoded size: `u32`.
pub(super) const ENCODED_SIZE_U32: usize = core::mem::size_of::<u32>();

/// Encoded size: `u64`.
pub(super) const ENCODED_SIZE_U64: usize = core::mem::size_of::<u64>();

/// The number of bytes in a serialized [`U128`](crate::U128).
pub(super) const ENCODED_SIZE_U128: usize = core::mem::size_of::<u128>();

/// The number of bytes in a serialized [`U256`](crate::U256).
pub(super) const ENCODED_SIZE_U256: usize = ENCODED_SIZE_U128 * 2;

/// The number of bytes in a serialized [`U512`](crate::U512).
pub(super) const ENCODED_SIZE_U512: usize = ENCODED_SIZE_U256 * 2;

/// The tag representing a `None` value.
pub(super) const TAG_OPTION_NONE: u8 = 0;

/// The tag representing a `Some` value.
pub(super) const TAG_OPTION_SOME: u8 = 1;

/// The tag representing an `Err` value.
pub(super) const TAG_RESULT_ERR: u8 = 0;

/// The tag representing an `Ok` value.
pub(super) const TAG_RESULT_OK: u8 = 1;
