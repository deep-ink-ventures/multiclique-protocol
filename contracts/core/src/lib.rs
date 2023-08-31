#![no_std]

use commons::traits::MultiCliquePolicyTrait;
use soroban_sdk::auth::Context;
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, Address, BytesN, Env, Symbol, Val, Vec,
};

mod errors;
mod events;
pub mod interface;

#[cfg(test)]
mod test;

use crate::errors::MultiCliqueError;
use crate::events::{
    DefaultThresholdChangedEventData, InitEvent, PolicyAddedEventData, PolicyRemovedEventData,
    SignerAddedEventData, SignerRemovedEventData, ADDED, CHANGED, GOV, INIT, POLICY, REMOVED,
    SIGNER,
};
use crate::interface::MultiCliqueTrait;

/// Declares the SignedMessage structure, containing the public key and signature.
#[contracttype]
#[derive(Clone)]
pub struct SignedMessage {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

/// Enum to represent different keys used in storage for the contract.
#[contracttype]
#[derive(Clone)]
enum DataKey {
    DefaultThreshold,
    Signers,
    SpendLimit(Address),
    Policy(Address),
}

pub const BUMP_A_YEAR: u32 = 6312000;

#[contract]
pub struct Contract;

/// see `MultiCliqueTrait` for documentation
#[contractimpl]
impl MultiCliqueTrait for Contract {
    fn init(env: Env, signers: Vec<BytesN<32>>, default_threshold: u32) {
        if env.storage().instance().has(&DataKey::Signers) {
            panic_with_error!(&env, MultiCliqueError::AlreadyInitialized);
        }

        let valid_thresholds = 0..signers.len() + 1;
        if !valid_thresholds.contains(&default_threshold) {
            panic_with_error!(&env, MultiCliqueError::InvalidThreshold);
        }

        env.storage().instance().set(&DataKey::Signers, &signers);
        env.storage()
            .instance()
            .set(&DataKey::DefaultThreshold, &default_threshold);

        env.storage().instance().bump(BUMP_A_YEAR);

        env.events().publish(
            (GOV, INIT),
            InitEvent {
                threshold: default_threshold,
                signer: signers,
            },
        );
    }

    fn add_signer(env: Env, signer: BytesN<32>) {
        env.current_contract_address().require_auth();
        let mut signers: Vec<BytesN<32>> = env.storage().instance().get(&DataKey::Signers).unwrap();

        if signers.contains(&signer) {
            panic_with_error!(&env, MultiCliqueError::SignerAlreadyAdded);
        }

        signers.push_back(signer.clone());
        env.storage().instance().set(&DataKey::Signers, &signers);
        env.events()
            .publish((SIGNER, ADDED), SignerAddedEventData { signer });
    }

    fn remove_signer(env: Env, signer: BytesN<32>) {
        env.current_contract_address().require_auth();
        let mut signers: Vec<BytesN<32>> = env.storage().instance().get(&DataKey::Signers).unwrap();
        let threshold = env
            .storage()
            .instance()
            .get(&DataKey::DefaultThreshold)
            .unwrap_or(0);

        if signers.len() == threshold {
            panic_with_error!(&env, MultiCliqueError::InvalidThreshold);
        }

        match signers.first_index_of(&signer) {
            None => panic_with_error!(&env, MultiCliqueError::SignerDoesNotExist),
            Some(index) => signers.remove(index),
        };

        env.storage().instance().set(&DataKey::Signers, &signers);
        env.events()
            .publish((SIGNER, REMOVED), SignerRemovedEventData { signer });
    }

    fn get_signers(env: Env) -> Vec<BytesN<32>> {
        env.storage().instance().get(&DataKey::Signers).unwrap()
    }

    fn set_default_threshold(env: Env, threshold: u32) {
        env.current_contract_address().require_auth();

        let signers: Vec<BytesN<32>> = env.storage().instance().get(&DataKey::Signers).unwrap();
        let valid_thresholds = 0..signers.len() + 1;

        if !valid_thresholds.contains(&threshold) {
            panic_with_error!(&env, MultiCliqueError::InvalidThreshold);
        }

        env.storage()
            .instance()
            .set(&DataKey::DefaultThreshold, &threshold);
        env.events().publish(
            (GOV, CHANGED),
            DefaultThresholdChangedEventData { threshold },
        );
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
        env.events()
            .publish((POLICY, ADDED), PolicyAddedEventData { policy, context });
    }

    fn detach_policy(env: Env, context: Vec<Address>) {
        env.current_contract_address().require_auth();
        for ctx in context.iter() {
            if !env.storage().instance().has(&DataKey::Policy(ctx.clone())) {
                panic_with_error!(&env, MultiCliqueError::ContractPolicyDoesNotExist);
            }
            env.storage().instance().remove(&DataKey::Policy(ctx));
        }
        env.events()
            .publish((POLICY, REMOVED), PolicyRemovedEventData { context });
    }

    fn get_policies(env: Env, context: Vec<Address>) -> Vec<Address> {
        let mut policies = Vec::new(&env);
        for ctx in context.iter() {
            if env.storage().instance().has(&DataKey::Policy(ctx.clone())) {
                policies.push_back(
                    env.storage()
                        .instance()
                        .get(&DataKey::Policy(ctx.clone()))
                        .unwrap(),
                );
            }
        }

        return policies;
    }

    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signed_messages: Vec<SignedMessage>,
        auth_context: Vec<Context>,
    ) -> Result<(), MultiCliqueError> {
        let signers: Vec<BytesN<32>> = env.storage().instance().get(&DataKey::Signers).unwrap();
        for i in 0..signed_messages.len() {
            let signature = signed_messages.get_unchecked(i);
            // todo: In CustomAccount there is a prevSig check here, investigate / ask why

            if signers.first_index_of(&signature.public_key).is_none() {
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
                                &num_signers,
                                &signers,
                                &contract_ctx.contract,
                                &contract_ctx.fn_name,
                                &contract_ctx.args,
                            );
                            if threshold > num_signers {
                                panic_with_error!(&env, MultiCliqueError::PolicyThresholdNotMet);
                            }
                            policy.run_policy(
                                &num_signers,
                                &signers,
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

#[contract]
struct Policy;

/// see `MultiCliquePolicyTrait` for documentation
#[contractimpl]
impl MultiCliquePolicyTrait for Policy {
    fn get_threshold(
        _env: Env,
        num_signers: u32,
        _signers: Vec<BytesN<32>>,
        _address: Address,
        _fn_name: Symbol,
        _args: Vec<Val>,
    ) -> u32 {
        num_signers
    }

    fn run_policy(
        _env: Env,
        _num_signers: u32,
        _signers: Vec<BytesN<32>>,
        _address: Address,
        _fn_name: Symbol,
        _args: Vec<Val>,
    ) {
        // do nothing
    }
}
