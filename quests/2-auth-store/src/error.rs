use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // You need to be a user to invoke this function, Cross Contract calls are only allowed to call the `get` function
    CrossContractCallProhibited = 1,
    // Input Value too short, provide at least 11 characters (bytes)
    InputValueTooShort = 2,
}
