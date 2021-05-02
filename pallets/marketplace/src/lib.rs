#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;

#[cfg(test)]
mod tests;

pub use pallet::*;

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn list() -> Weight;
    fn unlist() -> Weight;
    fn buy() -> Weight;
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use frame_system::pallet_prelude::*;
    use ternoa_common::traits::{LockableNFTs, NFTs};

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub type NFTIdOf<T> = <<T as Config>::NFTs as LockableNFTs>::NFTId;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Currency used to handle transactions and pay for the nfts.
        type Currency: Currency<Self::AccountId>;
        /// Pallet managing nfts.
        type NFTs: LockableNFTs<AccountId = Self::AccountId>
            + NFTs<AccountId = Self::AccountId, NFTId = NFTIdOf<Self>>;
        /// Weight values for this pallet
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Deposit a nft and list it on the marketplace
        #[pallet::weight(T::WeightInfo::list())]
        pub fn list(
            origin: OriginFor<T>,
            nft_id: NFTIdOf<T>,
            price: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(T::NFTs::owner(nft_id) == who, Error::<T>::NotNftOwner);

            T::NFTs::lock(nft_id)?;
            NFTsForSale::<T>::insert(nft_id, (who.clone(), price));

            Self::deposit_event(Event::NftListed(nft_id, price));

            Ok(().into())
        }

        /// Owner unlist the nfts
        #[pallet::weight(T::WeightInfo::unlist())]
        pub fn unlist(origin: OriginFor<T>, nft_id: NFTIdOf<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(T::NFTs::owner(nft_id) == who, Error::<T>::NotNftOwner);
            ensure!(
                NFTsForSale::<T>::contains_key(nft_id),
                Error::<T>::NftNotForSale
            );

            T::NFTs::unlock(nft_id);
            NFTsForSale::<T>::remove(nft_id);

            Self::deposit_event(Event::NftUnlisted(nft_id));

            Ok(().into())
        }

        /// Buy a listed nft
        #[pallet::weight(T::WeightInfo::buy())]
        pub fn buy(origin: OriginFor<T>, nft_id: NFTIdOf<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(
                NFTsForSale::<T>::contains_key(nft_id),
                Error::<T>::NftNotForSale
            );

            let (owner, price) = NFTsForSale::<T>::get(nft_id);
            ensure!(owner != who, Error::<T>::NftAlreadyOwned);

            // KeepAlive because they need to be able to use the NFT later on
            T::Currency::transfer(&who, &owner, price, ExistenceRequirement::KeepAlive)?;

            T::NFTs::unlock(nft_id);
            T::NFTs::set_owner(nft_id, &who)?;
            NFTsForSale::<T>::remove(nft_id);

            Self::deposit_event(Event::NftSold(nft_id, who));

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", NFTIdOf<T> = "NFTId", BalanceOf<T> = "Balance")]
    pub enum Event<T: Config> {
        /// A nft has been listed for sale. \[nft id, price\]
        NftListed(NFTIdOf<T>, BalanceOf<T>),
        /// A nft is removed from the marketplace by its owner. \[nft id\]
        NftUnlisted(NFTIdOf<T>),
        /// A nft has been sold. \[nft id, new owner\]
        NftSold(NFTIdOf<T>, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// This function is reserved to the owner of a nft.
        NotNftOwner,
        /// Nft is not present on the marketplace
        NftNotForSale,
        /// Yot cannot buy your own nft
        NftAlreadyOwned,
    }

    /// Nfts listed on the marketplace
    #[pallet::storage]
    #[pallet::getter(fn nft_for_sale)]
    pub type NFTsForSale<T: Config> =
        StorageMap<_, Blake2_128Concat, NFTIdOf<T>, (T::AccountId, BalanceOf<T>), ValueQuery>;
}
