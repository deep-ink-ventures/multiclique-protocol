use soroban_sdk::{contracttype, symbol_short, Address, Symbol};

// Symbol representing policy-related events.
pub const POLICY: Symbol = symbol_short!("POLICY");

// Symbol representing the policy contract init.
pub const INIT: Symbol = symbol_short!("init");

// Symbol representing a spend limit set event
pub const SPEND_LIMIT_SET: Symbol = symbol_short!("lmt_set");

// Symbol representing a spend limit reset event
pub const SPEND_LIMIT_RESET: Symbol = symbol_short!("lmt_reset");

// Symbol representing an already spent amount update
pub const ALREADY_SPENT_UPDATE: Symbol = symbol_short!("spent_upd");

// Event data for when a policy contract is initiated
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyInitEventData {
    pub multiclique_address: Address,
    pub core_address: Address,
    pub votes_address: Address,
    pub asset_address: Address,
}

// Event data for when a policy contract is initiated
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicySpendLimitSetEventData {
    pub address: Address,
    pub limit: i128,
}

// Event data for when a policy contract is initiated
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicySpendLimitResetEventData {
    pub address: Address,
}

// Event data for when a policy contract is initiated
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAlreadySpendUpdateEventData {
    pub address: Address,
    pub already_spend: i128,
}
