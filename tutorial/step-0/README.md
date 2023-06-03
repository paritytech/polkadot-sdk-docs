# Step 0

This is the shell pallet, that will act as the starting code.

- [Step 0](#step-0)
		- [Project setup.](#project-setup)
		- [State Transition, Background Knowledge](#state-transition-background-knowledge)
		- [In the code](#in-the-code)


## Project setup.

* in `Cargo.toml`, we are only bringing in `frame` package. This is an all-in-one crate to get your
started with FRAME. The rust docs of this frame should be TRIPLE A.
* At this stage, because you are not building any 'runtimes', your pallet is just like a normal rust
  caret that only compiles to native. We will cover this in a later chapter.
* `rustfmt` file is included. Use it.
* Explain broadly speaking that we want to build a currency and staking system, therefore this file
  name was chosen.

## State Transition, Background Knowledge

* Some general information about state transition, pallets and runtime should be covered beforehand.
Use this diagram.

https://mermaid.live/edit#pako:eNqNU1FPwjAQ_itNE8LLJB1sTPZggs4HEsUEDBE7HspWoLHrltKJOPbfbTdBUEy4h7X97rtr79tdAaM0ptCHoWg0CiaY8kHRVCua0KbfFDRXkvBmWTYaoTAUsODpJloRqcBzUAHG7nDEGRVqBq6ubsAIy1woltCZiQnFacg6ny8lyVZgiId0M1ZEGR74toN3rFJJlnRCeE5tXC1HNGMTG9vuEUZFDP5P9Egy-zQ-ZpJGiqUCPIxOPX27quNFX4B-XXpbu6Y2bqOzd-udqdls9-vhKQEO2DojKlodhd4Rzn9OT5ItmTik-ptjgAf6JzHCL1IOnVcOmcLqJ16sHsL6M7tYQoT7nEW07ogXdEZJhG_Tee2fogvkHFTUoNhrWNZwUMG7p7cdGB6Q1h66f9dtuT5h3ku5A4NQQAsmVCaExbr9C0MJYdX4IfT1NqYLknMV6skoNZXkKh1vRQR9JXNqwTyLtf4BI1qnBPoLwtcazYh4TdNkT9JH6BfwA_ptz255bde-Rt220-1dexbcarTntJDnuR2343Qcx_U6pQU_qwSo5VqQxkzr_1hPaDWo5ReGog5x

Terminology that must be covered:

* dispatch ~ call. Has arguments that are like the *payload*.
     * origin: *sender* of a dispatch.
* state ~ storage ~ storage value ~ storage map
* event, akin to return type of pallet
* `DispatchResult`, and `DispatchError`. The only thing we care about `DispatchError` is that it is
  `From<&'static str>`.


* Explain broadly that FRAME achieves giving you this set of abstractions via a set of macros. More
  documentation, plus the full list of macros should be in the docs of the `frame` crate.

## In the code

* Each pallet is coded as a module ie `mod` in Rust. It actually can be broken down into smaller
pieces (https://github.com/paritytech/substrate/pull/13950), but for the sake of simplicity, assume
each pallet must be a rust module for now.
* `dev_mode` will reduce the complation requirements. Will be covered later.
* A shell pallet only needs a config `trait` and a pallet `struct`.
  * The former is, as the name suggest, a way to configure the pallet. We won't need it for
    now.
  * The latter is the main struct that is mandatory to exist in the pallet, and will implement
	all the important functions of the pallet, among dispatchable function.

## Setup

- One should have and use Rust analyzer properly in the remainder of this tutorial.
- One should properly use the rust-docs of the hinted types of FRAME for the remainder of this
  tutorial.
