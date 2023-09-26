use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MultiCliqueError {
    /// Occurs in `attach_policy` when a policy already exists for the given context.
    ContractPolicyExists = 1000,

    /// Occurs in `detach_policy` when a policy does not exist for the given context.
    ContractPolicyDoesNotExist = 1001,

    /// Occurs if the maximum supported number of signers has been reached
    SignerLimitExceeded = 1002,

    /// Occurs in `__check_auth` when a public key in the signed messages is not found among the authorized signers.
    UnknownSigner = 1003,

    /// Occurs in `__check_auth` if the number of signers does not meet the default threshold for authorization.
    DefaultThresholdNotMet = 1004,

    /// Occurs in `__check_auth` if the number of signers does not meet the threshold defined by a specific policy for authorization.
    PolicyThresholdNotMet = 1005,

    /// Occurs in `remove_signer` if an attempt is made to remove a signer that does not exist in the list of authorized signers.
    SignerDoesNotExist = 1006,

    /// Occurs in `init` if the contract has already been initialized.
    AlreadyInitialized = 1007,

    /// Occurs if the threshold exceeds the number of signers
    InvalidThreshold = 1008,

    /// Occurs if a signer should be added multiple times
    SignerAlreadyAdded = 1009,
}
