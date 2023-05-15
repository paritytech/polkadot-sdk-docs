# polkadot-sdk-docs
A holistic, minimal documentation portal for the Polkadot developers.

## Master Tutorial

The very, very rough plan that I have so far is written in [SCRIPT.md](./SCRIPT.md). Note that this topic is chosen such that it complements the imagine drawn in `min_substrate` exercise. This is currently onyl used in PBA, but we hope to make it public, as an independent tutorial. 

This tutorial is being structured in branches. The `main` branch is pretty useless, and only contains empty tutorial folders, and the `staging` folder, which is just my playground. 

Each step of the tutorial has a branch. To minimize merge conflicts, the branches are only code in `.rs` file, with `README.md` in `master-tutorial-frame` (`master-tutorial-ink`, once it is created) that contains high level data-points that should be covered in this step. These `README.md` file can then be used to write: 

* a more structured written document
* a slide deck
* an interactive tutorial
* ... 

For each step, the goal is to compare it with the previous step, and the changes to `.rs` files should ideally contain the main substance of that step. For example, comparing step2 and step1 would look like [this](https://github.com/paritytech/polkadot-sdk-docs/compare/step-1-simple-currency...step-2-testing).

The existing branches: 

- https://github.com/paritytech/polkadot-sdk-docs/tree/step-0-beginner-code
- https://github.com/paritytech/polkadot-sdk-docs/tree/step-1-simple-currency
- https://github.com/paritytech/polkadot-sdk-docs/tree/step-2-testing
