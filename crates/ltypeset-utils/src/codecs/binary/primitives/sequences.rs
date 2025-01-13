use super::super::{
    constants,
    utils::{CodecError, Decode, Encode},
};
use std::collections::BTreeMap;

// ------------------------------------------------------------------------
// Codec: Vec<T>.
// ------------------------------------------------------------------------

impl<T: Decode> Decode for Vec<T> {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        // Vec size.
        let (size, mut bstream) = u32::decode(bstream).unwrap();
        if size == 0 {
            return Ok((Vec::new(), bstream));
        }

        // Vec data.
        let mut result = Vec::<T>::with_capacity(size as usize);
        for _ in 0..size {
            let (entity, bstream_1) = T::decode(bstream).unwrap();
            result.push(entity);
            bstream = bstream_1;
        }

        Ok((result, bstream))
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn get_encoded_size(&self) -> usize {
        let mut result = constants::ENCODED_SIZE_U32;
        for entity in self.iter() {
            result += entity.get_encoded_size();
        }
        result
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        // Vec size.
        let size: u32 = self
            .len()
            .try_into()
            .map_err(|_| CodecError::NotRepresentable)
            .unwrap();
        size.write_encoded(writer).unwrap();

        // Vec data.
        for entity in self.iter() {
            entity.write_encoded(writer).unwrap();
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BTreeMap<K, V>.
// ------------------------------------------------------------------------

impl<K, V> Decode for BTreeMap<K, V>
where
    K: Decode + Ord,
    V: Decode,
{
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        // BTreeMap size.
        let (size, mut bstream) = u32::decode(bstream).unwrap();

        // BTreeMap data.
        let mut result = BTreeMap::new();
        for _ in 0..size {
            let (k, bstream_1) = K::decode(bstream).unwrap();
            let (v, bstream_1) = V::decode(bstream_1).unwrap();
            result.insert(k, v);
            bstream = bstream_1;
        }

        Ok((result, bstream))
    }
}

impl<K, V> Encode for BTreeMap<K, V>
where
    K: Encode,
    V: Encode,
{
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U32
            + self
                .iter()
                .map(|(key, value)| key.get_encoded_size() + value.get_encoded_size())
                .sum::<usize>()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        // BTreeMap size.
        let size: u32 = self
            .len()
            .try_into()
            .map_err(|_| CodecError::NotRepresentable)
            .unwrap();
        size.write_encoded(writer).unwrap();

        // BTreeMap data.
        for (key, value) in self.iter() {
            key.write_encoded(writer).unwrap();
            value.write_encoded(writer).unwrap();
        }

        Ok(())
    }
}
