#![no_std]
use error::ContractError;
use soroban_sdk::{bytes, contractimpl, panic_with_error, AccountId, Address, Bytes, Env};

pub struct DataStoreContract;

/// The `DataStoreContract` contains all the functions our contract can run when
/// it is invoked: `put()`, `get()`, and `get_self()`
#[contractimpl]
impl DataStoreContract {
    /// The `put()` function takes a `value` parameter, accepting a Bytes object
    /// for it. This argument can be supplied an array of u8 values, an integer,
    /// or a hex-encoded string.
    pub fn put(env: Env, value: Bytes) -> Result<(), ContractError> {
        // We are using the `panic!` macro to ensure that this function cannot
        // be cross-called from another contract. Only an invoker of the
        // `AccountId` type, which is the identifier of a Stellar account
        // (ed25519 public key), can invoke this function.
        let key = match env.invoker() {
            Address::Account(account_id) => account_id,
            Address::Contract(_) => {
                panic_with_error!(&env, ContractError::CrossContractCallProhibited)
            }
        };

        // We are ensuring the provided Bytes value length is at least 11 since
        // we want users to perform the String to Bytes conversion on their own,
        // without passing simple values like Bytes(7). We also want to
        // highlight some differences between Bytes and symbols (which must be
        // 10 or fewer characters).
        if value.len() <= 10 {
            panic_with_error!(&env, ContractError::InputValueTooShort)
        }

        // We then use `env.data().set()` to store the value that was passed,
        // associating it with the contract invoker's AccountId.
        env.data().set(key, value);

        Ok(()) // return ok if function call succeeded
    }

    /// The `get()` function takes an `owner` parameter, accepting an AccountId
    /// object for it. We then use `env.data().get()` to retrieve the value
    /// which has been associated with the supplied AccountId. If there is no
    /// data associated, return Bytes of length 0.
    pub fn get(env: Env, owner: AccountId) -> Bytes {
        // Hmm. Interesting. This function doesn't enforce an `AccountId` type
        // of invoker. I guess this function *could* be invoked by another
        // contract. I wonder if that will be useful at some point? ;-)
        env.data()
            .get(owner)
            .unwrap_or_else(|| Ok(bytes!(&env))) // This uses `unwrap_or_else` and closure which only evaluates Bytes(0) when necessary.
            .unwrap()
    }

    // !!!
    // TODO Make sure someone implements and uncomments this 👇 function before Q2 goes live or everyone will Nesho it
    // NOTE When you implement the function you'll also need to update the test for `get_self`
    // !!!

    /// The `get_self()` function works similarly to `get()`, except `owner` is
    /// omitted. The AccountId to retrieve associated data for is supplied using
    /// a call to `env.invoker()`. Again we don't allow cross-contract
    /// invokations of this function. If there is no data associated, return
    /// Bytes of length 0.
    pub fn get_self(env: Env) -> Result<Bytes, ContractError> {
        unimplemented!("not implemented")
        //        let key = match env.invoker() {
        //            Address::Account(account_id) => account_id,
        //            Address::Contract(_) => {
        //                panic_with_error!(&env, ContractError::CrossContractCallProhibited)
        //            }
        //        };
        //        Ok(env
        //            .data()
        //            .get(key)
        //            .unwrap_or_else(|| Ok(bytes!(&env)))
        //            .unwrap())
    }
}

mod error;
mod test;
