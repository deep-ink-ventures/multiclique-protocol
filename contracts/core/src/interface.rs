use soroban_sdk::{Address, BytesN, Env, Vec};
use soroban_sdk::auth::Context;
use crate::SignedMessage;
use crate::errors::MultiCliqueError;

pub trait MultiCliqueTrait {

    fn init(env: Env, signers: Vec<BytesN<32>>, default_threshold: u32);

    fn add_signers(env: Env, signers: Vec<BytesN<32>>);

    fn remove_signers(env: Env, signers: Vec<BytesN<32>>);

    fn set_default_threshold(env: Env, threshold: u32);

    fn get_default_threshold(env: Env, ) -> u32;

    fn attach_policy(env: Env, policy: Address, context: Vec<Address>);

    fn detach_policy(env: Env, context: Vec<Address>);

    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signatures: Vec<SignedMessage>,
        auth_context: Vec<Context>,
    ) -> Result<(), MultiCliqueError>;

}