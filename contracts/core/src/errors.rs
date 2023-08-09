use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MultiCliqueError {
    ContractPolicyExists = 0,
    ContractPolicyDoesNotExist = 1,
    UnknownSigner = 3,
    DefaultThresholdNotMet = 4,
    PolicyThresholdNotMet = 5
}