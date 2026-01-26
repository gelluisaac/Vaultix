use super::*;
use soroban_sdk::{testutils::{Address as _, MockAuth, MockAuthInvoke}, Address, Env, vec};

#[test]
fn test_circuit_breaker() {
    let env = Env::default();
    let contract_id = env.register(VaultixContract, ());

    // Create admin and users
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);

    // Initialize contract
    let client = VaultixContractClient::new(&env, &contract_id);
    client.init(&admin);

    // Initially not paused
    assert!(!client.is_paused());

    // Create escrow should work when not paused
    client.create_escrow(&1, &seller, &500);

    // Pause the contract
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "set_paused",
            args: vec![],
            sub_invokes: &[],
        },
    }]);
    client.set_paused(&true);
    assert!(client.is_paused());

    // Unpause the contract
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "set_paused",
            args: vec![],
            sub_invokes: &[],
        },
    }]);
    client.set_paused(&false);
    assert!(!client.is_paused());

    // Now create escrow should work
    client.create_escrow(&2, &seller, &500);

    // Confirm delivery should work
    client.confirm_delivery(&1);
}

#[test]
#[should_panic(expected = "Contract is paused")]
fn test_create_escrow_paused() {
    let env = Env::default();
    let contract_id = env.register(VaultixContract, ());

    let admin = Address::generate(&env);
    let seller = Address::generate(&env);

    let client = VaultixContractClient::new(&env, &contract_id);
    client.init(&admin);

    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "set_paused",
            args: vec![],
            sub_invokes: &[],
        },
    }]);
    client.set_paused(&true);

    client.create_escrow(&1, &seller, &500);
}

#[test]
#[should_panic(expected = "Contract is paused")]
fn test_confirm_delivery_paused() {
    let env = Env::default();
    let contract_id = env.register(VaultixContract, ());

    let admin = Address::generate(&env);
    let seller = Address::generate(&env);

    let client = VaultixContractClient::new(&env, &contract_id);
    client.init(&admin);

    client.create_escrow(&1, &seller, &500);
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "set_paused",
            args: vec![],
            sub_invokes: &[],
        },
    }]);
    client.set_paused(&true);

    client.confirm_delivery(&1);
}
