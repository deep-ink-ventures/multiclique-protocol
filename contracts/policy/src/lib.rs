#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val, Vec};
use crate::interface::MultiCliquePolicy;

pub mod interface;

pub const BUMP_A_YEAR: u32 = 6312000;

#[contract]
pub struct Contract;

#[contractimpl]
impl MultiCliquePolicy for Contract {
    fn get_threshold(env: Env, address: Address, fn_name: Symbol, args: Vec<Val>) -> u32 {
        2
    }

    fn run_policy(env: Env, address: Address, fn_name: Symbol, args: Vec<Val>) {}
}
