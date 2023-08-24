#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val, Vec};
use commons::traits::MultiCliquePolicyTrait;

pub const BUMP_A_YEAR: u32 = 6312000;

mod multiclique {
    use soroban_sdk::auth::Context;
    soroban_sdk::contractimport!(file = "../../../wasm/multiclique.wasm");
}

#[contract]
pub struct Contract;

/// This is just a sample policy to get started.
#[contractimpl]
impl MultiCliquePolicyTrait for Contract {
    fn get_threshold(_env: Env, _address: Address, _fn_name: Symbol, _args: Vec<Val>) -> u32 {
        2
    }

    fn run_policy(_env: Env, _address: Address, _fn_name: Symbol, _args: Vec<Val>) {

    }
}
