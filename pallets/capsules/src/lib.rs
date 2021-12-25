#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;
mod types;

pub use default_weights::WeightInfo;
use frame_support::dispatch::DispatchResult;
pub use pallet::*;
pub use types::*;

use frame_support::traits::ExistenceRequirement::{AllowDeath, KeepAlive};
use frame_support::traits::{Currency, Get, StorageVersion};
use frame_support::traits::{ExistenceRequirement, WithdrawReasons};
use frame_support::PalletId;
use sp_runtime::traits::AccountIdConversion;
use sp_std::vec;
use ternoa_common::traits;
use ternoa_primitives::nfts::{NFTId, NFTSeriesId};
use ternoa_primitives::TernoaString;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use frame_support::transactional;
    use frame_support::{ensure, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::CheckedAdd;
    use sp_std::convert::TryInto;
    use ternoa_common::traits::{LockableNFTs, NFTs};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type WeightInfo: WeightInfo;

        /// Currency used to bill minting fees
        type Currency: Currency<Self::AccountId>;

        type NFTSTrait: traits::LockableNFTs<AccountId = Self::AccountId>
            + traits::NFTs<AccountId = Self::AccountId>;

        /// The minimum length a string may be.
        #[pallet::constant]
        type MinStringLength: Get<u16>;

        /// The maximum length a string may be.
        #[pallet::constant]
        type MaxStringLength: Get<u16>;

        /// The treasury's pallet id, used for deriving its sovereign account ID.
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            if !CapsuleMintFee::<T>::exists() {
                let fee: BalanceOf<T> = 1000000000000000000000u128.try_into().ok().unwrap();
                CapsuleMintFee::<T>::put(fee);

                return 1;
            }

            0
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates an NFT and coverts it into a capsule.
        #[pallet::weight(T::WeightInfo::create())]
        #[transactional]
        pub fn create(
            origin: OriginFor<T>,
            nft_ipfs_reference: TernoaString,
            capsule_ipfs_reference: TernoaString,
            series_id: Option<NFTSeriesId>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let lower_bound = capsule_ipfs_reference.len() >= T::MinStringLength::get() as usize;
            let upper_bound = capsule_ipfs_reference.len() <= T::MaxStringLength::get() as usize;
            ensure!(lower_bound, Error::<T>::TooShortIpfsReference);
            ensure!(upper_bound, Error::<T>::TooLongIpfsReference);

            // Reserve funds
            let amount = CapsuleMintFee::<T>::get();
            Self::send_funds(&who, &Self::account_id(), amount, KeepAlive)?;

            // Create NFT and capsule
            let nft_id = T::NFTSTrait::create_nft(who.clone(), nft_ipfs_reference, series_id)?;
            Self::new_capsule(&who, nft_id, capsule_ipfs_reference.clone(), amount);

            Self::deposit_event(Event::CapsuleDeposit { balance: amount });
            let event = Event::CapsuleCreated {
                owner: who,
                nft_id: nft_id,
                frozen_balance: amount,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        /// Converts an existing NFT into a capsule.
        #[pallet::weight(T::WeightInfo::create_from_nft())]
        #[transactional]
        pub fn create_from_nft(
            origin: OriginFor<T>,
            nft_id: NFTId,
            ipfs_reference: TernoaString,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let lower_bound = ipfs_reference.len() >= T::MinStringLength::get() as usize;
            let upper_bound = ipfs_reference.len() <= T::MaxStringLength::get() as usize;
            ensure!(lower_bound, Error::<T>::TooShortIpfsReference);
            ensure!(upper_bound, Error::<T>::TooLongIpfsReference);

            let nft_owner = T::NFTSTrait::owner(nft_id).ok_or(Error::<T>::NotOwner)?;
            ensure!(who == nft_owner, Error::<T>::NotOwner);

            let is_locked = T::NFTSTrait::locked(nft_id).ok_or(Error::<T>::NotOwner)?;
            ensure!(is_locked == false, Error::<T>::NftLocked);

            let exists = Capsules::<T>::contains_key(nft_id);
            ensure!(!exists, Error::<T>::CapsuleAlreadyExists);

            // Reserve funds
            let amount = CapsuleMintFee::<T>::get();
            Self::send_funds(&who, &Self::account_id(), amount, KeepAlive)?;

            // Create capsule
            Self::new_capsule(&who, nft_id, ipfs_reference.clone(), amount);

            Self::deposit_event(Event::CapsuleDeposit { balance: amount });
            let event = Event::CapsuleCreated {
                owner: who,
                nft_id,
                frozen_balance: amount,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        /// Converts a capsule into an NFT.
        #[pallet::weight(T::WeightInfo::remove())]
        #[transactional]
        pub fn remove(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let mut unused_funds = Default::default();

            Ledgers::<T>::mutate(&who, |x| -> DispatchResult {
                let data = x.as_mut().ok_or(Error::<T>::NotOwner)?;

                let error = Error::<T>::NotOwner;
                let index = data.iter().position(|x| x.0 == nft_id).ok_or(error)?;

                unused_funds = data[index].1;
                Self::send_funds(&Self::account_id(), &who, data[index].1, AllowDeath)?;

                data.swap_remove(index);
                if data.is_empty() {
                    *x = None;
                }

                Capsules::<T>::take(nft_id).ok_or(Error::<T>::InternalError)?;

                Ok(())
            })?;

            let event = Event::CapsuleRemoved {
                nft_id,
                unfrozen_balance: unused_funds,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        /// Adds additional funds to a capsule.
        #[pallet::weight(T::WeightInfo::add_funds())]
        #[transactional]
        pub fn add_funds(
            origin: OriginFor<T>,
            nft_id: NFTId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Ledgers::<T>::mutate(&who, |x| -> DispatchResult {
                let data = x.as_mut().ok_or(Error::<T>::NotOwner)?;
                let error = Error::<T>::NotOwner;
                let index = data.iter().position(|x| x.0 == nft_id).ok_or(error)?;

                Self::send_funds(&who, &Self::account_id(), amount, KeepAlive)?;

                let error = Error::<T>::ArithmeticError;
                data[index].1 = data[index].1.checked_add(&amount).ok_or(error)?;

                Ok(())
            })?;

            let event = Event::CapsuleFundsAdded {
                nft_id,
                balance: amount,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        /// Changes the capsule ipfs reference.
        #[pallet::weight(T::WeightInfo::set_ipfs_reference())]
        pub fn set_ipfs_reference(
            origin: OriginFor<T>,
            nft_id: NFTId,
            ipfs_reference: TernoaString,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let lower_bound = ipfs_reference.len() >= T::MinStringLength::get() as usize;
            let upper_bound = ipfs_reference.len() <= T::MaxStringLength::get() as usize;
            ensure!(lower_bound, Error::<T>::TooShortIpfsReference);
            ensure!(upper_bound, Error::<T>::TooLongIpfsReference);

            Capsules::<T>::mutate(nft_id, |x| -> DispatchResult {
                let data = x.as_mut().ok_or(Error::<T>::NotOwner)?;
                ensure!(data.owner == who, Error::<T>::NotOwner);

                data.ipfs_reference = ipfs_reference.clone();
                Ok(())
            })?;

            let event = Event::CapsuleIpfsReferenceChanged {
                nft_id,
                ipfs_reference,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        /// Sets the Capsule Mint Fee.
        #[pallet::weight(T::WeightInfo::set_capsule_mint_fee())]
        pub fn set_capsule_mint_fee(
            origin: OriginFor<T>,
            capsule_fee: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            CapsuleMintFee::<T>::put(capsule_fee);

            Self::deposit_event(Event::CapsuleMintFeeChanged { fee: capsule_fee });

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A capsule ipfs reference was changed.
        CapsuleIpfsReferenceChanged {
            nft_id: NFTId,
            ipfs_reference: TernoaString,
        },
        /// Additional funds were added to a capsule.
        CapsuleFundsAdded {
            nft_id: NFTId,
            balance: BalanceOf<T>,
        },
        /// A capsule was convert into an NFT.
        CapsuleRemoved {
            nft_id: NFTId,
            unfrozen_balance: BalanceOf<T>,
        },
        /// A capsule was created.
        CapsuleCreated {
            owner: T::AccountId,
            nft_id: NFTId,
            frozen_balance: BalanceOf<T>,
        },
        /// Capsule mint fee has been changed.
        CapsuleMintFeeChanged { fee: BalanceOf<T> },
        /// Some funds have been deposited.
        CapsuleDeposit { balance: BalanceOf<T> },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// This should never happen.
        ArithmeticError,
        /// Callers is not the NFT owner.
        NotOwner,
        /// Ipfs reference is too short.
        TooShortIpfsReference,
        /// Ipfs reference is too long.
        TooLongIpfsReference,
        /// Capsule already exists.
        CapsuleAlreadyExists,
        /// This should never happen.
        InternalError,
        /// NFT is locked.
        NftLocked,
    }

    /// Current capsule mint fee.
    #[pallet::storage]
    #[pallet::getter(fn capsule_mint_fee)]
    pub type CapsuleMintFee<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// List of NFTs that are capsulized.
    #[pallet::storage]
    #[pallet::getter(fn capsules)]
    pub type Capsules<T: Config> =
        StorageMap<_, Blake2_128Concat, NFTId, CapsuleData<T::AccountId>, OptionQuery>;

    /// List of accounts that hold capsulized NFTs.
    #[pallet::storage]
    #[pallet::getter(fn ledgers)]
    pub type Ledgers<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, CapsuleLedger<BalanceOf<T>>, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub capsule_mint_fee: BalanceOf<T>,
        pub capsules: Vec<(NFTId, T::AccountId, TernoaString)>,
        pub ledgers: Vec<(T::AccountId, Vec<(NFTId, BalanceOf<T>)>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                capsule_mint_fee: Default::default(),
                capsules: Default::default(),
                ledgers: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.capsules
                .clone()
                .into_iter()
                .for_each(|(nft_id, account, reference)| {
                    Capsules::<T>::insert(nft_id, CapsuleData::new(account, reference));
                });

            self.ledgers
                .clone()
                .into_iter()
                .for_each(|(account, data)| {
                    Ledgers::<T>::insert(account, data);
                });

            CapsuleMintFee::<T>::put(self.capsule_mint_fee);
        }
    }
}

impl<T: Config> Pallet<T> {
    fn new_capsule(
        owner: &T::AccountId,
        nft_id: NFTId,
        ipfs_reference: TernoaString,
        funds: BalanceOf<T>,
    ) {
        let data = CapsuleData::new(owner.clone(), ipfs_reference.clone());
        Capsules::<T>::insert(nft_id, data);

        Ledgers::<T>::mutate(&owner, |x| {
            if let Some(data) = x {
                data.push((nft_id, funds));
            } else {
                *x = Some(vec![(nft_id, funds)]);
            }
        });
    }

    fn account_id() -> T::AccountId {
        T::PalletId::get().into_account()
    }

    fn send_funds(
        sender: &T::AccountId,
        receiver: &T::AccountId,
        amount: BalanceOf<T>,
        liveness: ExistenceRequirement,
    ) -> DispatchResult {
        let imbalance = T::Currency::withdraw(&sender, amount, WithdrawReasons::FEE, liveness)?;
        T::Currency::resolve_creating(receiver, imbalance);

        Ok(())
    }
}

impl<T: Config> traits::CapsulesTrait for Pallet<T> {
    fn is_capsulized(nft_id: NFTId) -> bool {
        Capsules::<T>::contains_key(&nft_id)
    }
}
