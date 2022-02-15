#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod types;
pub mod weights;

use frame_support::traits::StorageVersion;
pub use pallet::*;
pub use types::*;
pub use weights::WeightInfo;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec;
    use sp_std::vec::Vec;
    use ternoa_common::helpers::check_bounds;
    use ternoa_primitives::TextFormat;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Weight
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set account
        #[pallet::weight(T::WeightInfo::set_account())]
        pub fn set_account(
            origin: OriginFor<T>,
            account_key: TextFormat,
            account_value: TextFormat,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let supported_accounts = Self::supported_accounts();
            let supported_account = supported_accounts.iter().find(|x| x.key == account_key);
            let supported_account = supported_account.ok_or(Error::<T>::UnknownAccountKey)?;

            check_bounds(
                account_value.len(),
                (supported_account.min_length, Error::<T>::ValueIsTooShort),
                (supported_account.max_length, Error::<T>::ValueIsTooLong),
            )?;

            let mut pays_fee = true;
            Users::<T>::mutate(&who, |maybe_accounts| {
                if let Some(accounts) = maybe_accounts {
                    let pos = accounts
                        .iter()
                        .position(|account| account.key == account_key);
                    if let Some(pos) = pos {
                        accounts[pos].value = account_value.clone();
                    } else {
                        accounts.push(Account::new(account_key.clone(), account_value.clone()));
                        pays_fee = supported_account.initial_set_fee;
                    }
                } else {
                    let account = Account::new(account_key.clone(), account_value.clone());
                    *maybe_accounts = Some(vec![account]);
                    pays_fee = supported_account.initial_set_fee;
                }
            });

            let event = Event::UserAccountAdded {
                user: who,
                account_key,
                account_value,
            };
            Self::deposit_event(event);

            if pays_fee {
                Ok(Pays::Yes.into())
            } else {
                Ok(Pays::No.into())
            }
        }

        #[pallet::weight(T::WeightInfo::add_new_supported_account())]
        pub fn add_new_supported_account(
            origin: OriginFor<T>,
            key: TextFormat,
            min_length: u16,
            max_length: u16,
            initial_set_fee: bool,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            SupportedAccounts::<T>::mutate(|x| {
                x.push(SupportedAccount {
                    key: key.clone(),
                    min_length,
                    max_length,
                    initial_set_fee,
                });
            });
            let event = Event::SupportedAccountAdded {
                key,
                min_length,
                max_length,
                initial_set_fee,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::remove_supported_account())]
        pub fn remove_supported_account(
            origin: OriginFor<T>,
            key: TextFormat,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            SupportedAccounts::<T>::mutate(|supported_accounts| {
                let pos = supported_accounts.iter().position(|x| x.key == key);
                if let Some(pos) = pos {
                    supported_accounts.remove(pos);
                }
            });

            let event = Event::SupportedAccountRemoved { key };
            Self::deposit_event(event);

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UserAccountAdded {
            user: T::AccountId,
            account_key: TextFormat,
            account_value: TextFormat,
        },
        SupportedAccountAdded {
            key: TextFormat,
            min_length: u16,
            max_length: u16,
            initial_set_fee: bool,
        },
        SupportedAccountRemoved {
            key: TextFormat,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Value length is lower than the minimal allowed length.
        ValueIsTooShort,
        /// Value length is higher than the maximal allowed length.
        ValueIsTooLong,
        /// That account key was not found.
        UnknownAccountKey,
    }

    /// List of Altvr datas create.
    #[pallet::storage]
    #[pallet::getter(fn users)]
    pub type Users<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Account>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn supported_accounts)]
    pub type SupportedAccounts<T: Config> = StorageValue<_, Vec<SupportedAccount>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub users: Vec<(T::AccountId, Vec<Account>)>,
        pub supported_accounts: Vec<SupportedAccount>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                users: Default::default(),
                supported_accounts: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.users
                .clone()
                .into_iter()
                .for_each(|(owner, accounts)| {
                    Users::<T>::insert(owner.clone(), accounts);
                });

            SupportedAccounts::<T>::put(self.supported_accounts.clone());
        }
    }
}
