# Step 6

In this step, we will introduce a dead simple staking pallet, write some basic tests for it, and
integrate it into our runtime. Interactions at the end as per usual.

- [Step 6](#step-6)
	- [information Points](#information-points)
		- [Staking System](#staking-system)
		- [Pallet Hooks](#pallet-hooks)
	- [In the code](#in-the-code)
	- [WHAT HAVE WE DONE?](#what-have-we-done)

## information Points


### Staking System

The staking system's spec:
- anyone can call `register(amount)`, which registers them as "wanna-be validator' with `amount` as
	their approval-stake. - There's no means to unregister yourself for now.
- anyone can call `delegate(who, amount)`, which increases the approval-stake of `who`.
- every x blocks, we want to get the top `x` wanna-be validators based on approval-stake, to the
  best of our abilities. We do this `on_initialize`.
- We want to make sure both validator and delegators actually hold the tokens they have, so we
  tightly couple ourselves with the currency pallet.

### Pallet Hooks

- demonstrated as one of the top notch features of a pallet against a contract.
- Look into the enhanced documentation of `Hooks` to learn more.
- Some basic diagram would be good.
- `on_initialize` should be explained as the most powerful of the hooks, but also one that requires
  the most responsibility.

> TODO: given `fn poll()` on the horizon already, we should quickly retract talking about `fn
> on_initialize`.

## In the code

- The logic of the pallet calls and intended storage items should be pretty self-explanatory given
  the above.
- Notice how `trait Config: system::Config + pallet_currency::Config`. This allows us to directly
  tap into the `Pallet` and storage items of the `pallet_currency`.
- We build a `GenesisConfig` very similar to what we just learned in mod 5.
- Look into `ensure!` and how it is syntactic sugar from FRAME.
- In the tests:
- we build a similar `ExtBuilder`.


TODO: in the future, we can easily come back to this, change the order of Staking and Currency in
the staking tests and see failures.


## WHAT HAVE WE DONE?

We have made a fairly complex pallet/runtime artefact thus far. But truth be told, we have cut
corners. Too many of them. Let's talk about them, and go fix them one by one:

1. Our pallet design is 100% not scalable. Let us be clear: the type of code you write in this step
   is horrible, and you should not attempt to write such garbage in production. Context:
   https://forum.polkadot.network/t/tellling-the-story-of-the-game-theory-behind-frame-pallets/2282/12?u=kianenigma

2. The currency pallet does not prevent transfer of tokens that are locked. We should add reserves
   and.

And many more, but for now we focus on these two, as they are the most critical ones.
