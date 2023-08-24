use soroban_sdk::contracterror;

#[contracterror]
#[derive(E
#[repr(u32)]
pub enum PolicyError {
    /// Error when the contract is already initialized.
    /// May occur in `init` function.
    AlreadyInitialized = 0,
    /// Error when the spend limit is exceeded.
    /// May occur in `run_asset_policy` function.
    SpendLimitExceeded = 1,
}