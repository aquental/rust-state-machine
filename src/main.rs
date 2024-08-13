mod balances;
mod proof_of_existence;
mod support;
mod system;

use crate::support::Dispatch;

/// The `mod types { ... }` block in the Rust code snippet is defining a module named `types` that
/// contains several type aliases. Here's what each alias represents:
mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use,
// functions implemented on the Runtime allow us to access those pallets and execute blocks of
// transactions.
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

// The main entry point for our simple state machine.
/// The function simulates a blockchain runtime with extrinsics for transferring balances and
/// creating/revoke claims, executing multiple blocks and printing the runtime state.
fn main() {
    // Create a new instance of the Runtime.
    // It will instantiate with it all the modules it uses.
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    // Initialize the system with some initial balance.
    runtime.balances.set_balance(&alice, 100);

    // Here are the extrinsics in our block.
    // You can add or remove these based on the modules and calls you have set up.
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 20,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: charlie,
                    amount: 20,
                }),
            },
        ],
    };

    // This code snippet is defining `block_2` as an instance of the `types::Block` struct. It
    // represents a block in a blockchain simulation with a specific block number and a list of
    // extrinsics (transactions) to be executed within that block.
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: &"Alice's document",
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: &"Bob's document",
                }),
            },
        ],
    };

    // The `let block_3 = types::Block { ... }` code snippet is defining `block_3` as an instance of the
    // `types::Block` struct. This block represents a specific block in a blockchain simulation with a
    // block number of 3 and a list of extrinsics (transactions) to be executed within that block.
    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: &"Alice's document",
                }),
            },
            support::Extrinsic {
                caller: bob,
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: &"Bob's new document",
                }),
            },
        ],
    };

    // Execute the extrinsics which make up our blocks.
    // If there are any errors, our system panics, since we should not execute invalid blocks.
    runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");
    runtime.execute_block(block_3).expect("invalid block");

    // Simply print the debug format of our runtime state.
    println!("{:#?}", runtime);
}
