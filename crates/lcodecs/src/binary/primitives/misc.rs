use super::super::constants;
use crate::binary::utils::{CodecError, Decode, Encode};

// ------------------------------------------------------------------------
// Codec: bool.
// ------------------------------------------------------------------------

impl Decode for bool {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (value, bstream) = match bstream.split_first() {
            None => panic!("Invalid bool encoding"),
            Some((val, bstream)) => match val {
                1 => (true, bstream),
                0 => (false, bstream),
                _ => panic!("Invalid bool encoding"),
            },
        };

        Ok((value, bstream))
    }
}

impl Encode for bool {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_BOOL
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        writer.push(u8::from(*self));
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: Option<T>.
// ------------------------------------------------------------------------

impl<T: Decode> Decode for Option<T> {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (tag, bstream) = u8::decode(bstream)?;
        let (d, bstream) = match tag {
            constants::TAG_OPTION_NONE => (None, bstream),
            constants::TAG_OPTION_SOME => {
                let (t, bstream) = T::decode(bstream)?;
                (Some(t), bstream)
            }
            _ => panic!("Invalid Option<T> type tag"),
        };

        Ok((d, bstream))
    }
}

impl<T: Encode> Encode for Option<T> {
    fn get_encoded_size(&self) -> usize {
        match self {
            Some(v) => constants::ENCODED_SIZE_U8 + v.get_encoded_size(),
            None => constants::ENCODED_SIZE_U8,
        }
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        match self {
            None => {
                writer.push(constants::TAG_OPTION_NONE);
            }
            Some(inner) => {
                writer.push(constants::TAG_OPTION_SOME);
                inner.write_encoded(writer).unwrap();
            }
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: Result<T, E>.
// ------------------------------------------------------------------------

impl<T: Decode, E: Decode> Decode for Result<T, E> {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (tag, bstream) = u8::decode(bstream)?;
        let (result, bstream) = match tag {
            constants::TAG_RESULT_ERR => {
                let (value, bstream) = E::decode(bstream)?;
                (Err(value), bstream)
            }
            constants::TAG_RESULT_OK => {
                let (value, bstream) = T::decode(bstream)?;
                (Ok(value), bstream)
            }
            _ => panic!("Invalid Result<T, E> type tag"),
        };

        Ok((result, bstream))
    }
}

impl<T: Encode, E: Encode> Encode for Result<T, E> {
    fn get_encoded_size(&self) -> usize {
        match self {
            Err(error) => constants::ENCODED_SIZE_U8 + error.get_encoded_size(),
            Ok(value) => constants::ENCODED_SIZE_U8 + value.get_encoded_size(),
        }
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        match self {
            Err(error) => {
                writer.push(constants::TAG_RESULT_ERR);
                error.write_encoded(writer).unwrap();
            }
            Ok(value) => {
                writer.push(constants::TAG_RESULT_OK);
                value.write_encoded(writer).unwrap();
            }
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: unit.
// ------------------------------------------------------------------------

impl Decode for () {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        Ok(((), bstream))
    }
}

impl Encode for () {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_UNIT
    }

    fn write_encoded(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        Ok(())
    }
}
