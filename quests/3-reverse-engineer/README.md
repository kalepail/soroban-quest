# Quest 3 - Reverse Engineer <!-- omit in toc -->

## TL;DR

You again!? You're back here looking for the quick task? Well, alright, if you
think you're really ready for it. Good luck!

**For this quest, we have already deployed the `ReverseEngineerContract` using
the account `GAC7HE4PQWI7H34JQN6QLQ7Y7NTAUZDWJZ4SJ6GXVC4K2DN7N65K5NLI`. You must
*find* the `contractId` for this contract! Then you must invoke the correct
function in that contract, while providing the correct argument.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
- [The Task at Hand](#the-task-at-hand)
  - [Explore the Contract Code](#explore-the-contract-code)
  - [Using Soroban-CLI to Decode XDR](#using-soroban-cli-to-decode-xdr)
  - [How do I find a `contractId`?](#how-do-i-find-a-contractid)
    - [Find an Operation](#find-an-operation)
    - [View the Transaction Result](#view-the-transaction-result)
    - [Sidenote About Reading Deployed WASM Binaries](#sidenote-about-reading-deployed-wasm-binaries)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

If you missed out on our previous quests, or you just need a refresher, we have
some (pretty extensive) instructions for the *mechanics* of completing these
quests (generating keypairs, checking your work, etc.).

All that information [can be found here][how-to-play] if you need to use those
instructions again.

## The Task at Hand

I hear you: "Let's get to the quest!!" I love your enthusiasm! But, I want to
~~implore~~ beg you to do one thing first: Read the code!

### Explore the Contract Code

Ok, so we've been at this a couple times before. You might feel like you're
*starting*  to understand the order of things, and how this all works. **Or**,
you might feel like you're totally lost, and you just want someone else to give
you the answer.

Whatever you're feeling, I want to put this in the most emphasized voice I can:
**Read the code that is contained within `src/lib.rs` and `src/test.rs`!** Read
it. The whole thing. Comments and all. Everything!

Trust me, you will want to read the contract for this one (all of the quests,
honestly). Sure, reading the code will help you understand the contract. Yes,
that's what we're trying to do. **But**, it will also answer many of the
questions you'll come up with as you work on this task.

### Using Soroban-CLI to Decode XDR

A term you may already be familiar with is "XDR" which stands for *eXternal Data
Representation*. So much of what happens on the Stellar network is done in XDR
format: transactions, ledger data, history, operation results, and the list goes
on. XDR is a compact, network-efficient, binary format. While it's great for
many things, it's not human readable, so it can be pretty confusing.

Thankfully, the [Soroban CLI][soroban-cli] makes it pretty easy to get decoded,
useful, and understandable output from supplied XDR. For example, when a
transaction is submitted to the Network, it's submitted in XDR format. Here is
how you could use the `soroban` cli to decode a Friendbot transaction XDR into a
more human-readable format.

```bash
soroban xdr dec --type TransactionEnvelope --xdr AAAAAgAAAABhi8yJmyMMTBza5emErFGm+xbj3PeggjF1g0CVlG+jOQAPQkAAAFRyAAAABgAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAABAAAAABB90WssODNIgi6BHveqzxTRmIpvAFRyVNM+Hm2GVuCcAAAAAAAAAABwLBiyzjjMVWCiToDYJPGrLhVy4+Ndx26l2x28bngMAwAAABdIdugAAAAAAAAAAAKUb6M5AAAAQDqxm6rPqDMypQWNAyZY17x5YG+cEuhZ8kqD868mMskTKE0jxD3lTY73gddFdmZZZ4Tit2pwQOtXI7w+M4OGoAiGVuCcAAAAQO0kX9PUv7sLyAaGvyw4l8NR86S5Pj9erMIOj0u2qmEgDW8YRQERFH3JkF/GX2B8NMy8NCd5/KWgj9b3iahk+QQ= --output json
```

I'll omit the output here, in an attempt to keep this README un-cluttered. But,
if you were to run the above command, you'd see a JSON-formatted representation
of a transaction creating an account with a starting balance of 10,000XLM
(futurenet XLM, of course).

There are many different "types" you could use the `soroban` cli to decode. If
you run `soroban xdr dec --help` in your terminal, you'll see all the different
options that are available to you.

### How do I find a `contractId`?

Have you ever heard the expression "There is more than one way to skin a cat"? I
know, it's nasty and gross! I don't know who wants to have even *one* way to
skin a cat!! Anyway, I'm getting sidetracked...

There are a few different ways you could find a `contractId` for an already
deployed contract. All of them involve decoding XDR, and that can be done using
the soroban-cli, or the Stellar Laboratory, or RunKit, or you could come up with
your own way to decode/encode the base64 as needed. But they all start with
finding the correct XDR to decode.

The following sensible approach to finding a `contractId` will start by finding
a relevant `operation` for the account in question, and then the corresponding
`transaction`. This definitely isn't the only way it could be done (it's not
even the quickest or easiest), but it *is* straight-forward, and easy to follow.

#### **Find an Operation**

We begin by heading to the [Stellar Lab][lab] (using futurenet), and we'll click
on **Explore Endpoints** -> **Operations** -> **Operations for Account**. Enter
the public key of an account that exists on futurenet (the example here shows an
account that was used to deploy and invoke the `HelloWorld` Contract from Quest
1), and click **Submit**. (If you prefer viewing the JSON output, you can get
that [here][ops].)

<details>
<summary>View screenshot</summary>

![Operations for Account](https://user-images.githubusercontent.com/2024293/202301202-4cc30e8e-b5ca-4efd-873d-52d75a43eb50.png)

</details>

**Reminder**: You are looking for a contract that was deployed by this address:
`GAC7HE4PQWI7H34JQN6QLQ7Y7NTAUZDWJZ4SJ6GXVC4K2DN7N65K5NLI`.

When the results appear, we're going to look for an operation of the "type"
`invoke_host_function`, **and** with a "function" field of
`HostFunctionHostFnCreateContractWithSourceAccount` (i.e. this operation is an
account uploading/deploying a smart contract). Our example operation:

```json5
{
  "_links": {
    "self": {...},
    // This is the link to the transaction you want to use (again, this is only an example)
    "transaction": {
      "href": "https://horizon-futurenet.stellar.org/transactions/6c0b18cba3400b5c766923aeaeefbce4749f96a62fafc3eabf91e7202b4dad47"
    },
    "effects": {...},
    "succeeds": {...},
    "precedes": {...}
  },
  "id": "3607489060802561",
  "paging_token": "3607489060802561",
  "transaction_successful": true,
  "source_account": "GBYCYGFSZY4MYVLAUJHIBWBE6GVS4FLS4PRV3R3OUXNR3PDOPAGAGPGK",
  // We want to find an operation where (type === "invoke_host_function")
  "type": "invoke_host_function",
  "type_i": 24,
  "created_at": "2022-11-16T21:38:03Z",
  "transaction_hash": "6c0b18cba3400b5c766923aeaeefbce4749f96a62fafc3eabf91e7202b4dad47",
  "parameters": [
    {
      "value": "AAAABAAAAAEAAAAEAAABgABhc20BAAAAAQ8DYAF+AX5gAn5+AX5gAAACDQIBdgFfAAABdgE2AAEDAwIAAgUDAQAQBhkDfwFBgIDAAAt/AEGAgMAAC38AQYCAwAALBzEFBm1lbW9yeQIABWhlbGxvAAIBXwADCl9fZGF0YV9lbmQDAQtfX2hlYXBfYmFzZQMCCqABApoBAwF/AX4CfyOAgICAAEEgayIBJICAgIAAAkAgAEIPg0IJUg0AQgUQgICAgAAhAiABQRxqQQI2AgAgASAANwMQIAFCyY7H1RM3AwggAUEIaiEDQQEhBAJAA0AgBEEDRg0BIAEgBDYCGCAEQQFqIQQgAiADKQMAEIGAgIAAIQIgA0EIaiEDDAALCyABQSBqJICAgIAAIAIPCwAACwIACwAeEWNvbnRyYWN0ZW52bWV0YXYwAAAAAAAAAAAAAAAXADsOY29udHJhY3RzcGVjdjAAAAAAAAAABWhlbGxvAAAAAAAAAQAAAAJ0bwAAAAAABgAAAAEAAAPqAAAABg==",
      "type": "Obj"
    },
    {
      "value": "AAAABAAAAAEAAAAEAAAAIP0hkKG8/MVooXYMhSjPxLDr9m0fTdFoOnXSFj36OY2X",
      "type": "Obj"
    }
  ],
  // AND this operation should be where (function === "HostFunctionHostFnCreateContractWithSourceAccount")
  "function": "HostFunctionHostFnCreateContractWithSourceAccount",
  "footprint": "AAAAAAAAAAEAAAAGUun3Vzk8sIx5g6OSFP0G+TFcYMwLubcSywQrXO9DqzYAAAADAAAAAw=="
}
```

**Note**: You could also use this same technique to find some pretty useful
information from `HostFunctionHostFnInvokeContract` operations. You could use
those operations to see exactly what a given account used to invoke a given
contract. Check out [this video][twitch] to learn a bit more!

#### **View the Transaction Result**

From there, we find the link to the **transaction** that contains this
operation. It's provided in the operation's `_links.transaction` object. If
you're in the Lab, you can click on that link and it will open up the endpoint
explorer with the fields for that transaction pre-filled, and you just have to
click **Submit** once again. (For the JSON among us, you can copy/paste the link
into your browser, or you can [click here][tx].)

In the transaction information, you're looking for the `result_meta_xdr` field.
This contains the result from the transaction, as well as what has changed in
the networ as a result of the transaction. Most pertinent to this quest, it will
contain the `contractId` of the deployed contract. In the Lab, if you click on
that XDR string, it will take you to the XDR viewer, where you can find the
`contractId` (don't forget to [decode the base64][twitch-clip] somehow).

<details>
<summary>View screenshot</summary>

![Transaction Result Meta XDR](https://user-images.githubusercontent.com/2024293/202301714-082efbb5-7350-45ec-8a1a-a062ea8fe444.png)

</details>

Alternatively, you could copy/paste the whole Result Meta XDR string and decode
it using the Soroban CLI to get the information you're after.

#### **Sidenote About Reading Deployed WASM Binaries**

This is merely tangential to today's Quest, but it is very interesting and
useful nonetheless.

<details>
<summary>Are you curious? Go ahead. Read on...</summary>

The reason we've taken you to see the full transaction meta is to point out that
included in this XDR is also the `contractCode`! Yeah, that's right. The whole
thing! You could even decode it and use it as a normal WASM file, too. You'd
have to do it like this:

```bash
soroban xdr dec \
    --type TransactionMeta \
    --xdr <put your transaction_result_meta XDR here> \
    --output json

# look for the output field labeled `contractCode`. It's displayed in hex, so
# you'd have to convert it into a binary file, maybe like this:

echo "<the-wasm-hex>" | xxd -r -p > output.wasm
```

The `output.wasm` file resulting after that would be identical to the compiled
contract that was initially deployed. You could re-deploy it, use `soroban gen`
to get information about it, or whatever else you could come up with. Cool,
huh!?

Like we said, this isn't particularly important to this quest, but it could come
in handy at some point for you.
</details>

## Further Reading

- [Learn more about XDR][xdr] in the Stellar Developer Documentation.
- [This episode][twitch-full] of "Soroban Talks" is **SO** useful, and can help
  you get a handle on what's happening inside of Soroban. (Hint: Starting around
  [23:14][twitch] is a *really* useful discussion about decoding the XDR values
  into something a little more user-friendly.)
- Developers can also use the Soroban-RPC interface to interact with futurenet
  and get current state data. [This design doc][soroban-rpc] is being used
  discuss and develop how this API functions.
- Some basic information about the usage of the Soroban CLI can be found on the
  [Soroban Docs website][install-soroban]. In addition to that page many of the
  tutorials and examples contain example CLI commands.

## Still Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

[how-to-play]: ../1-hello-world/README.md#how-to-play
[xdr]: https://developers.stellar.org/docs/encyclopedia/xdr
[soroban-cli]: https://github.com/stellar/soroban-cli
[lab]: https://laboratory.stellar.org
[ops]: https://horizon-futurenet.stellar.org/accounts/GBYCYGFSZY4MYVLAUJHIBWBE6GVS4FLS4PRV3R3OUXNR3PDOPAGAGPGK/operations?order=asc
[tx]: https://horizon-futurenet.stellar.org/transactions/6c0b18cba3400b5c766923aeaeefbce4749f96a62fafc3eabf91e7202b4dad47
[twitch]: https://www.twitch.tv/videos/1642865389?t=00h23m14s
[twitch-clip]: https://clips.twitch.tv/FragileSneakyOstrichGivePLZ-DK9h3VVmUjqVDDZG
[twitch-full]: https://www.twitch.tv/videos/1642865389
[soroban-rpc]: https://docs.google.com/document/d/1TZUDgo_3zPz7TiPMMHVW_mtogjLyPL0plvzGMsxSz6A/edit#heading=h.ohr0vgpzoi7r
[install-soroban]: https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli
