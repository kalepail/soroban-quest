#![cfg(test)]

use super::*;

use soroban_sdk::{testutils::Accounts, BytesN, Env};

/// These tests are a a lot more interesting and much more involved than the
/// first quest, so let's dive into them a bit deeper. We have two test
/// functions that are testing as if the contract were called by an `AccountId`,
/// or by a `Contract`.

/// The first function, `test_store()`, will test the values that are being
/// stored by our contract. This is accomplished by generating a couple user
/// accounts, storing data as those users, and ensuring retrieved data matches
/// what we would expect it to be. We are also checking against a keypair that
/// hasn't stored any data, ensuring we receive Bytes of length 0 in return.
#[test]
fn test_store() {
    // Here we register the DataStore contract in a default Soroban
    // environment, and build a client that can be used to invoke the contract.
    let env = Env::default();
    let contract_id = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_id);

    // We're generating two test users, `u1` and `u2` that will be the invokers
    // of the contract functions.
    let u1 = env.accounts().generate();
    let u2 = env.accounts().generate();

    // For our `u1` account, we store the `Bytes` represetation of "Hello
    // Soroban!" using the contract's `put()` function. We then use the
    // contracts `get()` function to ensure we receive back the expected value.
    client
        .with_source_account(&u1)
        .put(&bytes!(&env, 0x48656c6c6f20536f726f62616e21)); // hex value for "Hello Soroban!"
    assert_eq!(
        client.get(&u1),
        bytes!(&env, 0x48656c6c6f20536f726f62616e21)
    );

    // Before storing any value as the `u2` account, we check to ensure `get()`
    // returns 0 Bytes (i.e. the account has no data to get).
    assert_eq!(client.get(&u2).len(), 0);

    // Now, as `u2`, we invoke the `put()` function, storing the `Bytes`
    // represetation of "Soroban Quest 2", asserting that `get()` should return
    // the same back to us.
    client
        .with_source_account(&u2)
        .put(&bytes![&env, 0x536f726f62616e2051756573742032]); // hex value for "Soroban Quest 2"
    assert_eq!(
        client.get(&u2),
        bytes![&env, 0x536f726f62616e2051756573742032]
    );

    // Of course, we expect that the data for `u1` has not been overwritten by
    // `u2` invoking the `put()` function.
    assert_eq!(
        client.get(&u1),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_store_value_too_short() {
    // Here we register the DataStore contract in a default Soroban
    // environment, and build a client that can be used to invoke the contract.
    let env = Env::default();
    let contract_id = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_id);

    // We're generating a single test user, `u1`, which will be the invoker of
    // the contract's `put()` function.
    let u1 = env.accounts().generate();

    // For our `u1` account, we attempt to store `Bytes(0, 7)` using the
    // contract's `put()` function. We stop there, since we're just expecting
    // the contract to panic with the argument that's too short.
    client.with_source_account(&u1).put(&bytes![&env, 0x007]);
}

/// For the next few tests, we are going to test our DataStore contract's
/// behavior when it is invoked from another contract. So, we are creating a
/// very simple Smart Contract here, that we can use in them. It's quite simple,
/// and only exists as a client to invoke the main contract's `put()`, `get()`,
/// and `get_self()` functions.
pub struct CallerContract;

#[contractimpl]
impl CallerContract {
    // This function passes our supplied `data` argument to the DataStore
    // contract's `put()` function. This is one of two panic responses we are
    // testing for.
    pub fn try_put(env: Env, contract_id: BytesN<32>, data: Bytes) {
        let cli = DataStoreContractClient::new(&env, contract_id);
        cli.put(&data);
    }

    // This function invokes the `get_self()` function from the DataStore
    // contract. This is the second panic responses we are testing for.
    pub fn try_get_s(env: Env, contract_id: BytesN<32>) -> Bytes {
        let cli = DataStoreContractClient::new(&env, contract_id);
        cli.get_self()
    }

    // This function invokes the `get()` function from the DataStore contract,
    // passing along an `owner` argument containing an AccountId.
    pub fn try_get(env: Env, contract_id: BytesN<32>, owner: AccountId) -> Bytes {
        let cli = DataStoreContractClient::new(&env, contract_id);
        cli.get(&owner)
    }
}

/// This test tries to invoke the `put()` method of the DataStore contract, as
/// another smart contract. This is expected to fail since that method is only
/// available to an `Account`, and not a `Contract`.
#[test]
#[should_panic(expected = "Status(ContractError(1))")] // We want this test to panic since it uses a forbidden function.
fn test_contract_store() {
    // Similar to all Soroban tests, we create an environment, and register the
    // DataStore contract in it.
    let env = Env::default();
    let contract_id_data_store = env.register_contract(None, DataStoreContract);

    // We take an extra step to register our Caller contract in the environment,
    // so we can test the cross-contract calls, using its client.
    let contract_id_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, contract_id_caller);

    // We are invoking the the DataStore contract's `put()` function using our
    // Caller contract's `try_put()` function. We expect this to panic.
    caller_client.try_put(
        &contract_id_data_store,
        &bytes![&env, 0x48656c6c6f20536f726f62616e21],
    );
}

/// This test tries to invoke the `get_self()` method of the DataStore contract,
/// as another smart contract. This is expected to fail since that method is
/// only available to an `Account`, and not a `Contract`.
#[test]
#[should_panic(expected = "Status(ContractError(1))")] // We want this test to panic since it uses a forbidden function.
fn test_contract_get_self() {
    // We create an environment, and register the DataStore contract in it.
    let env = Env::default();
    let contract_id_data_store = env.register_contract(None, DataStoreContract);

    // We take an extra step to register our Caller contract in the environment,
    // so we can test the cross-contract calls, using its client.
    let contract_id_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, contract_id_caller);

    // We are invoking the the DataStore contract's `get_self()` function using
    // our Caller contract's `try_get_s() function. We expect this to panic.
    caller_client.try_get_s(&contract_id_data_store);
}

/// This test tries to invoke the `get()` method of the DataStore contract, as
/// another smart contract. This is NOT expected to panic since `get()` is
/// exposed to being invoked from a Contract.
#[test]
fn test_contract_get() {
    // We create an environment, and register the DataStore contract in it. We
    // are also creating a client for this contract, so we can invoke the
    // `get()` function and expect some real data back (not Bytes(0)).
    let env = Env::default();
    let contract_id_data_store = env.register_contract(None, DataStoreContract);
    let client_data_store = DataStoreContractClient::new(&env, &contract_id_data_store);

    // We take an extra step to register our Caller contract in the environment,
    // so we can test the cross-contract calls, using its client.
    let contract_id_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, contract_id_caller);

    // We create an Account, `u1`, so we can invoke the `put()` function, and
    // test against the value we store, when called from our contract later.
    let u1 = env.accounts().generate();
    client_data_store
        .with_source_account(&u1)
        .put(&bytes!(&env, 0x48656c6c6f20536f726f62616e21));

    // We are invoking the the DataStore contract's `get()` function by using
    // the `try_get()` method from the Caller contract. We don't expect this to
    // panic, since the `get()` function can be invoked by a Contract. We do,
    // however, expect our returned value to match the value we `put` before.
    let value = caller_client.try_get(&contract_id_data_store, &u1);
    assert_eq!(value, bytes!(&env, 0x48656c6c6f20536f726f62616e21));
}
