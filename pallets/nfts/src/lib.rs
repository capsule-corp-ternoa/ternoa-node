#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;
mod migrations;

pub use default_weights::WeightInfo;
pub use pallet::*;

use frame_support::pallet_prelude::{ensure, DispatchError};
use frame_support::traits::{Get, StorageVersion};
use sp_runtime::DispatchResult;
use sp_std::result;
use sp_std::vec::Vec;
use ternoa_common::traits::{LockableNFTs, NFTs};
use ternoa_primitives::nfts::{NFTData, NFTDetails, NFTId, NFTSeriesDetails, NFTSeriesId};

const STORAGE_VERSION: StorageVersion = StorageVersion::new(5);

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::{Currency, OnUnbalanced, WithdrawReasons};
    use frame_support::{pallet_prelude::*, transactional};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::StaticLookup;
    use sp_runtime::DispatchResult;
    use ternoa_common::traits::NFTs;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type WeightInfo: WeightInfo;
        /// Currency used to bill minting fees
        type Currency: Currency<Self::AccountId>;
        /// Host much does it cost to mint a NFT (extra fee on top of the tx fees)
        type MintFee: Get<BalanceOf<Self>>;
        /// What we do with additional fees
        type FeesCollector: OnUnbalanced<NegativeImbalanceOf<Self>>;
    }

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::NegativeImbalance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            migrations::migrate::<T>()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new NFT with the provided details. An ID will be auto
        /// generated and logged as an event, The caller of this function
        /// will become the owner of the new NFT.
        #[pallet::weight(if details.series_id == NFTSeriesId::default() {T::WeightInfo::create()} else {T::WeightInfo::create_with_series()})]
        // have to be transactional otherwise we could make people pay the mint
        // even if the creation fails.
        #[transactional]
        pub fn create(origin: OriginFor<T>, details: NFTDetails) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let imbalance = T::Currency::withdraw(
                &who,
                T::MintFee::get(),
                WithdrawReasons::FEE,
                frame_support::traits::ExistenceRequirement::KeepAlive,
            )?;
            T::FeesCollector::on_unbalanced(imbalance);

            let _id = <Self as NFTs>::create(&who, details)?;

            Ok(().into())
        }

        /// Update the details included in an NFT. Must be called by the owner of
        /// the NFT and while the NFT is not sealed.
        #[pallet::weight(T::WeightInfo::mutate())]
        pub fn mutate(
            origin: OriginFor<T>,
            id: NFTId,
            details: NFTDetails,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <Self as NFTs>::mutate(id, |owner, dets| -> DispatchResult {
                ensure!(owner == &who, Error::<T>::NotOwner);

                *dets = details;

                Ok(())
            })?;

            Ok(().into())
        }

        /// Transfer an NFT from an account to another one. Must be called by the
        /// actual owner of the NFT.
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            id: NFTId,
            to: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let to_unlookup = T::Lookup::lookup(to)?;
            let mut data = Data::<T>::get(id);

            ensure!(data.owner == who, Error::<T>::NotOwner);
            ensure!(!data.locked, Error::<T>::Locked);

            data.owner = to_unlookup.clone();
            Data::<T>::insert(id, data);

            Self::deposit_event(Event::Transfer(id, who, to_unlookup));

            Ok(().into())
        }

        /// Mark an NFT as sealed, thus disabling further details modifications (but
        /// not preventing future transfers). Must be called by the owner of the NFT.
        #[pallet::weight(T::WeightInfo::seal())]
        pub fn seal(origin: OriginFor<T>, id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let mut data = Data::<T>::get(id);

            ensure!(!data.sealed, Error::<T>::Sealed);
            ensure!(data.owner == who, Error::<T>::NotOwner);

            data.sealed = true;
            Data::<T>::insert(id, data);

            Self::deposit_event(Event::Sealed(id));

            Ok(().into())
        }

        /// Remove an NFT from the storage. This operation is irreversible which means
        /// once the NFT is removed (burned) from the storage there is no way to
        /// get it back.
        /// Must be called by the owner of the NFT.
        #[pallet::weight(T::WeightInfo::burn())]
        pub fn burn(origin: OriginFor<T>, id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let data = Data::<T>::get(id);

            ensure!(data.owner == who, Error::<T>::NotOwner);
            ensure!(!data.locked, Error::<T>::Locked);
            <Self as NFTs>::burn(id).expect("Call to Burn function should never fail!");

            Ok(().into())
        }

        /// Transfer an NFT series from one account to another one. Must be called by the
        /// actual owner of the NFT series.
        #[pallet::weight(T::WeightInfo::transfer_series())]
        pub fn transfer_series(
            origin: OriginFor<T>,
            id: NFTSeriesId,
            to: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let to_unlookup = T::Lookup::lookup(to)?;

            ensure!(id != NFTSeriesId::default(), Error::<T>::NotSeriesOwner);
            Series::<T>::mutate(id, |series| {
                if let Some(series) = series {
                    ensure!(series.owner == who, Error::<T>::NotSeriesOwner);
                    series.owner = to_unlookup.clone();
                    Ok(())
                } else {
                    Err(Error::<T>::NFTSeriesNotFound)
                }
            })?;

            Self::deposit_event(Event::SeriesTransfer(id, who, to_unlookup));

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", NFTId = "NFTId")]
    pub enum Event<T: Config> {
        /// A new NFT was created. \[nft id, owner, series id, uri\]
        Created(NFTId, T::AccountId, NFTSeriesId, Vec<u8>),
        /// An NFT was transferred to someone else. \[nft id, old owner, new owner\]
        Transfer(NFTId, T::AccountId, T::AccountId),
        /// An NFT was updated by its owner. \[nft id\]
        Mutated(NFTId),
        /// An NFT was sealed, preventing any new mutations. \[nft id\]
        Sealed(NFTId),
        /// An NFT has been locked, preventing transfers until it is unlocked.
        /// \[nft id\]
        Locked(NFTId),
        /// A locked NFT has been unlocked. \[nft id\]
        Unlocked(NFTId),
        /// An NFT that was burned. \[nft id\]
        Burned(NFTId),
        /// An NFT Series was transferred to someone else. \[nft series id, old owner, new owner\]
        SeriesTransfer(NFTSeriesId, T::AccountId, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// We do not have any NFT id left, a runtime upgrade is necessary.
        NFTIdOverflow,
        /// This function can only be called by the owner of the nft.
        NotOwner,
        /// NFT is sealed and no longer accepts mutations.
        Sealed,
        /// NFT is locked and thus its owner cannot be changed until it
        /// is unlocked.
        Locked,
        /// Cannot add nfts to a series that is not owned.
        NotSeriesOwner,
        /// No one can be the owner the of the default series id.
        NFTSeriesLocked,
        /// No series was found with that given id.
        NFTSeriesNotFound,
    }

    /// The number of NFTs managed by this pallet
    #[pallet::storage]
    #[pallet::getter(fn nft_id_generator)]
    pub type NftIdGenerator<T: Config> = StorageValue<_, NFTId, ValueQuery>;

    /// Data related to NFTs.
    #[pallet::storage]
    #[pallet::getter(fn data)]
    pub type Data<T: Config> =
        StorageMap<_, Blake2_128Concat, NFTId, NFTData<T::AccountId>, ValueQuery>;

    /// Data related to NFT Series.
    #[pallet::storage]
    #[pallet::getter(fn series)]
    pub type Series<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        NFTSeriesId,
        NFTSeriesDetails<T::AccountId, NFTId>,
        OptionQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub nfts: Vec<(T::AccountId, NFTDetails)>,
        pub series: Vec<(T::AccountId, NFTSeriesId)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                nfts: Default::default(),
                series: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.series
                .clone()
                .into_iter()
                .for_each(|(account, series_id)| {
                    drop(<Pallet<T> as NFTs>::set_series_owner(series_id, &account));
                });

            self.nfts
                .clone()
                .into_iter()
                .for_each(|(account, details)| {
                    drop(<Pallet<T> as NFTs>::create(&account, details))
                });
        }
    }
}

impl<T: Config> NFTs for Pallet<T> {
    type AccountId = T::AccountId;
    type NFTDetails = NFTDetails;
    type NFTSeriesId = NFTSeriesId;
    type NFTId = NFTId;

    fn create(
        owner: &Self::AccountId,
        details: Self::NFTDetails,
    ) -> result::Result<Self::NFTId, DispatchError> {
        // Check for series prerequisites.
        let series_id = details.series_id;
        let mut nft_series = Self::get_nft_series_data(owner, series_id)?;

        // Get current and next nft id.
        let (nft_id, next_nft_id) = Self::get_next_nft_id()?;

        if details.unique_series() {
            nft_series.push(nft_id);
            Series::<T>::insert(series_id, NFTSeriesDetails::new(owner.clone(), nft_series));
        }

        let uri = details.offchain_uri.clone();
        let value = NFTData::new(owner.clone(), details, false, false);

        Data::<T>::insert(nft_id, value);
        NftIdGenerator::<T>::put(next_nft_id);

        Self::deposit_event(Event::Created(nft_id, owner.clone(), series_id, uri));

        Ok(nft_id)
    }

    fn mutate<F: FnOnce(&Self::AccountId, &mut Self::NFTDetails) -> DispatchResult>(
        id: Self::NFTId,
        f: F,
    ) -> DispatchResult {
        let mut data = Data::<T>::get(id);
        let mut details = data.details;

        ensure!(!data.sealed, Error::<T>::Sealed);
        f(&data.owner, &mut details)?;

        data.details = details;
        Data::<T>::insert(id, data);

        Self::deposit_event(Event::Mutated(id));
        Ok(())
    }

    fn set_owner(id: Self::NFTId, owner: &Self::AccountId) -> DispatchResult {
        Data::<T>::try_mutate(id, |data| -> DispatchResult {
            ensure!(!data.locked, Error::<T>::Locked);
            (*data).owner = owner.clone();
            Ok(())
        })?;

        Ok(())
    }

    fn details(id: Self::NFTId) -> Self::NFTDetails {
        Data::<T>::get(id).details
    }

    fn owner(id: Self::NFTId) -> Self::AccountId {
        Data::<T>::get(id).owner
    }

    fn seal(id: Self::NFTId) -> DispatchResult {
        Data::<T>::mutate(id, |d| (*d).sealed = true);
        Self::deposit_event(Event::Sealed(id));
        Ok(())
    }

    fn sealed(id: Self::NFTId) -> bool {
        Data::<T>::get(id).sealed
    }

    fn burn(id: Self::NFTId) -> DispatchResult {
        if let Some(series_id) = Self::series_id(id) {
            Series::<T>::mutate(series_id, |series| {
                if let Some(series) = series {
                    if let Some(index) = series.nfts.iter().position(|x| *x == id) {
                        series.nfts.remove(index);
                    }
                }
            });
        }

        Data::<T>::remove(id);
        Self::deposit_event(Event::Burned(id));

        Ok(())
    }

    fn series_id(id: Self::NFTId) -> Option<Self::NFTSeriesId> {
        if Data::<T>::contains_key(id) {
            Some(Data::<T>::get(id).details.series_id)
        } else {
            None
        }
    }

    fn series_length(id: Self::NFTSeriesId) -> Option<usize> {
        Some(Series::<T>::get(id)?.nfts.len())
    }

    fn series_owner(id: Self::NFTSeriesId) -> Option<Self::AccountId> {
        Some(Series::<T>::get(id)?.owner)
    }

    fn set_series_owner(id: Self::NFTSeriesId, owner: &Self::AccountId) -> DispatchResult {
        ensure!(id != NFTSeriesId::default(), Error::<T>::NFTSeriesLocked);

        Series::<T>::mutate(id, |series| {
            if let Some(series) = series {
                series.owner = owner.clone();
            } else {
                *series = Some(NFTSeriesDetails::new(owner.clone(), sp_std::vec![]));
            }
        });

        Ok(())
    }

    fn is_capsule(id: Self::NFTId) -> bool {
        Data::<T>::get(id).details.is_capsule
    }
}

impl<T: Config> LockableNFTs for Pallet<T> {
    type AccountId = T::AccountId;
    type NFTId = NFTId;

    fn lock(id: Self::NFTId) -> DispatchResult {
        Data::<T>::try_mutate(id, |d| -> DispatchResult {
            ensure!(!d.locked, Error::<T>::Locked);
            (*d).locked = true;
            Ok(())
        })
    }

    fn unlock(id: Self::NFTId) {
        Data::<T>::mutate(id, |d| (*d).locked = false);
    }

    fn locked(id: Self::NFTId) -> bool {
        Data::<T>::get(id).locked
    }
}

impl<T: Config> Pallet<T> {
    fn get_nft_series_data(
        owner: &T::AccountId,
        series_id: NFTSeriesId,
    ) -> Result<Vec<NFTId>, Error<T>> {
        let mut nft_series = sp_std::vec![];
        if series_id != NFTSeriesId::default() {
            if let Some(series) = Series::<T>::get(series_id) {
                ensure!(series.owner == *owner, Error::<T>::NotSeriesOwner);
                nft_series = series.nfts;
            }
        }

        Ok(nft_series)
    }

    fn get_next_nft_id() -> Result<(NFTId, NFTId), Error<T>> {
        let nft_id = NftIdGenerator::<T>::get();
        let next_id = nft_id.checked_add(1).ok_or(Error::<T>::NFTIdOverflow)?;

        Ok((nft_id, next_id))
    }
}
