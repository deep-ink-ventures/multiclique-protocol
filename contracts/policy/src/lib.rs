#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec, Address, Val};
mod interface;


use crate::interface::MultiCliquePolicy;

pub const BUMP_A_YEAR: u32 = 6312000;

#[contract]
pub struct Contract;

#[contractimpl]
impl MultiCliquePolicy for Contract {
    fn get_threshold(env: Env, address: Address, fn_name: Symbol, args: Vec<Val>) -> u32 {
        2
    }

    fn run_policy(env: Env, address: Address, fn_name: Symbol, args: Vec<Val>) {

    }
}

