# Step 1

In this step we will build a very very simple currency pallet, that can mint and trasnfer tokens.

- [Step 1](#step-1)
	- [State Transition, Background Knowledge](#state-transition-background-knowledge)
	- [In the code](#in-the-code)


## State Transition, Background Knowledge

* Brief intro about what total issuance is, although it is pretty self-explanatory.
* Technically you want `mint` to be permissioned, but for now we don't have the tools, so anyone can mint anything.


## In the code

* `frame::prelude`, followed by `use super::*`. A very common pattern.
* `#[pallet::storage]` declares a type as *state*. For now, we only know
  * Mapping.
  * Single value.
  * `ValueQuery` vs `OptionQuery`. (TODO: imporve RustDocs).
  * How to navigate the storage APIs. No need to explain every single one, just point them to the right rust-docs.
  * eg. `get`, `put`, `insert` and `mutate` has been used.

* Where is `T::AccountId` coming from? better spend some time on this now and explain it well.
* As noted in step0, `Err("AnyString".into())` is acceptable for `DispatchResult`.
