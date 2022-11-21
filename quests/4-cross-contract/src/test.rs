#![cfg(test)]

// We declare the we are planning to `use` the listed `crates` in our test
// (these have been defined in `lib.rs`)
use crate::{storage_contract, CrossContractCallContract, CrossContractCallContractClient};
use soroban_sdk::{bytes, testutils::Accounts, Env};

#[test]
fn get_cross_call() {
    // Here we register the DataStore contract in a default Soroban environment,
    // and build a client that can be used to invoke the contract.
    let env = Env::default();
    let storage_contract_id = env.register_contract_wasm(None, storage_contract::WASM);
    let storage_contract_client = storage_contract::Client::new(&env, storage_contract_id.clone());

    // Here we register the CrossContractCall contract in a default Soroban
    // environment, and build a client that can be used to invoke the contract.
    let cross_call_contract_id = env.register_contract(None, CrossContractCallContract);
    let cross_call_contract_client =
        CrossContractCallContractClient::new(&env, cross_call_contract_id);

    // We generate a test user, and invoke the `put` function to store some data
    // in the `DataStore` contract.
    let u1 = env.accounts().generate();
    env.set_source_account(&u1);
    storage_contract_client.put(&bytes![&env, 0x48656c6c6f20536f726f62616e21]);

    // We invoke the `inv_get()` function using our `CrossContractCall` client,
    // supplying the `DataStore` contract id and our test user as arguments. We
    // expect it to return the same data stored in the `DataStore` contract.
    assert_eq!(
        cross_call_contract_client.inv_get(&storage_contract_id, &u1),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}
