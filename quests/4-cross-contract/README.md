# Quest 4 - Cross Contract <!-- omit in toc -->

## TL;DR

You again!? You're back here looking for the quick task? Well, alright, if you
think you're really ready for it. Good luck!

**For this quest, you'll use your Quest Account to deploy the `DataStore`
contract from Quest 2, invoking the `put` function in it to store some data on
chain. You must also use your Quest Account to deploy this Quest's
`CrossContractCall` contract, and use it to make a cross-contract invocation of
the `get` function from your `DataStore` contract.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [The Task at Hand](#the-task-at-hand)
  - [Making an On-Chain Oracle](#making-an-on-chain-oracle)
  - [Importing Contracts](#importing-contracts)
    - [Caveat on Contract Compilation Order](#caveat-on-contract-compilation-order)
  - [Using a Contract Client](#using-a-contract-client)
  - [Passing Arguments to Soroban CLI](#passing-arguments-to-soroban-cli)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

If you missed out on our previous quests, or you just need a refresher, we have
some (pretty extensive) instructions for the *mechanics* of completing these
quests (generating keypairs, checking your work, etc.).

All that information [can be found here][how-to-play] if you need to use those
instructions again.

## The Task at Hand

We know you're excited, and ready to get questing! But, again, please please
please: **Read the code!!** There is important stuff that you need to know
inside of there.

Now, let's talk theory:

### Making an On-Chain Oracle

A blockchain "oracle" might seem like one of those buzz-words that *sounds* like
something cool, but you're not really sure what it's supposed to mean. You could
think of an oracle as a window into the "outside world" from within a
blockchain. An oracle brings in outside data for use in the network. You could
make one that pulls in all kinds of data! Maybe it's:

- weather data from around the world,
- the current speedrunning records for Super Mario,
- football scores and rankings leading up to the World Cup,
- the price of Bitcoin against another asset,
- you get the idea... it could be pretty much anything!

An oracle could also exist a bit like an on-chain database. Maybe you populate
it with the public addresses of your closest friends (or your enemies). Or, you
could store your recipes for delicious guacamole. That data is then available
for use in other smart contracts, where you might need to use that data for
various (nefarious?) purposes. Pretty cool, right!?

Perhaps you could re-purpose the Quest 2 contract to be some kind of on-chain
datastore that contains whatever you want! There is, after all, a `get()`
function which can be invoked from other contracts.

### Importing Contracts

So, you have a contract that you want to invoke from inside your own? And, you
want to know how it's done? In order to invoke `contract_a` from inside
`contract_b`, you first must import the compiled `contract_a` binary into your
code for `contract_b`. Doing so makes a couple things happen inside
`contract_b`:

- any custom types that are declared in `contract_a` are now useable in
  `contract_b`
- a `ContractClient` is generated that can be used to invoke `contract_a`
  functions

Here's how this might play out in the `contract_b/src/lib.rs` file:

```rust
// We put this inside a `mod{}` block to avoid collisions between type names
mod contract_a {
    soroban_sdk::contractimport!(file = "contract_a.wasm");
}
```

**Note**: When importing a contract file into another contract, it's a good time
to think about whether or not you want to optimize your build process. You can
read more about [Optimizing Builds][optimizing] in the Soroban docs.

#### Caveat on Contract Compilation Order <!--omit in toc -->

If you are using our `Makefile` to compile your contracts, and you run into an
error when you first run `make build` for this quest, have no fear! This is
expected behavior when compiling a contract that imports another contract which
may not actually exist yet.

A second run of `make build` should clear it right up for you.

### Using a Contract Client

Once `contract_a` has been imported into `contract_b`, utilizing a cross-contract call is
quite simple. The process looks like this:

- `contract_b` creates a client it will use to invoke functions in `contract_a`
- `contract_b` makes an invocation using that client, and supplying any
  arguments that may be needed
- `contract_a` runs the invoked function and returns its response to
  `contract_b`
- `contract_b` then takes the response and does whatever is needed with it
  (returns all or part of the response, processes the response and returns
  something else, calls yet another contract, you get the idea)

You can think of this contract client as if it were an existing "module" that
you're using in your own contract. Not too bad, Soroban my ol' buddy!

### Passing Arguments to Soroban CLI

Remember back to Quest 2, for a moment. If you were one of many folks, you may
have found yourself with a deployed contract that didn't contain the
`get_self()` function. After staring in confusion for a few moments, you may
have begun to think how it might be possible to invoke that `get()` function. If
you managed to figure that out, well done! It's not an immediately obvious
task... So, let's learn a bit about the process.

In case you haven't realized yet, Soroban depends on [Remote Procedure Call
(RPC)][rpc-wiki] to pass messages between clients and the network. RPC is used
by a client to request that a server should run a function, and which arguments
it should supply. The server then runs the function, and reports back to the
client. The advantage of this approach is that the function doesn't have to
exist or run in the client. Here's [an illustrated article][rpc-gforg] on
GeeksforGeeks that goes much further in depth.

Specifically, Soroban utilizes [JSON-RPC][jsonrpc] to pass and read messages
between clients and servers. This uses the JSON data format for those messages.
So, in some cases you can pass a JSON string as an argument to a contract's
function. The [Auth (Advanced)][auth-advanced] example in the documentation
describes how you can use this technique to invoke a contract, and tell the
contract to run using `invoker()` authentication. The use of the word `Invoker`
in the example below is a *fixed* way of authenticating with the example
contract. This means the `invoker()` authentication would not work if you were
to supply any other word, such as `myPassword` or `AccountId` for example.

```bash
soroban invoke \
    --wasm target/wasm32-unknown-unknown/release/soroban_auth_advanced_contract.wasm \
    --id 1 \
    --account GC24I42QMKKR4NE6IYNPCQHUO4PXWXDGNZ7QVMMSR5EWAYSGKBHPLGHH \
    --fn increment \
    --arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
    --arg 0
```

You can see that it's clearly a JSON object that isn't presented in a "pretty"
manner. Some other examples of these JSON arguments might look like this:

```json
'[{"u32":5}]' // the integer 5
'{"object":{"vec":[{"symbol":"<helloworld>"}]}}' // a single-element `Vec` containing the `Symbol` "Invoker"
'{"map":[{"key":{"symbol":"<key>"},"value":{"symbol" :"<value>"}}]}' // a `Map` with a key-value pair of {"<key>": "<value>"}
'{"contractCode":{"wasm":"<raw_wasm_hex_encoded>"}}' // the hex-encoded binary that makes up a contract
```

So, that's how to pass some of the more advanced arguments to the Soroban CLI,
but you need to figure out how to pass the *right* argument for this quest. Our
very own @Smephite has put together an [**incredible** guide][smephite-guide] to
the various soroban types and how to use them in contract invocations. Read
that! For real! You'll **need** some of the information in that document to
complete this quest.

## Further Reading

- The [Cross Contract Calls example][ccc-example] contract in the Soroban
  documentation has even more details and hints regarding this topic.
- Read more about [`traits` in Rust][rust-traits] in The Rust Reference
- The "Learn" section of the Soroban documentation has an article all about
  [interacting with contracts][interacting-contracts], and it's **definitely**
  worth the read!
- We didn't explore the finer details of keeping data on chain in this quest,
  but there is so much more to learn about this! Please check out the
  [persisting data][persisting-data] article in the Soroban documentation.
- This [Simple Guide to Soroban Types][smephite-guide] is an absolute
  game-changer for interacting with smart contracts from the Soroban CLI.

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[ccc-example]: https://soroban.stellar.org/docs/examples/cross-contract-call
[rpc-wiki]: https://en.wikipedia.org/wiki/Remote_procedure_call
[rpc-gforg]: https://www.geeksforgeeks.org/remote-procedure-call-rpc-in-operating-system/
[jsonrpc]: https://www.jsonrpc.org/
[auth-advanced]: https://soroban.stellar.org/docs/examples/auth-advanced#run-the-contract
[optimizing]: https://soroban.stellar.org/docs/tutorials/build-optimized
[rust-traits]: https://doc.rust-lang.org/book/ch10-02-traits.html
[interacting-contracts]: https://soroban.stellar.org/docs/learn/interacting-with-contracts
[persisting-data]: https://soroban.stellar.org/docs/learn/persisting-data
[smephite-guide]: https://gist.github.com/Smephite/09b40e842ef454effe4693e0d18246d7
[account-id]: https://gist.github.com/Smephite/09b40e842ef454effe4693e0d18246d7#sco_account_id
