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
use ternoa_primitives::TernoaString;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(7);

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
    use ternoa_common::traits;
    use ternoa_common::traits::{CapsulesTrait, LockableNFTs, NFTs};

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
        type NFTs: LockableNFTs<AccountId = Self::AccountId> + NFTs<AccountId = Self::AccountId>;
        /// Weight values for this pallet
        type WeightInfo: WeightInfo;

        /// Caps Currency
        type CurrencyCaps: Currency<Self::AccountId>;

        /// Tiime Currency
        type CurrencyTiime: Currency<Self::AccountId>;

        /// Place where the marketplace fees go.
        type FeesCollector: OnUnbalanced<NegativeImbalanceCaps<Self>>;

        /// Capsule trait
        type CapsulesTrait: traits::CapsulesTrait;

        /// The minimum length a string may be.
        #[pallet::constant]
        type MinStringLength: Get<u16>;

        /// The maximum length a string may be.
        #[pallet::constant]
        type MaxStringLength: Get<u16>;

        /// The minimum length a description may be.
        #[pallet::constant]
        type MinDescriptionLength: Get<u16>;

        /// The maximum length a description may be.
        #[pallet::constant]
        type MaxDescriptionLength: Get<u16>;
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
            let mkp_id = marketplace_id.unwrap_or(0);

            let is_owner = T::NFTs::owner(nft_id) == Some(account_id.clone());
            ensure!(is_owner, Error::<T>::NotNftOwner);

            let is_series_completed = T::NFTs::is_series_completed(nft_id) == Some(true);
            ensure!(is_series_completed, Error::<T>::SeriesNotCompleted);

            let is_capsulized = T::CapsulesTrait::is_capsulized(nft_id);
            ensure!(!is_capsulized, Error::<T>::NFTIsCapsulized);

            let market = Marketplaces::<T>::get(mkp_id).ok_or(Error::<T>::UnknownMarketplace)?;

            if market.kind == MarketplaceType::Private {
                let is_on_list = market.allow_list.contains(&account_id);
                ensure!(is_on_list, Error::<T>::NotAllowed);
            }

            T::NFTs::lock(nft_id)?;

            let sale_info = SaleInformation::new(account_id, price.clone(), mkp_id);
            NFTsForSale::<T>::insert(nft_id, sale_info);

            Self::deposit_event(Event::NftListed {
                nft_id,
                price,
                marketplace_id: mkp_id,
            });

            Ok(().into())
        }

        /// Owner unlist the nfts
        #[pallet::weight(T::WeightInfo::unlist())]
        pub fn unlist(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(T::NFTs::owner(nft_id) == Some(who), Error::<T>::NotNftOwner);
            ensure!(
                NFTsForSale::<T>::contains_key(nft_id),
                Error::<T>::NftNotForSale
            );

            T::NFTs::unlock(nft_id);
            NFTsForSale::<T>::remove(nft_id);

            Self::deposit_event(Event::NftUnlisted { nft_id });

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

            let sale = NFTsForSale::<T>::get(nft_id).ok_or(Error::<T>::NftNotForSale)?;
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

            Self::deposit_event(Event::NftSold {
                nft_id,
                owner: caller_id,
            });

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::create())]
        #[transactional]
        pub fn create(
            origin: OriginFor<T>,
            kind: MarketplaceType,
            commission_fee: u8,
            name: TernoaString,
            uri: Option<URI>,
            logo_uri: Option<URI>,
            description: Option<TernoaString>,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;

            ensure!(commission_fee <= 100, Error::<T>::InvalidCommissionFeeValue);
            let lower_bound = name.len() >= T::MinStringLength::get() as usize;
            let upper_bound = name.len() <= T::MaxStringLength::get() as usize;
            ensure!(lower_bound, Error::<T>::TooShortMarketplaceName);
            ensure!(upper_bound, Error::<T>::TooLongMarketplaceName);

            if let Some(uri_value) = uri.as_ref() {
                ensure!(
                    uri_value.len() <= T::MaxStringLength::get() as usize,
                    Error::<T>::TooLongMarketplaceUri
                );
                ensure!(
                    uri_value.len() >= T::MinStringLength::get() as usize,
                    Error::<T>::TooShortMarketplaceUri
                );
            }

            if let Some(logo_uri_value) = logo_uri.as_ref() {
                ensure!(
                    logo_uri_value.len() <= T::MaxStringLength::get() as usize,
                    Error::<T>::TooLongMarketplaceLogoUri
                );
                ensure!(
                    logo_uri_value.len() >= T::MinStringLength::get() as usize,
                    Error::<T>::TooShortMarketplaceLogoUri
                );
            }

            if let Some(description_value) = description.as_ref() {
                ensure!(
                    description_value.len() >= T::MinDescriptionLength::get() as usize,
                    Error::<T>::TooShortMarketplaceDescription
                );
                ensure!(
                    description_value.len() <= T::MaxDescriptionLength::get() as usize,
                    Error::<T>::TooLongMarketplaceDescription
                );
            }

            // Needs to have enough money
            let imbalance = T::CurrencyCaps::withdraw(
                &caller_id,
                MarketplaceMintFee::<T>::get(),
                WithdrawReasons::FEE,
                KeepAlive,
            )?;
            T::FeesCollector::on_unbalanced(imbalance);

            let marketplace = MarketplaceInformation::new(
                kind,
                commission_fee,
                caller_id.clone(),
                Vec::default(),
                Vec::default(),
                name,
                uri,
                logo_uri,
                description,
            );

            let id = MarketplaceIdGenerator::<T>::get();
            let id = id.checked_add(1).ok_or(Error::<T>::MarketplaceIdOverflow)?;

            Marketplaces::<T>::insert(id, marketplace);
            MarketplaceIdGenerator::<T>::set(id);
            Self::deposit_event(Event::MarketplaceCreated {
                marketplace_id: id,
                owner: caller_id,
            });

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

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(
                    market_info.owner == caller_id,
                    Error::<T>::NotMarketplaceOwner
                );
                ensure!(
                    market_info.kind == MarketplaceType::Private,
                    Error::<T>::UnsupportedMarketplace
                );
                market_info.allow_list.push(account_id.clone());
                Ok(())
            })?;

            let event = Event::AccountAddedToAllowList {
                marketplace_id,
                owner: account_id,
            };
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

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(
                    market_info.owner == caller_id,
                    Error::<T>::NotMarketplaceOwner
                );
                ensure!(
                    market_info.kind == MarketplaceType::Private,
                    Error::<T>::UnsupportedMarketplace
                );
                let index = market_info.allow_list.iter().position(|x| *x == account_id);
                let index = index.ok_or(Error::<T>::AccountNotFound)?;
                market_info.allow_list.swap_remove(index);
                Ok(())
            })?;

            let event = Event::AccountRemovedFromAllowList {
                marketplace_id,
                owner: account_id,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::add_account_to_disallow_list())]
        pub fn add_account_to_disallow_list(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            account_id: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;
            let account_id = T::Lookup::lookup(account_id)?;

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(
                    market_info.owner == caller_id,
                    Error::<T>::NotMarketplaceOwner
                );
                ensure!(
                    market_info.kind == MarketplaceType::Public,
                    Error::<T>::UnsupportedMarketplace
                );
                market_info.disallow_list.push(account_id.clone());
                Ok(())
            })?;

            let event = Event::AccountAddedToDisallowList {
                marketplace_id,
                account_id,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::remove_account_from_disallow_list())]
        pub fn remove_account_from_disallow_list(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            account_id: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;
            let account_id = T::Lookup::lookup(account_id)?;

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(
                    market_info.owner == caller_id,
                    Error::<T>::NotMarketplaceOwner
                );
                ensure!(
                    market_info.kind == MarketplaceType::Public,
                    Error::<T>::UnsupportedMarketplace
                );
                let index = market_info
                    .disallow_list
                    .iter()
                    .position(|x| *x == account_id);
                let index = index.ok_or(Error::<T>::AccountNotFound)?;
                market_info.disallow_list.swap_remove(index);
                Ok(())
            })?;

            let event = Event::AccountRemovedFromDisallowList {
                marketplace_id,
                account_id,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::set_owner())]
        pub fn set_owner(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            account_id: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;
            let account_id = T::Lookup::lookup(account_id)?;

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(
                    market_info.owner == caller_id,
                    Error::<T>::NotMarketplaceOwner
                );
                market_info.owner = account_id.clone();
                Ok(())
            })?;

            let event = Event::MarketplaceChangedOwner {
                marketplace_id,
                owner: account_id,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::set_market_type())]
        pub fn set_market_type(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            kind: MarketplaceType,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(
                    market_info.owner == caller_id,
                    Error::<T>::NotMarketplaceOwner
                );
                market_info.kind = kind;
                Ok(())
            })?;

            let event = Event::MarketplaceTypeChanged {
                marketplace_id,
                kind,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::set_name())]
        pub fn set_name(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            name: TernoaString,
        ) -> DispatchResultWithPostInfo {
            let caller_id = ensure_signed(origin)?;

            let lower_bound = name.len() >= T::MinStringLength::get() as usize;
            let upper_bound = name.len() <= T::MaxStringLength::get() as usize;
            ensure!(lower_bound, Error::<T>::TooShortMarketplaceName);
            ensure!(upper_bound, Error::<T>::TooLongMarketplaceName);

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(
                    market_info.owner == caller_id,
                    Error::<T>::NotMarketplaceOwner
                );
                market_info.name = name.clone();
                Ok(())
            })?;

            let event = Event::MarketplaceNameChanged {
                marketplace_id,
                name,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::set_marketplace_mint_fee())]
        pub fn set_marketplace_mint_fee(
            origin: OriginFor<T>,
            mint_fee: BalanceCaps<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            MarketplaceMintFee::<T>::put(mint_fee);

            Self::deposit_event(Event::MarketplaceMintFeeChanged { fee: mint_fee });

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::set_commission_fee())]
        pub fn set_commission_fee(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            commission_fee: u8,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(commission_fee <= 100, Error::<T>::InvalidCommissionFeeValue);

            Marketplaces::<T>::mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(market_info.owner == who, Error::<T>::NotMarketplaceOwner);
                market_info.commission_fee = commission_fee;
                Ok(())
            })?;

            let event = Event::MarketplaceCommissionFeeChanged {
                marketplace_id,
                fee: commission_fee,
            };
            Self::deposit_event(event);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::set_uri())]
        pub fn set_uri(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            uri: URI,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(
                uri.len() <= T::MaxStringLength::get() as usize,
                Error::<T>::TooLongMarketplaceUri
            );
            ensure!(
                uri.len() >= T::MinStringLength::get() as usize,
                Error::<T>::TooShortMarketplaceUri
            );

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(market_info.owner == who, Error::<T>::NotMarketplaceOwner);
                market_info.uri = Some(uri.clone());
                Ok(())
            })?;

            let event = Event::MarketplaceUriUpdated {
                marketplace_id,
                uri,
            };
            Self::deposit_event(event);
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::set_logo_uri())]
        pub fn set_logo_uri(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            logo_uri: URI,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(
                logo_uri.len() <= T::MaxStringLength::get() as usize,
                Error::<T>::TooLongMarketplaceLogoUri
            );
            ensure!(
                logo_uri.len() >= T::MinStringLength::get() as usize,
                Error::<T>::TooShortMarketplaceLogoUri
            );

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(market_info.owner == who, Error::<T>::NotMarketplaceOwner);
                market_info.logo_uri = Some(logo_uri.clone());
                Ok(())
            })?;

            let event = Event::MarketplaceLogoUriUpdated {
                marketplace_id,
                uri: logo_uri,
            };
            Self::deposit_event(event);
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::set_logo_uri())]
        pub fn set_description(
            origin: OriginFor<T>,
            marketplace_id: MarketplaceId,
            description: TernoaString,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(
                description.len() <= T::MaxDescriptionLength::get() as usize,
                Error::<T>::TooShortMarketplaceDescription
            );
            ensure!(
                description.len() >= T::MinDescriptionLength::get() as usize,
                Error::<T>::TooLongMarketplaceDescription
            );

            Marketplaces::<T>::try_mutate(marketplace_id, |x| -> Result<(), Error<T>> {
                let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
                ensure!(market_info.owner == who, Error::<T>::NotMarketplaceOwner);
                market_info.description = Some(description.clone());
                Ok(())
            })?;

            let event = Event::MarketplaceDescriptionUpdated {
                marketplace_id,
                description,
            };
            Self::deposit_event(event);
            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A nft has been listed for sale.
        NftListed {
            nft_id: NFTId,
            price: NFTCurrency<BalanceCaps<T>, BalanceTiime<T>>,
            marketplace_id: MarketplaceId,
        },
        /// A nft is removed from the marketplace by its owner.
        NftUnlisted { nft_id: NFTId },
        /// A nft has been sold.
        NftSold { nft_id: NFTId, owner: T::AccountId },
        /// A marketplace has been created.
        MarketplaceCreated {
            marketplace_id: MarketplaceId,
            owner: T::AccountId,
        },
        /// Account added to marketplace allow list.
        AccountAddedToAllowList {
            marketplace_id: MarketplaceId,
            owner: T::AccountId,
        },
        /// Account removed from marketplace allow list.
        AccountRemovedFromAllowList {
            marketplace_id: MarketplaceId,
            owner: T::AccountId,
        },
        /// Marketplace changed owner.
        MarketplaceChangedOwner {
            marketplace_id: MarketplaceId,
            owner: T::AccountId,
        },
        /// Marketplace changed type.
        MarketplaceTypeChanged {
            marketplace_id: MarketplaceId,
            kind: MarketplaceType,
        },
        /// Marketplace changed name.
        MarketplaceNameChanged {
            marketplace_id: MarketplaceId,
            name: TernoaString,
        },
        /// Marketplace mint fee changed.
        MarketplaceMintFeeChanged { fee: BalanceCaps<T> },
        /// Marketplace mint fee changed.
        MarketplaceCommissionFeeChanged {
            marketplace_id: MarketplaceId,
            fee: u8,
        },
        /// Marketplace URI updated.
        MarketplaceUriUpdated {
            marketplace_id: MarketplaceId,
            uri: URI,
        },
        /// Marketplace Logo URI updated.
        MarketplaceLogoUriUpdated {
            marketplace_id: MarketplaceId,
            uri: URI,
        },
        /// Account added to disallow list for a marketplace.
        AccountAddedToDisallowList {
            marketplace_id: MarketplaceId,
            account_id: T::AccountId,
        },
        /// Account removed from disallow list for a marketplace.
        AccountRemovedFromDisallowList {
            marketplace_id: MarketplaceId,
            account_id: T::AccountId,
        },
        /// Marketplace description updated.
        MarketplaceDescriptionUpdated {
            marketplace_id: MarketplaceId,
            description: TernoaString,
        },
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
        /// Marketplace name is too short.
        TooShortMarketplaceName,
        /// Marketplace name is too long.
        TooLongMarketplaceName,
        /// Series is not completed.
        SeriesNotCompleted,
        // Marketplace URI is too long.
        TooLongMarketplaceUri,
        // Marketplace URI is too short.
        TooShortMarketplaceUri,
        // Marketplace Logo URI is too long.
        TooLongMarketplaceLogoUri,
        // Marketplace Logo URI is too short.
        TooShortMarketplaceLogoUri,
        /// Nft is capsulized.
        NFTIsCapsulized,
        /// Marketplace Description in too short.
        TooShortMarketplaceDescription,
        /// Marketplace Description in too long.
        TooLongMarketplaceDescription,
    }

    /// Nfts listed on the marketplace
    #[pallet::storage]
    #[pallet::getter(fn nft_for_sale)]
    pub type NFTsForSale<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        NFTId,
        SaleInformation<T::AccountId, BalanceCaps<T>, BalanceTiime<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn marketplace_id_generator)]
    pub type MarketplaceIdGenerator<T: Config> = StorageValue<_, MarketplaceId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn marketplaces)]
    pub type Marketplaces<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        MarketplaceId,
        MarketplaceInformation<T::AccountId>,
        OptionQuery,
    >;

    /// Host much does it cost to create a marketplace.
    #[pallet::storage]
    #[pallet::getter(fn marketplace_mint_fee)]
    pub type MarketplaceMintFee<T: Config> = StorageValue<_, BalanceCaps<T>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub nfts_for_sale: Vec<(
            NFTId,
            SaleInformation<T::AccountId, BalanceCaps<T>, BalanceTiime<T>>,
        )>,
        pub marketplaces: Vec<(MarketplaceId, MarketplaceInformation<T::AccountId>)>,
        pub marketplace_mint_fee: BalanceCaps<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                nfts_for_sale: Default::default(),
                marketplaces: Default::default(),
                marketplace_mint_fee: Default::default(),
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
            MarketplaceMintFee::<T>::put(self.marketplace_mint_fee);
        }
    }
}
