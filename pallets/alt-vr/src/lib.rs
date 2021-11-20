#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;
mod types;

pub use default_weights::WeightInfo;
pub use pallet::*;
pub use types::*;

use frame_support::traits::{Get, StorageVersion};

use ternoa_primitives::ternoa;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use frame_support::{ensure, pallet_prelude::*, transactional};

    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type WeightInfo: WeightInfo;

        /// The minimum length a name string (username or vchatname) may be.
        #[pallet::constant]
        type MinNameLength: Get<u8>;

        /// The maximum length a name string (username or vchatname) may be.
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
        /// Creates altvr informations.
        #[pallet::weight(T::WeightInfo::create_altvr())]
        #[transactional]
        pub fn create_altvr(
            origin: OriginFor<T>,
            username: ternoa::String,
            vchatname: ternoa::String,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            let username_lower_bound = username.len() >= T::MinNameLength::get() as usize;
            let username_upper_bound = username.len() <= T::MaxNameLength::get() as usize;
            let vchatname_lower_bound = vchatname.len() >= T::MinNameLength::get() as usize;
            let vchatname_upper_bound = vchatname.len() <= T::MaxNameLength::get() as usize;

            ensure!(username_lower_bound, Error::<T>::TooShortUsername);
            ensure!(username_upper_bound, Error::<T>::TooLongUsername);
            ensure!(vchatname_lower_bound, Error::<T>::TooShortVchatname);
            ensure!(vchatname_upper_bound, Error::<T>::TooLongVchatname);

            // Create Altvr data
            Self::new_altvr(&owner, &username, &vchatname);

            let event = Event::AltvrCreated(owner, username, vchatname);
            Self::deposit_event(event);

            Ok(().into())
        }

        /// Update Altvr username of the caller
        #[pallet::weight(T::WeightInfo::update_username())]
        #[transactional]
        pub fn update_username(
            origin: OriginFor<T>,
            username: ternoa::String,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            let username_lower_bound = username.len() >= T::MinNameLength::get() as usize;
            let username_upper_bound = username.len() <= T::MaxNameLength::get() as usize;

            ensure!(username_lower_bound, Error::<T>::TooShortUsername);
            ensure!(username_upper_bound, Error::<T>::TooLongUsername);

            Altvrs::<T>::try_mutate(owner.clone(), |res| -> Result<(), Error<T>> {
                let altvrdata = res.as_mut().ok_or(Error::<T>::UserNotFound)?;
                altvrdata.username = username.clone();
                Ok(())
            })?;

            let event = Event::AltvrUsernameUpdated(owner, username);
            Self::deposit_event(event);

            Ok(().into())
        }

        /// Update Altvr vchatname of the caller
        #[pallet::weight(T::WeightInfo::update_vchatname())]
        #[transactional]
        pub fn update_vchatname(
            origin: OriginFor<T>,
            vchatname: ternoa::String,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            let username_lower_bound = vchatname.len() >= T::MinNameLength::get() as usize;
            let username_upper_bound = vchatname.len() <= T::MaxNameLength::get() as usize;

            ensure!(username_lower_bound, Error::<T>::TooShortUsername);
            ensure!(username_upper_bound, Error::<T>::TooLongVchatname);

            Altvrs::<T>::try_mutate(owner.clone(), |res| -> Result<(), Error<T>> {
                let altvrdata = res.as_mut().ok_or(Error::<T>::UserNotFound)?;
                altvrdata.vchatname = vchatname.clone();
                Ok(())
            })?;

            let event = Event::AltvrVchatnameUpdated(owner, vchatname);
            Self::deposit_event(event);

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Altvr was created. \[owner, username, vchatname\]
        AltvrCreated(T::AccountId, ternoa::String, ternoa::String),
        /// Altvr username updated  \[owner, username\]
        AltvrUsernameUpdated(T::AccountId, ternoa::String),
        /// Altvr vchatname updated  \[owner, vchatname\]
        AltvrVchatnameUpdated(T::AccountId, ternoa::String),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Altvr username is too short.
        TooShortUsername,
        /// Altvr username is too long.
        TooLongUsername,
        /// Vchat name is too short.
        TooShortVchatname,
        /// Vchat name is too long.
        TooLongVchatname,
        /// User not found
        UserNotFound,
    }

    /// List of Altvr datas create.
    #[pallet::storage]
    #[pallet::getter(fn altvrs)]
    pub type Altvrs<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AltvrData, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub altvrs: Vec<(T::AccountId, ternoa::String, ternoa::String)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                altvrs: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.altvrs
                .clone()
                .into_iter()
                .for_each(|(owner, username, vchatname)| {
                    Altvrs::<T>::insert(owner.clone(), AltvrData::new(username, vchatname));
                });
        }
    }
}

impl<T: Config> Pallet<T> {
    fn new_altvr(owner: &T::AccountId, username: &ternoa::String, vchatname: &ternoa::String) {
        let data = AltvrData::new(username.clone(), vchatname.clone());
        Altvrs::<T>::insert(owner, data);
    }
}
