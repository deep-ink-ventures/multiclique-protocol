use soroban_sdk::{contracttype, symbol_short, Address, BytesN, Symbol, Vec};

pub const SIGNER: Symbol = symbol_short!("SIGNER");
pub const POLICY: Symbol = symbol_short!("POLICY");
pub const GOV: Symbol = symbol_short!("GOV");

pub const ADDED: Symbol = symbol_short!("added");
pub const REMOVED: Symbol = symbol_short!("removed");
pub const CHANGED: Symbol = symbol_short!("changed");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignerAddedEventData {
    pub signer: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignerRemovedEventData {
    pub signer: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAddedEventData {
    pub policy: Address,
    pub context: Vec<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyRemovedEventData {
    pub context: Vec<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultThresholdChangedEventData {
    pub threshold: u32,
}
