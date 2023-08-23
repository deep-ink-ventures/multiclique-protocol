#![cfg(test)]

use ed25519_dalek::{Keypair, Signer};
use hex::decode;

use soroban_sdk::testutils::{Address as _, BytesN as _, Events as _};
use soroban_sdk::{vec, Address, BytesN, Env, IntoVal, Vec, Symbol, Val, token, contract, contractimpl};
use soroban_sdk::auth::{Context, ContractContext};
use crate::policy_contract::{Client as PolicyClient, WASM as PolicyWASM};

use crate::{Contract, ContractClient, SignedMessage};
use crate::errors::MultiCliqueError;

const ALICE_SECRET: &str = "be2161a67ad224bc3fc4237c30d8bf0ddbab03c0bcb9d186096df882e8f9d36cf1c3908c1f23e8b1e086c12a7a1a346f783821fc2dbffabed0cd974ab48eb6c2";
const BOB_SECRET: &str = "2a4a6cf377240d0aad16513dce93b67cd356ca79ef509e80b6e71cbd569d499a8e5b4ee27e0c55a3facaa102c2a2211171a423afbbea89f68f688de5d52b2863";
const EVE_SECRET: &str = "9ecd51618af6af2e1bbf600e5293546809d67f241afd476cc8fbb83c1a964b0b2658f2d0b1cc3a8925e519a834fd45fc366a68a98195262952241f583a695644";

fn sign(e: &Env, signer: &Keypair, payload: &BytesN<32>) -> Val {
    SignedMessage {
        public_key: signer.public.to_bytes().into_val(e),
        signature: signer.sign(payload.to_array().as_slice()).to_bytes().into_val(e)
    }
    .into_val(e)
}

struct Protocol {
    native_address: Address,
    protocol_address: Address,
    env: Env,
    client: ContractClient<'static>,
    signers: Vec<BytesN<32>>,
    token_client: token::Client<'static>,
    threshold: u32,
}

impl Protocol {
    fn new() -> Self {
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

        let threshold = 2;
        client.init(&signers, &threshold);

        let native_address = env.register_stellar_asset_contract(Address::random(&env));
        let native_asset_admin = token::AdminClient::new(&env, &native_address);

        native_asset_admin.mint(&protocol_address, &1000_i128);
        let token_client = token::Client::new(&env, &native_address);

        Protocol {
            env,
            client,
            signers,
            threshold,
            protocol_address,
            native_address,
            token_client
        }
    }

    fn create_policy(&self) -> PolicyClient {
        let address = self.env.register_contract_wasm(None, PolicyWASM);
        self.client.attach_policy(&address, &vec![&self.env, self.native_address.clone()]);
        PolicyClient::new(&self.env, &address)

    }
}

#[test]
fn test_default_threshold_not_met() {
    let protocol = Protocol::new();
    let env = protocol.env;
    let eve = Address::random(&env);
    let alice = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap();

    let payload = BytesN::random(&env);

    let invocation = env.try_invoke_contract_check_auth::<MultiCliqueError>(
        &protocol.protocol_address.contract_id(),
        &payload,
        &vec![&env, sign(&env, &alice, &payload)],
        &vec![&env, Context::Contract(ContractContext {
            contract: protocol.native_address.clone(),
            fn_name: Symbol::new(&env, "transfer"),
            args: ((), (), 100_i128).into_val(&env),
        })]
    );
    assert_eq!(invocation.err().unwrap().unwrap(), MultiCliqueError::DefaultThresholdNotMet);
}

#[test]
fn test_default_threshold_met_but_wrong_signer() {
    let protocol = Protocol::new();
    let env = protocol.env;
    let dave = Address::random(&env);
    let alice = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap();
    let eve = Keypair::from_bytes(&decode(EVE_SECRET).unwrap()).unwrap();

    let payload = BytesN::random(&env);

    let invocation = env.try_invoke_contract_check_auth::<MultiCliqueError>(
        &protocol.protocol_address.contract_id(),
        &payload,
        &vec![&env, sign(&env, &alice, &payload), sign(&env, &eve, &payload)],
        &vec![&env, Context::Contract(ContractContext {
            contract: protocol.native_address.clone(),
            fn_name: Symbol::new(&env, "transfer"),
            args: (protocol.protocol_address, dave, 100_i128).into_val(&env),
        })]
    );
    assert_eq!(invocation.err().unwrap().unwrap(), MultiCliqueError::UnknownSigner);

}




#[test]
fn test_default_threshold_met() {
    let protocol = Protocol::new();
    let env = protocol.env;
    let eve = Address::random(&env);
    let alice = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap();
    let bob = Keypair::from_bytes(&decode(BOB_SECRET).unwrap()).unwrap();

    let payload = BytesN::random(&env);

    let invocation = env.try_invoke_contract_check_auth::<MultiCliqueError>(
        &protocol.protocol_address.contract_id(),
        &payload,
        &vec![&env, sign(&env, &alice, &payload), sign(&env, &bob, &payload)],
        &vec![&env, Context::Contract(ContractContext {
            contract: protocol.native_address.clone(),
            fn_name: Symbol::new(&env, "transfer"),
            args: (protocol.protocol_address.clone(), eve.clone(), 100_i128).into_val(&env),
        })]
    );
    assert!(invocation.is_ok());
}


#[test]
fn test_default_threshold_set_on_init() {
    let Protocol {
        client, threshold, ..
    } = Protocol::new();
    assert_eq!(client.get_default_threshold(), threshold);
}

#[test]
fn test_add_signer() {
    let Protocol { client, env, .. } = Protocol::new();
    assert_eq!(client.get_signers().len(), 2);
    let pair = Keypair::from_bytes(&decode(EVE_SECRET).unwrap()).unwrap();
    let key = pair.public.to_bytes().into_val(&client.env);
    client.add_signer(&key);
    assert_eq!(client.get_signers().len(), 3);
    assert_eq!(env.events().all().len(), 1);
}

#[test]
fn test_remove_signer() {
    let Protocol { client, env, .. } = Protocol::new();
    assert_eq!(client.get_signers().len(), 2);
    let pair = Keypair::from_bytes(&decode(ALICE_SECRET).unwrap()).unwrap();
    let key = pair.public.to_bytes().into_val(&client.env);
    client.remove_signer(&key);
    assert_eq!(client.get_signers().len(), 1);
    assert_eq!(env.events().all().len(), 1);
}

#[test]
#[should_panic(expected = "#6")]
fn test_remove_signer_fails_if_not_exists() {
    let Protocol { client, env, .. } = Protocol::new();
    let pair = Keypair::from_bytes(&decode(EVE_SECRET).unwrap()).unwrap();
    let key = pair.public.to_bytes().into_val(&client.env);
    client.remove_signer(&key);
    assert_eq!(env.events().all().len(), 1);
}

#[test]
fn test_attach_policy() {
    let Protocol { client, env, .. } = Protocol::new();
    let policy = Address::random(&env);
    let context = vec![&env, Address::random(&env)];
    assert_eq!(client.get_policies(&context).len(), 0);
    client.attach_policy(&policy, &context);
    assert_eq!(client.get_policies(&context).len(), 1);
    assert_eq!(env.events().all().len(), 1);
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
    assert_eq!(env.events().all().len(), 2);
}

#[test]
#[should_panic(expected = "#1")]
fn test_detach_policy_fails_if_not_exists() {
    let Protocol { client, env, .. } = Protocol::new();
    let context = vec![&env, Address::random(&env)];
    client.detach_policy(&context);
}
