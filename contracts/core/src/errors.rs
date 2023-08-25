use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MultiCliqueError {
    /// Occurs in `attach_policy` when a policy already exists for the given context.
    ContractPolicyExists = 0,

    /// Occurs in `detach_policy` when a policy does not exist for the given context.
    ContractPolicyDoesNotExist = 1,

    /// Occurs in `__check_auth` when a public key in the signed messages is not found among the authorized signers.
    UnknownSigner = 3,

    /// Occurs in `__check_auth` if the number of signers does not meet the default threshold for authorization.
    DefaultThresholdNotMet = 4,

    /// Occurs in `__check_auth` if the number of signers does not meet the threshold defined by a specific policy for authorization.
    PolicyThresholdNotMet = 5,

    /// Occurs in `remove_signer` if an attempt is made to remove a signer that does not exist in the list of authorized signers.
    SignerDoesNotExist = 6,

    /// Occurs in `init` if the contract has already been initialized.
    AlreadyInitialized = 7,
}
