use frame::prelude::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	pub type Balance = u128;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// TODO: not sure why the pallet macro stubs are not in scope and docs not working.
	#[pallet::storage]
	pub type Balances<T: Config> = StorageMap<_, _, T::AccountId, Balance>;

	#[pallet::storage]
	pub type TotalIssuance<T: Config> = StorageValue<_, Balance, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn mint(origin: OriginFor<T>, to: T::AccountId, amount: Balance) -> DispatchResult {
			let _anyone = ensure_signed(origin)?;

			Balances::<T>::mutate(to, |b| *b = Some(b.unwrap_or(0) + amount));
			TotalIssuance::<T>::mutate(|t| *t += amount);

			Ok(())
		}

		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: Balance) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let sender_balance = Balances::<T>::get(&sender).ok_or("NonExistentAccount")?;
			if sender_balance < amount {
				return Err("notEnoughBalance".into());
			}
			let reminder = sender_balance - amount;

			Balances::<T>::mutate(to, |b| *b = Some(b.unwrap_or(0) + amount));
			Balances::<T>::insert(&sender, reminder);

			Ok(())
		}
	}

	#[cfg(test)]
	mod tests {
		use super::*;
		use frame::{primitives, testing::prelude::*, traits};

		type Extrinsic = MockUncheckedExtrinsic<Runtime>;
		type Block = MockBlock<Runtime>;

		frame_support::construct_runtime!(
			pub struct Runtime
			where
				Block = Block,
				NodeBlock = Block,
				UncheckedExtrinsic = Extrinsic,
			{
				System: frame_system,
				Currency: pallet,
			}
		);

		impl frame_system::Config for Runtime {
			type RuntimeOrigin = RuntimeOrigin;
			type RuntimeCall = RuntimeCall;
			type RuntimeEvent = RuntimeEvent;
			type PalletInfo = PalletInfo;

			type BlockWeights = ();
			type BlockLength = ();
			type Index = u64;
			type BlockNumber = u64;
			type AccountId = u64;
			type BaseCallFilter = frame::traits::Everything;
			type Hash = primitives::H256;
			type Hashing = primitives::BlakeTwo256;
			type Lookup = traits::IdentityLookup<Self::AccountId>;
			type Header = <Block as traits::Block>::Header;
			type BlockHashCount = traits::ConstU64<250>;
			type MaxConsumers = traits::ConstU32<16>;
			type Version = ();
			type AccountData = ();
			type OnNewAccount = ();
			type OnKilledAccount = ();
			type SystemWeightInfo = ();
			type SS58Prefix = ();
			type OnSetCode = ();
			type DbWeight = ();
		}

		impl pallet::Config for Runtime {}

		const ALICE: <Runtime as frame_system::Config>::AccountId = 1;
		const BOB: <Runtime as frame_system::Config>::AccountId = 2;
		const EVE: <Runtime as frame_system::Config>::AccountId = 3;

		fn test_state() -> TestState {
			let mut state = TestState::new_empty();
			state.execute_with(|| {
				pallet::Balances::<Runtime>::insert(&ALICE, 100);
				pallet::Balances::<Runtime>::insert(&BOB, 100);
				pallet::TotalIssuance::<Runtime>::put(200);
			});

			state
		}

		#[test]
		fn initial_state_works() {
			test_state().execute_with(|| {
				assert_eq!(pallet::Balances::<Runtime>::get(&ALICE), Some(100));
				assert_eq!(pallet::Balances::<Runtime>::get(&BOB), Some(100));
				assert_eq!(pallet::Balances::<Runtime>::get(&EVE), None);
				assert_eq!(pallet::TotalIssuance::<Runtime>::get(), 200);
			});
		}
		#[test]
		fn test_mint() {
			test_state().execute_with(|| {
				// given the initial state, when:
				assert_ok!(pallet::Pallet::<Runtime>::mint(RuntimeOrigin::signed(ALICE), BOB, 100));

				// then:
				assert_eq!(pallet::Balances::<Runtime>::get(&BOB), Some(200));
				assert_eq!(pallet::TotalIssuance::<Runtime>::get(), 300);

				// given:
				assert_ok!(pallet::Pallet::<Runtime>::mint(RuntimeOrigin::signed(ALICE), EVE, 100));

				// then:
				assert_eq!(pallet::Balances::<Runtime>::get(&EVE), Some(100));
				assert_eq!(pallet::TotalIssuance::<Runtime>::get(), 400);
			});
		}

		#[test]
		fn transfer_works() {
			test_state().execute_with(|| {
				// given the the initial state, when:
				assert_ok!(pallet::Pallet::<Runtime>::transfer(
					RuntimeOrigin::signed(ALICE),
					BOB,
					50
				));

				// them:
				assert_eq!(pallet::Balances::<Runtime>::get(&ALICE), Some(50));
				assert_eq!(pallet::Balances::<Runtime>::get(&BOB), Some(150));
				assert_eq!(pallet::TotalIssuance::<Runtime>::get(), 200);

				// when:
				assert_ok!(pallet::Pallet::<Runtime>::transfer(
					RuntimeOrigin::signed(BOB),
					ALICE,
					50
				));

				// then:
				assert_eq!(pallet::Balances::<Runtime>::get(&ALICE), Some(100));
				assert_eq!(pallet::Balances::<Runtime>::get(&BOB), Some(100));
				assert_eq!(pallet::TotalIssuance::<Runtime>::get(), 200);
			});
		}
	}
}
