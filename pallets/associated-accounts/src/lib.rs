#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;

pub use default_weights::WeightInfo;
use frame_support::traits::{Get, StorageVersion};
pub use pallet::*;

use ternoa_primitives::TernoaString;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::ensure;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type WeightInfo: WeightInfo;

        /// The minimum length a username string may be.
        #[pallet::constant]
        type MinNameLength: Get<u8>;

        /// The maximum length a username may be.
        #[pallet::constant]
        type MaxNameLength: Get<u8>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set Altvr username
        #[pallet::weight(T::WeightInfo::set_altvr_username())]
        pub fn set_altvr_username(
            origin: OriginFor<T>,
            value: TernoaString,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            let username_lower_bound = value.len() >= T::MinNameLength::get() as usize;
            let username_upper_bound = value.len() <= T::MaxNameLength::get() as usize;

            ensure!(username_lower_bound, Error::<T>::TooShortUsername);
            ensure!(username_upper_bound, Error::<T>::TooLongUsername);

            AltVRUsers::<T>::insert(owner.clone(), value.clone());

            let event = Event::AltVRUsernameChanged(owner, value);
            Self::deposit_event(event);

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Altvr username updated  \[owner, username\]
        AltVRUsernameChanged(T::AccountId, TernoaString),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Altvr username is too short.
        TooShortUsername,
        /// Altvr username is too long.
        TooLongUsername,
    }

    /// List of Altvr datas create.
    #[pallet::storage]
    #[pallet::getter(fn altvr_users)]
    pub type AltVRUsers<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, TernoaString, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub altvr_users: Vec<(T::AccountId, TernoaString)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                altvr_users: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.altvr_users
                .clone()
                .into_iter()
                .for_each(|(owner, username)| {
                    AltVRUsers::<T>::insert(owner.clone(), username);
                });
        }
    }
}
