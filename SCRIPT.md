# Script for the Master Tutorial

## Part 0: What are we building?

- What was a runtime and pallet again?
- What is the anatomy of a pallet?
	- config, call, pallet, event, error, hook

## Part 1: Build a simple balance pallet

### Step 1: Simplest pallet
- `type Balances = StorageMap<Key = T::AccountId, Value = Balance>`
- `type TotalIssuance = StorageValue<Balance>`
- basic call to `transfer` from a to b.
- basic call to mint if anyone is calling
- 100% using `dev_mode`.

### Step 2 : Tests
- Adding pallet to a mock runtime.
- basic tests.
- introduction to all interactions

### Step 3: Interactions

#### Adding pallet to the node-template runtime + `#[pallet::genesis_config]`

#### PJS-Apps

#### subxt

#### CAPI/PJS-API

(Optional)

> After this, all steps should have an section where we update the tests according to the new steps, and interact with the chain using one or more of the tools

## Part 2: Build a BROKEN Validator Selection Pallet

### Step 1: Basic Staking; use of `on_initialize`

- anyone can call `register(amount)`, which registers them as "wanna-be validator' with `amount` as their approval-stake.
	- There's no means to unregister yourself for now.
- anyone can call `delegate(who, amount)`, which increases the approval-stake of `who`.
- tightly coupled with currency, we check that any of the above two have enough funds
	- but not reserving happens.
- every x blocks, we want to get the top `x` wanna-be validators based on approval-stake, to the best of our abilities.
- write tests for this as well.

But, before going any further,  we should acknowledge that this is broken in different ways.

* We need to add reserves, and make funds not transferrable.
	* A test can demonstrate this.
* It is not Sybil resistent + Scalable.

The full answer should be here: https://forum.polkadot.network/t/tellling-the-story-of-the-game-theory-behind-frame-pallets/2282/12?u=kianenigma

### Step 2 Adding Reserves

- Adding reserve functionality on the balances side
- Fixing staking to use the reserves.
- Updating tests.
- Adding a reserve high enough for `register`.
- Potentially adding an absolute maximum as well, a good segway into `CountedMap`.

> This will only fix this using the crypto-economic axis. Once we build a governance system, we will also allow the onchain governance to whitelist a set of accounts.

### Step 3: Permissionless sort

Allow anyone to submit a the top x for a good amount of deposit. Reward if success, slash if fail.

### Step 4: Blockspace scaling with `on_idle`

TODO: I want to add another alternative here, where we gradually sort the list, to showcase the edge that FRAME has over ink! a bit more.

This can be considered optional.

## Part 3: Facelift Session

> I plan to have multiple "facelift" stop points in the tutorial, where we make progress fast, but cut corners, and once we have something tangible, we come back to it.

At this point, we have something that kinda kinda works. It is time to take a step back, and improve where we have cut corners. Ideas:
- Start using safe math.
- Start properly using pallet errors.
- Start properly introducing events.
- Update tests to reflect both. `assert_events` as a practice.
- introduce `ensure_root`, us it for mint and other permissioned operations.
- Loosely couple Staking and Currency.
	- First using a custom fake trait.
	- Then using `fungibles::*`.
- Introduce the concept of `ExistentialDeposit` (good one, useful IRL).
- For account storage, use system (mehhh).
- Talk about the choice of `Get<_>` as opposed to storage items for configurations.


## Other Part Ideas:
- build a simple governance system.
- build a sudo pallet. Or, build something that is like council but for validators, such that 3/4 of validators can dispatch anything. Anything that covers custom origins will be great.
- build a liquid staking system where we issue representative tokens. If a governance pallet exists, making the liquid tokens usable for governance.
- Still need a good reason to use a double/N map, and do a deep dive on storage.
- At some point, really circle back to the fact that: This code is leveraging `transactional`,
- Adding transaction-payment and configuring it to the runtime.

## Appendix: List of all FRAME macros

Useful to think what is being covered where.

- [ ] call
- [ ] call_index(_)
- [ ] compact
- [ ] composite_enum
- [ ] config
- [ ] constant
- [ ] disable_frame_system_supertrait_check
- [ ] error
- [ ] event
- [ ] extra_constants
- [ ] generate_deposit
- [ ] generate_storage_info
- [ ] genesis_build
- [ ] genesis_config
- [ ] hooks
- [ ] inherent
- [ ] origin
- [ ] pallet
- [ ] storage
- [ ] storage_prefix
- [ ] type_value
- [ ] unbounded
- [ ] validate_unsigned
- [ ] weight
- [ ] whitelist_storage
- [ ] without_storage_info
