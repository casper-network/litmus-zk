use super::super::{
    constants,
    utils::{CodecError, Decode, Encode},
};
use ltypeset::{
    chain::{
        Block, BlockHash, BlockHeight, BlockV1, BlockV1Body, BlockV1Header, BlockV2, BlockV2Body,
        BlockV2Header, EraEndV2, EraId, ProtocolVersion,
    },
    crypto::{Digest, PublicKey},
    primitives::Timestamp,
};

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const TAG_BLOCK_V1: u8 = 0;
const TAG_BLOCK_V2: u8 = 1;

// ------------------------------------------------------------------------
// Codec: Block.
// ------------------------------------------------------------------------

impl Decode for Block {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (version_tag, bstream) = u8::decode(bstream).unwrap();
        let (block, bstream) = match version_tag {
            TAG_BLOCK_V1 => {
                let (inner, bstream) = BlockV1::decode(bstream).unwrap();
                (Block::new_v1(inner), bstream)
            }
            TAG_BLOCK_V2 => {
                let (inner, bstream) = BlockV2::decode(bstream).unwrap();
                (Block::new_v2(inner), bstream)
            }
            _ => panic!("Invalid block version tag"),
        };

        Ok((block, bstream))
    }
}

impl Encode for Block {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U8
            + match self {
                Block::V1(inner) => inner.get_encoded_size(),
                Block::V2(inner) => inner.get_encoded_size(),
            }
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        match self {
            Block::V1(inner) => {
                writer.push(TAG_BLOCK_V1);
                inner.write_encoded(writer).unwrap();
            }
            Block::V2(inner) => {
                writer.push(TAG_BLOCK_V2);
                inner.write_encoded(writer).unwrap();
            }
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockHash.
// ------------------------------------------------------------------------

impl Decode for BlockHash {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bstream) = Digest::decode(bstream).unwrap();

        Ok((BlockHash::new(inner), bstream))
    }
}

impl Encode for BlockHash {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_encoded(writer).unwrap();

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockHeight.
// ------------------------------------------------------------------------

impl Decode for BlockHeight {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bstream) = u64::decode(&bstream).unwrap();

        Ok((Self::new(inner), &bstream))
    }
}

impl Encode for BlockHeight {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_encoded(writer).unwrap();

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV1.
// ------------------------------------------------------------------------

impl Decode for BlockV1 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (hash, bstream) = BlockHash::decode(bstream).unwrap();
        let (header, bstream) = BlockV1Header::decode(bstream).unwrap();
        let (body, bstream) = BlockV1Body::decode(bstream).unwrap();

        Ok((BlockV1::new(body, hash, header), bstream))
    }
}

impl Encode for BlockV1 {
    fn get_encoded_size(&self) -> usize {
        self.hash().get_encoded_size()
            + self.header().get_encoded_size()
            + self.body().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.hash().write_encoded(writer).unwrap();
        self.header().write_encoded(writer).unwrap();
        self.body().write_encoded(writer).unwrap();

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV1Body.
// ------------------------------------------------------------------------

impl Decode for BlockV1Body {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV1Body {
    fn get_encoded_size(&self) -> usize {
        unimplemented!();
    }

    fn write_encoded(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!();
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV1Header.
// ------------------------------------------------------------------------

impl Decode for BlockV1Header {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV1Header {
    fn get_encoded_size(&self) -> usize {
        unimplemented!();
    }

    fn write_encoded(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!();
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV2.
// ------------------------------------------------------------------------

impl Decode for BlockV2 {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (hash, bstream) = BlockHash::decode(bstream).unwrap();
        let (header, bstream) = BlockV2Header::decode(bstream).unwrap();
        let (body, bstream) = BlockV2Body::decode(bstream).unwrap();

        Ok((BlockV2::new(body, hash, header), bstream))
    }
}

impl Encode for BlockV2 {
    fn get_encoded_size(&self) -> usize {
        self.hash().get_encoded_size()
            + self.header().get_encoded_size()
            + self.body().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.hash().write_encoded(writer).unwrap();
        self.header().write_encoded(writer).unwrap();
        self.body().write_encoded(writer).unwrap();

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV2Body.
// ------------------------------------------------------------------------

impl Decode for BlockV2Body {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!()
    }
}

impl Encode for BlockV2Body {
    fn get_encoded_size(&self) -> usize {
        self.transactions().get_encoded_size() + self.rewarded_signatures().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.transactions().write_encoded(writer).unwrap();
        self.rewarded_signatures().write_encoded(writer).unwrap();

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV2Header.
// ------------------------------------------------------------------------

impl Decode for BlockV2Header {
    fn decode(bstream: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (parent_hash, bstream) = BlockHash::decode(bstream).unwrap();
        let (state_root_hash, bstream) = Digest::decode(bstream).unwrap();
        let (body_hash, bstream) = Digest::decode(bstream).unwrap();
        let (random_bit, bstream) = bool::decode(bstream).unwrap();
        let (accumulated_seed, bstream) = Digest::decode(bstream).unwrap();
        let (era_end, bstream) = Option::<EraEndV2>::decode(bstream).unwrap();
        let (timestamp, bstream) = Timestamp::decode(bstream).unwrap();
        let (era_id, bstream) = EraId::decode(bstream).unwrap();
        let (height, bstream) = BlockHeight::decode(bstream).unwrap();
        let (protocol_version, bstream) = ProtocolVersion::decode(bstream).unwrap();
        let (proposer, bstream) = PublicKey::decode(bstream).unwrap();
        let (current_gas_price, bstream) = u8::decode(bstream).unwrap();
        let (last_switch_block_hash, bstream) = Option::<BlockHash>::decode(bstream).unwrap();

        Ok((
            BlockV2Header::new(
                accumulated_seed,
                body_hash,
                current_gas_price,
                era_end,
                era_id,
                height,
                last_switch_block_hash,
                parent_hash,
                proposer,
                protocol_version,
                random_bit,
                state_root_hash,
                timestamp,
            ),
            bstream,
        ))
    }
}

impl Encode for BlockV2Header {
    fn get_encoded_size(&self) -> usize {
        self.parent_hash().get_encoded_size()
            + self.state_root_hash().get_encoded_size()
            + self.body_hash().get_encoded_size()
            + self.random_bit().get_encoded_size()
            + self.accumulated_seed().get_encoded_size()
            + self.era_end().get_encoded_size()
            + self.timestamp().get_encoded_size()
            + self.era_id().get_encoded_size()
            + self.height().get_encoded_size()
            + self.protocol_version().get_encoded_size()
            + self.proposer().get_encoded_size()
            + self.current_gas_price().get_encoded_size()
            + self.last_switch_block_hash().get_encoded_size()
    }

    fn write_encoded(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.parent_hash().write_encoded(writer).unwrap();
        self.state_root_hash().write_encoded(writer).unwrap();
        self.body_hash().write_encoded(writer).unwrap();
        self.random_bit().write_encoded(writer).unwrap();
        self.accumulated_seed().write_encoded(writer).unwrap();
        self.era_end().write_encoded(writer).unwrap();
        self.timestamp().write_encoded(writer).unwrap();
        self.era_id().write_encoded(writer).unwrap();
        self.height().write_encoded(writer).unwrap();
        self.protocol_version().write_encoded(writer).unwrap();
        self.proposer().write_encoded(writer).unwrap();
        self.current_gas_price().write_encoded(writer).unwrap();
        self.last_switch_block_hash().write_encoded(writer).unwrap();
        Ok(())
    }
}
