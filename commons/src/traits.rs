use soroban_sdk::{Address, Env, Symbol, Val, Vec, BytesN};

/// The `MultiCliquePolicyTrait` defines the interface for interacting with the MultiClique policy system.
/// It provides methods to get the required signing threshold for executing a function and to run the policy
/// for a given function call. The trait is designed to offer flexibility and adaptability for various policy requirements.
pub trait MultiCliquePolicyTrait {
    /// Determines the required threshold of signers for a specific function call.
    ///
    /// # Parameters
    /// * `env`: The environment that provides access to the contract's storage and other functionalities.
    /// * `num_signers`: The total number of signers in the MultiClique system.
    /// * `signers`: A vector of signers represented by their public keys.
    /// * `address`: The address of the contract that the function belongs to.
    /// * `fn_name`: The symbol representing the function name.
    /// * `args`: A vector of values representing the arguments for the function call.
    ///
    /// # Returns
    /// Returns the required number of signers (threshold) to execute the specified function.
    fn get_threshold(env: Env, num_signers: u32, signers: Vec<BytesN<32>>, address: Address, fn_name: Symbol, args: Vec<Val>) -> u32;

    /// Executes the policy logic for a given function call.
    ///
    /// This method runs the policy for a specific function, taking into account the signers, address, function name, and arguments.
    /// It may include additional checks, validations, or operations based on the policy implementation.
    ///
    /// # Parameters
    /// * `env`: The environment that provides access to the contract's storage and other functionalities.
    /// * `num_signers`: The total number of signers in the MultiClique system.
    /// * `signers`: A vector of signers represented by their public keys.
    /// * `address`: The address of the contract that the function belongs to.
    /// * `fn_name`: The symbol representing the function name.
    /// * `args`: A vector of values representing the arguments for the function call.
    fn run_policy(env: Env, num_signers: u32, signers: Vec<BytesN<32>>, address: Address, fn_name: Symbol, args: Vec<Val>);
}
