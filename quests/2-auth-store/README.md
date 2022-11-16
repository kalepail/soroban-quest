# Quest 2 - Auth Store <!-- omit in toc -->

## TL;DR

Do you consider yourself "advanced"? Do you think you can skip the high-level
theory and get away with rushing into this quest? Do you feel comfortable
completely mucking up your account before you even understand the assignment?

**In this quest, you will deploy the quest 2 Auth Store contract to the Stellar
Futurenet. Then you must successfully invoke the `put()` function to store some
data on the ledger, and then successfully invoke either the `get()` or
`get_self()` function to retrieve that same data.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [A Quick Hint](#a-quick-hint)
- [The Task at Hand](#the-task-at-hand)
  - [Explore the Contract Code](#explore-the-contract-code)
  - [Storing Data](#storing-data)
  - [Retrieving Data](#retrieving-data)
  - [Simple Authentication](#simple-authentication)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

If you missed out on our previous quest, or you just need a refresher, we have
some (pretty extensive) instructions for the *mechanics* of completing these
quests (generating keypairs, checking your work, etc.).

All that information [can be found here][how-to-play] if you need to use those
instructions again.

## A Quick Hint

In the last quest, there were several users who struggled to deploy or invoke
any functions on the Futurenet network, because their RPC endpoint hadn't yet
completed the sync process with the other peer nodes in the network.

Here's a quick hint, if you want to check the state of your RPC endpoint. Run
this command in your Futurenet terminal:

```bash
curl -s https://horizon-futurenet.stellar.org/ | grep ingest_latest_ledger && curl -s http://127.0.0.1:8000 | grep ingest_latest_ledger
```

You should get output that resembles the following:

```bash
gitpod /workspace/soroban-quest (main) $ curl -s https://horizon-futurenet.stellar.org/ | grep ingest_latest_ledger && curl -s http://127.0.0.1:8000 | grep ingest_latest_ledger
  "ingest_latest_ledger": 799299,
  "ingest_latest_ledger": 0,
```

The first line represents the latest ledger on the SDF-maintained Futurenet
nodes, and the second line represents the latest ledger on your Gitpod's
workspace. If those two numbers are too far apart, you will not be able to
interact with the Futurenet through your local RPC endpoint.

In that case, you'll just have to patiently wait while your Futurenet node syncs
with the rest of the network.

## The Task at Hand

So, down to brass tacks! Let's figure out how this quest works. A Soroban
contract can store arbitrary data in the ledger, and this data can then be
retrieved at a later time. Your job today is to both store, and subsequently
retrieve data by invoking a couple of functions in your contract. (After it's
been deployed to the Futurenet, of course!)

### Explore the Contract Code

Just like all the quests, this quest's [`lib.rs`](src/lib.rs) and
[`test.rs`](src/test.rs) files are commented with handy documentation and
explanations of what is happening. Be sure to check them out and read through
what the contract is up to.

**Important**: This quest has very complete comments and documentation
(particularly in the tests) in those two files. It will go **miles** to help
your understanding of Soroban, if you read through those files, and took the
time to understand what's happening.

### Storing Data

Soroban uses the `Env.data().set()` function to store data in
a contract's ledger entries. You can think of these ledger entries as
key-value storage that can only be accessed through the contract that owns it.
You can construct a contract's ledger entries in many different ways. They could
be made up of very simple elements like a symbol or number. Or, they can also be
made from very complex vectors or maps.

The ledger entries for this quest will store a supplied `value` alongside an
`AccountId`. Using Soroban's `Env.invoker()` function gives us a simple method
of authenticating a user. Only someone who could successfully sign for an
account (and, thus, invoke the contract from the account) is permitted to store
data in this contract as that account.

*Invoke the contract's `put()` function to store some kind of data into the
contract's ledger entries.*

### Retrieving Data

To retrieve data from within a contract's ledger entries, the `Env.data().get()`
function is available to us. When called with a `key` that corresponds to data
the contract has previously stored, we get the `value` stored alongside it in
return.

The contract's `get_self()` function searches for data associated with the
invoker's account. You don't need to pass any arguments here, since we know to
search for the invoker.

On the other hand, the `get()` function will retrieve stored data associated
with any account. When supplied with an `AccountId` as an argument, this
function will search for stored data corresponding to that account. (I wonder if
this might come into play at some point in the future?)

To complete this quest, you only have to invoke one of these functions. You
don't have to try and invoke both of them, although it wouldn't hurt to try!

*Invoke either the contract's `get()` or `get_self()` function to retrieve some
contract data.*

### Simple Authentication

OK, so what's the point of all this? Sure, it's pretty neat to be able to store
and retrieve data from the smart contract network. But, is there anything
more... "useful" about this?!

Well, sure! For starters, you can control *who* is allowed to set *which* of the
contract's data keys. The way we've coded this contract, **only** a person who
can sign for a given Stellar Account (i.e. a public key) can store or modify
data associated with that Account ID. The `Env.invoker()` function can be a
pretty quick and simple way of authenticating that a contract is being invoked
by someone who has the proper authorization to invoke a contract using a given
account.

## Further Reading

- Check out the [storing data][data-example] example contract for some further
  discussion about this method of storing and retrieving data.
- A discussion of more advanced authentication methods can be found in the
  [auth (advanced)][auth-advanced] example contract.
- You can learn more about [persisting data][persist-data] in the "Learn"
  section of the Soroban documentation.

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[data-example]: https://soroban.stellar.org/docs/examples/storing-data
[auth-advanced]: https://soroban.stellar.org/docs/examples/auth-advanced
[persist-data]: https://soroban.stellar.org/docs/learn/persisting-data
