use frame::prelude::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use crate::currency::pallet::{self as pallet_currency, Balance};
	use frame::derive::{Decode, DefaultNoBound, Encode, TypeInfo};

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_currency::Config {
		type ValidatorCount: Get<u32>;
		type EraDuration: Get<Self::BlockNumber>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, TypeInfo, Eq, PartialEq, Clone, Debug)]
	pub struct ValidatorStake {
		pub(crate) own: Balance,
		pub(crate) delegated: Balance,
	}

	#[pallet::storage]
	pub type Validators<T: Config> = StorageMap<_, _, T::AccountId, ValidatorStake>;

	#[pallet::storage]
	pub type Delegators<T: Config> = StorageMap<_, _, T::AccountId, Balance>;

	#[pallet::storage]
	pub type ActiveValidators<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[derive(DefaultNoBound)]
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		validators: Vec<(T::AccountId, Balance)>,
		delegators: Vec<(T::AccountId, T::AccountId, Balance)>,
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			use frame::deps::frame_support::assert_ok;
			use frame_system::RawOrigin;
			for (validator, self_stake) in &self.validators {
				let raw_origin = RawOrigin::Signed(validator.clone());

				assert_ok!(Pallet::<T>::register(raw_origin.into(), *self_stake));
			}

			for (delegator, delegatee, stake) in &self.delegators {
				let raw_origin = RawOrigin::Signed(delegator.clone());
				assert_ok!(Pallet::<T>::delegate(raw_origin.into(), delegatee.clone(), *stake));
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn register(origin: OriginFor<T>, amount: Balance) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!Validators::<T>::contains_key(&who), "AlreadyRegistered");
			ensure!(
				pallet_currency::Balances::<T>::get(&who).map_or(false, |b| b >= amount),
				"InsufficientFunds"
			);

			Validators::<T>::insert(&who, ValidatorStake { own: amount, delegated: 0 });

			Ok(())
		}

		pub fn delegate(origin: OriginFor<T>, to: T::AccountId, amount: Balance) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!Delegators::<T>::contains_key(&who), "AlreadyDelegator");
			ensure!(
				pallet_currency::Balances::<T>::get(&who).map_or(false, |b| b >= amount),
				"InsufficientFunds"
			);

			// TODO: we can basically remove this because we have transactional.
			ensure!(Validators::<T>::contains_key(&to), "NotRegistered");

			Delegators::<T>::insert(&who, amount);
			Validators::<T>::mutate(&to, |maybe_stake| {
				maybe_stake.as_mut().map(|mut stake| stake.delegated += amount)
			});

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(now: T::BlockNumber) -> Weight {
			use frame::traits::Zero;

			if (now % T::EraDuration::get()).is_zero() && !now.is_zero() {
				let mut all_validators = Validators::<T>::iter().collect::<Vec<_>>();
				all_validators.sort_by_key(|(_, stake)| stake.own + stake.delegated);
				all_validators.reverse();
				ActiveValidators::<T>::put(
					all_validators
						.into_iter()
						.take(T::ValidatorCount::get() as usize)
						.map(|(acc, _)| acc)
						.collect::<Vec<_>>(),
				);
			}

			Default::default()
		}
	}

	#[cfg(test)]
	mod tests {
		use super::pallet as pallet_staking;
		use crate::currency::pallet::{self as pallet_currency, Balance};
		use frame::{prelude::*, testing_prelude::*};
		use pallet_staking::{ActiveValidators, ValidatorStake, Validators};

		#[frame::macros::use_attr]
		use frame::deps::frame_support::derive_impl;

		type Extrinsic = MockUncheckedExtrinsic<Runtime>;
		type Block = MockBlock<Runtime>;

		construct_runtime!(
			pub struct Runtime
			where
				// TODO It really sucks that we have to specify these... but there is really no way.
				// https://github.com/paritytech/substrate/issues/14126
				Block = Block,
				NodeBlock = Block,
				UncheckedExtrinsic = Extrinsic,
			{
				System: frame_system,
				Currency: pallet_currency,
				Staking: pallet_staking,
			}
		);

		#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
		impl frame_system::Config for Runtime {
			type BaseCallFilter = frame_support::traits::Everything;
			type RuntimeOrigin = RuntimeOrigin;
			type RuntimeCall = RuntimeCall;
			type RuntimeEvent = RuntimeEvent;
			type PalletInfo = PalletInfo;
			type OnSetCode = ();
		}

		// TODO: if we were to have private `struct` runtime, then these would also not need to be
		// pub.
		parameter_types! {
			pub static ValidatorCount: u32 = 2;
			pub const EraDuration: <Runtime as frame_system::Config>::BlockNumber = 3;
		}

		impl pallet_staking::Config for Runtime {
			type ValidatorCount = ValidatorCount;
			type EraDuration = EraDuration;
		}

		impl pallet_currency::Config for Runtime {}

		struct ExtBuilder {
			validators: Vec<(<Runtime as frame_system::Config>::AccountId, Balance)>,
			delegators: Vec<(
				<Runtime as frame_system::Config>::AccountId,
				<Runtime as frame_system::Config>::AccountId,
				Balance,
			)>,
			balances: Vec<(<Runtime as frame_system::Config>::AccountId, Balance)>,
		}

		impl Default for ExtBuilder {
			fn default() -> Self {
				let instance = Self {
					validators: Default::default(),
					delegators: Default::default(),
					balances: Default::default(),
				};
				instance.add_validator(1, 10).add_validator(2, 20).add_validator(3, 30)
			}
		}

		impl ExtBuilder {
			fn add_validator(
				mut self,
				validator: <Runtime as frame_system::Config>::AccountId,
				self_stake: Balance,
			) -> Self {
				self.balances.push((validator, self_stake));
				self.validators.push((validator, self_stake));
				self
			}

			fn add_delegator(
				mut self,
				delegator: <Runtime as frame_system::Config>::AccountId,
				delegatee: <Runtime as frame_system::Config>::AccountId,
				stake: Balance,
			) -> Self {
				self.balances.push((delegator, stake));
				self.delegators.push((delegator, delegatee, stake));
				self
			}

			fn build_and_execute(self, test: impl FnOnce() -> ()) {
				// In this example, we care about the order of genesis-initialization, so we use the
				// alternative syntax.
				let mut storage: Storage = Default::default();
				frame_system::GenesisConfig::default()
					.assimilate_storage::<Runtime>(&mut storage)
					.unwrap();
				pallet_currency::GenesisConfig::<Runtime> { balances: self.balances }
					.assimilate_storage(&mut storage)
					.unwrap();
				pallet_staking::GenesisConfig::<Runtime> {
					validators: self.validators,
					delegators: self.delegators,
				}
				.assimilate_storage(&mut storage)
				.unwrap();

				let mut ext = TestState::new(storage);
				// process block 0 to simulate a proper genesis. Not mandatory to be done this way.
				// This sets the current block number (to be executed) to 1.
				ext.execute_with(next_block);
				ext.execute_with(test);
			}
		}

		fn next_block() {
			let now = frame_system::Pallet::<Runtime>::block_number();
			pallet_staking::Pallet::<Runtime>::on_initialize(now);
			frame_system::Pallet::<Runtime>::set_block_number(now + 1);
		}

		#[test]
		fn basic_setup_works() {
			ExtBuilder::default().build_and_execute(|| {
				assert_eq!(frame_system::Pallet::<Runtime>::block_number(), 1);
				assert_eq!(
					Validators::<Runtime>::get(1).unwrap(),
					ValidatorStake { own: 10, delegated: 0 }
				);
				assert_eq!(
					Validators::<Runtime>::get(2).unwrap(),
					ValidatorStake { own: 20, delegated: 0 }
				);
				assert_eq!(
					Validators::<Runtime>::get(3).unwrap(),
					ValidatorStake { own: 30, delegated: 0 }
				);
				assert_eq!(Validators::<Runtime>::iter().count(), 3);
				assert!(ActiveValidators::<Runtime>::get().is_empty());
			})
		}

		#[test]
		fn selects_validators() {
			ExtBuilder::default().build_and_execute(|| {
				// given initial state,

				// when processing block 1, nothing will happen.
				next_block();
				assert!(ActiveValidators::<Runtime>::get().is_empty());

				// when processing block 2, nothing will happen.
				next_block();
				assert!(ActiveValidators::<Runtime>::get().is_empty());

				// when processing block 3, new validators will be selected.
				next_block();
				assert_eq!(ActiveValidators::<Runtime>::get(), vec![3, 2]);
			})
		}

		#[test]
		fn considers_delegators() {
			// typically 2 and 3 win, and 1 and 3
			ExtBuilder::default().add_delegator(42, 1, 30).build_and_execute(|| {
				// given initial state,
				assert!(pallet_staking::Delegators::<Runtime>::get(42).is_some());

				// when processing block 1 and 2, nothing will happen.
				next_block();
				next_block();
				assert!(ActiveValidators::<Runtime>::get().is_empty());

				// when processing block 3, new validators will be selected.
				next_block();
				assert_eq!(ActiveValidators::<Runtime>::get(), vec![1, 3]);
			})
		}

		#[test]
		fn selects_right_number_of_validators() {
			ExtBuilder::default().build_and_execute(|| {
				// when processing block 1 and 2, nothing will happen.
				next_block();
				next_block();
				assert!(ActiveValidators::<Runtime>::get().is_empty());

				// set the `Get` implementor static test variable to 3.
				ValidatorCount::set(3);

				next_block();
				assert_eq!(ActiveValidators::<Runtime>::get(), vec![3, 2, 1]);

				// this time, set to 1.
				next_block();
				next_block();
				assert_eq!(ActiveValidators::<Runtime>::get(), vec![3, 2, 1]);

				ValidatorCount::set(1);
				next_block();
				assert_eq!(ActiveValidators::<Runtime>::get(), vec![3]);
			})
		}
	}
}
