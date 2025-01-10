use super::EraId;
use super::Motes;
use super::ValidatorWeight;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Information scoped by era pertinent to consensus.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct EraConsensusInfo {
    /// Era identifier.
    era_id: EraId,

    /// Era scoped validator voting weights.
    validator_weights: Vec<ValidatorWeight>,

    /// Total voting weight.
    total_weight: Motes,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl EraConsensusInfo {
    pub fn new(era_id: EraId, validator_weights: Vec<ValidatorWeight>) -> Self {
        let total_weight: &Motes = &validator_weights
            .iter()
            .fold(Motes::MIN, |acc, x| acc + x.weight());

        Self {
            era_id,
            validator_weights,
            total_weight: total_weight.clone(),
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl EraConsensusInfo {
    pub fn era_id(&self) -> &EraId {
        &self.era_id
    }
    pub fn validator_weights(&self) -> &Vec<ValidatorWeight> {
        &self.validator_weights
    }
    pub fn total_weight(&self) -> Motes {
        self.total_weight
    }
}
