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
    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        /* Insert `amount` into the BTreeMap under `who`. */
        self.balances.insert(who.to_string(), amount);
    }
    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &String) -> u128 {
        /* Return the balance of `who`, returning zero if `None`. */
        *self.balances.get(who).unwrap_or(&0)
    }
}
