use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
    /// The current block number.
    pub block_number: u32,
    /// A map from an account to their nonce.
    pub nonce: BTreeMap<String, u32>,
}

impl Pallet {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        //Return a new instance of the `Pallet` struct.
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }
    /// Get the current block number.
    pub fn block_number(&self) -> u32 {
        //Return the current block number.
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        // Increment the current block number by one.
        self.block_number += 1;
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &String) {
        // Get the current nonce of `who`, and increment it by one.
        let current_nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.to_string(), current_nonce + 1);
    }

    //get nonce
    #[cfg(test)]
    pub fn get_nonce(&self, who: &String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        // TODO: Create a test which checks the following:
        let mut system = super::Pallet::new();
        // Increment the current block number.
        let bn = system.block_number();
        system.inc_block_number();
        let nbn = system.block_number();
        // Check the block number is what we expect.
        assert_eq!(bn + 1, nbn);
        // Increment the nonce of `alice`.
        let alice_nonce = system.get_nonce(&"alice".to_string());
        system.inc_nonce(&"alice".to_string());
        let new_alice_nonce = system.get_nonce(&"alice".to_string());
        // Check the nonce of `alice` is what we expect.
        assert_eq!(alice_nonce + 1, new_alice_nonce);
    }
}
