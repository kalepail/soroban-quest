#![cfg(test)]

use super::{TypesContract, TypesContractClient};

use super::types::*;

use soroban_sdk::{bytes, testutils::Accounts, Env};

#[test]
fn test_types() {
    // We register the TypesContract contract in a default Soroban environment,
    // and build a client that can be used to invoke the contract.
    let env = Env::default();
    let contract_id = env.register_contract(None, TypesContract);
    let client = TypesContractClient::new(&env, &contract_id);

    // We create a Rectangle `rect` using our custom type and invoke `c_rect`
    let rect = Rectangle {
        width: 5,
        height: 8,
    };
    client.c_rect(&rect);

    // We create an Animal::Cat `cat` (although I'm really more of a `dog`
    // person) using our custom type and invoke `c_animal`
    let cat = Animal::Cat;
    client.c_animal(&cat);

    // We create a User `user` using our custom type and invoke `c_user` (notice
    // how we use the `Animal` custom type again for the `pet` field)
    let user = User {
        name: bytes!(&env, 0x7374656c6c6172),
        age: 8,
        pet: Animal::Dog,
    };
    client.c_user(&user);

    // We create a RGB value `rgb` using our custom type and invoke `c_rgb`
    let rgb = RGB(0, 0, 0);
    client.c_rgb(&rgb);

    // We create a Color `color` using our custom type and invoke `c_color`
    let color: Color = Color::RGB(RGB(0, 0, 0));
    client.c_color(&color);

    // We generate a test user `u1` to use when we create a `Participant` from
    // our custom type, and then we invoke `c_part`
    let u1 = env.accounts().generate();
    let participant = Participant::Account(u1);
    client.c_part(&participant);

    // We also invoke `c_part` using our existing `contract_id`
    let contract_participant = Participant::Contract(contract_id);
    client.c_part(&contract_participant);

    // We create a RoyalCard `jack` using our custom type and invoke `c_card`
    let jack = RoyalCard::Jack;
    client.c_card(&jack);
}
