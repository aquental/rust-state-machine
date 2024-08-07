mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
    /// field `system` which is of type `system::Pallet`.
    pub system: system::Pallet,
    /// field `balances` which is of type `balances::Pallet`.
    pub balances: balances::Pallet,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        // Return a new `Runtime` by creating new instances of `system` and `balances`.
        let system = system::Pallet::new();
        let balances = balances::Pallet::new();
        Self { system, balances }
    }
}

fn main() {
    println!("Hello, Rust State Machine!");
}
