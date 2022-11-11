#![cfg(test)]

use super::*;
use soroban_sdk::{symbol, vec, Env};

// The purpose of this file is to run automated tests on the contract code we've
// written in `lib.rs`. Writing tests can be quite a big topic, and we'll dive
// in further in a future quest. Just you wait!
#[test]
fn test() {
    // we register the contract in a Soroban environment, and build a client we
    // can use to invoke the contract
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    // Next, we call `client.hello()`, supplying "Dev" as our `to` argument.
    let words = client.hello(&symbol!("Dev"));
    
    // We assert the contract must return a Vec that matches what we would
    // expect to receive from our contract: [Symbol("Hello"), Symbol("Dev")]
    assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
}
