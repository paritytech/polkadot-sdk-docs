use frame::prelude::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	pub type Balance = u128;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	/// Mapping from account ID to balance.
	#[pallet::storage]
	pub type Balances<T: Config> = StorageMap<_, _, T::AccountId, Balance>;

	/// Sum of all the tokens in existence.
	#[pallet::storage]
	pub type TotalIssuance<T: Config> = StorageValue<_, Balance, ValueQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Mint `amount` new tokens for `to`.
		pub fn mint(origin: OriginFor<T>, to: T::AccountId, amount: Balance) -> DispatchResult {
			let _anyone = ensure_signed(origin)?;

			Balances::<T>::mutate(to, |b| *b = Some(b.unwrap_or(0) + amount));
			TotalIssuance::<T>::mutate(|t| *t += amount);

			Ok(())
		}

		/// Transfer exactly `amount` from `origin` to `to`. `origin` must exist, and `to` may not.
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: Balance) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let sender_balance = Balances::<T>::get(&sender).ok_or("NonExistentAccount")?;
			if sender_balance < amount {
				return Err("notEnoughBalance".into())
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
		use frame::testing_prelude::*;

		type Extrinsic = MockUncheckedExtrinsic<Runtime>;
		type Block = MockBlock<Runtime>;

		construct_runtime!(
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

		// TODO: this is not optimal
		#[frame::macros::use_attr]
		use frame::deps::frame_support::derive_impl;

		#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
		impl frame_system::Config for Runtime {
			type RuntimeOrigin = RuntimeOrigin;
			type RuntimeCall = RuntimeCall;
			type RuntimeEvent = RuntimeEvent;
			type PalletInfo = PalletInfo;
			type BaseCallFilter = frame::traits::Everything;
			type OnSetCode = ();
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

				// then:
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

		#[test]
		fn transfer_from_non_existent_fails() {
			test_state().execute_with(|| {
				// given the the initial state, when:
				assert_err!(
					pallet::Pallet::<Runtime>::transfer(RuntimeOrigin::signed(EVE), ALICE, 10),
					"NonExistentAccount"
				);

				// then nothing has changed.
				assert_eq!(pallet::Balances::<Runtime>::get(&ALICE), Some(100));
				assert_eq!(pallet::Balances::<Runtime>::get(&BOB), Some(100));
				assert_eq!(pallet::Balances::<Runtime>::get(&EVE), None);
				assert_eq!(pallet::TotalIssuance::<Runtime>::get(), 200);

				// in fact, this frame-helper ensures that nothing in the state has been updated
				// prior and after execution:
				assert_noop!(
					pallet::Pallet::<Runtime>::transfer(RuntimeOrigin::signed(EVE), ALICE, 10),
					"NonExistentAccount"
				);
			});
		}
	}
}
