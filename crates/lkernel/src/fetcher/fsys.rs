use super::FetcherBackend;
use camino::{Utf8Path, Utf8PathBuf};
use ltypeset::chain::{BlockHash, BlockHeight, BlockID, BlockWithProofs};
use std::{
    fs::{self, DirEntry},
    io::Error,
};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Debug)]
struct BlockFileInfo {
    hash: BlockHash,
    height: BlockHeight,
    path: Utf8PathBuf,
}

pub struct Fetcher {
    fileset: Vec<BlockFileInfo>,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl BlockFileInfo {
    fn new(hash: BlockHash, height: BlockHeight, path_to_file: Utf8PathBuf) -> Self {
        Self {
            hash,
            height,
            path: path_to_file,
        }
    }
}

impl Fetcher {
    pub fn new(path_to_root: &Utf8Path) -> Self {
        fn get_fileset(path_to_root: &Utf8Path) -> Vec<BlockFileInfo> {
            fs::read_dir(path_to_root)
                .unwrap()
                .map(|f| BlockFileInfo::try_from(&f.unwrap()).unwrap())
                .collect()
        }

        Self {
            fileset: get_fileset(path_to_root),
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl BlockFileInfo {
    fn hash(&self) -> &BlockHash {
        &self.hash
    }

    fn height(&self) -> BlockHeight {
        self.height
    }

    fn path(&self) -> &Utf8PathBuf {
        &self.path
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl FetcherBackend for Fetcher {
    fn get_block_with_proofs(&self, block_id: BlockID) -> Option<BlockWithProofs> {
        for file_info in &self.fileset {
            match block_id {
                BlockID::BlockHash(block_hash) => {
                    if file_info.hash == block_hash {
                        return Option::Some(BlockWithProofs::from(file_info));
                    }
                }
                BlockID::BlockHeight(block_height) => {
                    if file_info.height() == block_height {
                        return Option::Some(BlockWithProofs::from(file_info));
                    }
                }
            }
        }

        None
    }

    fn init(&self) -> Result<(), Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl From<&BlockFileInfo> for BlockWithProofs {
    fn from(value: &BlockFileInfo) -> Self {
        serde_json::from_str(&fs::read_to_string(&value.path).unwrap()).unwrap()
    }
}

impl From<&DirEntry> for BlockFileInfo {
    fn from(value: &DirEntry) -> Self {
        let fpath = Utf8PathBuf::from_path_buf(value.path()).unwrap();
        let fname = value.file_name().into_string().unwrap();
        let fparts: Vec<&str> = fname.split("-").collect();

        BlockFileInfo::new(
            BlockHash::from(&fparts[2][..64]),
            BlockHeight::from(fparts[1]),
            fpath,
        )
    }
}

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::Fetcher;
    use std::env;

    fn get_path_to_root() -> String {
        format!(
            "{}/fixtures/blocks",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        )
    }

    #[test]
    fn test_that_instance_can_be_instantiated() {
        Fetcher::new(get_path_to_root());
    }
}
