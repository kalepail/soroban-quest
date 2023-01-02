# Quest 5 - Custom Types <!-- omit in toc -->

## TL;DR

Ok, fine! I guess you *maybe* kinda sorta have what it takes to get started with
just a short prompt. Fine! Be my guest! But don't forget: Knowledge abounds
further down in the README!

**ALSO**: We're going to flip things around on you today! You've gotten pretty
good at the deploy and invoke steps of the Soroban process (and, yes, you'll
have to do those today, as well). But, how are your Rust chops?? That's right,
today you're going to be *writing* some actual Rust in your contract!

**For today's quest, you must create a set of custom types in your contract that
conform to the specifications listed below. You must also deploy that contract,
and then make the necessary invocations for each of the custom types. Finally,
you must invoke the `verify` function of our verification contract
`422d1c0091a0528e3630fb5301389da530c40be4ce2eb8fcadf426a3d53627d3` so we can
double-check your custom type definitions.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [The Task at Hand](#the-task-at-hand)
  - [Custom Types in Rust](#custom-types-in-rust)
    - [Rust `enum`s](#rust-enums)
    - [Rust `struct`s](#rust-structs)
  - [Custom Types in Soroban](#custom-types-in-soroban)
  - [Create Your Custom Types](#create-your-custom-types)
    - [Rectangle](#rectangle)
    - [Animal](#animal)
    - [User](#user)
    - [RGB](#rgb)
    - [Color](#color)
    - [Participant](#participant)
    - [RoyalCard](#royalcard)
  - [Invoke Your Contract](#invoke-your-contract)
  - [Let Us Check Your Work](#let-us-check-your-work)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

If you missed out on our previous quests, or you just need a refresher, we have
some (pretty extensive) instructions for the *mechanics* of completing these
quests (generating keypairs, checking your work, etc.).

All that information [can be found here][how-to-play] if you need to use those
instructions again.

## The Task at Hand

This is a big moment! You feel prepared. You feel ready. Quite frankly, you
*are* ready! You've got the gumption and the tenacity to tackle this quest!

But, again, please: **Read the code!!** There is important stuff that you need
to know inside of there. (Plus, we worked really hard on it, and you should
totally use it to the fullest extent!)

*Bonus*: You've heard us say you should "read `lib.rs`" like a hundred times by
now. But today there's a fancy new `types.rs` file you should take a gander at.

### Custom Types in Rust

A custom type in the [Soroban Rust dialect][soroban-dialect] is declared using
the `contracttype` attribute macro on either a `struct` or `enum` definition.
But, before we dive into the Soroban-specific information, let's camp out with
how this concept plays out in a standard Rust environment. Don't worry: this'll
be quick.

#### Rust `enum`s

In Rust, we can use an `enum` (short for `enumeration`) to define a type by
listing (or "enumerating") all of its possible variants. You could think of a
defined `enum` as a "menu" from which you choose one item. You won't necessarily
choose the same thing every time, but you'll choose only one when it is time to
make a selection. For example:

```rust
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    Skittles, // an important part of a balanced diet
}
```

The *Rust Book* contains a [whole chapter on `enum`s][rust-enums], and it has a
lot more very valuable information that you can learn. Be sure to check it out!

#### Rust `struct`s

In Rust, a `struct` allows us to gather together and name multiple values that
are related in some way. Such a `struct` will be a custom data type that
represents a meaningful group of those values. Rather than a menu from which you
choose, a `struct` is more like that "template" document you save so you can
just make a copy and fill out a few things when the time comes to start on your
homework. For example:

```rust
struct Homework {
    class: String, // remember soroban does not include `String` - it's only here as an example
    subject: String,
    studentId: u32,
    complete: bool,
    date: String,
}
```

We could then create a new *instance* of a `Homework` assignment by doing
something like this:

```rust
let homework1 = Homework {
    class: String::from("AA429"),
    subject: String::from("Advanced Astrophotography"),
    studentId: 8675309,
    complete: false,
    date: String::from("Next Wednesday"),
};
```

The *Rust Book* also has a [whole chapter on `struct`s][rust-struct], and it has
even more information! Check this one out, too!

### Custom Types in Soroban

If you choose to click only one link in this entire README, please make it this
one: The **[Custom Types][learn-ct]** article in the Soroban documentation is
just truly *very* good. The custom types you can create on Soroban are made up
of `struct` types and `enum` types, though there are a few different conventions
used to define those types. The broad categories of custom types you can create
are:

- [Struct with Named Fields](https://soroban.stellar.org/docs/learn/custom-types#structs-with-named-fields)
- [Struct with Unnamed Fields](https://soroban.stellar.org/docs/learn/custom-types#structs-with-unamed-fields)
- [Enum with Unit and Tuple Variants](https://soroban.stellar.org/docs/learn/custom-types#enum-unit-and-tuple-variants)
- [Enum with Integer Variants](https://soroban.stellar.org/docs/learn/custom-types#enum-integer-variants)

It's also important to understand that enums are currently supported as contract
types in Soroban only when all variants have an explicit integer literal, **or**
when all variants are unit or single field variants.

In the **[Custom Types][learn-ct]** article you'll even learn quite a bit about
how Soroban will store your custom types on the Ledger, XDR conversion, JSON
representation, and more!

An additional (and very useful) resource in the Soroban documentation can be
found here: [Error Enums][error-enums] describes how you might use an `enum` to
meaningfully convey error information. That might seem vaguely familiar, if you
remember having to frantically figure out what error you were receiving (and
why) during [Quest 2](../2-auth-store/README.md).

### Create Your Custom Types

Ok, that was some **great** educational content, but we're back on track! For
this quest, you must create and then invoke the following custom types in your
contract:

#### Rectangle

The `Rectangle` type must be a `struct`, with two fields: `width` and `height`
which both must be a `u32` value.

Invoke the `c_rect` function to create a `Rectangle` using something like:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_rect \
    --arg '{"object":{"map":[{"key":{"symbol":"width"},"val":{"u32":<a-u32-integer>}},{"key":{"symbol":"height"},"val":{"u32":<a-u32-integer>}}]}}'
```

#### Animal

The `Animal` type must be an `enum`, with at least two variations: `Cat` and
`Dog`.

Invoke the `c_animal` function to create an `Animal` using something like:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_animal \
    --arg '{"object":{"vec":[{"symbol":"<a-relevant-animal-symbol>"}]}}'
```

#### User

The `User` type must be a `struct` with `name`, `age`, and `pet` fields,
corresponding to `Bytes`, `u32`, and `Animal` values, respectively.

Invoke the `c_user` function to create a `User` using something like:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_user \
    --arg '{"object":{"map":[{"key":{"symbol":"name"},"val":{"object":{"bytes":"<a-hex-encoded-string>"}}},{"key":{"symbol":"age"},"val":{"u32":<a-u32-integer>}},{"key":{"symbol":"pet"},"val":<an-animal-object>}]}}'
```

*Note*: You will need to use some JSON for the `Animal` field of the user. You
could even copy the argument you used when invoking `c_animal`.

#### RGB

The `RGB` type must be a tuple `struct` type made with a tuple of 3 `u32`
values.

Invoke the `c_rgb` function to create a `RGB` value using something like:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_rgb \
    --arg '{"object":{"vec":[{"u32":<a-u32-integer>},{"u32":<a-u32-integer>},{"u32":<a-u32-integer>}]}}'
```

#### Color

The `Color` type will combine the `RGB` custom type nested within a tuple `enum`
type. Construct your `RGB` struct type as described bove, Then, your `Color`
enum type must be defined as a variant with a name of "RGB" and an instance of
your `RGB` type.

Invoke the `c_color` function to create a `Color` using something like:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_color \
    --arg '{"object":{"vec":[{"symbol":"RGB"},<a-rgb-object>]}}'
```

#### Participant

The `Participant` type must be an `enum` with single-value tuple variants as
follows:

- An "Account" variant with an `AccountId` type
- A "Contract" variant with a `BytesN<32>` type

Invoke the `c_part` function to create an account `Participant` using something
like:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_part \
    --arg '{"object":{"vec":[{"symbol":"Account"},{"object":{"accountId":{"publicKeyTypeEd25519":"<hex-encoded-account-id>"}}}]}}'
```

Also invoke the `c_part` function to create a contract `Participant` using
something like:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_part \
    --arg '{"object":{"vec":[{"symbol":"Contract"},{"object":{"bytes":"<contract-id>"}}]}}'
```

#### RoyalCard

The `RoyalCard` type must be an `enum` containing three `u32` integer variations
as follows:

- A "Jack" variant, with a value of 11
- A "Queen" variant, with a value of 12
- A "King" variant, with a value of 13

Invoke the `c_card` function using something like:

```bash
soroban invoke \
    --id <contract-id> \
    --fn c_card \
    --arg '{"u32":<a-u32-integer>}'
```

### Invoke Your Contract

That was a lot of work, wasn't it! You should be really proud of yourself. I
know I am. Now that you have all your custom types written and deployed (oh
yeah, don't forget to deploy your contract!), you need to *invoke* each of the
functions listed in `src/lib.rs` and pass a valid `--arg` for your custom type.
There's some helpful hints on how to invoke these throughout this README.

In case you lost track, you must invoke the following functions providing an
argument of the custom type you created:

| Function | Argument Type |
| --- | --- |
| c_rect | [Rectangle](#rectangle) |
| c_animal | [Animal](#animal) |
| c_user | [User](#user) |
| c_rgb | [RGB](#rgb) |
| c_color | [Color](#color) |
| c_part | [Participant](#participant)* |
| c_card | [RoyalCard](#royalcard) |

> \* Don't forget the `Participant` type must be invoked twice, once as an
> `Account`, and once as a `Contract`.

### Let Us Check Your Work

Well done, you've customized all the types, you've invoked all the things, and
you're ready to claim your prize! Before we get on with it, just *one* more
thing: **You need to invoke our verification contract.**

Using your Quest Keypair, you must invoke the `verify` function on the contract
with the ID `40d12b03a08f5dde4e0068aa752fa65eddf905e82a18f522efe350e0cd268b8a`,
supplying your own contract ID as an argument. We'll double-check all your hard
work, and make sure you've implemented the required custom types with the
necessary fields, variants, values, etc.

**Then**, you are free to use `sq check 5` to (try and) claim your prize!

## Further Reading

- Again, just trust me and **read this**: The [Custom Types][learn-ct] article
  in the Learn section of the Soroban docs could *not* be more useful!
- You can look in the [SDK Docs][sdk-contracttype] to learn more about the
  `contracttype` attribute macro.
- There is an entire [Custom Types example contract][example-ct] you can look at
  and read through in the Soroban docs. It's great for inspiration, or to see
  how all these pieces can fit together.
- Read more about the [Contract Dialect][soroban-dialect] of Rust used in
  Soroban in the documentation.
- I know we mentioned it a few times last week, but seriously! Trust us when we
  tell you [Smephite's Guide][smephite-guide] is incredibly helpful!

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[sdk-contracttype]: https://docs.rs/soroban-sdk/latest/soroban_sdk/attr.contracttype.html
[learn-ct]: https://soroban.stellar.org/docs/learn/custom-types
[example-ct]: https://soroban.stellar.org/docs/examples/custom-types
[rust-struct]: https://doc.rust-lang.org/book/ch05-00-structs.html
[rust-enums]: https://doc.rust-lang.org/book/ch06-00-enums.html
[error-enums]: https://soroban.stellar.org/docs/learn/errors#error-enums
[soroban-dialect]: https://soroban.stellar.org/docs/learn/rust-dialect
[smephite-guide]: https://gist.github.com/Smephite/09b40e842ef454effe4693e0d18246d7
