use crate::primitives::SemanticVersion;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
pub struct ProtocolVersion(SemanticVersion);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl ProtocolVersion {
    pub fn new(semantic_version: SemanticVersion) -> Self {
        Self(semantic_version)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl ProtocolVersion {
    pub fn inner(&self) -> &SemanticVersion {
        &self.0
    }
}
