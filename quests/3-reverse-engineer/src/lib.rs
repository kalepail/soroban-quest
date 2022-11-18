#![no_std]
use soroban_sdk::{contractimpl, Symbol, symbol, Env};

pub struct ReverseEngineerContract;

/// A constant is an *always immutable* value that is valid for the entire time
/// the program runs. When declaring a `const` in Rust, you must explicitly type
/// them at that time. Here the `SECRET` constant is typed using `Symbol` and we
/// define what that secret is.
const SECRET : Symbol = symbol!("dancinRaph");

/// The `ReverseEngineerContract` contains only one function: `submit()`.
#[contractimpl]
impl ReverseEngineerContract {
    // The `submit()` function only takes a single argument, `secret` and
    // returns either `true` or `false`, letting you know whether you submitted
    // the correct secret.
    pub fn submit(_: Env, secret: Symbol) -> bool{
        secret == SECRET
    }
}

mod test;