use soroban_sdk::{contracttype, symbol_short, Address, BytesN, Symbol, Vec};

// Symbol representing signer-related events.
pub const SIGNER: Symbol = symbol_short!("SIGNER");

// Symbol representing policy-related events.
pub const POLICY: Symbol = symbol_short!("POLICY");

// Symbol representing governance-related events.
pub const GOV: Symbol = symbol_short!("GOV");

// Symbol representing an added event.
pub const ADDED: Symbol = symbol_short!("added");

// Symbol representing a removed event.
pub const REMOVED: Symbol = symbol_short!("removed");

// Symbol representing a changed event.
pub const CHANGED: Symbol = symbol_short!("changed");

// Event data for when a signer is added.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignerAddedEventData {
    pub signer: BytesN<32>,
}

// Event data for when a signer is removed.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignerRemovedEventData {
    pub signer: BytesN<32>,
}

// Event data for when a policy is added along with its associated context.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAddedEventData {
    pub policy: Address,
    pub context: Vec<Address>,
}

// Event data for when a policy is removed from its associated context.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyRemovedEventData {
    pub context: Vec<Address>,
}

// Event data for when the default threshold is changed.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultThresholdChangedEventData {
    pub threshold: u32,
}
