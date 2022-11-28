/// This seems like as good a place as any to quickly point out a handy
/// difference to know about two types we're importing from `soroban_sdk`:
/// `Bytes` and `BytesN`. This won't affect today's quest, just good to know.
/// - `Bytes` is a *growable* array of `u8` values
/// - `BytesN` is a *fixed-size* array of `u8` values
use soroban_sdk::{contracttype, AccountId, Bytes, BytesN};

/**
BEGIN EXAMPLES
Let's start with a couple examples to get you started. Below, we have coded two
custom types: `Cylinder` and `Car`. They are not meant to be used or invoked.
They are simply here as a place to make some sensible documentation.
*/

///  Each and every custom type that you write into a smart contract, must be
///  annotated with the `contracttype` macro. This generates conversions from
///  the created type from/into a `RawVal` Soroban can use.
#[contracttype]
// Here we create a `Cylinder` type as an example. We use the `struct` keyword
// to define the type, which allows us to structure together multiple related
// values. Inside the struct, we create the `fields` that will describe an
// instance of the `Cylinder` type. A `field` is made up of a name (`radius`)
// paired with a type the associated value must conform to (`u32`).
pub struct Cylinder {
    pub radius: u32,
    pub height: u32,
}

/// Our example `Car` type is created using the `enum` keyword. Enums allow you
/// to define a custom type by listing its possible variants. In this case, any
/// instance of `Car` could be manufactured by `Ford`, `Honda`, `Porsche`, etc.
#[contracttype]
pub enum Car {
    Chevrolet,
    Ford,
    Mercedes,
    Porsche,
    Honda,
    Toyota
}

/**
END EXAMPLES
We're going to turn you loose now! Below, each of the custom types need to
be completed before you deploy and invoke your contract. If you have any
questions during this part of the task, you should *really* check out the
"Custom Types" article that's linked in the README.
*/

/// Now, it's your turn! Create your custom `Rectangle` type below. It will be
/// quite similar to the `Cylinder` example: a `struct` with two `u32` fields.
#[contracttype]
pub struct Rectangle {
    // TODO: create your fields here for your `Rectangle` type
}

/// Now, it's your turn! Create your custom `Animal` type below. It will be
/// quite similar to the `Car` example: an `enum` with (at least) 2 variants.
#[contracttype]
pub enum Animal {
    // TODO: create the variants here for your `Animal` type
}

/// The `User` type will be similar in design to the `Rectangle` type above. It
/// is also constructed using a `struct`. Take note the `pet` field requires a
/// different custom type to be nested within the `User` type.
#[contracttype]
pub struct User {
    // TODO: create your fields here for your `User` type
}

/// The `RGB` type will be a `struct` and it must be defined using 3 unnamed
/// `u32` integer values.
#[contracttype]
pub struct RGB(
    // TODO: create your fields here for your `RGB` type
);

/// The `Color` type will be a tuple variant `enum` that contains a single
/// variant with a name of `RGB` and containing an `RGB` value (see above type).
#[contracttype]
pub enum Color {
    // TODO: create the variant here for your `Color` type
}

/// The `Participant` type will be similar in design to the `Animal` type above.
/// It is constructed using an `enum`. It should be constructed as specified in
/// the README.
#[contracttype]
pub enum Participant {
    // TODO: create the variants here for your `Participant` type
}

/// Our `RoyalCard` type will be created using the `enum` keyword. Here we are
/// specifying a custom discriminant value.
#[contracttype]
#[derive(Clone, Copy)]
// The `repr` attribute is here to specify the memory alignment for this type
#[repr(u32)]
pub enum RoyalCard {
    // TODO: create the fields here for your `RoyalCard` type
}
