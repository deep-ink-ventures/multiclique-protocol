use soroban_sdk::{Address, Env, Symbol, Val, Vec};

pub trait MultiCliquePolicy {
    fn get_threshold(env: Env, address: Address, fn_name: Symbol, args: Vec<Val>) -> u32;

    fn run_policy(env: Env, address: Address, fn_name: Symbol, args: Vec<Val>);
}
