use frame::prelude::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use crate::currency::pallet::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Number of validators that we want to select.
		type ValidatorCount: Get<u32>;

		/// The minimum amount that a validator can bond.
		type MinimumValidatorBond: Get<Balance>;

		/// The minimum amount that a validator can bond.
		type MinimumDelegatorBond: Get<Balance>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn register(origin: OriginFor<T>, amount: Balance) -> DispatchResult {
			unimplemented!()
		}

		pub fn delegate(origin: OriginFor<T>, to: T::AccountId, amount: Balance) -> DispatchResult {
			unimplemented!()
		}
	}
}
