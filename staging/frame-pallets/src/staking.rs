use frame::prelude::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use crate::currency;
	use crate::currency::pallet::Balance;
	use currency::pallet::Pallet as CurrencyPallet;
	use frame::derive;

	#[pallet::config]
	pub trait Config: system::Config + currency::pallet::Config {
		/// Number of validators that we want to select.
		type ValidatorCount: Get<u32>;

		/// The number of blocks after which we try and select a new validator set.
		type EraDuration: Get<Self::BlockNumber>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(derive::Encode, derive::Decode, derive::TypeInfo, Eq, PartialEq, Clone, Debug)]
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

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn register(origin: OriginFor<T>, amount: Balance) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// This is shorter than needing to write `if` statements repeatedly.
			ensure!(!Validators::<T>::contains_key(&who), "AlreadyRegistered");
			ensure!(
				currency::pallet::Balances::<T>::get(&who).map_or(false, |b| b >= amount),
				"InsufficientFunds"
			);

			Validators::<T>::insert(&who, ValidatorStake { own: amount, delegated: 0 });

			Ok(())
		}

		pub fn delegate(origin: OriginFor<T>, to: T::AccountId, amount: Balance) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!Delegators::<T>::contains_key(&who), "AlreadyDelegator");
			ensure!(
				currency::pallet::Balances::<T>::get(&who).map_or(false, |b| b >= amount),
				"InsufficientFunds"
			);

			// we can basically remove this because we have transactional.
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

			if (now % T::EraDuration::get()).is_zero() {
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

			// don't care about weight for now.
			Default::default()
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;
		use frame::{primitives, testing_prelude::*, traits, runtime::construct_runtime};

		type Extrinsic = MockUncheckedExtrinsic<Runtime>;
		type Block = MockBlock<Runtime>;

		construct_runtime!(
			pub struct Runtime
			where
				// It really sucks that we have to specify these... but there is really no way.
				// https://github.com/paritytech/substrate/issues/14126
				Block = Block,
				NodeBlock = Block,
				UncheckedExtrinsic = Extrinsic,
			{
				System: frame_system,
				Staking: pallet,
				Currency: currency::pallet,
			}
		);

		impl frame::system::Config for Runtime {
			type RuntimeOrigin = RuntimeOrigin;
			type RuntimeCall = RuntimeCall;
			type RuntimeEvent = RuntimeEvent;
			type PalletInfo = PalletInfo;
			type BaseCallFilter = frame::traits::Everything;
			type OnSetCode = ();

			type AccountId = u64;

			type BlockNumber = u64;
			type Hash = primitives::H256;
			type Hashing = primitives::BlakeTwo256;
			type Lookup = traits::IdentityLookup<Self::AccountId>;
			type Header = <Block as traits::Block>::Header;
			type BlockHashCount = traits::ConstU64<250>;
			type MaxConsumers = traits::ConstU32<16>;
			type BlockWeights = ();
			type BlockLength = ();
			type Index = u64;
			type Version = ();
			type AccountData = ();
			type OnNewAccount = ();
			type OnKilledAccount = ();
			type SystemWeightInfo = ();
			type SS58Prefix = ();
			type DbWeight = ();
		}

		// TODO: if we were to have private `struct` runtime, then these would also not need to be pub.
		parameter_types! {
			pub const ValidatorCount: u32 = 2;
			pub const EraDuration: u64 = 3;
		}

		impl Config for Runtime {
			type ValidatorCount = ValidatorCount;
			type EraDuration = EraDuration;
		}

		impl currency::pallet::Config for Runtime {}

		fn new_test_state() -> TestState {
			let mut state = TestState::new_empty();
			state.execute_with(|| {
				frame_system::Pallet::<Runtime>::set_block_number(1);
				// give everyone some money.
				currency::pallet::Balances::<Runtime>::insert(1, 10);
				currency::pallet::Balances::<Runtime>::insert(2, 20);
				currency::pallet::Balances::<Runtime>::insert(3, 30);
				// register them all as validators
				pallet::Pallet::<Runtime>::register(RuntimeOrigin::signed(1), 10).unwrap();
				pallet::Pallet::<Runtime>::register(RuntimeOrigin::signed(2), 20).unwrap();
				pallet::Pallet::<Runtime>::register(RuntimeOrigin::signed(3), 30).unwrap();
			});
			state
		}

		fn next_block() {
			let now = frame_system::Pallet::<Runtime>::block_number();
			pallet::Pallet::<Runtime>::on_initialize(now);
			frame_system::Pallet::<Runtime>::set_block_number(now + 1);
		}

		#[test]
		fn basic_setup_works() {
			new_test_state().execute_with(|| {
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
			new_test_state().execute_with(|| {
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
	}
}
