#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val, Vec, BytesN};
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
    fn get_threshold(_env: Env,  num_signers: u32, _signers: Vec<BytesN<32>>, _address: Address, _fn_name: Symbol, _args: Vec<Val>) -> u32 {
        num_signers
    }

    fn run_policy(_env: Env, _num_signers: u32, _signers: Vec<BytesN<32>>, _address: Address, _fn_name: Symbol, _args: Vec<Val>) {

    }
}
