use soroban_sdk::{Address, Env, testutils::Address as _, Val, vec, Symbol, Vec, BytesN, IntoVal};
use crate::{Contract, ContractClient};
use crate::errors::PolicyError;

struct Protocol {
    env: Env,
    protocol_address: Address,
    client: ContractClient<'static>,

    core_address: Address,
    votes_address: Address,
    asset_address: Address,
    multiclique_address: Address,

    signers: Vec<BytesN<32>>,
    args: Vec<Val>

}

impl Protocol {
    fn new() -> Self {
        let env = Env::default();
        env.budget().reset_unlimited();
        env.mock_all_auths();

        let protocol_address = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &protocol_address);

        let core_address = Address::random(&env);
        let votes_address = Address::random(&env);
        let asset_address = Address::random(&env);
        let multiclique_address = Address::random(&env);
        let signers = vec![&env, Address::random(&env).contract_id(), Address::random(&env).contract_id()];
        let args = vec![&env];


        client.init(&multiclique_address, &core_address, &votes_address, &asset_address);


        Protocol {
            env,
            client,
            protocol_address,
            core_address,
            votes_address,
            asset_address,
            multiclique_address,
            signers,
            args
        }
    }
}

#[test]
fn threshold() {
    let Protocol { env, client, signers, args, core_address, .. } = Protocol::new();
    let num_signers = 1;
    let threshold = client.get_threshold(&num_signers, &signers, &core_address, &Symbol::new(&env,"destroy_dao"), &args);
    assert_eq!(threshold, 1);
}

#[test]
fn core_threshold() {
    let Protocol { env, client, signers, args, core_address, .. } = Protocol::new();

    let num_signers = 10;
    let threshold = client.get_threshold(&num_signers, &signers, &core_address, &Symbol::new(&env,"destroy_dao"), &args);
    assert_eq!(threshold, 8);

    let threshold = client.get_threshold(&num_signers, &signers, &core_address, &Symbol::new(&env,"change_owner"), &args);
    assert_eq!(threshold, 8);

    let threshold = client.get_threshold(&num_signers, &signers, &core_address, &Symbol::new(&env,"something"), &args);
    assert_eq!(threshold, 6);
}

#[test]
fn votes_threshold() {
    let Protocol { env, client, signers, args, votes_address, .. } = Protocol::new();

    let num_signers = 10;
    let threshold = client.get_threshold(&num_signers, &signers, &votes_address, &Symbol::new(&env,"fault_proposal"), &args);
    assert_eq!(threshold, 1);

    let threshold = client.get_threshold(&num_signers, &signers, &votes_address, &Symbol::new(&env,"mark_implemented"), &args);
    assert_eq!(threshold, 5);

    let threshold = client.get_threshold(&num_signers, &signers, &votes_address, &Symbol::new(&env,"something"), &args);
    assert_eq!(threshold, 6);
}

#[test]
fn assets_threshold() {
    let Protocol { env, client, signers, args, asset_address, .. } = Protocol::new();

    let num_signers = 10;
    let threshold = client.get_threshold(&num_signers, &signers, &asset_address, &Symbol::new(&env,"set_owner"), &args);
    assert_eq!(threshold, 8);

    let threshold = client.get_threshold(&num_signers, &signers, &asset_address, &Symbol::new(&env,"set_core_address"), &args);
    assert_eq!(threshold, 8);

    let threshold = client.get_threshold(&num_signers, &signers, &asset_address, &Symbol::new(&env,"something"), &args);
    assert_eq!(threshold, 5);
}

#[test]
fn test_spend_limit() {
    let Protocol { env, client, signers, args, asset_address, multiclique_address, .. } = Protocol::new();

    let num_signers = 10;
    client.set_spend_limit(&asset_address, &1000_i128);
    let args = ((multiclique_address), (), 400_i128).into_val(&env);

    client.run_policy(&num_signers, &signers, &asset_address, &Symbol::new(&env,"xfer"), &args);
    client.run_policy(&num_signers, &signers, &asset_address, &Symbol::new(&env,"incr_allowance"), &args);
    let result = client.try_run_policy(&num_signers, &signers, &asset_address, &Symbol::new(&env,"xfer"), &args);

    assert!(result.is_err());
    let x = 1;
}