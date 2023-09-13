#![cfg(test)]

use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;
use hex::decode;

use soroban_sdk::auth::{Context, ContractContext};
use soroban_sdk::testutils::{Address as _, BytesN as _, Events as _};
use soroban_sdk::{vec, Address, BytesN, Env, IntoVal, Symbol, Val, Vec};

use crate::errors::MultiCliqueError;
use crate::{Contract, ContractClient, SignedMessage};

const ALICE_SECRET: &str = "be2161a67ad224bc3fc4237c30d8bf0ddbab03c0bcb9d186096df882e8f9d36cf1c3908c1f23e8b1e086c12a7a1a346f783821fc2dbffabed0cd974ab48eb6c2";
const BOB_SECRET: &str = "2a4a6cf377240d0aad16513dce93b67cd356ca79ef509e80b6e71cbd569d499a8e5b4ee27e0c55a3facaa102c2a2211171a423afbbea89f68f688de5d52b2863";
const EVE_SECRET: &str = "9ecd51618af6af2e1bbf600e5293546809d67f241afd476cc8fbb83c1a964b0b2658f2d0b1cc3a8925e519a834fd45fc366a68a98195262952241f583a695644";

fn sign(e: &Env, signer: &Keypair, payload: &BytesN<32>) -> Val {
    SignedMessage {
        public_key: signer.public.to_bytes().into_val(e),
        signature: signer
            .sign(payload.to_array().as_slice())
            .to_bytes()
            .into_val(e),
    }
    .into_val(e)
}

struct Protocol {
    protocol_address: Address,
    env: Env,
    client: ContractClient<'static>,
    threshold: u32,
    signers: Vec<BytesN<32>>,
}

impl Protocol {
    fn new(threshold: u32) -> Self {
        let env = Env::default();
        env.budget().reset_unlimited();
        env.mock_all_auths();

        let protocol_address = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &protocol_address);
        let signers = vec![
            &env,
            Keypair::from_bytes(&decode(ALICE_SECRET).unwrap())
                .unwrap()
                .public
                .to_bytes()
                .into_val(&env),
            Keypair::from_bytes(&decode(BOB_SECRET).unwrap())
                .unwrap()
                .public
                .to_bytes()
                .into_val(&env),
        ];

        client.init(&signers, &threshold);

        Protocol {
            env,
            client,
            threshold,
            protocol_address,
            signers,
        }
    }
}

#[test]
#[should_panic(expected = "#7")]
fn init_only_once() {
    let Protocol {
        client,
        signers,
        threshold,
        ..
    } = Protocol::new(2);
    client.init(&signers, &threshold);
}

#[test]
fn test_default_threshold_not_met() {
    let protocol = Protocol::new(2);
    let env = protocol.env;
    let alice = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap();

    let payload = BytesN::random(&env);

    let invocation = env.try_invoke_contract_check_auth::<MultiCliqueError>(
        &protocol.protocol_address.contract_id(),
        &payload,
        &vec![&env, sign(&env, &alice, &payload)],
        &vec![
            &env,
            Context::Contract(ContractContext {
                contract: Address::random(&env),
                fn_name: Symbol::new(&env, "transfer"),
                args: ((), (), 100_i128).into_val(&env),
            }),
        ],
    );
    assert_eq!(
        invocation.err().unwrap().unwrap(),
        MultiCliqueError::DefaultThresholdNotMet
    );
}

#[test]
fn test_default_threshold_met_but_wrong_signer() {
    let protocol = Protocol::new(2);
    let env = protocol.env;
    let dave = Address::random(&env);
    let alice = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap();
    let eve = Keypair::from_bytes(&decode(EVE_SECRET).unwrap()).unwrap();

    let payload = BytesN::random(&env);

    let invocation = env.try_invoke_contract_check_auth::<MultiCliqueError>(
        &protocol.protocol_address.contract_id(),
        &payload,
        &vec![
            &env,
            sign(&env, &alice, &payload),
            sign(&env, &eve, &payload),
        ],
        &vec![
            &env,
            Context::Contract(ContractContext {
                contract: Address::random(&env),
                fn_name: Symbol::new(&env, "transfer"),
                args: (protocol.protocol_address, dave, 100_i128).into_val(&env),
            }),
        ],
    );
    assert_eq!(
        invocation.err().unwrap().unwrap(),
        MultiCliqueError::UnknownSigner
    );
}

#[test]
fn test_default_threshold_met() {
    let protocol = Protocol::new(2);
    let env = protocol.env;
    let eve = Address::random(&env);
    let alice = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap();
    let bob = Keypair::from_bytes(&decode(BOB_SECRET).unwrap()).unwrap();

    let payload = BytesN::random(&env);

    let invocation = env.try_invoke_contract_check_auth::<MultiCliqueError>(
        &protocol.protocol_address.contract_id(),
        &payload,
        &vec![
            &env,
            sign(&env, &alice, &payload),
            sign(&env, &bob, &payload),
        ],
        &vec![
            &env,
            Context::Contract(ContractContext {
                contract: Address::random(&env),
                fn_name: Symbol::new(&env, "transfer"),
                args: (protocol.protocol_address.clone(), eve.clone(), 100_i128).into_val(&env),
            }),
        ],
    );
    assert!(invocation.is_ok());
}

#[test]
fn test_default_threshold_set_on_init() {
    let Protocol {
        client,
        threshold,
        env,
        ..
    } = Protocol::new(2);
    assert_eq!(client.get_default_threshold(), threshold);
    assert_eq!(env.events().all().len(), 1);
}

#[test]
fn test_add_signer() {
    let Protocol { client, env, .. } = Protocol::new(2);
    assert_eq!(client.get_signers().len(), 2);
    let pair = Keypair::from_bytes(&decode(EVE_SECRET).unwrap()).unwrap();
    let key = pair.public.to_bytes().into_val(&client.env);
    client.add_signer(&key);
    assert_eq!(client.get_signers().len(), 3);
    assert_eq!(env.events().all().len(), 2);
}

#[test]
fn test_remove_signer() {
    let Protocol { client, env, .. } = Protocol::new(1);
    assert_eq!(client.get_signers().len(), 2);
    let pair = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap();
    let key = pair.public.to_bytes().into_val(&client.env);
    client.remove_signer(&key);
    assert_eq!(client.get_signers().len(), 1);
    assert_eq!(env.events().all().len(), 2);
}

#[test]
#[should_panic(expected = "#6")]
fn test_remove_signer_fails_if_not_exists() {
    let Protocol { client, env, .. } = Protocol::new(1);
    let pair = Keypair::from_bytes(&decode(EVE_SECRET).unwrap()).unwrap();
    let key = pair.public.to_bytes().into_val(&client.env);
    client.remove_signer(&key);
    assert_eq!(env.events().all().len(), 2);
}

#[test]
fn test_attach_policy() {
    let Protocol { client, env, .. } = Protocol::new(2);
    let policy = Address::random(&env);
    let context = vec![&env, Address::random(&env)];
    assert_eq!(client.get_policies(&context).len(), 0);
    client.attach_policy(&policy, &context);
    assert_eq!(client.get_policies(&context).len(), 1);
    assert_eq!(env.events().all().len(), 2);
}

#[test]
#[should_panic(expected = "#0")]
fn test_attach_policy_fails_if_already_exists() {
    let Protocol { client, env, .. } = Protocol::new(2);
    let policy = Address::random(&env);
    let other = Address::random(&env);
    let context = vec![&env, Address::random(&env)];
    client.attach_policy(&policy, &context);
    client.attach_policy(&other, &context);
}

#[test]
fn test_detach_policy() {
    let Protocol { client, env, .. } = Protocol::new(2);
    let policy = Address::random(&env);
    let context = vec![&env, Address::random(&env)];
    assert_eq!(client.get_policies(&context).len(), 0);
    client.attach_policy(&policy, &context);
    assert_eq!(client.get_policies(&context).len(), 1);
    client.detach_policy(&context);
    assert_eq!(client.get_policies(&context).len(), 0);
    assert_eq!(env.events().all().len(), 3);
}

#[test]
#[should_panic(expected = "#1")]
fn test_detach_policy_fails_if_not_exists() {
    let Protocol { client, env, .. } = Protocol::new(2);
    let context = vec![&env, Address::random(&env)];
    client.detach_policy(&context);
}

#[test]
#[should_panic(expected = "#8")]
fn test_invalid_threshold_on_init_fails() {
    Protocol::new(10);
}

#[test]
#[should_panic(expected = "#8")]
fn test_invalid_threshold_on_update_fails() {
    let Protocol { client, .. } = Protocol::new(2);
    client.set_default_threshold(&10);
}

#[test]
#[should_panic(expected = "#2")]
fn test_exceeding_signer_limit_on_update_fails() {
    let Protocol { client, env, .. } = Protocol::new(2);
    let mut csprng = OsRng{};

    for _ in 0..15 {
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let bytes = keypair.public.to_bytes().into_val(&env);
        client.add_signer(&bytes);
    }
}

#[test]
#[should_panic(expected = "#9")]
fn test_signers_cannot_be_added_multiple_times() {
    let Protocol { client, env, .. } = Protocol::new(2);
    let candidate = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap())
        .unwrap()
        .public
        .to_bytes()
        .into_val(&env);
    client.add_signer(&candidate);
}

#[test]
#[should_panic(expected = "#8")]
fn test_signers_cannot_be_removed_if_threshold_not_reduced() {
    let Protocol { client, env, .. } = Protocol::new(2);
    let candidate = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap())
        .unwrap()
        .public
        .to_bytes()
        .into_val(&env);
    client.remove_signer(&candidate);
}
