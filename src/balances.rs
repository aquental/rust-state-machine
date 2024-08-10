use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config: crate::system::Config {
    type Balance: Zero + One + CheckedAdd + CheckedSub + Copy + AddAssign + std::fmt::Debug;
}
/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // A simple storage mapping from accounts (`String`/AccountID) to their balances (`u128`/Balance).
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
        /* Insert `amount` into the BTreeMap under `who`. */
        self.balances.insert(who, amount);
    }
    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: T::AccountId) -> T::Balance {
        /* Return the balance of `who`, returning zero if `None`. */
        *self.balances.get(&who).unwrap_or(&T::Balance::zero())
    }
    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        //Get the balance of account `caller`.
        let caller_balance = self.balance(caller.clone());
        //println!(" [{:?}] caller_balance: {:?}", caller, caller_balance);
        //Get the balance of account `to`.
        let to_balance = self.balance(to.clone());
        //println!(" [{:?}] to_balance: {:?}", to.clone(), to_balance);

        //Use safe math to calculate a `new_caller_balance`.
        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("insufficient balance");
        if new_caller_balance.is_ok() {
            println!(
                "new_caller_balance: ({:?})->{:?}",
                caller_balance,
                new_caller_balance.unwrap()
            );
        } else {
            println!("new_caller_balance: [insufficient balance!]");
        }
        //Use safe math to calculate a `new_to_balance`.
        let new_to_balance = to_balance.checked_add(&amount).ok_or(0);
        if new_to_balance.is_ok() {
            println!(
                "new_to_balance: ({:?})->{:?}",
                to_balance,
                new_to_balance.unwrap()
            );
        } else {
            println!("new_to_balance: [overflow!]");
        }

        match (new_caller_balance, new_to_balance) {
            (Ok(new_caller_balance), Ok(new_to_balance)) => {
                //Insert the new balance of `caller`.
                self.set_balance(caller, new_caller_balance);
                //Insert the new balance of `to`.
                self.set_balance(to, new_to_balance);
                Ok(())
            }
            _ => Err("Error"),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::system;

//     struct TestConfig;
//     impl system::Config for TestConfig {
//         type AccountId = String;
//         type BlockNumber = u32;
//         type Nonce = u32;
//     }
//     #[test]
//     fn init_balances() {
//         /* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
//         let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
//         /* TODO: Assert that the balance of `alice` starts at zero. */
//         assert_eq!(balances.balance("alice".to_string()), 0);
//         /* TODO: Set the balance of `alice` to 100. */
//         let alice = "alice".to_string();
//         balances.set_balance(alice, 100);
//         /* TODO: Assert the balance of `alice` is now 100. */
//         assert_eq!(balances.balance("alice".to_string()), 100);
//         /* TODO: Assert the balance of `bob` has not changed and is 0. */
//         assert_eq!(balances.balance("bob".to_string()), 0);
//     }
//     #[test]
//     fn transfer_balance() {
//         let blc: u128 = 50;
//         let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
//         // Create a test that checks the following:
//         // That `alice`(0) cannot transfer funds she does not have.
//         assert!(balances
//             .transfer("alice".to_string(), "bob".to_string(), blc)
//             .is_err());
//         // That `alice` can successfully transfer funds to `bob`.
//         balances.set_balance("alice".to_string(), blc * 2);
//         assert!(balances
//             .transfer("alice".to_string(), "bob".to_string(), blc)
//             .is_ok());
//         // That the balance of `alice` and `bob` is correctly updated.
//         assert_eq!(balances.balance("alice".to_string()), 100 - blc);
//         assert_eq!(balances.balance("bob".to_string()), blc);
//     }
// }
