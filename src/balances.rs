use std::collections::BTreeMap;

/*
    TODO: Define the common types used in this pallet:
        - `AccountID`
        - `Balance`

    Then update this pallet to use these common types.
*/
type AccountID = String;
type Balance = u128;

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet {
    // A simple storage mapping from accounts (`String`/AccountID) to their balances (`u128`/Balance).
    balances: BTreeMap<AccountID, Balance>,
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
    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        //Get the balance of account `caller`.
        let caller_balance = self.balance(&caller);
        println!(" [{}] caller_balance: {}", caller, caller_balance);
        //Get the balance of account `to`.
        let to_balance = self.balance(&to);
        println!(" [{}] to_balance: {}", to, to_balance);

        //Use safe math to calculate a `new_caller_balance`.
        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("insufficient balance");
        if new_caller_balance.is_ok() {
            println!(
                "new_caller_balance: ({})->{}",
                caller_balance,
                new_caller_balance.unwrap()
            );
        } else {
            println!("new_caller_balance: [insufficient balance!]");
        }
        //Use safe math to calculate a `new_to_balance`.
        let new_to_balance = to_balance.checked_add(amount).ok_or(0);
        if new_to_balance.is_ok() {
            println!(
                "new_to_balance: ({})->{}",
                to_balance,
                new_to_balance.unwrap()
            );
        } else {
            println!("new_to_balance: [overflow!]");
        }

        match (new_caller_balance, new_to_balance) {
            (Ok(new_caller_balance), Ok(new_to_balance)) => {
                //Insert the new balance of `caller`.
                self.set_balance(&caller, new_caller_balance);
                //Insert the new balance of `to`.
                self.set_balance(&to, new_to_balance);
                Ok(())
            }
            _ => Err("Error"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() {
        /* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
        let mut balances = super::Pallet::new();
        /* TODO: Assert that the balance of `alice` starts at zero. */
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        /* TODO: Set the balance of `alice` to 100. */
        balances.set_balance(&"alice".to_string(), 100);
        /* TODO: Assert the balance of `alice` is now 100. */
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        /* TODO: Assert the balance of `bob` has not changed and is 0. */
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }
    #[test]
    fn transfer_balance() {
        let blc: u128 = 50;
        let mut balances = super::Pallet::new();
        // Create a test that checks the following:
        // That `alice`(0) cannot transfer funds she does not have.
        assert!(balances
            .transfer("alice".to_string(), "bob".to_string(), blc)
            .is_err());
        // That `alice` can successfully transfer funds to `bob`.
        balances.set_balance(&"alice".to_string(), blc * 2);
        assert!(balances
            .transfer("alice".to_string(), "bob".to_string(), blc)
            .is_ok());
        // That the balance of `alice` and `bob` is correctly updated.
        assert_eq!(balances.balance(&"alice".to_string()), 100 - blc);
        assert_eq!(balances.balance(&"bob".to_string()), blc);
    }
}
