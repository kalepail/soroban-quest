# Quest 6 - Asset Interop <!-- omit in toc -->

## TL;DR

Big day, huh!? Final Quest in our inaugural series of Soroban Quest! This has
been so awesome right! And there's only more greatness to come so stay tuned!

If you're looking for the _very_ next thing be sure to check out and contribute
to the [Sorobanathon][sorobanathon].

Today's Quest is a banger! It will not only challenge you, but will also
show you some of the **amazing** stuff that's possible in this brave new
Soroban-ified world! Now, it's a doozy so you're going to _really want_ to read
through this document. But, here's the short-n-sweet instructions, if you want
to jump ahead and muck things up.

There are two relevant accounts today:

- `Parent_Account` will be your Quest Account, (what you are given when you run
  `sq play {n}`) and will be used to deploy an `AllowanceContract` contract.
- `Child_Account` will be a secondary account which will interact with
  your contract. Create and fund this Futurenet account on your own.

**For our finale quest, you must build and deploy the `AllowanceContract`
contract using your Quest Account (`Parent_Account`). Then use that same account
to import XLM from "Classic" Stellar into Soroban. Next approve your deployed
contract to act as a proxy enabling it to transfer XLM from the parent to the
child. Then, using either account, you must withdraw an allowance to the
`Child_Account` using the contract deployed by the `Parent_Account`. Finally,
export that allowance back into the `Child_Account`'s "Classic" Stellar
account.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [The Task at Hand](#the-task-at-hand)
  - ["Classic" Stellar Assets vs. Soroban "Smart" Tokens](#classic-stellar-assets-vs-soroban-smart-tokens)
  - [The Built-In Token Contract](#the-built-in-token-contract)
  - [Yeah, but How do I Use That Built-In Token Contract?](#yeah-but-how-do-i-use-that-built-in-token-contract)
  - [Native XLM on Soroban](#native-xlm-on-soroban)
  - [Back to Your Quest](#back-to-your-quest)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

If you missed out on our previous quests, or you just need a refresher, we have
some (pretty extensive) instructions for the _mechanics_ of completing these
quests (generating keypairs, checking your work, etc.).

All that information [can be found here][how-to-play] if you need those
instructions again.

## The Task at Hand

For a moment, put yourself in the position of a parent, sending their child out
into the world to face the challenges of college and/or the workforce. I know,
it's emotional. That's ok. You don't want to send them off with nothing. You
want to give them some kind of confidence they won't have an empty stomach at
the end of the day, no matter what happens.

So, you're setting up an allowance contract for them. You will deploy a contract
that will be a one-time action on your part, but will make available to them a
steady trickle of resources should they ever have need. This approach gives us
two very powerful wins: _they_ can't withdraw everything at once, but _you_
don't have to remember to make transfers all the time!

Back in reality, we're ready to talk assets! But, first, we _have_ to say it one
last time, **read the code!** This contract is relatively complex. We won't make
you fiddle with it today, but there's no better way to understand what's
happening than to actually _read_ it. Seriously.

### "Classic" Stellar Assets vs. Soroban "Smart" Tokens

> _Please Note_: If you've forgotten, Soroban is still under active development,
> design, and discussion. Significant changes can happen and should even be
> expected. The area of asset interoperability between "Classic" Stellar and
> Soroban "Smart" Tokens is one such area that is under active consideration. We
> have designed this quest to be as up-to-date as possible, but the conventions,
> steps, terminology, architecture, etc. used in today's quest are subject to
> change in the future.

One of the defining characteristics of the "Classic" Stellar network is that
assets are a first-class citizen. They are easy to create, cheap to
use/transfer/trade, and useful for many use-cases. There also exists an
extensive set of authorization tools that asset issuers can use to control who
can acquire, use, or retain those assets. We won't spend _too_ much time here,
because you are probably already up-to-speed here. If you need a refresher the
[developer documentation][docs-assets] and our own [Stellar Quest
Learn][sq-learn] course has **loads** of information about assets. For now, just
remember that the `native` asset on Stellar (both "Classic" and Soroban) is the
[Lumen][lumens]. It's identified using the asset code `XLM`.

As Soroban development continues, one of the _key_ requirements is that assets
issued on "Classic" Stellar can be used and incorporated into Soroban. It's even
[one of the FAQs][assets-faq]! This interoperability is facilitated by using the
`import` and `export` functions that exist in [the built-in token
contract](#the-built-in-token-contract), which allows assets issued on "Classic"
Stellar to be brought into play on Soroban. (Note assets minted on Soroban
cannot be exported to a "Classic" Stellar asset.)

### The Built-In Token Contract

Soroban development regarding Assets involves an effort to decide what a
"Standardized Asset" looks like in a smart contract context. These decisions,
and related discussions, are recorded in [CAP-0046-06][cap-46-6]. If you're
familiar with Ethereum, this proposal tries to follow an ERC-20 model, where
applicable.

The [Built-In Token Contract][token-contract] is an implementation of the
CAP-46-6 proposal. It can be used to to create a new token on Soroban, or to
wrap a "Classic" asset and transfer it to Soroban. For today, we'll be using it
to wrap and import Lumens from "Classic" Stellar. This built-in token contract
implements a [token interface][built-in-interface] that is quite feature-full.
The most notable functions you'll need from it today are `import`, `approve`,
and `export`.

### Yeah, but How do I Use That Built-In Token Contract?

> It should be noted that a Soroban token developer can choose to implement any
> interface they choose. There isn't any _requirement_ to implement everything
> from CAP-46-6, but doing so does allow a token to interoperate with other
> tokens which _are_ compliant with CAP-46-6. You can learn more about the
> [suggested token interface][token-interface] in the Soroban docs.

So, how do we actually make one of them tokens, then? There are a few methods
available to us. Let's (briefly) look at them.

1. (Spicy 🌶️🌶️🌶️) You _could_ write the whole thing from scratch, implementing
   whatever features, functions, and fun suit your needs. That would take a lot
   of work, but you could do it. I won't stop you.

2. (Medium 🌶️🌶️) There's a `create.py` script in the `py-scripts/` directory here
   that will do a lot of the heavy lifting for you. This can be used and adapted
   to match whatever asset you're trying to create. It's a fantastic
   starting point.

3. (Mild 🌶️) Fun fact, the Soroban CLI has a handy little helper command built
   right into it that will (we promise, we're not making this up) do
   _everything_ for you! You don't have to code anything, just run the command a
   single time, and the contract is **deployed**. You could use it like this:

```bash
soroban token wrap --asset QUEST6:GBCXQUEPSEGIKXLYODHKMZD7YMTZ4IUY3BYPRZL4D5MSJZHHE7HG6RWR

output:
42d792eb17c983b62bfac05fc31f9588675efd65867f26c56bafb2b15adb6e04

# It even works with the `native` asset!
soroban token wrap --asset native
```

It should be noted using the Soroban CLI to deploy a wrapped asset will work
exactly one time per asset (per network). So, the `native` asset is already
deployed to the Futurenet, and trying to wrap that again (on the Futurenet) will
return an error rather than a `contractId`.

> It should _also_ be noted you don't need to deploy or wrap any tokens or
> assets for this quest. We just put this here for fun!

### Native XLM on Soroban

Speaking of the `native` asset: One of the cool things about Soroban's built-in
token contract is that even the native XLM token utilizes it to bring XLM onto
Soroban. To use it, we just need to figure out the `contractId` that we should
invoke. That can be done easily enough with one of the Stellar SDKs (below,
we're using Python):

```python
import hashlib
from stellar_sdk import Asset, xdr

# This will work using either native or issued assets
native_asset = Asset.native()
issued_asset = Asset("QUEST6", "GBCXQUEPSEGIKXLYODHKMZD7YMTZ4IUY3BYPRZL4D5MSJZHHE7HG6RWR")

data = xdr.HashIDPreimage(
    xdr.EnvelopeType.ENVELOPE_TYPE_CONTRACT_ID_FROM_ASSET,
    from_asset=native_asset.to_xdr_object(),
)
contract_id = hashlib.sha256(data.to_xdr_bytes()).hexdigest()
print(f"Contract ID: {contract_id}")
```

An expanded version of the above script, as well as some other _very_ handy
Python scripts (big shoutout to [Jun Luo (@overcat)][overcat]) in the
`py-scripts/` directory. They deal with all kinds of Soroban tokens tasks:
importing/exporting balances, creating a wrapped "Classic" token, finding
contract IDs, etc.

As per our tl;dr at the top, this contract will need to be invoked _at least_
three times:

1. The `Parent_Account` will need to `import` the native asset.
2. The `Parent_Account` will need to `approve` the `AllowanceContract` as a
   proxy spender.
3. The `Child_Account` will (eventually) need to `export` their allowance back
   to "Classic" Stellar.

Don't forget to look into the [Built-In Token Interface][built-in-interface] to
figure out which arguments you'll need to use when making those invocations. You
remember how to format those arguments, don't you? What!? You don't?! Ok, ok,
ok. It's gonna be fine. Check back to [Quest 4](../4-cross-contract/README.md)
and [Quest 5](../5-custom-types/README.md) for a recap.

<sup><sub><sup><sub><sup><sub>
or poke around in here some more
</sup></sub></sup></sub></sup></sub>

### Back to Your Quest

Ok, so we've gone through a bunch of theory, and looked at how assets can (or
cannot) interact and transfer between "Classic" Stellar and Soroban. Now, it's
time to let you go and bring this thing home!

If you forgot what your task is, here it is again:

- [ ] Deploy the `AllowanceContract` as the `Parent_Account`
- [ ] Invoke the `init` function of the `AllowanceContract`
- [ ] `import` some XLM into the `Parent_Account` from their "Classic" Stellar
  account
- [ ] `approve` the `AllowanceContract` to make proxy transfers from the
  `Parent_Account` to the `Child_Account`
- [ ] Invoke the `withdraw` function of the `AllowanceContract` with either the
  `Child_Account` or `Parent_Account`
- [ ] `export` some XLM from the `Child_Account` to their "Classic" Stellar
  account

While performing the above steps, you'll want to consider the amount of XLM
you're using along the way. In Soroban, most assets are quantified using
[Stroop][stroop]s (that is, one ten-millionth of the asset). For example, if you
want to _import_ 1 XLM, you'll need to supply `10000000`, `10_000_000` or `1 *
10**7` Stroops as an argument in your invocation.

Additionally, the astute observer might notice an interesting separation between
the Parent's "Classic" Stellar balance and the approved allowance the contract
has access to at any given time. For example you could `import` 100,000 XLM into
the Parent account but only `approve` a "first tranche" to the contract of
10,000 XLM and then the contract – depending on the `init` arguments passed –
might `withdraw` 5,000 XLM during each successful invocation. The contract will
only ever be able to proxy from the parent to the child as per the contract
arithmetic but this flexibility allows the parent to more safely and sensibly
control the flow of funds. All the levers! You **are** the man behind the
curtain!

Finally, given this flexibility, great care should be taken when calling the
various invocations, as you don't want to enable a `withdraw` to take place that
would be greater than the contract's available allowance. Choose your numbers
wisely, my friend.

> If you're really confused about the units, digits, and numbers to use, read
> through the `src/test.rs` file for some inspiration and to see which numbers
> we used during development.

## Further Reading

- **[Core Advancement Proposal 0046-06][cap-46-6]** contains more information
  than you probably want about how the asset interoperability is intended to
  work. These "CAP" documents are excellent resources for discovering not only
  _how_ something on Stellar works, but also _why_ it is designed that way.
  Check it out some time.
- The **[Built-In Token Contract][token-contract]** article in the Soroban
  documentation is a probably less intimidating resource. It has so much more
  good stuff than we could even mention here. This is definitely one to read
  through.
- The **[Timelock][timelock]**, **[Single Offer Sale][single-offer]**, and
  **[Liquidity Pool][liquidity-pool]** example contracts are a great place to
  learn more about how assets on Soroban can interact with each other, and how
  they could be interacted with. These are great examples for a real world use
  of the concepts we've discussed today.
- **[Assets][docs-assets]** in Stellar are an enormous part of the network
  architecture. If you're unfamiliar with how assets work with "Classic"
  Stellar, than the Developer Documentation has all the information you'll need.
  Or, if you want to earn some more sweet badges while you learn, level 1 of
  [Stellar Quest Learn][sq-learn] is exactly what you want! Lots of excellent
  knowledge about assets and payments there.
- Today's quest makes use of the **[`soroban-auth` SDK][rust-auth]** for the
  first time in any quest. It allows you and your contract to authenticate users
  in a variety of ways. If you're building something on Soroban, you'll want to
  become familiar with this SDK
- Soroban doesn't know a _whole lot_ about the state of the Stellar network at
  execution time. But, it does know a few things, and those are presented to it
  as a `Ledger` data structure. There is pretty significant stuff to know, so
  here's the relevant **[documentation page][sdk-ledger]** all about it!

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[token-contract]: https://soroban.stellar.org/docs/built-in-contracts/token
[token-interface]: https://soroban.stellar.org/docs/common-interfaces/token
[built-in-interface]: https://soroban.stellar.org/docs/built-in-contracts/token#contract-interface
[cap-46-6]: https://stellar.org/protocol/cap-46-06
[docs-assets]: https://developers.stellar.org/docs/fundamentals-and-concepts/stellar-data-structures/assets
[assets-faq]: https://soroban.stellar.org/docs/faq#can-soroban-contracts-interact-with-stellar-assets
[lumens]: https://developers.stellar.org/docs/fundamentals-and-concepts/lumens
[overcat]: https://github.com/overcat
[stroop]: https://developers.stellar.org/docs/glossary#stroop
[timelock]: https://soroban.stellar.org/docs/examples/timelock
[single-offer]: https://soroban.stellar.org/docs/examples/single-offer-sale
[liquidity-pool]: https://soroban.stellar.org/docs/examples/liquidity-pool
[sq-learn]: https://quest.stellar.org/learn
[rust-auth]: https://soroban.stellar.org/docs/SDKs/rust-auth
[sdk-ledger]: https://docs.rs/soroban-sdk/latest/soroban_sdk/ledger/struct.Ledger.html
[sorobanathon]: https://github.com/stellar/sorobanathon
