# Quest 1 - Hello World <!-- omit in toc -->

## TL;DR

I know you're in a hurry. There are Lumens on the line! Here's the quick-n-dirty
version of what you need to do to get that awesome SQ badge.

**In this quest, you will use your Quest Account to deploy and invoke the quest
1 Hello World contract on the Stellar Futurenet.**

## Table of Contents <!-- omit in toc -->

- [TL;DR](#tldr)
- [How to Play](#how-to-play)
  - [Authenticate with Discord](#authenticate-with-discord)
  - [Pull in New Quests](#pull-in-new-quests)
  - [Retrieve your Quest Account](#retrieve-your-quest-account)
  - [Fund that Account](#fund-that-account)
  - [Quest Your Heart Out](#quest-your-heart-out)
  - [Check your Quest Answer](#check-your-quest-answer)
  - [Claim your Badge](#claim-your-badge)
- [The Task at Hand](#the-task-at-hand)
  - [Explore the Contract Code](#explore-the-contract-code)
  - [Build the Contract](#build-the-contract)
  - [Run a Test](#run-a-test)
  - [Deploy to Futurenet](#deploy-to-futurenet)
  - [Invoke it on Futurenet](#invoke-it-on-futurenet)
- [Finish the Quest](#finish-the-quest)
- [Further Reading](#further-reading)
- [Still Stuck?](#still-stuck)

## How to Play

Before we can get you questing, we need to make sure you are actually *ready* to
quest! So, do this before you move on to anything else!

**Note:** You will be required to follow this basic procedure for each of the
quests during this series. Please take a moment to review the following
instructions and make yourself familiar with them. You'll be thankful you did!

### Authenticate with Discord

In one of your `bash` shells in the bottom panel of your Gitpod Workspace, run
the following command:

```bash
sq user
```

If you see that you are successfully authenticated, you can keep moving.
Otherwise, you should run:

```bash
sq login
```

to sign in with your Discord account. You'll then be prompted to complete KYC
or submit tax information, if either are required of you.

**Note:** If you haven't completed the KYC flow and tax submission on the
[Stellar Quest site][sq-site], your ability to claim XLM rewards will be
affected by that. Complete those steps so we can give you the rewards you've
worked really hard for!

### Pull in New Quests

When the countdown reaches 0 for each quest release, you can pull in the newly published quest by running the following command:

```bash
cd /workspace/<the-workspace-name> # `sq pull` must be run from inside the workspace root
sq pull
```

You can also do this manually, if you're a `git` veteran. Find the specifics
over in our [Pioneer Quest][pq-new-quests].

One that's complete you'll see the new quests delivered to your
`/workspace/<the-workspace-name>/quests` directory.

### Retrieve your Quest Account

Before you can play, you must retrieve the `Quest Keypair` for the quest you
want to play. You get that information by running the following:

```bash
sq play 1 # use whichever quest number you are trying to play
```

Save this information, because (trust me) you'll need it later!

### Fund that Account

We even put together a handy way for you to get your Futurenet Lumens from
Futurenet Friendbot (you might know his cousin...). `sq` can help you with that
like so:

```bash
sq fund <your-quest-keypair-public-key>
```

### Quest Your Heart Out

Now you're ready to move on to the actual quest part of this! Please skip ahead
to [this section](#the-task-at-hand) to begin the fun part!

When you think you've finished all that's required, come back here and check
your work!

### Check your Quest Answer

You've done the hard work, and you're ready to see if it'ss paid off! Run the
following command to verify your work:

```bash
sq check 1 # use whichever quest number you are trying to verify
```

If you still haven't completed KYC and/or tax submission, you'll be reminded one
more time that you can't claim XLM rewards without them. If you still have more
work to do, you'll be given a clue as to what might be missing from your Quest
Account.

### Claim your Badge

If your check was successful, the `sq` CLI will let you know with a celebratory
emoji! You'll then be able to choose how you want to claim your prize: sign a
transaction using **Albedo**, or sign the **Raw XDR**.

If you select "Albedo," a window will open asking you to sign the transaction,
and thus allowing you to claim your badge and any prize XLM you may have earned.

If you choose "Raw XDR," the transaction will be output to the terminal window,
and you will need to sign it using Stellar Laboratory (or your preferred
method). Then you must submit the signed transaction XDR using the `sq submit` command.
That will look something like this:

```bash
sq submit --xdr AAAAAgAAAADQTypL...fZDVsq3ibUcAg=
```

Then you can pat yourself on the back and bask in the glow of your amazing new
Stellar Quest badge!

## The Task at Hand

This first quest is a pretty simple one, and it comes almost directly from the
Soroban examples, too! All we're doing here is getting our feet wet, so you
won't need to worry about *writing* any Rust contracts, or anything like that.
Here's what you need to know to complete this quest.

### Explore the Contract Code

If you open up the [`lib.rs` file](src/lib.rs), you'll be able to see some
helpful comments that briefly describe what each portion of the code is doing.

*This contract accepts an argument and responds with a greeting containing the
argument that was supplied.*

A more in-depth dissection of this example smart contract can be found in our
[Pioneer Quest][pq-lib].

### Build the Contract

We can now move on to actually *building* our contract! I know you didn't think you
would make it this far. Give yourself a pat on the back!

*The build process compiles our Rust code that is purpose-built for the
WebAssembly environment that Soroban will run it in.*

If you need some instructions to help you along with this step, you can check
out the [build tutorial][docs-build] in the Soroban documentation.

### Run a Test

You can also look at the [`test.rs` file](src/test.rs), and see it also has some
helpful comments, too.

*This test invokes the contract's `hello()` function, and checks to make sure
the contract gives the expected response.*

Once you've got an understanding of what's happening in the test scenario, go
ahead and run the test, ensuring that our contract is behaving properly.

Just like the file before, the [Pioneer Quest][pq-test] contains a much more
elaborate explanation of this file, and what is happening along each step of the way.

### Deploy to Futurenet

The Stellar Futurenet is a safe playground where your contract code can live and
work while it is still in development and (potentially) unstable.

*Deploying the contract will upload the built binary file to the Futurenet,
making it readily available for invocation in Futurenet's Soroban
environment.*

You can find some guidance on deploying your contract to the Futurenet in the
[deploy tutorial][docs-deploy] from the Soroban documentation.

### Invoke it on Futurenet

Now comes the exciting part! Your job is to **invoke** the smart contract that
you just uploaded to the Futurenet. Doesn't it just make you want to shout with
excitement!?

*Invoking your contract will use the Stellar network to call up the contract you
just uploaded and execute the `hello()` function.*

The [deploy tutorial][docs-deploy] can also give you some guidance on the finer
points of invoking a Futurenet contract, if you need.

## Finish the Quest

If you've made it this far, and you haven't blown up anything, you're probably
on the right track! Now's a great time to use the `sq` CLI to check your work,
and try to claim your prize!

You can find those [instructions here](#check-your-quest-answer).

## Stuck?

If you're hitting a brick wall, and you're not sure what your next move is,
check out [this section](../../README.md#feeling-lost) in our main README. It's
got a couple of suggestions for where you might go from here.

## Further Reading

Now that you've completed your quest, you might have your interest piqued, and
be curious about all sorts of things. Here's a collection of some related
resources that might be of interest to you:

- The official [Soroban Site][soroban] contains announcements, even more
  resources, exciting learning opportunities, and more!
- The [Soroban CAP][cap] (and the related ones) outline the design choices and
  implementations of the Soroban smart contract platform. (Warning: these
  documents can be quite technical.)
- The Soroban documentation contains a number of [example contracts][examples]
  that can help you wrap your mind around even more of the concepts and ideas
  Soroban is built on.
- You can learn more about the [contract lifecycle][contract-lifecycle] in the
  "Learn" section of the Soroban documentation. There are tons of great articles
  here, and you should read through as many of them as you can!
- Not done learning and earning? Don't want to wait for the next Live Quest? Check out the [Sorobanathon][sorobanathon].

[pq-lib]: https://github.com/tyvdh/soroban-quest--pioneer/blob/main/quests/0-hello-world/src/lib.rs
[pq-test]: https://github.com/tyvdh/soroban-quest--pioneer/blob/main/quests/0-hello-world/src/test.rs
[pq-new-quests]: https://github.com/tyvdh/soroban-quest--pioneer#getting-new-quests
[docs-build]: https://soroban.stellar.org/docs/tutorials/build
[docs-deploy]: https://soroban.stellar.org/docs/tutorials/deploy-to-futurenet
[soroban]: https://soroban.stellar.org
[cap]: https://github.com/stellar/stellar-protocol/blob/master/core/cap-0046.md
[examples]: https://soroban.stellar.org/docs/category/examples
[contract-lifecycle]: https://soroban.stellar.org/docs/learn/contract-lifecycle
[sq-site]: https://quest.stellar.org/
[sorobanathon]: https://github.com/stellar/sorobanathon
