#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;
mod migrations;
mod types;

pub use pallet::*;
pub use types::*;

use frame_support::traits::StorageVersion;
use frame_support::weights::Weight;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(4);

pub trait WeightInfo {
    fn list() -> Weight;
    fn unlist() -> Weight;
    fn buy() -> Weight;
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::ExistenceRequirement::KeepAlive;
    use frame_support::traits::{Currency, OnUnbalanced, WithdrawReasons};
    use frame_support::transactional;
    use frame_system::pallet_prelude::*;
    use ternoa_common::traits::{LockableNFTs, NFTs};

    pub type NFTIdOf<T> = <<T as Config>::NFTs as LockableNFTs>::NFTId;

    pub type BalanceCaps<T> =
        <<T as Config>::CurrencyCaps as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub type BalanceTiime<T> =
        <<T as Config>::CurrencyTiime as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    pub type NegativeImbalanceCaps<T> = <<T as Config>::CurrencyCaps as Currency<
        <T as frame_system::Config>::AccountId,
    >>::NegativeImbalance;

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

        /// Host much does it cost to create a marketplace.
        type MarketplaceFee: Get<BalanceCaps<Self>>;
        /// Place where the marketplace fees go.
        type FeesCollector: OnUnbalanced<NegativeImbalanceCaps<Self>>;
    }

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
        /// Deposit a nft and list it on the marketplace
        #[pallet::weight(T::WeightInfo::list())]
        pub fn list(
            origin: OriginFor<T>,
            nft_id: NFTIdOf<T>,
            price: NFTCurrency<T>,
            marketplace_id: Option<MarketplaceId>,
        ) -> DispatchResultWithPostInfo {
            let account_id = ensure_signed(origin)?;
            let marketplace_id = marketplace_id.unwrap_or(0);

            ensure!(
                T::NFTs::owner(nft_id) == account_id,
                Error::<T>::NotNftOwner
            );
            ensure!(
                marketplace_id <= MarketplaceCount::<T>::get(),
                Error::<T>::UnknownMarketplace
            );

            T::NFTs::lock(nft_id)?;

            let sale_info = SaleInformation::<T>::new(account_id, price.clone(), marketplace_id);
            NFTsForSale::<T>::insert(nft_id, sale_info);

            Self::deposit_event(Event::NftListed(nft_id, price, marketplace_id));

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
            let caller_id = ensure_signed(origin)?;
            ensure!(
                NFTsForSale::<T>::contains_key(nft_id),
                Error::<T>::NftNotForSale
            );

            let sale = NFTsForSale::<T>::get(nft_id);
            ensure!(sale.account_id != caller_id, Error::<T>::NftAlreadyOwned);

            // KeepAlive because they need to be able to use the NFT later on
            match currency {
                NFTCurrencyId::CAPS => {
                    let value = sale.price.caps().ok_or(Error::<T>::WrongCurrencyUsed)?;
                    T::CurrencyCaps::transfer(&caller_id, &sale.account_id, value, KeepAlive)?;
                }
                NFTCurrencyId::TIIME => {
                    let value = sale.price.tiime().ok_or(Error::<T>::WrongCurrencyUsed)?;
                    T::CurrencyTiime::transfer(&caller_id, &sale.account_id, value, KeepAlive)?;
                }
            }

            T::NFTs::unlock(nft_id);
            T::NFTs::set_owner(nft_id, &caller_id)?;
            NFTsForSale::<T>::remove(nft_id);

            Self::deposit_event(Event::NftSold(nft_id, caller_id));

            Ok(().into())
        }

        #[pallet::weight(1)]
        #[transactional]
        pub fn create(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;

            // Needs to have enough money
            let imbalance = T::CurrencyCaps::withdraw(
                &caller_id,
                T::MarketplaceFee::get(),
                WithdrawReasons::FEE,
                KeepAlive,
            )?;
            T::FeesCollector::on_unbalanced(imbalance);

            let last_id = MarketplaceCount::<T>::get();
            let last_id = last_id
                .checked_add(1)
                .ok_or(Error::<T>::MarketplaceIdOverflow)?;

            MarketplaceOwners::<T>::insert(last_id, caller_id.clone());
            MarketplaceCount::<T>::set(last_id);

            Self::deposit_event(Event::MarketplaceCreated(last_id, caller_id));

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", NFTIdOf<T> = "NFTId", CommonBalanceT<T> = "Balance")]
    pub enum Event<T: Config> {
        /// A nft has been listed for sale. \[nft id, nft currency, marketplace id\]
        NftListed(NFTIdOf<T>, NFTCurrency<T>, MarketplaceId),
        /// A nft is removed from the marketplace by its owner. \[nft id\]
        NftUnlisted(NFTIdOf<T>),
        /// A nft has been sold. \[nft id, new owner\]
        NftSold(NFTIdOf<T>, T::AccountId),
        /// A marketplace has been created.  \[marketplace id, new owner\]
        MarketplaceCreated(MarketplaceId, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// This function is reserved to the owner of a nft.
        NotNftOwner,
        /// Nft is not present on the marketplace.
        NftNotForSale,
        /// Yot cannot buy your own nft.
        NftAlreadyOwned,
        /// Used wrong currency to buy an nft.
        WrongCurrencyUsed,
        /// We do not have any marketplace ids left, a runtime upgrade is necessary.
        MarketplaceIdOverflow,
        /// No marketplace found with that Id.
        UnknownMarketplace,
    }

    /// Nfts listed on the marketplace
    #[pallet::storage]
    #[pallet::getter(fn nft_for_sale)]
    pub type NFTsForSale<T: Config> =
        StorageMap<_, Blake2_128Concat, NFTIdOf<T>, SaleInformation<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn marketplace_last_id)]
    pub type MarketplaceCount<T: Config> = StorageValue<_, MarketplaceId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn marketplace_owners)]
    pub type MarketplaceOwners<T: Config> =
        StorageMap<_, Blake2_128Concat, MarketplaceId, T::AccountId, OptionQuery>;
}
