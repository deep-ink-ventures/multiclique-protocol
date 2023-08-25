use soroban_sdk::{Address, Env, testutils::Address as _, Val, vec, Symbol, Vec, BytesN};
use crate::{Contract, ContractClient};

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
    let mut protocol = Protocol::new();
    let client = &protocol.client;
    let env = &protocol.env;
    let protocol_address = &protocol.protocol_address;
    let core_address = &protocol.core_address;
    let votes_address = &protocol.votes_address;
    let asset_address = &protocol.asset_address;
    let multiclique_address = &protocol.multiclique_address;

    let num_signers = 3;
    let signers = vec![&env, Address::random(&env).contract_id(), Address::random(&env).contract_id()];
    let args: Vec<Val> = vec![&env];
    let threshold = client.get_threshold(&num_signers, &signers, &core_address, &Symbol::new(&env,"destroy_dao"), &args);
    assert_eq!(threshold, 2);

}