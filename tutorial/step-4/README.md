# Step 4

In this step, we will only tweak the existing runtime such that it will compile to WASM as well.

- [Step 4](#step-4)
	- [information Points](#information-points)
	- [In the code](#in-the-code)
	- [Interaction](#interaction)

## information Points

This section is about establishing:

- what is `feature = std`
- what is `no_std`
- how we make substrate crates, including the `runtime` build in wasm.

TODO: link to a reference doc page about this. The "FRAME Tips and Tricks in PBA" also covers this.

## In the code

- In the pallets, we now have to update each dependency to have `default-features = false`. This
  disables the `std` feature, and enables it only in the newly declared `[feature] std = [...]`.
- We add the `#![cfg_attr(not(feature = "std"), no_std)]` to the entire crate, which reads: "if
  feature 'std' is not enabled, build this in a no_std env".
- We apply a similar change in the `runtime` section with deps
- We add a `build.rs` that will build the WASM blob. (no need to know the details of
  `substrate_wasm_builder`).

Next, we will add new features that will allow our pallets to have some initial state, aka genesis
state. Then, we will use this in both our tests, and in the node that we run.

## Interaction

At this point, the learner can add this pallet to the node-template provided that only works with
manual-seal, and run it.

TODO: the node template needs to be packed nicely for them to use. See:
https://github.com/paritytech/substrate/issues/13951.

Using 1 node per chapter might be way too hard.
But using just one node is also not feasible. Need something like https://github.com/paritytech/substrate/issues/14268

