use soroban_sdk::{Address, Env, Symbol, Val, Vec, BytesN};

pub trait MultiCliquePolicyTrait {
    fn get_threshold(env: Env, num_signers: u32, signers: Vec<BytesN<32>>, address: Address, fn_name: Symbol, args: Vec<Val>) -> u32;

    fn run_policy(env: Env, num_signers: u32, signers: Vec<BytesN<32>>, address: Address, fn_name: Symbol, args: Vec<Val>);
}
