// We don't include the standard library to minimize compiled size.
// We also import a few macros and types we need from the `soroban_sdk`.
#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

pub struct HelloContract;

// Our `HelloContract` implementation contains only one function, `hello()`.
// This function will receive a `to` argument, and return a Vec made up of
// "Hello" and the supplied `to` value.
#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}

// This `mod` declaration inserts the contents of `test.rs` into this file.
mod test;