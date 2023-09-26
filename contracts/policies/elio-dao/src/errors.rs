use soroban_sdk::contracterror;

#[contracterror]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum PolicyError {
    /// Error when the contract is already initialized.
    /// May occur in `init` function.
    AlreadyInitialized = 1100,
    /// Error when the spend limit is exceeded.
    /// May occur in `run_asset_policy` function.
    SpendLimitExceeded = 1101,
}
