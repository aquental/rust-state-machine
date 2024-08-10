use num::traits::{CheckedAdd, One, Zero};
use std::collections::BTreeMap;
use std::ops::AddAssign;

pub trait Config {
    type AccountId: Ord + Clone + std::fmt::Debug;
    type BlockNumber: Zero + One + CheckedAdd + Copy + AddAssign + std::fmt::Debug;
    type Nonce: Zero + One + CheckedAdd + Copy + std::fmt::Debug;
}
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// The current block number.
    pub block_number: T::BlockNumber,
    /// A map from an account to their nonce.
    pub nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        //Return a new instance of the `Pallet` struct.
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        //Return the current block number.
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        // Increment the current block number by one.
        self.block_number += T::BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: T::AccountId) {
        // Get the current nonce of `who`, and increment it by one.
        let current_nonce = *self.nonce.get(&who).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(who, current_nonce + T::Nonce::one());
    }

    //get nonce
    #[cfg(test)]
    pub fn get_nonce(&self, who: T::AccountId) -> T::Nonce {
        *self.nonce.get(&who).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod test {
    use super::Config;

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    #[test]
    fn init_system() {
        // TODO: Create a test which checks the following:
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        // Increment the current block number.
        let bn = system.block_number();
        system.inc_block_number();
        let nbn = system.block_number();
        // Check the block number is what we expect.
        assert_eq!(bn + 1, nbn);
        // Increment the nonce of `alice`.
        let alice_nonce = system.get_nonce("alice".to_string());
        system.inc_nonce("alice".to_string());
        let new_alice_nonce = system.get_nonce("alice".to_string());
        // Check the nonce of `alice` is what we expect.
        assert_eq!(alice_nonce + 1, new_alice_nonce);
    }
}
