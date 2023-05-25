# Step 0 - Defining a pallet

This is the shell pallet, that will act as the starting code.

- [Step 0 - Defining a pallet](#step-0---defining-a-pallet)
  - [Context: Project Setup and Background Knowledge](#context-project-setup-and-background-knowledge)
    - [Project Setup](#project-setup)
    - [Background Knowledge: State Transitions](#background-knowledge-state-transitions)
  - [Tutorial Objectives](#tutorial-objectives)
  - [In the Code](#in-the-code)
  - [Implementation](#implementation)


## Context: Project Setup and Background Knowledge

Welcome to the first step in the FRAME master tutorial!  For reference, each step will be structured as follows: 

- Each section of this tutorial will incrementally uncover concepts you need to build a pallet using FRAME
- **Context** - this will give you an overview of what you will acclomplish, along with the end result.
- **In the code** - this will clarify the design decisions made, explain why the code is implemented in this way, and provide more Rust-context for the step.
- **Tutorial Objectives** - An explaination regarding what is being completed in this module, and what functionality it will bring.
- **Implementation** - The code will present some todo items to complete based on the previously explained concepts.  

### Project Setup

Before starting this step, make sure you followed the (insert installation place here) to ensure you have all dependencies to begin development. 

Make sure this repo is cloned, and navigate to `tutorial/frame-pallets/Cargo.toml`:

* In `Cargo.toml`, we are only bringing in `frame` package. This is an all-in-one crate to get started with FRAME. The Rust docs of this frame should be TRIPLE A.

* At this stage, because you are not building any 'runtimes', your pallet is just like a normal rust caret that only compiles to native. We will cover this in a later chapter.
  
* A `rustfmt` file is included - use it.  `cargo clippy` is also encouraged!
  
* Explain broadly speaking that we want to build a currency and staking system, therefore this file name was chosen.

### Background Knowledge: State Transitions

* Some general information about state transition, pallets and runtime should be covered beforehand.
* 
Use this diagram.

https://mermaid.live/edit#pako:eNqNU1FPwjAQ_itNE8LLJB1sTPZggs4HEsUEDBE7HspWoLHrltKJOPbfbTdBUEy4h7X97rtr79tdAaM0ptCHoWg0CiaY8kHRVCua0KbfFDRXkvBmWTYaoTAUsODpJloRqcBzUAHG7nDEGRVqBq6ubsAIy1woltCZiQnFacg6ny8lyVZgiId0M1ZEGR74toN3rFJJlnRCeE5tXC1HNGMTG9vuEUZFDP5P9Egy-zQ-ZpJGiqUCPIxOPX27quNFX4B-XXpbu6Y2bqOzd-udqdls9-vhKQEO2DojKlodhd4Rzn9OT5ItmTik-ptjgAf6JzHCL1IOnVcOmcLqJ16sHsL6M7tYQoT7nEW07ogXdEZJhG_Tee2fogvkHFTUoNhrWNZwUMG7p7cdGB6Q1h66f9dtuT5h3ku5A4NQQAsmVCaExbr9C0MJYdX4IfT1NqYLknMV6skoNZXkKh1vRQR9JXNqwTyLtf4BI1qnBPoLwtcazYh4TdNkT9JH6BfwA_ptz255bde-Rt220-1dexbcarTntJDnuR2343Qcx_U6pQU_qwSo5VqQxkzr_1hPaDWo5ReGog5x

Terminology that must be covered:

* dispatch ~ call. Has arguments that are like the *payload*.
     * origin: *sender* of a dispatch.
* state ~ storage ~ storage value ~ storage map
* event, akin to return type of pallet
* `DispatchResult`, and `DispatchError`. The only thing we care about `DispatchError` is that it is
  `From<&'static str>`.

* Explain broadly that FRAME achieves giving you this set of abstractions via a set of macros. More documentation, plus the full list of macros should be in the docs of the `frame` crate.


## Tutorial Objectives


## In the Code

* Each pallet is coded as a module ie `mod` in Rust. It actually can be broken down into smaller
pieces (https://github.com/paritytech/substrate/pull/13950), but for the sake of simplicity, assume
each pallet must be a rust module for now.
* `dev_mode` will reduce the complation requirements. Will be covered later.
* A shell pallet only needs a config `trait` and a pallet `struct`.
  * The former is, as the name suggest, a way to configure the pallet. We won't need it for
    now.
  * The latter is the main struct that is mandatory to exist in the pallet, and will implement
	all the important functions of the pallet, among dispatchable function.


## Implementation