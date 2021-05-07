#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;
#[cfg(test)]
mod tests;

pub use pallet::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use frame_support::pallet_prelude::{ensure, DispatchError};
use frame_support::weights::Weight;
use sp_runtime::traits::CheckedAdd;
use sp_runtime::{DispatchResult, RuntimeDebug};
use sp_std::result;
use sp_std::vec::Vec;
use ternoa_common::traits::{LockableNFTs, NFTs};

/// Data related to an NFT, such as who is its owner.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTData<AccountId, NFTDetails, NFTSeriesId> {
    pub owner: AccountId,
    pub details: NFTDetails,
    /// Set to true to prevent further modifications to the details struct
    pub sealed: bool,
    /// Set to true to prevent changes to the owner variable
    pub locked: bool,
    /// The Id of the Series that this NFT belongs. Zero means that it doesn't belong to any series.
    pub series_id: NFTSeriesId,
}

/// Data related to an NFT Series.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTSeriesDetails<AccountId, NFTId> {
    // Series owner.
    pub owner: AccountId,
    // NFTs that are part of the same series.
    pub nfts: Vec<NFTId>,
}

impl<AccountId, NFTId> NFTSeriesDetails<AccountId, NFTId> {
    pub fn new(owner: AccountId, nfts: Vec<NFTId>) -> Self {
        Self { owner, nfts }
    }
}

pub trait WeightInfo {
    fn create() -> Weight;
    fn create_with_series() -> Weight;
    fn mutate() -> Weight;
    fn seal() -> Weight;
    fn transfer() -> Weight;
    fn burn() -> Weight;
    fn transfer_series() -> Weight;
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, StaticLookup};
    use sp_runtime::DispatchResult;
    use ternoa_common::traits::NFTs;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// How NFTs are represented
        type NFTId: Parameter + Default + CheckedAdd + Copy + Member + From<u8>;
        /// How NFT details are represented
        type NFTDetails: Parameter + Member + MaybeSerializeDeserialize + Default;
        type WeightInfo: WeightInfo;
        /// How the NFT series id is represented.
        type NFTSeriesId: Parameter
            + Copy
            + Default
            + CheckedAdd
            + Member
            + From<u32>
            + MaybeSerializeDeserialize;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new NFT with the provided details. An ID will be auto
        /// generated and logged as an event, The caller of this function
        /// will become the owner of the new NFT.
        #[pallet::weight(if *series_id == Default::default() {T::WeightInfo::create()} else {T::WeightInfo::create_with_series()})]
        pub fn create(
            origin: OriginFor<T>,
            details: T::NFTDetails,
            series_id: T::NFTSeriesId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let _id = <Self as NFTs>::create(&who, details, series_id)?;

            Ok(().into())
        }

        /// Update the details included in an NFT. Must be called by the owner of
        /// the NFT and while the NFT is not sealed.
        #[pallet::weight(T::WeightInfo::mutate())]
        pub fn mutate(
            origin: OriginFor<T>,
            id: T::NFTId,
            details: T::NFTDetails,
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
            id: T::NFTId,
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
        pub fn seal(origin: OriginFor<T>, id: T::NFTId) -> DispatchResultWithPostInfo {
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
        pub fn burn(origin: OriginFor<T>, id: T::NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let data = Data::<T>::get(id);

            ensure!(data.owner == who, Error::<T>::NotOwner);
            <Self as NFTs>::burn(id).expect("Call to Burn function should never fail!");

            Ok(().into())
        }

        /// Transfer an NFT series from one account to another one. Must be called by the
        /// actual owner of the NFT series.
        #[pallet::weight(T::WeightInfo::transfer_series())]
        pub fn transfer_series(
            origin: OriginFor<T>,
            id: T::NFTSeriesId,
            to: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let to_unlookup = T::Lookup::lookup(to)?;

            ensure!(id != Default::default(), Error::<T>::NotSeriesOwner);
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
    #[pallet::metadata(T::AccountId = "AccountId", T::NFTId = "NFTId")]
    pub enum Event<T: Config> {
        /// A new NFT was created. \[nft id, owner\]
        Created(T::NFTId, T::AccountId),
        /// An NFT was transferred to someone else. \[nft id, old owner, new owner\]
        Transfer(T::NFTId, T::AccountId, T::AccountId),
        /// An NFT was updated by its owner. \[nft id\]
        Mutated(T::NFTId),
        /// An NFT was sealed, preventing any new mutations. \[nft id\]
        Sealed(T::NFTId),
        /// An NFT has been locked, preventing transfers until it is unlocked.
        /// \[nft id\]
        Locked(T::NFTId),
        /// A locked NFT has been unlocked. \[nft id\]
        Unlocked(T::NFTId),
        /// An NFT that was burned. \[nft id\]
        Burned(T::NFTId),
        /// An NFT Series was transferred to someone else. \[nft series id, old owner, new owner\]
        SeriesTransfer(T::NFTSeriesId, T::AccountId, T::AccountId),
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
    #[pallet::getter(fn total)]
    pub type Total<T: Config> = StorageValue<_, T::NFTId, ValueQuery>;

    /// Data related to NFTs.
    #[pallet::storage]
    #[pallet::getter(fn data)]
    pub type Data<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::NFTId,
        NFTData<T::AccountId, T::NFTDetails, T::NFTSeriesId>,
        ValueQuery,
    >;

    /// Data related to NFT Series.
    #[pallet::storage]
    #[pallet::getter(fn series)]
    pub type Series<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::NFTSeriesId,
        NFTSeriesDetails<T::AccountId, T::NFTId>,
        OptionQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub nfts: Vec<(T::AccountId, T::NFTDetails, T::NFTSeriesId)>,
        pub series: Vec<(T::AccountId, T::NFTSeriesId)>,
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
                .for_each(|(account, details, series_id)| {
                    drop(<Pallet<T> as NFTs>::create(&account, details, series_id))
                });
        }
    }
}

impl<T: Config> NFTs for Pallet<T> {
    type AccountId = T::AccountId;
    type NFTDetails = T::NFTDetails;
    type NFTId = T::NFTId;
    type NFTSeriesId = T::NFTSeriesId;

    fn create(
        owner: &Self::AccountId,
        details: Self::NFTDetails,
        series_id: Self::NFTSeriesId,
    ) -> result::Result<Self::NFTId, DispatchError> {
        // Check if the owner is even allowed to add anything to the series.
        let mut nft_series = sp_std::vec![];
        if series_id != Default::default() {
            if let Some(series) = Series::<T>::get(series_id) {
                ensure!(series.owner == *owner, Error::<T>::NotSeriesOwner);
                nft_series = series.nfts;
            }
        }

        // Create the nft.
        let nft_id = Total::<T>::get();
        Total::<T>::put(
            nft_id
                .checked_add(&1.into())
                .ok_or(Error::<T>::NFTIdOverflow)?,
        );

        // Create or expand a series.
        if series_id != Default::default() {
            nft_series.push(nft_id);
            Series::<T>::insert(series_id, NFTSeriesDetails::new(owner.clone(), nft_series));
        }

        Data::<T>::insert(
            nft_id,
            NFTData {
                owner: owner.clone(),
                details,
                sealed: false,
                locked: false,
                series_id,
            },
        );

        Self::deposit_event(Event::Created(nft_id, owner.clone()));

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
            Some(Data::<T>::get(id).series_id)
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
        ensure!(id != Default::default(), Error::<T>::NFTSeriesLocked);

        Series::<T>::mutate(id, |series| {
            if let Some(series) = series {
                series.owner = owner.clone();
            } else {
                *series = Some(NFTSeriesDetails::new(owner.clone(), sp_std::vec![]));
            }
        });

        Ok(())
    }
}

impl<T: Config> LockableNFTs for Pallet<T> {
    type AccountId = T::AccountId;
    type NFTId = T::NFTId;

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
