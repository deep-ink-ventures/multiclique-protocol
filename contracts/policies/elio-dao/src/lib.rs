#![no_std]

mod errors;

#[cfg(test)]
mod test;

use commons::traits::MultiCliquePolicyTrait;
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, Address, BytesN, Env, Symbol,
    TryIntoVal, Val, Vec,
};

/// # Contract
///
/// This contract defines the behavior and rules for managing a multi-clique policy in a DAO.
/// It includes functions for setting thresholds, initializing the contract, and managing asset policies.
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
    AlreadySpend(Address),
}

trait ElioDaoPolicyTrait {
    /// ## Init
    /// Initializes the contract by setting the addresses for MultiClique, Core, Votes, and Asset.
    ///
    /// - `env`: Environment context.
    /// - `multiclique_address`: Address of the MultiClique protocol
    /// - `core_address`: Address of the Elio DAO Core contract.
    /// - `votes_address`: Address of the Elio DAO Votes contract.
    /// - `asset_address`: Address of the Elio DAO Asset contract.
    fn init(
        env: Env,
        multiclique_address: Address,
        core_address: Address,
        votes_address: Address,
        asset_address: Address,
    );

    /// ## Set Spend Limit
    ///
    /// Sets the spend limit for a given token address (expects the soroban token interface).
    ///
    /// - `env`: Environment context.
    /// - `address`: Target address.
    /// - `limit`: Spend limit to set.
    fn set_spend_limit(env: Env, address: Address, limit: i128);

    /// ## Reset Spend Limit
    ///
    /// Resets the spend limit for a given address to zero (expects the soroban token interface).
    ///
    /// - `env`: Environment context.
    /// - `address`: Target address.
    fn reset_spend_limit(env: Env, address: Address);

    /// ## Get Spend Limit
    ///
    /// Returns the spend limit for a given address.
    ///
    /// - `env`: Environment context.
    /// - `address`: Target address.
    fn get_spend_limit(env: Env, address: Address) -> i128;

    /// ## Get Already Spend
    ///
    /// Returns the amount already spent for a given address.
    ///
    /// - `env`: Environment context.
    /// - `address`: Target address.
    fn get_already_spend(env: Env, address: Address) -> i128;
}

#[contractimpl]
impl ElioDaoPolicyTrait for Contract {
    // see: ElioDaoPolicyTrait
    fn init(
        env: Env,
        multiclique_address: Address,
        core_address: Address,
        votes_address: Address,
        asset_address: Address,
    ) {
        if env.storage().instance().has(&DataKey::MultiClique) {
            panic_with_error!(&env, errors::PolicyError::AlreadyInitialized);
        }
        env.storage()
            .instance()
            .set(&DataKey::MultiClique, &multiclique_address);
        env.storage().instance().set(&DataKey::Core, &core_address);
        env.storage()
            .instance()
            .set(&DataKey::Votes, &votes_address);
        env.storage()
            .instance()
            .set(&DataKey::Asset, &asset_address);
    }

    // see: ElioDaoPolicyTrait
    fn set_spend_limit(env: Env, address: Address, limit: i128) {
        let contract_address: Address =
            env.storage().instance().get(&DataKey::MultiClique).unwrap();
        contract_address.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::SpendLimit(address), &limit);
    }

    // see: ElioDaoPolicyTrait
    fn reset_spend_limit(env: Env, address: Address) {
        let contract_address: Address =
            env.storage().instance().get(&DataKey::MultiClique).unwrap();
        contract_address.require_auth();
        env.storage()
            .instance()
            .set(&DataKey::AlreadySpend(address), &0_i128);
    }

    // see: ElioDaoPolicyTrait
    fn get_spend_limit(env: Env, address: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::SpendLimit(address))
            .unwrap_or(0_i128)
    }

    // see: ElioDaoPolicyTrait
    fn get_already_spend(env: Env, address: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::AlreadySpend(address))
            .unwrap_or(0_i128)
    }
}

#[contractimpl]
impl MultiCliquePolicyTrait for Contract {
    /// ## Get Threshold
    ///
    /// Returns the threshold required for a particular action in the DAO.
    ///
    /// - `env`: Environment context.
    /// - `num_signers`: Number of signers involved in the action.
    /// - `signers`: List of signers' addresses.
    /// - `address`: Target address for the action.
    /// - `fn_name`: Function name representing the action.
    /// - `args`: Additional arguments for the action.
    ///
    /// **Returns**: Threshold as a `u32`.
    fn get_threshold(
        env: Env,
        num_signers: u32,
        signers: Vec<BytesN<32>>,
        address: Address,
        fn_name: Symbol,
        args: Vec<Val>,
    ) -> u32 {
        if num_signers < 2 {
            return 1;
        }
        if address == env.storage().instance().get(&DataKey::Core).unwrap() {
            get_core_threshold(&env, &num_signers, &signers, &fn_name, &args)
        } else if address == env.storage().instance().get(&DataKey::Votes).unwrap() {
            get_votes_threshold(&env, &num_signers, &signers, &fn_name, &args)
        } else if address == env.storage().instance().get(&DataKey::Asset).unwrap() {
            get_asset_threshold(&env, &num_signers, &signers, &fn_name, &args)
        } else if env
            .storage()
            .instance()
            .has(&DataKey::SpendLimit(address.clone()))
        {
            (num_signers * 50) / 100
        } else {
            num_signers
        }
    }

    /// ## Run Policy
    ///
    /// Executes the policy rules based on the given action and parameters.
    ///
    /// - `env`: Environment context.
    /// - `num_signers`: Number of signers involved in the action.
    /// - `signers`: List of signers' addresses.
    /// - `address`: Target address for the action.
    /// - `fn_name`: Function name representing the action.
    /// - `args`: Additional arguments for the action.
    fn run_policy(
        env: Env,
        num_signers: u32,
        signers: Vec<BytesN<32>>,
        address: Address,
        fn_name: Symbol,
        args: Vec<Val>,
    ) {
        if env
            .storage()
            .instance()
            .has(&DataKey::SpendLimit(address.clone()))
        {
            run_asset_policy(&env, &num_signers, address, &signers, &fn_name, &args)
        }
    }
}

/// ## Get Core Threshold
///
/// Returns the threshold for core-related actions.
///
/// - `env`: Environment context.
/// - `num_signers`: Number of signers involved.
/// - `signers`: List of signers' addresses.
/// - `fn_name`: Function name representing the action.
/// - `args`: Additional arguments for the action.
///
/// **Returns**: Threshold as a `u32`.
fn get_core_threshold(
    env: &Env,
    num_signers: &u32,
    _signers: &Vec<BytesN<32>>,
    fn_name: &Symbol,
    _args: &Vec<Val>,
) -> u32 {
    if fn_name == &Symbol::new(&env, "destroy_dao") || fn_name == &Symbol::new(&env, "change_owner")
    {
        return (num_signers * 80) / 100;
    }
    (num_signers * 66) / 100
}

/// ## Get Votes Threshold
///
/// Returns the threshold for votes-related actions.
///
/// - `env`: Environment context.
/// - `num_signers`: Number of signers involved.
/// - `signers`: List of signers' addresses.
/// - `fn_name`: Function name representing the action.
/// - `args`: Additional arguments for the action.
///
/// **Returns**: Threshold as a `u32`.
fn get_votes_threshold(
    env: &Env,
    num_signers: &u32,
    _signers: &Vec<BytesN<32>>,
    fn_name: &Symbol,
    _args: &Vec<Val>,
) -> u32 {
    if fn_name == &Symbol::new(&env, "fault_proposal") {
        return 1;
    } else if fn_name == &Symbol::new(&env, "mark_implemented") {
        return (num_signers * 50) / 100;
    }
    (num_signers * 66) / 100
}

/// ## Get Asset Threshold
///
/// Returns the threshold for asset-related actions.
///
/// - `env`: Environment context.
/// - `num_signers`: Number of signers involved.
/// - `signers`: List of signers' addresses.
/// - `fn_name`: Function name representing the action.
/// - `args`: Additional arguments for the action.
///
/// **Returns**: Threshold as a `u32`.
fn get_asset_threshold(
    env: &Env,
    num_signers: &u32,
    _signers: &Vec<BytesN<32>>,
    fn_name: &Symbol,
    _args: &Vec<Val>,
) -> u32 {
    if fn_name == &Symbol::new(&env, "set_owner")
        || fn_name == &Symbol::new(&env, "set_core_address")
    {
        return (num_signers * 80) / 100;
    }
    (num_signers * 50) / 100
}

/// ## Run Asset Policy
///
/// Executes the policy rules for asset-related actions.
///
/// - `env`: Environment context.
/// - `num_signers`: Number of signers involved.
/// - `address`: Target address for the action.
/// - `signers`: List of signers' addresses.
/// - `fn_name`: Function name representing the action.
/// - `args`: Additional arguments for the action.
fn run_asset_policy(
    env: &Env,
    _num_signers: &u32,
    address: Address,
    _signers: &Vec<BytesN<32>>,
    fn_name: &Symbol,
    args: &Vec<Val>,
) {
    let contract_address: Address = env.storage().instance().get(&DataKey::MultiClique).unwrap();

    let is_xfer = fn_name == &Symbol::new(&env, "xfer");
    let is_incr_allowance = fn_name == &Symbol::new(&env, "incr_allowance");

    if is_xfer || is_incr_allowance {
        let from: Address = args.get(0).unwrap().try_into_val(env).unwrap();
        let amount: i128 = args.get(2).unwrap().try_into_val(env).unwrap();

        if from == contract_address {
            let spend_limit = env
                .storage()
                .instance()
                .get(&DataKey::SpendLimit(address.clone()))
                .unwrap_or(0_i128);
            let already_spend = env
                .storage()
                .instance()
                .get(&DataKey::AlreadySpend(address.clone()))
                .unwrap_or(0_i128);

            if already_spend + amount > spend_limit {
                panic_with_error!(&env, errors::PolicyError::SpendLimitExceeded);
            }
            env.storage()
                .instance()
                .set(&DataKey::AlreadySpend(address), &(already_spend + amount));
        }
    }
}
