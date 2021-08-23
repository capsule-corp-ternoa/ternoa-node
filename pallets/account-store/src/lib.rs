//! This pallet implements the `StoredMap` trait for compatibility
//! with `pallet_balances` instantiation.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::StoredMap;
pub use pallet::*;
use sp_runtime::DispatchError;

#[frame_support::pallet]
pub mod pallet {
    use codec::FullCodec;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Data to be associated with an account.
        type AccountData: Member + FullCodec + Clone + Default;
    }

    pub type AccountDataOf<T> = <T as Config>::AccountData;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Account states stored in this pallet
    #[pallet::storage]
    #[pallet::getter(fn account)]
    pub type Account<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AccountDataOf<T>, ValueQuery>;
}

impl<T: Config> StoredMap<T::AccountId, AccountDataOf<T>> for Pallet<T> {
    fn get(k: &T::AccountId) -> AccountDataOf<T> {
        Account::<T>::get(k)
    }

    fn try_mutate_exists<R, E: From<DispatchError>>(
        k: &T::AccountId,
        f: impl FnOnce(&mut Option<AccountDataOf<T>>) -> Result<R, E>,
    ) -> Result<R, E> {
        // Just proxy this to our storage instance
        Account::<T>::try_mutate_exists(k, f)
    }
}
