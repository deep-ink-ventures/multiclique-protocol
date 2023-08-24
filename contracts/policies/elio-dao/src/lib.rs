#![no_std]

mod errors;

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, TryIntoVal, Val, Vec, BytesN, contracttype, panic_with_error};
use commons::traits::MultiCliquePolicyTrait;
#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone)]
enum DataKey {
    MultiClique,
    Core,
    Votes,
    Asset,
    SpendLimit(Address),
    AlreadySpend(Address)
}

/// This is just a sample policy to get started.
#[contractimpl]
impl MultiCliquePolicyTrait for Contract {

    fn get_threshold(env: Env,  num_signers: u32, signers: Vec<BytesN<32>>, address: Address, fn_name: Symbol, args: Vec<Val>) -> u32 {
        if num_signers < 2 {
            return 1;
        }
        if address == env.storage().instance().get(&DataKey::Core).unwrap() {
            get_core_threshold(&env, &num_signers, &signers, &fn_name, &args)
        } else if address == env.storage().instance().get(&DataKey::Votes).unwrap() {
            get_votes_threshold(&env, &num_signers, &signers, &fn_name, &args)
        } else if address == env.storage().instance().get(&DataKey::Asset).unwrap() {
            get_asset_threshold(&env, &num_signers, &signers, &fn_name, &args)
        } else if env.storage().instance().has(&DataKey::SpendLimit(address.clone())) {
            (num_signers * 50) / 100
        } else {
            num_signers
        }
    }

    fn run_policy(env: Env, num_signers: u32, signers: Vec<BytesN<32>>, address: Address, fn_name: Symbol, args: Vec<Val>) {
        if env.storage().instance().has(&DataKey::SpendLimit(address.clone())) {
            run_asset_policy(&env, &num_signers, address, &signers, &fn_name, &args)
        }
    }
}

#[contractimpl]
impl Contract {
    fn init(env: Env, multiclique_address: Address, core_address: Address, votes_address: Address, asset_address: Address) {
        if env.storage().instance().has(&DataKey::MultiClique) {
            panic_with_error!(&env, errors::PolicyError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::MultiClique, &multiclique_address);
        env.storage().instance().set(&DataKey::Core, &core_address);
        env.storage().instance().set(&DataKey::Votes, &votes_address);
        env.storage().instance().set(&DataKey::Asset, &asset_address);
    }

    fn set_spend_limit(env: Env, address: Address, limit: i128) {
        let contract_address: Address = env.storage().instance().get(&DataKey::MultiClique).unwrap();
        contract_address.require_auth();
        env.storage().instance().set(&DataKey::SpendLimit(address), &limit);
    }

    fn reset_spend_limit(env: Env, address: Address) {
        let contract_address: Address = env.storage().instance().get(&DataKey::MultiClique).unwrap();
        contract_address.require_auth();
        env.storage().instance().set(&DataKey::AlreadySpend(address), &0_i128);
    }
}

fn get_core_threshold(env: &Env, num_signers: &u32, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  -> u32 {
    if fn_name == &Symbol::new(&env,"destroy_dao") || fn_name == &Symbol::new(&env,"change_owner") {
        return (num_signers * 80) / 100;
    }
    (num_signers * 66) / 100
}

fn get_votes_threshold(env: &Env, num_signers: &u32, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  -> u32 {
    if fn_name == &Symbol::new(&env,"fault_proposal") {
        return 1;
    } else if fn_name == &Symbol::new(&env,"mark_implemented") {
        return (num_signers * 50) / 100;
    }
    (num_signers * 66) / 100
}

fn get_asset_threshold(env: &Env, num_signers: &u32, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  -> u32 {
    if fn_name == &Symbol::new(&env,"set_owner") || fn_name == &Symbol::new(&env,"set_core_address") {
        return (num_signers * 80) / 100;
    }
    (num_signers * 50) / 100
}

fn run_asset_policy(env: &Env, num_signers: &u32, address: Address, signers: &Vec<BytesN<32>>, fn_name: &Symbol, args: &Vec<Val>)  {
    let contract_address: Address = env.storage().instance().get(&DataKey::MultiClique).unwrap();

    let is_xfer = fn_name == &Symbol::new(&env, "xfer");
    let is_incr_allowance = fn_name == &Symbol::new(&env, "incr_allowance");

    if is_xfer || is_incr_allowance {
        let from: Address = args.get(0).unwrap().try_into_val(env)
            .unwrap();
        let amount: i128 = args.get(2).unwrap().try_into_val(env).unwrap();

        if from == contract_address {
            let spend_limit = env.storage().instance().get(&DataKey::SpendLimit(address.clone())).unwrap_or(0_128);
            let already_spend = env.storage().instance().get(&DataKey::AlreadySpend(address.clone())).unwrap_or(0_128);

            if already_spend + amount > spend_limit {
                panic_with_error!(&env, errors::PolicyError::SpendLimitExceeded);
            }
            env.storage().instance().set(&DataKey::AlreadySpend(address), &(already_spend + amount));
        }
    }
}

