use std::collections::BTreeMap;
#[warn(dead_code)]
pub struct Pallet {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
}
