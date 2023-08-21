#![cfg(test)]

use ed25519_dalek::Keypair;
use hex::decode;
use soroban_sdk::arbitrary::std::println;
use soroban_sdk::{BytesN, Env, IntoVal, Vec, vec};

use crate::{Contract, ContractClient};

const ALICE_SECRET: &str = "be2161a67ad224bc3fc4237c30d8bf0ddbab03c0bcb9d186096df882e8f9d36cf1c3908c1f23e8b1e086c12a7a1a346f783821fc2dbffabed0cd974ab48eb6c2";
const BOB_SECRET: &str = "2a4a6cf377240d0aad16513dce93b67cd356ca79ef509e80b6e71cbd569d499a8e5b4ee27e0c55a3facaa102c2a2211171a423afbbea89f68f688de5d52b2863";

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
        let signers =  vec![
            &env,
            Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap().public.to_bytes().into_val(&env),
            Keypair::from_bytes(&decode(BOB_SECRET).unwrap()).unwrap().public.to_bytes().into_val(&env),
        ];

        let threshold = 2;
        client.init(&signers, &threshold);

        Protocol { env, client, signers, threshold }
    }
}

#[test]
fn test_default_threshold() {
    let Protocol { client, threshold, .. } = Protocol::new();
    assert_eq!(client.get_default_threshold(), threshold);
}