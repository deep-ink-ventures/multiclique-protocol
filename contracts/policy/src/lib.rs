#![no_std]

mod interface;
mod errors;

use soroban_sdk::{contract, contracttype, contractimpl, symbol_short, vec, Env, Symbol, Vec, BytesN, Address, panic_with_error};
use soroban_sdk::auth::Context;
use crate::interface::MultiCliquePolicy;

pub const BUMP_A_YEAR: u32 = 6312000;

#[contract]
pub struct Contract;

#[contractimpl]
impl MultiCliquePolicy for Contract {
    fn get_threshold(env: Env, ctx: Context) -> u32 {
        2
    }

    fn run_policy(env: Env, ctx: Context) {

    }
}

