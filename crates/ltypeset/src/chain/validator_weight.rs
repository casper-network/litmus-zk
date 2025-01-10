use super::{Motes, ValidatorID};
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Weight of an identity's vote within the context of some form of governance process.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct ValidatorWeight {
    /// Identity of a voting entity.
    #[serde(rename = "validator")]
    validator_id: ValidatorID,

    /// Relative weight of vote.
    weight: Motes,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl ValidatorWeight {
    pub fn new(validator_id: ValidatorID, weight: Motes) -> Self {
        Self {
            validator_id,
            weight,
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl ValidatorWeight {
    pub fn validator_id(&self) -> ValidatorID {
        self.validator_id
    }
    pub fn weight(&self) -> Motes {
        self.weight
    }
}
