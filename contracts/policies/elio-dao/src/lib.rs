#![no_std]

mod errors;

use core::ops::Add;
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Val, Vec, BytesN, contracttype, symbol_short, panic_with_error};
use commons::traits::MultiCliquePolicyTrait;
#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone)]
enum ElioDaoAdds {
    Core,
    Votes,
    Asset,
}

/// This is just a sample policy to get started.
#[contractimpl]
impl MultiCliquePolicyTrait for Contract {

    fn get_threshold(env: Env,  num_signers: u32, signers: Vec<BytesN<32>>, address: Address, fn_name: Symbol, args: Vec<Val>) -> u32 {
        if address == env.storage().instance().get(&ElioDaoAdds::Core).unwrap() {
            get_core_threshold(&env, &num_signers, &signers, &fn_name, &args)
        } else if address == env.storage().instance().get(&ElioDaoAdds::Votes).unwrap() {
            get_votes_threshold(&env, &num_signers, &signers, &fn_name, &args)
        } else if address == env.storage().instance().get(&ElioDaoAdds::Asset).unwrap() {
            get_asset_threshold(&env, &num_signers, &signers, &fn_name, &args)
        } else {
            num_signers
        }
    }

    fn run_policy(env: Env, num_signers: u32, signers: Vec<BytesN<32>>, address: Address, fn_name: Symbol, args: Vec<Val>) {
        if address == env.storage().instance().get(&ElioDaoAdds::Core).unwrap() {
            run_core_policy(&env, &num_signers, &signers, &fn_name, &args)
        } else if address == env.storage().instance().get(&ElioDaoAdds::Votes).unwrap() {
            run_votes_policy(&env, &num_signers, &signers, &fn_name, &args)
        } else if address == env.storage().instance().get(&ElioDaoAdds::Asset).unwrap() {
            run_asset_policy(&env, &num_signers, &signers, &fn_name, &args)
        }
    }
}

#[contractimpl]
impl Contract {
    fn init(env: Env, core_address: Address, votes_address: Address, asset_addres: Address) {
        if env.storage().instance().has(&ElioDaoAdds::Core) {
            panic_with_error!(&env, errors::PolicyError::AlreadyInitialized);
        }
        env.storage().instance().set(&ElioDaoAdds::Core, &core_address);
        env.storage().instance().set(&ElioDaoAdds::Votes, &votes_address);
        env.storage().instance().set(&ElioDaoAdds::Asset, &asset_addres);
    }
}

fn get_core_threshold(env: &Env, num_signers: &u32, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  -> u32 {
    2
}

fn get_votes_threshold(env: &Env, num_signers: &u32, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  -> u32 {
    2
}

fn get_asset_threshold(env: &Env, num_signers: &u32, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  -> u32 {
    2
}

fn run_core_policy(env: &Env, num_signers: &u32, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  {

}

fn run_votes_policy(env: &Env, num_signers: &u32, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  {

}

fn run_asset_policy(env: &Env, num_signers: &u32, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  {

}

