#![cfg(test)]

use ed25519_dalek::Keypair;
use hex::decode;

use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, BytesN, Env, IntoVal, Vec};

use crate::{Contract, ContractClient};

const ALICE_SECRET: &str = "be2161a67ad224bc3fc4237c30d8bf0ddbab03c0bcb9d186096df882e8f9d36cf1c3908c1f23e8b1e086c12a7a1a346f783821fc2dbffabed0cd974ab48eb6c2";
const BOB_SECRET: &str = "2a4a6cf377240d0aad16513dce93b67cd356ca79ef509e80b6e71cbd569d499a8e5b4ee27e0c55a3facaa102c2a2211171a423afbbea89f68f688de5d52b2863";
const EVE_SECRET: &str = "9ecd51618af6af2e1bbf600e5293546809d67f241afd476cc8fbb83c1a964b0b2658f2d0b1cc3a8925e519a834fd45fc366a68a98195262952241f583a695644";

struct Protocol {
    env: Env,
    client: ContractClient<'static>,
    signers: Vec<BytesN<32>>,
    threshold: u32,
}

impl Protocol {
    fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        env.budget().reset_unlimited();

        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);
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

        let threshold = 2;
        client.init(&signers, &threshold);

        Protocol {
            env,
            client,
            signers,
            threshold,
        }
    }
}

#[test]
fn test_default_threshold() {
    let Protocol {
        client, threshold, ..
    } = Protocol::new();
    assert_eq!(client.get_default_threshold(), threshold);
}

#[test]
fn test_add_signer() {
    let Protocol { client, .. } = Protocol::new();
    assert_eq!(client.get_signers().len(), 2);
    let pair = Keypair::from_bytes(&decode(EVE_SECRET).unwrap()).unwrap();
    let key = pair.public.to_bytes().into_val(&client.env);
    client.add_signer(&key);
    assert_eq!(client.get_signers().len(), 3);
}

#[test]
fn test_remove_signer() {
    let Protocol { client, .. } = Protocol::new();
    assert_eq!(client.get_signers().len(), 2);
    let pair = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap();
    let key = pair.public.to_bytes().into_val(&client.env);
    client.remove_signer(&key);
    assert_eq!(client.get_signers().len(), 1);
}

#[test]
#[should_panic(expected = "#6")]
fn test_remove_signer_fails_if_not_exists() {
    let Protocol { client, .. } = Protocol::new();
    let pair = Keypair::from_bytes(&decode(EVE_SECRET).unwrap()).unwrap();
    let key = pair.public.to_bytes().into_val(&client.env);
    client.remove_signer(&key);
}

#[test]
fn test_attach_policy() {
    let Protocol { client, env, .. } = Protocol::new();
    let policy = Address::random(&env);
    let context = vec![&env, Address::random(&env)];
    assert_eq!(client.get_policies(&context).len(), 0);
    client.attach_policy(&policy, &context);
    assert_eq!(client.get_policies(&context).len(), 1);
}

#[test]
#[should_panic(expected = "#0")]
fn test_attach_policy_fails_if_already_exists() {
    let Protocol { client, env, .. } = Protocol::new();
    let policy = Address::random(&env);
    let other = Address::random(&env);
    let context = vec![&env, Address::random(&env)];
    client.attach_policy(&policy, &context);
    client.attach_policy(&other, &context);
}

#[test]
fn test_detach_policy() {
    let Protocol { client, env, .. } = Protocol::new();
    let policy = Address::random(&env);
    let context = vec![&env, Address::random(&env)];
    assert_eq!(client.get_policies(&context).len(), 0);
    client.attach_policy(&policy, &context);
    assert_eq!(client.get_policies(&context).len(), 1);
    client.detach_policy(&context);
    assert_eq!(client.get_policies(&context).len(), 0);
}

#[test]
#[should_panic(expected = "#1")]
fn test_detach_policy_fails_it_not_exists() {
    let Protocol { client, env, .. } = Protocol::new();
    let context = vec![&env, Address::random(&env)];
    client.detach_policy(&context);
}
