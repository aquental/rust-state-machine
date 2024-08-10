use num::traits::{CheckedAdd, One, Zero};
use std::collections::BTreeMap;
use std::ops::AddAssign;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    /// The current block number.
    pub block_number: BlockNumber,
    /// A map from an account to their nonce.
    pub nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone + std::fmt::Debug,
    BlockNumber: Zero + One + CheckedAdd + Copy + AddAssign + std::fmt::Debug,
    Nonce: Zero + One + CheckedAdd + Copy + std::fmt::Debug,
{
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        //Return a new instance of the `Pallet` struct.
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        //Return the current block number.
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        // Increment the current block number by one.
        self.block_number += BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: AccountId) {
        // Get the current nonce of `who`, and increment it by one.
        let current_nonce = *self.nonce.get(&who).unwrap_or(&Nonce::zero());
        self.nonce.insert(who, current_nonce + Nonce::one());
    }

    //get nonce
    #[cfg(test)]
    pub fn get_nonce(&self, who: AccountId) -> Nonce {
        *self.nonce.get(&who).unwrap_or(&Nonce::zero())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        // TODO: Create a test which checks the following:
        let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();
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
