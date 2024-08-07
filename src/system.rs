/* TODO: You might need to update your imports. */

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
    /// The current block number.
    pub block_number: u32,
    /// A map from an account to their nonce.
    pub nonce: std::collections::BTreeMap<String, u32>,
}

impl Pallet {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        /* TODO: Return a new instance of the `Pallet` struct. */
        Self {
            block_number: 0,
            nonce: std::collections::BTreeMap::new(),
        }
    }
}
