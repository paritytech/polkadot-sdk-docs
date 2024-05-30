# DEPRECATED polkadot-sdk-docs

> This reposityory was moslty used as an issue tracker and playground for what has come to fuitition in `polkadot-sdk` repository as `polkadot-sdk-docs` crate: https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/index.html.

--- 

A holistic, minimal documentation portal for the Polkadot developers.

## Master Tutorial

The very, very rough plan that I have so far is written in [SCRIPT.md](./SCRIPT.md). Note that this
topic is chosen such that it complements the imagine drawn in `mini_substrate` exercise in the PBA
Rust entrance exam.

## Tutorial Structure

This tutorial is being structured in different folders. That is, each distinct step is one folder in
the [tutorial](./tutorial/) folder.

The [staging](./staging/) folder is only for the author, and serves as a playground.

Each step of the tutorial, in its current state, is merely:

1. The code required to achieve that step.
2. A README.md file containing very high level instructions about how to achieve those.

These markdown files are not the final version of this tutorial and are meant to be raw material
that can be later used to craft:

1. a written book
2. a slide deck
3. an, indeed, a somewhat interactive tutorial.


The best way to check and study each part is to `diff` the two folders. For that, we suggest a
better toll like `delta`:

```
delta tutorial/step-3 tutorial/step-4
```

The `README` file of step `n` should justify all changes that were made in the code of step `n-1`
against step `n`.
