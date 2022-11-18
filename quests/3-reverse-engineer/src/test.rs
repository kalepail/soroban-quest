#![cfg(test)]

use super::*;

use soroban_sdk::Env;

/// We only have one test this time around. There is a single function in our
/// contract, and it only takes a single argument. So, we are only testing that
/// function.
#[test]
fn test_q3() {
    // Here we register the ReverseEngineer contract in a default Soroban
    // environment, and build a client that can be used to invoke the contract.
    let env = Env::default();
    let contract_id = env.register_contract(None, ReverseEngineerContract);
    let client = ReverseEngineerContractClient::new(&env, &contract_id);

    // We invoke the ReverseEngineer contract's `submit()` function, providing a
    // value of "wrong" and we expect the contract to return `false`.
    assert_eq!(client.submit(&symbol!("wrong")), false);

    // We invoke the the function this time, with the correct secret word, and
    // we expect the contract to return `true` this time.
    assert_eq!(client.submit(&SECRET), true);
}
