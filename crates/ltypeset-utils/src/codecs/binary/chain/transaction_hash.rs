use super::super::utils::{CodecError, Decode, Encode};
use ltypeset::{
    chain::{TransactionV1Hash, TransactionV2Hash},
    crypto::Digest,
};

// ------------------------------------------------------------------------
// Codec: TransactionV1Hash.
// ------------------------------------------------------------------------

impl Decode for TransactionV1Hash {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bstream) = Digest::decode(bstream).unwrap();

        Ok((TransactionV1Hash::new(inner), bstream))
    }
}

impl Encode for TransactionV1Hash {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_encoded(writer).unwrap();
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: TransactionV2Hash.
// ------------------------------------------------------------------------

impl Decode for TransactionV2Hash {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bstream) = Digest::decode(bstream).unwrap();

        Ok((TransactionV2Hash::new(inner), bstream))
    }
}

impl Encode for TransactionV2Hash {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_encoded(writer).unwrap();
        Ok(())
    }
}
