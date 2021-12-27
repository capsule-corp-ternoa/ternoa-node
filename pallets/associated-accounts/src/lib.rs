#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;

pub use default_weights::WeightInfo;
use frame_support::traits::{Get, StorageVersion};
pub use pallet::*;

use ternoa_primitives::TextFormat;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use ternoa_common::helpers::check_bounds;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type WeightInfo: WeightInfo;

        /// Min Altvr name len.
        #[pallet::constant]
        type MinAltvrUsernameLen: Get<u16>;

        /// Max Altvr name len.
        #[pallet::constant]
        type MaxAltvrUsernameLen: Get<u16>;
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
            value: TextFormat,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            check_bounds(
                value.len(),
                (
                    T::MinAltvrUsernameLen::get(),
                    Error::<T>::TooShortAltvrUsername,
                ),
                (
                    T::MaxAltvrUsernameLen::get(),
                    Error::<T>::TooLongAltvrUsername,
                ),
            )?;

            AltVRUsers::<T>::insert(owner.clone(), value.clone());

            let event = Event::AltVRUsernameChanged {
                owner,
                username: value,
            };
            Self::deposit_event(event);

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Altvr username updated.
        AltVRUsernameChanged {
            owner: T::AccountId,
            username: TextFormat,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Altvr username is too short.
        TooShortAltvrUsername,
        /// Altvr username is too long.
        TooLongAltvrUsername,
    }

    /// List of Altvr datas create.
    #[pallet::storage]
    #[pallet::getter(fn altvr_users)]
    pub type AltVRUsers<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, TextFormat, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub altvr_users: Vec<(T::AccountId, TextFormat)>,
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
