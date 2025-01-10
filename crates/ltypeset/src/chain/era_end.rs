extern crate alloc;

use super::{Motes, ValidatorWeight};
use crate::crypto::PublicKey;
use alloc::collections::BTreeMap;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct EraEndV1 {}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct EraEndV2 {
    /// Set of validators deemed to have equivocated.
    equivocators: Vec<PublicKey>,

    /// Validators that did not produce units during era.
    inactive_validators: Vec<PublicKey>,

    /// Validator set for upcoming era plus respective weights.
    next_era_validator_weights: Vec<ValidatorWeight>,

    /// Rewards to be distributed to validators within era.
    rewards: BTreeMap<PublicKey, Vec<Motes>>,

    /// Gas price to be applied in next era.
    next_era_gas_price: u8,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl EraEndV2 {
    pub fn new(
        equivocators: Vec<PublicKey>,
        inactive_validators: Vec<PublicKey>,
        next_era_validator_weights: Vec<ValidatorWeight>,
        rewards: BTreeMap<PublicKey, Vec<Motes>>,
        next_era_gas_price: u8,
    ) -> Self {
        Self {
            equivocators,
            inactive_validators,
            next_era_validator_weights,
            rewards,
            next_era_gas_price,
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl EraEndV2 {
    pub fn equivocators(&self) -> &Vec<PublicKey> {
        &self.equivocators
    }

    pub fn inactive_validators(&self) -> &Vec<PublicKey> {
        &self.inactive_validators
    }

    pub fn next_era_validator_weights(&self) -> &Vec<ValidatorWeight> {
        &self.next_era_validator_weights
    }

    pub fn rewards(&self) -> &BTreeMap<PublicKey, Vec<Motes>> {
        &self.rewards
    }

    pub fn next_era_gas_price(&self) -> u8 {
        self.next_era_gas_price
    }
}
