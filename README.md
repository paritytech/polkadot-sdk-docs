# polkadot-sdk-docs
A holistic, minimal documentation portal for the Polkadot developers.

## Master Tutorial

The very, very rough plan that I have so far is written in [SCRIPT.md](./SCRIPT.md). Note that this topic is chosen such that it complements the imagine drawn in `mini_substrate` exercise in the PBA Rust entrance exam.

## Tutorial Structure

This tutorial is being structured in branches. The `main` branch only contains [`staging`](./staging/) folder that contains all of the written code. This is where the author of the tutorial prepares the next steps. Feel free to put whatever you need in there.

each tutorial step is places in a folder with the name `step-{number}-more-info`.

In this folder, we make to changes to `staging`, and instead only make changes to `tutorial`.

In order to develop a new step:

* checkout to the corresponding steps branch.
* merge `main`. This should always be conflict-free as it only brings the latest `staging`.
* merge `step-{n-1}` to get the code of the previous step.
* update content in `tutorial` based on the new step.
* update `README.md` in `tutorial` to reflect the new step.

These `README.md` file can then be used to write:

* a more structured written document
* a slide deck
* an interactive tutorial
* ...

Existing steps:

- https://github.com/paritytech/polkadot-sdk-docs/tree/step-0-beginner-code
- https://github.com/paritytech/polkadot-sdk-docs/tree/step-1-simple-currency
- https://github.com/paritytech/polkadot-sdk-docs/tree/step-2-testing
- https://github.com/paritytech/polkadot-sdk-docs/tree/step-3-runtime-native
- https://github.com/paritytech/polkadot-sdk-docs/tree/step-4-runtime-wasm

In order to review each step, open the diff between step `n` and `n-1` in github, and ignore anything in `staging`.
