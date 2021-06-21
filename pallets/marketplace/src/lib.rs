#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;

#[cfg(test)]
mod tests;
mod types;

pub use pallet::*;
pub use types::*;

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn list() -> Weight;
    fn unlist() -> Weight;
    fn buy() -> Weight;
}

#[frame_support::pallet]
pub mod pallet {
    use sp_std::convert::TryInto;

    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Currency;
    use frame_support::traits::ExistenceRequirement;
    use frame_system::pallet_prelude::*;
    use ternoa_common::traits::{LockableNFTs, NFTs};

    pub type NFTIdOf<T> = <<T as Config>::NFTs as LockableNFTs>::NFTId;

    pub type BalanceCaps<T> =
        <<T as Config>::CurrencyCaps as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub type BalanceTiime<T> =
        <<T as Config>::CurrencyTiime as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Pallet managing nfts.
        type NFTs: LockableNFTs<AccountId = Self::AccountId>
            + NFTs<AccountId = Self::AccountId, NFTId = NFTIdOf<Self>>;
        /// Weight values for this pallet
        type WeightInfo: WeightInfo;

        /// Currency used to handle transactions and pay for the nfts.
        type CurrencyCaps: Currency<Self::AccountId>;
        type CurrencyTiime: Currency<Self::AccountId>;
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
            price: NFTCurrency,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(T::NFTs::owner(nft_id) == who, Error::<T>::NotNftOwner);

            T::NFTs::lock(nft_id)?;
            NFTsForSale::<T>::insert(nft_id, (who.clone(), price.clone()));

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
        pub fn buy(
            origin: OriginFor<T>,
            nft_id: NFTIdOf<T>,
            currency: NFTCurrencyId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(
                NFTsForSale::<T>::contains_key(nft_id),
                Error::<T>::NftNotForSale
            );

            let (owner, price) = NFTsForSale::<T>::get(nft_id);
            ensure!(owner != who, Error::<T>::NftAlreadyOwned);

            // KeepAlive because they need to be able to use the NFT later on#
            let keep_alive = ExistenceRequirement::KeepAlive;
            match currency {
                NFTCurrencyId::CAPS => {
                    if let Some(price) = price.caps() {
                        let value: BalanceCaps<T> = price.try_into().ok().unwrap();
                        T::CurrencyCaps::transfer(&who, &owner, value, keep_alive)?;
                    } else {
                    }
                }
                NFTCurrencyId::TIIME => {
                    if let Some(price) = price.tiime() {
                        let value: BalanceTiime<T> = price.try_into().ok().unwrap();
                        T::CurrencyTiime::transfer(&who, &owner, value, keep_alive)?;
                    } else {
                    }
                }
            }

            T::NFTs::unlock(nft_id);
            T::NFTs::set_owner(nft_id, &who)?;
            NFTsForSale::<T>::remove(nft_id);

            Self::deposit_event(Event::NftSold(nft_id, who));

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", NFTIdOf<T> = "NFTId", CommonBalanceT<T> = "Balance")]
    pub enum Event<T: Config> {
        /// A nft has been listed for sale. \[nft id, currency id, price\]
        NftListed(NFTIdOf<T>, NFTCurrency),
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
        StorageMap<_, Blake2_128Concat, NFTIdOf<T>, (T::AccountId, NFTCurrency), ValueQuery>;
}
