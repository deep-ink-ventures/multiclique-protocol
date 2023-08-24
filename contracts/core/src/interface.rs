use crate::errors::MultiCliqueError;
use crate::SignedMessage;
use soroban_sdk::auth::Context;
use soroban_sdk::{Address, BytesN, Env, Vec};

/// The `MultiCliqueTrait` defines the core functionalities of the MultiClique protocol.
/// It offers methods for managing signers, setting default thresholds, and working with policies.
pub trait MultiCliqueTrait {
    /// Initializes the MultiClique contract with a set of signers and a default threshold.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    /// - `signers`: A vector of signers' public keys.
    /// - `default_threshold`: The default threshold required for authorization.
    fn init(env: Env, signers: Vec<BytesN<32>>, default_threshold: u32);

    /// Adds a signer to the MultiClique contract.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    /// - `signer`: The public key of the signer to add.
    fn add_signer(env: Env, signer: BytesN<32>);

    /// Removes a signer from the MultiClique contract.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    /// - `signer`: The public key of the signer to remove.
    fn remove_signer(env: Env, signer: BytesN<32>);

    /// Retrieves the signers associated with the MultiClique contract.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    ///
    /// # Returns
    /// A vector of signers' public keys.
    fn get_signers(env: Env) -> Vec<BytesN<32>>;

    /// Sets the default threshold for the MultiClique contract.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    /// - `threshold`: The default threshold to set.
    fn set_default_threshold(env: Env, threshold: u32);

    /// Retrieves the default threshold of the MultiClique contract.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    ///
    /// # Returns
    /// The default threshold value.
    fn get_default_threshold(env: Env) -> u32;

    /// Attaches a policy to a specific context within the MultiClique contract.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    /// - `policy`: The address of the policy to attach.
    /// - `context`: The context addresses to which the policy should be attached.
    fn attach_policy(env: Env, policy: Address, context: Vec<Address>);

    /// Detaches a policy from a specific context within the MultiClique contract.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    /// - `context`: The context addresses from which the policy should be detached.
    fn detach_policy(env: Env, context: Vec<Address>);

    /// Retrieves the policies associated with specific contexts within the MultiClique contract.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    /// - `context`: The context addresses to query.
    ///
    /// # Returns
    /// A vector of policy addresses.
    fn get_policies(env: Env, context: Vec<Address>) -> Vec<Address>;

    /// Internal method for checking the authorization of a transaction within the MultiClique contract.
    ///
    /// # Parameters
    /// - `env`: The execution environment.
    /// - `signature_payload`: The payload to be signed.
    /// - `signatures`: The vector of signed messages.
    /// - `auth_context`: The authorization context information.
    ///
    /// # Returns
    /// A result indicating success or an error.
    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signatures: Vec<SignedMessage>,
        auth_context: Vec<Context>,
    ) -> Result<(), MultiCliqueError>;
}
