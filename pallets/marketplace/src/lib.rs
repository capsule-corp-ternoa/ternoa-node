#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;
mod migrations;
mod types;

use frame_support::dispatch::DispatchResultWithPostInfo;
pub use pallet::*;
pub use types::*;

use default_weights::WeightInfo;
use frame_support::traits::StorageVersion;
use frame_support::weights::Weight;
use ternoa_primitives::nfts::NFTId;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(6);

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::ExistenceRequirement::KeepAlive;
    use frame_support::traits::{Currency, OnUnbalanced, WithdrawReasons};
    use frame_support::transactional;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedDiv, CheckedSub, StaticLookup};
    use sp_std::vec::Vec;
    use ternoa_common::traits::{LockableNFTs, NFTs};

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
        type NFTs: LockableNFTs<AccountId = Self::AccountId, NFTId = NFTId>
            + NFTs<AccountId = Self::AccountId, NFTId = NFTId>;
        /// Weight values for this pallet
        type WeightInfo: WeightInfo;

        /// Currency used to handle transactions and pay for the nfts.
        type CurrencyCaps: Currency<Self::AccountId>;
        type CurrencyTiime: Currency<Self::AccountId>;

        /// Host much does it cost to create a marketplace.
        type MarketplaceFee: Get<BalanceCaps<Self>>;
        /// Place where the marketplace fees go.
        type FeesCollector: OnUnbalanced<NegativeImbalanceCaps<Self>>;

        /// The minimum length a name may be.
        #[pallet::constant]
        type MinNameLength: Get<u32>;

        /// The maximum length a name may be.
        #[pallet::constant]
        type MaxNameLength: Get<u32>;
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
            nft_id: NFTId,
            price: NFTCurrency<BalanceCaps<T>, BalanceTiime<T>>,
            marketplace_id: Option<MarketplaceId>,
        ) -> DispatchResultWithPostInfo {
            let account_id = ensure_signed(origin)?;
            let marketplace_id = marketplace_id.unwrap_or(0);

            ensure!(
                T::NFTs::owner(nft_id) == account_id,
                Error::<T>::NotNftOwner
            );

            let market_info =
                Marketplaces::<T>::get(marketplace_id).ok_or(Error::<T>::UnknownMarketplace)?;

            if market_info.kind == MarketplaceType::Private {
                ensure!(
                    market_info.allow_list.contains(&account_id),
                    Error::<T>::NotAllowed
                );
            }

            T::NFTs::lock(nft_id)?;

            let sale_info = SaleInformation::new(account_id, price.clone(), marketplace_id);
            NFTsForSale::<T>::insert(nft_id, sale_info);

            Self::deposit_event(Event::NftListed(nft_id, price, marketplace_id));

            Ok(().into())
        }

        /// Owner unlist the nfts
        #[pallet::weight(T::WeightInfo::unlist())]
        pub fn unlist(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
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
        #[transactional]
        pub fn buy(
            origin: OriginFor<T>,
            nft_id: NFTId,
            currency: NFTCurrencyId,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;
            ensure!(
                NFTsForSale::<T>::contains_key(nft_id),
                Error::<T>::NftNotForSale
            );

            let sale = NFTsForSale::<T>::get(nft_id);
            ensure!(sale.account_id != caller_id, Error::<T>::NftAlreadyOwned);

            // Check if there is any commission fee.
            let market_info = Marketplaces::<T>::get(sale.marketplace_id)
                .ok_or(Error::<T>::UnknownMarketplace)?;

            let commission_fee = market_info.commission_fee;

            // KeepAlive because they need to be able to use the NFT later on
            match currency {
                NFTCurrencyId::Caps => {
                    let mut value = sale.price.caps().ok_or(Error::<T>::WrongCurrencyUsed)?;
                    if commission_fee != 0 {
                        let tmp = 100u8
                            .checked_div(commission_fee)
                            .ok_or(Error::<T>::InternalMathError)?;

                        let fee = value
                            .checked_div(&(tmp.into()))
                            .ok_or(Error::<T>::InternalMathError)?;

                        value = value
                            .checked_sub(&fee)
                            .ok_or(Error::<T>::InternalMathError)?;

                        T::CurrencyCaps::transfer(&caller_id, &market_info.owner, fee, KeepAlive)?;
                    }

                    T::CurrencyCaps::transfer(&caller_id, &sale.account_id, value, KeepAlive)?;
                }
                NFTCurrencyId::Tiime => {
                    let mut value = sale.price.tiime().ok_or(Error::<T>::WrongCurrencyUsed)?;
                    if commission_fee != 0 {
                        let tmp = 100u8
                            .checked_div(commission_fee)
                            .ok_or(Error::<T>::InternalMathError)?;

                        let fee = value
                            .checked_div(&(tmp.into()))
                            .ok_or(Error::<T>::InternalMathError)?;

                        value = value
                            .checked_sub(&fee)
                            .ok_or(Error::<T>::InternalMathError)?;

                        T::CurrencyTiime::transfer(&caller_id, &market_info.owner, fee, KeepAlive)?;
                    }

                    T::CurrencyTiime::transfer(&caller_id, &sale.account_id, value, KeepAlive)?;
                }
            }

            T::NFTs::unlock(nft_id);
            T::NFTs::set_owner(nft_id, &caller_id)?;
            NFTsForSale::<T>::remove(nft_id);

            Self::deposit_event(Event::NftSold(nft_id, caller_id));

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::create())]
        #[transactional]
        pub fn create(
            origin: OriginFor<T>,
            kind: MarketplaceType,
            commission_fee: u8,
            name: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            ensure!(commission_fee <= 100, Error::<T>::InvalidCommissionFeeValue);

            let caller_id = ensure_signed(origin)?;

            // Needs to have enough money
            let imbalance = T::CurrencyCaps::withdraw(
                &caller_id,
                T::MarketplaceFee::get(),
                WithdrawReasons::FEE,
                KeepAlive,
            )?;
            T::FeesCollector::on_unbalanced(imbalance);

            let marketplace = MarketplaceInformation::new(
                kind,
                commission_fee,
                caller_id.clone(),
                Vec::default(),
                name,
            );

            let id = MarketplaceIdGenerator::<T>::get();
            let id = id.checked_add(1).ok_or(Error::<T>::MarketplaceIdOverflow)?;

            Marketplaces::<T>::insert(id, marketplace);
            MarketplaceIdGenerator::<T>::set(id);
            Self::deposit_event(Event::MarketplaceCreated(id, caller_id));

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::add_account_to_allow_list())]
        pub fn add_account_to_allow_list(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            account_id: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;
            let account_id = T::Lookup::lookup(account_id)?;

            Marketplaces::<T>::mutate(marketplace_id, |x| {
                if let Some(market_info) = x {
                    if market_info.owner != caller_id {
                        return Err(Error::<T>::NotMarketplaceOwner);
                    }

                    if market_info.kind != MarketplaceType::Private {
                        return Err(Error::<T>::UnsupportedMarketplace);
                    }

                    market_info.allow_list.push(account_id.clone());

                    Ok(())
                } else {
                    Err(Error::<T>::UnknownMarketplace)
                }
            })?;

            let event = Event::AccountAddedToMarketplace(marketplace_id, account_id);
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::remove_account_from_allow_list())]
        pub fn remove_account_from_allow_list(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            account_id: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;
            let account_id = T::Lookup::lookup(account_id)?;

            Marketplaces::<T>::mutate(marketplace_id, |x| {
                if let Some(market_info) = x {
                    if market_info.owner != caller_id {
                        return Err(Error::<T>::NotMarketplaceOwner);
                    }

                    if market_info.kind != MarketplaceType::Private {
                        return Err(Error::<T>::UnsupportedMarketplace);
                    }

                    let index = market_info.allow_list.iter().position(|x| *x == account_id);
                    let index = index.ok_or(Error::<T>::AccountNotFound)?;
                    market_info.allow_list.swap_remove(index);

                    Ok(())
                } else {
                    Err(Error::<T>::UnknownMarketplace)
                }
            })?;

            let event = Event::AccountRemovedFromMarketplace(marketplace_id, account_id);
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::change_owner())]
        pub fn change_owner(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            account_id: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;
            let account_id = T::Lookup::lookup(account_id)?;

            Marketplaces::<T>::mutate(marketplace_id, |x| {
                if let Some(market_info) = x {
                    if market_info.owner != caller_id {
                        return Err(Error::<T>::NotMarketplaceOwner);
                    }

                    market_info.owner = account_id.clone();

                    Ok(())
                } else {
                    Err(Error::<T>::UnknownMarketplace)
                }
            })?;

            let event = Event::MarketplaceChangedOwner(marketplace_id, account_id);
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::change_market_type())]
        pub fn change_market_type(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            kind: MarketplaceType,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;

            Marketplaces::<T>::mutate(marketplace_id, |x| {
                if let Some(market_info) = x {
                    if market_info.owner != caller_id {
                        return Err(Error::<T>::NotMarketplaceOwner);
                    }

                    market_info.kind = kind;

                    Ok(())
                } else {
                    Err(Error::<T>::UnknownMarketplace)
                }
            })?;

            let event = Event::MarketplaceTypeChanged(marketplace_id, kind);
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::set_name())]
        pub fn set_name(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            name: Vec<u8>,
        ) -> DispatchResult {
            ensure!(
                name.len() >= T::MinNameLength::get() as usize,
                Error::<T>::TooShortName
            );
            ensure!(
                name.len() <= T::MaxNameLength::get() as usize,
                Error::<T>::TooLongName
            );
            let caller_id = ensure_signed(origin)?;

            Marketplaces::<T>::mutate(marketplace_id, |x| {
                if let Some(market_info) = x {
                    if market_info.owner != caller_id {
                        return Err(Error::<T>::NotMarketplaceOwner);
                    }

                    market_info.name = name.clone();

                    Ok(())
                } else {
                    Err(Error::<T>::UnknownMarketplace)
                }
            })?;

            let event = Event::MarketplaceNameChanged(marketplace_id, name);
            Self::deposit_event(event);

            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", NFTIdOf<T> = "NFTId", CommonBalanceT<T> = "Balance")]
    pub enum Event<T: Config> {
        /// A nft has been listed for sale. \[nft id, nft currency, marketplace id\]
        NftListed(
            NFTId,
            NFTCurrency<BalanceCaps<T>, BalanceTiime<T>>,
            MarketplaceId,
        ),
        /// A nft is removed from the marketplace by its owner. \[nft id\]
        NftUnlisted(NFTId),
        /// A nft has been sold. \[nft id, new owner\]
        NftSold(NFTId, T::AccountId),
        /// A marketplace has been created.  \[marketplace id, new owner\]
        MarketplaceCreated(MarketplaceId, T::AccountId),
        /// Account added to marketplace.  \[marketplace id, account\]
        AccountAddedToMarketplace(MarketplaceId, T::AccountId),
        /// Account removed from marketplace.  \[marketplace id, account\]
        AccountRemovedFromMarketplace(MarketplaceId, T::AccountId),
        /// Marketplace changed owner.  \[marketplace id, new owner\]
        MarketplaceChangedOwner(MarketplaceId, T::AccountId),
        /// Marketplace changed type.  \[marketplace id, marketplace type\]
        MarketplaceTypeChanged(MarketplaceId, MarketplaceType),
        /// Marketplace changed name. \[marketplace id, marketplace name\]
        MarketplaceNameChanged(MarketplaceId, Vec<u8>),
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
        /// Commission fee cannot be more then 100.
        InvalidCommissionFeeValue,
        /// This function is reserved to the owner of a marketplace.
        NotMarketplaceOwner,
        /// This marketplace does not allow for this operation to be executed.
        UnsupportedMarketplace,
        /// Account not found.
        AccountNotFound,
        /// Internal math error.
        InternalMathError,
        /// Account not on the allow list should not be able to buy gated nfts.
        NotAllowed,
        /// Too short marketplace name
        TooShortName,
        /// Too long marketplace name
        TooLongName,
    }

    /// Nfts listed on the marketplace
    #[pallet::storage]
    #[pallet::getter(fn nft_for_sale)]
    pub type NFTsForSale<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        NFTId,
        SaleInformation<T::AccountId, BalanceCaps<T>, BalanceTiime<T>>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn marketplace_id_generator)]
    pub type MarketplaceIdGenerator<T: Config> = StorageValue<_, MarketplaceId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn marketplaces)]
    pub type Marketplaces<T: Config> =
        StorageMap<_, Blake2_128Concat, MarketplaceId, MarketplaceInformation<T>, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub nfts_for_sale: Vec<(
            NFTId,
            SaleInformation<T::AccountId, BalanceCaps<T>, BalanceTiime<T>>,
        )>,
        pub marketplaces: Vec<(MarketplaceId, MarketplaceInformation<T>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                nfts_for_sale: Default::default(),
                marketplaces: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            self.nfts_for_sale
                .clone()
                .into_iter()
                .for_each(|(nft_id, sale_information)| {
                    NFTsForSale::<T>::insert(nft_id, sale_information);
                });

            self.marketplaces
                .clone()
                .into_iter()
                .for_each(|(market_id, market_info)| {
                    Marketplaces::<T>::insert(market_id, market_info);
                });
        }
    }
}
