#![no_std]
use soroban_sdk::auth::Context;
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, Address, BytesN, Env, Vec,
};

mod policy_contract {
    soroban_sdk::contractimport!(file = "../../wasm/multiclique_policy.wasm");
}

use policy_contract::Client as PolicyClient;

mod errors;
mod interface;

use crate::errors::MultiCliqueError;
use crate::interface::MultiCliqueTrait;

#[contracttype]
#[derive(Clone)]
pub struct SignedMessage {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    DefaultThreshold,
    Signer(BytesN<32>),
    SpendLimit(Address),
    Policy(Address),
}

pub const BUMP_A_YEAR: u32 = 6312000;

#[contract]
pub struct Contract;

#[contractimpl]
impl MultiCliqueTrait for Contract {
    fn init(env: Env, signers: Vec<BytesN<32>>, default_threshold: u32) {
        for signer in signers.iter() {
            env.storage().instance().set(&DataKey::Signer(signer), &());
        }
        env.storage()
            .instance()
            .set(&DataKey::DefaultThreshold, &default_threshold);

        env.storage().instance().bump(BUMP_A_YEAR);
    }

    fn add_signer(env: Env, signer: BytesN<32>) {
        env.current_contract_address().require_auth();
        env.storage().instance().set(&DataKey::Signer(signer), &());
    }

    fn remove_signer(env: Env, signer: BytesN<32>) {
        env.current_contract_address().require_auth();
        env.storage().instance().remove(&DataKey::Signer(signer));
    }

    fn set_default_threshold(env: Env, threshold: u32) {
        env.current_contract_address().require_auth();
        env.storage()
            .instance()
            .set(&DataKey::DefaultThreshold, &threshold);
    }

    fn get_default_threshold(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::DefaultThreshold)
            .unwrap_or(0)
    }

    fn attach_policy(env: Env, policy: Address, context: Vec<Address>) {
        env.current_contract_address().require_auth();
        for ctx in context.iter() {
            if env.storage().instance().has(&DataKey::Policy(ctx.clone())) {
                panic_with_error!(&env, MultiCliqueError::ContractPolicyExists);
            }
            env.storage().instance().set(&DataKey::Policy(ctx), &policy);
        }
    }

    fn detach_policy(env: Env, context: Vec<Address>) {
        env.current_contract_address().require_auth();
        for ctx in context.iter() {
            if !env.storage().instance().has(&DataKey::Policy(ctx.clone())) {
                panic_with_error!(&env, MultiCliqueError::ContractPolicyDoesNotExist);
            }
            env.storage().instance().remove(&DataKey::Policy(ctx));
        }
    }

    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signed_messages: Vec<SignedMessage>,
        auth_context: Vec<Context>,
    ) -> Result<(), MultiCliqueError> {
        for i in 0..signed_messages.len() {
            let signature = signed_messages.get_unchecked(i);
            // todo: In CustomAccount there is a prevSig check here, investigate / ask why
            if !env
                .storage()
                .instance()
                .has(&DataKey::Signer(signature.public_key.clone()))
            {
                panic_with_error!(&env, MultiCliqueError::UnknownSigner);
            }
            env.crypto().ed25519_verify(
                &signature.public_key,
                &signature_payload.clone().into(),
                &signature.signature,
            );
        }
        let num_signers = signed_messages.len();

        for ctx in auth_context.iter() {
            match ctx.clone() {
                Context::Contract(contract_ctx) => {
                    match env
                        .storage()
                        .instance()
                        .get(&DataKey::Policy(contract_ctx.clone().contract))
                    {
                        Some(address) => {
                            let policy = PolicyClient::new(&env, &address);
                            let threshold = policy.get_threshold(
                                &contract_ctx.contract,
                                &contract_ctx.fn_name,
                                &contract_ctx.args,
                            );
                            if threshold > num_signers {
                                panic_with_error!(&env, MultiCliqueError::PolicyThresholdNotMet);
                            }
                            policy.run_policy(
                                &contract_ctx.contract,
                                &contract_ctx.fn_name,
                                &contract_ctx.args,
                            );
                        }
                        None => {
                            let default_threshold = env
                                .storage()
                                .instance()
                                .get(&DataKey::DefaultThreshold)
                                .unwrap_or(0);
                            if default_threshold > num_signers {
                                panic_with_error!(&env, MultiCliqueError::DefaultThresholdNotMet);
                            }
                        }
                    };
                }
                // todo: Policy for this?
                Context::CreateContractHostFn(_) => (),
            }
        }
        Ok(())
    }
}
