#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;
mod migrations;
pub mod traits;

pub use default_weights::WeightInfo;
pub use pallet::*;

use frame_support::pallet_prelude::ensure;
use frame_support::traits::{Get, StorageVersion};
use sp_runtime::DispatchResult;
use sp_std::vec;
use sp_std::vec::Vec;
use ternoa_primitives::nfts::{NFTData, NFTId, NFTSeriesDetails, NFTSeriesId, NFTString};

const STORAGE_VERSION: StorageVersion = StorageVersion::new(5);

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::ExistenceRequirement::KeepAlive;
    use frame_support::traits::{Currency, OnUnbalanced, WithdrawReasons};
    use frame_support::{pallet_prelude::*, transactional};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::StaticLookup;

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
        #[pallet::weight(T::WeightInfo::create())]
        // have to be transactional otherwise we could make people pay the mint
        // even if the creation fails.
        #[transactional]
        pub fn create(
            origin: OriginFor<T>,
            ipfs_reference: NFTString,
            series_id: Option<NFTSeriesId>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // Checks
            // The Caller needs to pay the NFT Mint fee.
            let fee = T::MintFee::get();
            let reason = WithdrawReasons::FEE;
            let imbalance = T::Currency::withdraw(&who, fee, reason, KeepAlive)?;
            T::FeesCollector::on_unbalanced(imbalance);

            // Check if the series exists. If it exists and the caller is not the owner throw error.
            let series_exits = Self::series_exists(&who, &series_id)?;

            // Execute
            let nft_id = Self::generate_nft_id();
            let series_id = series_id.unwrap_or_else(|| Self::generate_series_id());

            let value = NFTData::new(
                who.clone(),
                ipfs_reference.clone(),
                series_id.clone(),
                false,
            );

            // Save
            Data::<T>::insert(nft_id, value);
            if !series_exits {
                Series::<T>::insert(series_id.clone(), NFTSeriesDetails::new(who.clone(), true));
            }

            Self::deposit_event(Event::Created(nft_id, who, series_id, ipfs_reference));

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
            let to = T::Lookup::lookup(to)?;

            let mut data = Data::<T>::get(id).ok_or(Error::<T>::InvalidNFTId)?;
            let series = Series::<T>::get(&data.series_id).ok_or(Error::<T>::SeriesNotFound)?;

            ensure!(data.owner == who, Error::<T>::NotOwner);
            ensure!(!data.locked, Error::<T>::Locked);
            ensure!(!series.draft, Error::<T>::SeriesIsInDraft);

            data.owner = to.clone();
            Data::<T>::insert(id, data);

            Self::deposit_event(Event::Transfer(id, who, to));

            Ok(().into())
        }

        /// Remove an NFT from the storage. This operation is irreversible which means
        /// once the NFT is removed (burned) from the storage there is no way to
        /// get it back.
        /// Must be called by the owner of the NFT.
        #[pallet::weight(T::WeightInfo::burn())]
        pub fn burn(origin: OriginFor<T>, id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let data = Data::<T>::get(id).ok_or(Error::<T>::InvalidNFTId)?;

            ensure!(data.owner == who, Error::<T>::NotOwner);
            ensure!(!data.locked, Error::<T>::Locked);

            Data::<T>::remove(id);
            Self::deposit_event(Event::Burned(id));

            Ok(().into())
        }

        /// TODO!
        #[pallet::weight(T::WeightInfo::finish_series())]
        pub fn finish_series(
            origin: OriginFor<T>,
            series_id: NFTSeriesId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Series::<T>::mutate(&series_id, |x| {
                if let Some(series) = x {
                    if series.owner != who {
                        return Err(Error::<T>::NotSeriesOwner);
                    }
                    if !series.draft {
                        return Err(Error::<T>::SeriesIsCompleted);
                    }

                    series.draft = false;

                    Ok(())
                } else {
                    Err(Error::<T>::SeriesNotFound)?
                }
            })?;

            Self::deposit_event(Event::SeriesFinished(series_id));

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", NFTId = "NFTId", NFTString = "String")]
    pub enum Event<T: Config> {
        /// A new NFT was created. \[nft id, owner, series id, ipfs reference\]
        Created(NFTId, T::AccountId, NFTSeriesId, NFTString),
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
        /// An NFT was burned. \[nft id\]
        Burned(NFTId),
        /// A series has been completed. \[series id\]
        SeriesFinished(NFTSeriesId),
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
        /// TODO!
        SeriesIsInDraft,
        /// TODO!
        SeriesIsCompleted,
        /// Series not Found
        SeriesNotFound,
        /// No NFT was found with that NFT id.
        InvalidNFTId,
    }

    /// The number of NFTs managed by this pallet
    #[pallet::storage]
    #[pallet::getter(fn nft_id_generator)]
    pub type NftIdGenerator<T: Config> = StorageValue<_, NFTId, ValueQuery>;

    /// Data related to NFTs.
    #[pallet::storage]
    #[pallet::getter(fn data)]
    pub type Data<T: Config> =
        StorageMap<_, Blake2_128Concat, NFTId, NFTData<T::AccountId>, OptionQuery>;

    /// Data related to NFT Series.
    #[pallet::storage]
    #[pallet::getter(fn series)]
    pub type Series<T: Config> =
        StorageMap<_, Blake2_128Concat, NFTSeriesId, NFTSeriesDetails<T::AccountId>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn series_id_generator)]
    pub type SeriesIdGenerator<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub nfts: Vec<(NFTId, NFTData<T::AccountId>)>,
        pub series: Vec<(NFTSeriesId, NFTSeriesDetails<T::AccountId>)>,
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
                .for_each(|(series_id, series)| {
                    Series::<T>::insert(series_id, series);
                });

            let mut current_nft_id: NFTId = 0;
            self.nfts.clone().into_iter().for_each(|(nft_id, data)| {
                Data::<T>::insert(nft_id, data);
                current_nft_id = current_nft_id.max(nft_id);
            });

            NftIdGenerator::<T>::put(current_nft_id + 1);
            SeriesIdGenerator::<T>::put(0);
        }
    }
}

impl<T: Config> traits::NFTs for Pallet<T> {
    type AccountId = T::AccountId;

    fn set_owner(id: NFTId, owner: &Self::AccountId) -> DispatchResult {
        Data::<T>::try_mutate(id, |data| {
            if let Some(data) = data {
                ensure!(!data.locked, Error::<T>::Locked);
                (*data).owner = owner.clone();
                Ok(())
            } else {
                Err(Error::<T>::InvalidNFTId)
            }
        })?;

        Ok(())
    }

    fn owner(id: NFTId) -> Option<Self::AccountId> {
        Some(Data::<T>::get(id)?.owner)
    }

    fn is_series_completed(id: NFTId) -> Option<bool> {
        let series_id = Data::<T>::get(id)?.series_id;
        Some(!Series::<T>::get(series_id)?.draft)
    }

    /*     fn create_nft(
        owner: Self::AccountId,
        ipfs_reference: NFTString,
        series_id: Option<NFTSeriesId>,
    ) -> Result<NFTId, DispatchErrorWithPostInfo> {
        Self::create(Origin::<T>::Signed(owner).into(), ipfs_reference, series_id)?;
        return Ok(Self::nft_id_generator() - 1);
    } */
}

impl<T: Config> traits::LockableNFTs for Pallet<T> {
    type AccountId = T::AccountId;

    fn lock(id: NFTId) -> DispatchResult {
        Data::<T>::try_mutate(id, |d| {
            if let Some(d) = d {
                ensure!(!d.locked, Error::<T>::Locked);
                (*d).locked = true;
                Ok(())
            } else {
                Err(Error::<T>::InvalidNFTId)
            }
        })?;

        Ok(())
    }

    fn unlock(id: NFTId) -> bool {
        Data::<T>::mutate(id, |d| {
            if let Some(d) = d {
                (*d).locked = false;
                return true;
            } else {
                return false;
            }
        })
    }

    fn locked(id: NFTId) -> Option<bool> {
        Some(Data::<T>::get(id)?.locked)
    }
}

impl<T: Config> Pallet<T> {
    fn generate_nft_id() -> NFTId {
        let nft_id = NftIdGenerator::<T>::get();
        let next_id = nft_id
            .checked_add(1)
            .expect("If u32 is not enough we should crash for safety; qed.");
        NftIdGenerator::<T>::put(next_id);

        return nft_id;
    }

    fn generate_series_id() -> NFTSeriesId {
        let mut id = SeriesIdGenerator::<T>::get();
        loop {
            let id_vec = u32_to_text(id);
            if !Series::<T>::contains_key(&id_vec) {
                break;
            }
            id = id
                .checked_add(1)
                .expect("If u32 is not enough we should crash for safety; qed.");
        }
        SeriesIdGenerator::<T>::put(
            id.checked_add(1)
                .expect("If u32 is not enough we should crash for safety; qed."),
        );

        return u32_to_text(id);
    }

    fn series_exists(
        owner: &T::AccountId,
        series_id: &Option<NFTSeriesId>,
    ) -> Result<bool, Error<T>> {
        if let Some(id) = series_id {
            if let Some(series) = Series::<T>::get(id) {
                if series.owner != *owner {
                    return Err(Error::<T>::NotSeriesOwner);
                }
                if !series.draft {
                    return Err(Error::<T>::SeriesIsCompleted);
                }

                return Ok(true);
            }
        }

        return Ok(false);
    }
}

fn u32_to_text(num: u32) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![];
    let mut dc: usize = 0;

    fn inner(n: u32, vec: &mut Vec<u8>, dc: &mut usize) {
        *dc += 1;
        if n >= 10 {
            inner(n / 10, vec, dc);
        }

        if vec.is_empty() {
            *vec = Vec::with_capacity(*dc);
        }

        let char = u8_to_char((n % 10) as u8);
        vec.push(char);
    }

    inner(num, &mut vec, &mut dc);
    vec
}

const fn u8_to_char(num: u8) -> u8 {
    return num + 48;
}
