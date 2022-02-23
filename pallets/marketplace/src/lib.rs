#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;
mod migrations;
mod types;

use frame_support::dispatch::{DispatchErrorWithPostInfo, DispatchResult};
pub use pallet::*;
pub use types::*;

use default_weights::WeightInfo;
use frame_support::{
	ensure,
	pallet_prelude::DispatchResultWithPostInfo,
	traits::{
		Currency, ExistenceRequirement::KeepAlive, Get, OnUnbalanced, StorageVersion,
		WithdrawReasons,
	},
};
// use frame_support::weights::Weight;
use frame_system::Origin;
use sp_std::vec::Vec;
use ternoa_common::{helpers::check_bounds, traits::MarketplaceTrait};
use ternoa_primitives::{
	marketplace::{MarketplaceId, MarketplaceInformation, MarketplaceType},
	nfts::NFTId,
	TextFormat,
};

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, transactional};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{CheckedDiv, CheckedSub, StaticLookup};
	use ternoa_common::traits::NFTTrait;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Pallet managing nfts.
		type NFTs: NFTTrait<AccountId = Self::AccountId>;

		/// Weight values for this pallet
		type WeightInfo: WeightInfo;

		/// Caps Currency
		type Currency: Currency<Self::AccountId>;

		/// Place where the marketplace fees go.
		type FeesCollector: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// Min name length.
		#[pallet::constant]
		type MinNameLen: Get<u16>;

		/// Max name length.
		#[pallet::constant]
		type MaxNameLen: Get<u16>;

		/// Min description length.
		#[pallet::constant]
		type MinDescriptionLen: Get<u16>;

		/// Max description length.
		#[pallet::constant]
		type MaxDescriptionLen: Get<u16>;

		/// Min uri length.
		#[pallet::constant]
		type MinUriLen: Get<u16>;

		/// Max uri length.
		#[pallet::constant]
		type MaxUriLen: Get<u16>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/*         fn on_runtime_upgrade() -> frame_support::weights::Weight {
			migrations::migrate::<T>()
		} */
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Deposit a nft and list it on the marketplace
		#[pallet::weight(T::WeightInfo::list())]
		pub fn list(
			origin: OriginFor<T>,
			nft_id: NFTId,
			price: BalanceOf<T>,
			marketplace_id: Option<MarketplaceId>,
		) -> DispatchResultWithPostInfo {
			let account_id = ensure_signed(origin)?;
			let mkp_id = marketplace_id.unwrap_or(0);

			let nft = T::NFTs::get_nft(nft_id).ok_or(Error::<T>::UnknownNFT)?;
			ensure!(nft.owner == account_id, Error::<T>::NotNftOwner);
			ensure!(!nft.converted_to_capsule, Error::<T>::CannotListCapsules);
			ensure!(!nft.listed_for_sale, Error::<T>::AlreadyListedForSale);
			ensure!(nft.viewer.is_none(), Error::<T>::CannotListLentNFTs);

			let is_nft_in_completed_series =
				T::NFTs::is_nft_in_completed_series(nft_id) == Some(true);
			ensure!(is_nft_in_completed_series, Error::<T>::SeriesNotCompleted);

			let market = Marketplaces::<T>::get(mkp_id).ok_or(Error::<T>::UnknownMarketplace)?;

			if market.kind == MarketplaceType::Private {
				let is_on_list = market.allow_list.contains(&account_id);
				ensure!(is_on_list, Error::<T>::NotAllowedToList);
			} else {
				let is_on_list = market.disallow_list.contains(&account_id);
				ensure!(!is_on_list, Error::<T>::NotAllowedToList);
			}

			T::NFTs::set_listed_for_sale(nft_id, true)?;

			let sale_info = SaleInformation::new(account_id, price.clone(), mkp_id);
			NFTsForSale::<T>::insert(nft_id, sale_info);

			Self::deposit_event(Event::NftListed { nft_id, price, marketplace_id: mkp_id });

			Ok(().into())
		}

		/// Owner unlist the nfts
		#[pallet::weight(T::WeightInfo::unlist())]
		pub fn unlist(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(T::NFTs::owner(nft_id) == Some(who), Error::<T>::NotNftOwner);
			ensure!(NFTsForSale::<T>::contains_key(nft_id), Error::<T>::NftNotForSale);

			T::NFTs::set_listed_for_sale(nft_id, false)?;
			NFTsForSale::<T>::remove(nft_id);

			Self::deposit_event(Event::NftUnlisted { nft_id });

			Ok(().into())
		}

		/// Buy a listed nft
		#[pallet::weight(T::WeightInfo::buy())]
		#[transactional]
		pub fn buy(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			let sale = NFTsForSale::<T>::get(nft_id).ok_or(Error::<T>::NftNotForSale)?;
			ensure!(sale.account_id != caller, Error::<T>::NftAlreadyOwned);

			// Check if there is any commission fee.
			let market = Marketplaces::<T>::get(sale.marketplace_id)
				.ok_or(Error::<T>::UnknownMarketplace)?;

			let commission_fee = market.commission_fee;
			let mut price = sale.price;

			// KeepAlive because they need to be able to use the NFT later on
			if commission_fee != 0 {
				let tmp = 100u8.checked_div(commission_fee).ok_or(Error::<T>::InternalMathError)?;

				let fee = price.checked_div(&(tmp.into())).ok_or(Error::<T>::InternalMathError)?;

				price = price.checked_sub(&fee).ok_or(Error::<T>::InternalMathError)?;

				T::Currency::transfer(&caller, &market.owner, fee, KeepAlive)?;
			}

			T::Currency::transfer(&caller, &sale.account_id, price, KeepAlive)?;

			T::NFTs::set_listed_for_sale(nft_id, false)?;
			T::NFTs::set_owner(nft_id, &caller)?;

			NFTsForSale::<T>::remove(nft_id);

			let event = Event::NftSold { nft_id, owner: caller };
			Self::deposit_event(event);

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::create())]
		#[transactional]
		pub fn create(
			origin: OriginFor<T>,
			kind: MarketplaceType,
			commission_fee: u8,
			name: TextFormat,
			uri: Option<TextFormat>,
			logo_uri: Option<TextFormat>,
			description: Option<TextFormat>,
		) -> DispatchResultWithPostInfo {
			let caller_id = ensure_signed(origin)?;

			ensure!(commission_fee <= 100, Error::<T>::InvalidCommissionFeeValue);
			check_bounds(
				name.len(),
				(T::MinNameLen::get(), Error::<T>::TooShortMarketplaceName),
				(T::MaxNameLen::get(), Error::<T>::TooLongMarketplaceName),
			)?;

			if let Some(text) = uri.as_ref() {
				check_bounds(
					text.len(),
					(T::MinUriLen::get(), Error::<T>::TooShortUri),
					(T::MaxUriLen::get(), Error::<T>::TooLongUri),
				)?;
			}

			if let Some(text) = logo_uri.as_ref() {
				check_bounds(
					text.len(),
					(T::MinUriLen::get(), Error::<T>::TooShortLogoUri),
					(T::MaxUriLen::get(), Error::<T>::TooLongLogoUri),
				)?;
			}

			if let Some(text) = description.as_ref() {
				check_bounds(
					text.len(),
					(T::MinDescriptionLen::get(), Error::<T>::TooShortDescription),
					(T::MaxDescriptionLen::get(), Error::<T>::TooLongDescription),
				)?;
			}

			// Needs to have enough money
			let imbalance = T::Currency::withdraw(
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
			Self::deposit_event(Event::MarketplaceCreated { marketplace_id: id, owner: caller_id });

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

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == caller_id, Error::<T>::NotMarketplaceOwner);
				ensure!(
					market_info.kind == MarketplaceType::Private,
					Error::<T>::UnsupportedMarketplace
				);

				market_info.allow_list.push(account_id.clone());
				Ok(())
			})?;

			let event = Event::AccountAddedToAllowList { marketplace_id, owner: account_id };
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

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == caller_id, Error::<T>::NotMarketplaceOwner);
				ensure!(
					market_info.kind == MarketplaceType::Private,
					Error::<T>::UnsupportedMarketplace
				);

				let index = market_info.allow_list.iter().position(|x| *x == account_id);
				let index = index.ok_or(Error::<T>::AccountNotFound)?;
				market_info.allow_list.swap_remove(index);
				Ok(())
			})?;

			let event = Event::AccountRemovedFromAllowList { marketplace_id, owner: account_id };
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

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == caller_id, Error::<T>::NotMarketplaceOwner);
				ensure!(
					market_info.kind == MarketplaceType::Public,
					Error::<T>::UnsupportedMarketplace
				);

				market_info.disallow_list.push(account_id.clone());
				Ok(())
			})?;

			let event = Event::AccountAddedToDisallowList { marketplace_id, account_id };
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

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == caller_id, Error::<T>::NotMarketplaceOwner);
				ensure!(
					market_info.kind == MarketplaceType::Public,
					Error::<T>::UnsupportedMarketplace
				);

				let index = market_info.disallow_list.iter().position(|x| *x == account_id);
				let index = index.ok_or(Error::<T>::AccountNotFound)?;
				market_info.disallow_list.swap_remove(index);
				Ok(())
			})?;

			let event = Event::AccountRemovedFromDisallowList { marketplace_id, account_id };
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

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == caller_id, Error::<T>::NotMarketplaceOwner);
				market_info.owner = account_id.clone();
				Ok(())
			})?;

			let event = Event::MarketplaceChangedOwner { marketplace_id, owner: account_id };
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

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == caller_id, Error::<T>::NotMarketplaceOwner);

				market_info.kind = kind;
				Ok(())
			})?;

			let event = Event::MarketplaceTypeChanged { marketplace_id, kind };
			Self::deposit_event(event);

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::set_name())]
		pub fn set_name(
			origin: OriginFor<T>,
			marketplace_id: MarketplaceId,
			name: TextFormat,
		) -> DispatchResultWithPostInfo {
			let caller_id = ensure_signed(origin)?;

			check_bounds(
				name.len(),
				(T::MinNameLen::get(), Error::<T>::TooShortMarketplaceName),
				(T::MaxNameLen::get(), Error::<T>::TooLongMarketplaceName),
			)?;

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == caller_id, Error::<T>::NotMarketplaceOwner);
				market_info.name = name.clone();
				Ok(())
			})?;

			let event = Event::MarketplaceNameChanged { marketplace_id, name };
			Self::deposit_event(event);

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::set_marketplace_mint_fee())]
		pub fn set_marketplace_mint_fee(
			origin: OriginFor<T>,
			mint_fee: BalanceOf<T>,
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

			Marketplaces::<T>::mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == who, Error::<T>::NotMarketplaceOwner);
				market_info.commission_fee = commission_fee;
				Ok(())
			})?;

			let event =
				Event::MarketplaceCommissionFeeChanged { marketplace_id, fee: commission_fee };
			Self::deposit_event(event);

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::set_uri())]
		pub fn set_uri(
			origin: OriginFor<T>,
			marketplace_id: MarketplaceId,
			uri: TextFormat,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			check_bounds(
				uri.len(),
				(T::MinUriLen::get(), Error::<T>::TooShortUri),
				(T::MaxUriLen::get(), Error::<T>::TooLongUri),
			)?;

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == who, Error::<T>::NotMarketplaceOwner);
				market_info.uri = Some(uri.clone());
				Ok(())
			})?;

			let event = Event::MarketplaceUriUpdated { marketplace_id, uri };
			Self::deposit_event(event);
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::set_logo_uri())]
		pub fn set_logo_uri(
			origin: OriginFor<T>,
			marketplace_id: MarketplaceId,
			logo_uri: TextFormat,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			check_bounds(
				logo_uri.len(),
				(T::MinUriLen::get(), Error::<T>::TooShortLogoUri),
				(T::MaxUriLen::get(), Error::<T>::TooLongLogoUri),
			)?;

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == who, Error::<T>::NotMarketplaceOwner);
				market_info.logo_uri = Some(logo_uri.clone());
				Ok(())
			})?;

			let event = Event::MarketplaceLogoUriUpdated { marketplace_id, uri: logo_uri };
			Self::deposit_event(event);
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::set_logo_uri())]
		pub fn set_description(
			origin: OriginFor<T>,
			marketplace_id: MarketplaceId,
			description: TextFormat,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			check_bounds(
				description.len(),
				(T::MinDescriptionLen::get(), Error::<T>::TooShortDescription),
				(T::MaxDescriptionLen::get(), Error::<T>::TooLongDescription),
			)?;

			Marketplaces::<T>::try_mutate(marketplace_id, |x| -> DispatchResult {
				let market_info = x.as_mut().ok_or(Error::<T>::UnknownMarketplace)?;
				ensure!(market_info.owner == who, Error::<T>::NotMarketplaceOwner);
				market_info.description = Some(description.clone());
				Ok(())
			})?;

			let event = Event::MarketplaceDescriptionUpdated { marketplace_id, description };
			Self::deposit_event(event);
			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A nft has been listed for sale.
		NftListed { nft_id: NFTId, price: BalanceOf<T>, marketplace_id: MarketplaceId },
		/// A nft is removed from the marketplace by its owner.
		NftUnlisted { nft_id: NFTId },
		/// A nft has been sold.
		NftSold { nft_id: NFTId, owner: T::AccountId },
		/// A marketplace has been created.
		MarketplaceCreated { marketplace_id: MarketplaceId, owner: T::AccountId },
		/// Account added to marketplace allow list.
		AccountAddedToAllowList { marketplace_id: MarketplaceId, owner: T::AccountId },
		/// Account removed from marketplace allow list.
		AccountRemovedFromAllowList { marketplace_id: MarketplaceId, owner: T::AccountId },
		/// Marketplace changed owner.
		MarketplaceChangedOwner { marketplace_id: MarketplaceId, owner: T::AccountId },
		/// Marketplace changed type.
		MarketplaceTypeChanged { marketplace_id: MarketplaceId, kind: MarketplaceType },
		/// Marketplace changed name.
		MarketplaceNameChanged { marketplace_id: MarketplaceId, name: TextFormat },
		/// Marketplace mint fee changed.
		MarketplaceMintFeeChanged { fee: BalanceOf<T> },
		/// Marketplace mint fee changed.
		MarketplaceCommissionFeeChanged { marketplace_id: MarketplaceId, fee: u8 },
		/// Marketplace TextFormat updated.
		MarketplaceUriUpdated { marketplace_id: MarketplaceId, uri: TextFormat },
		/// Marketplace Logo TextFormat updated.
		MarketplaceLogoUriUpdated { marketplace_id: MarketplaceId, uri: TextFormat },
		/// Account added to disallow list for a marketplace.
		AccountAddedToDisallowList { marketplace_id: MarketplaceId, account_id: T::AccountId },
		/// Account removed from disallow list for a marketplace.
		AccountRemovedFromDisallowList { marketplace_id: MarketplaceId, account_id: T::AccountId },
		/// Marketplace description updated.
		MarketplaceDescriptionUpdated { marketplace_id: MarketplaceId, description: TextFormat },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// TODO!
		CannotListLentNFTs,

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
		/// Account not allowed to list NFTs on that marketplace.
		NotAllowedToList,
		/// Marketplace name is too short.
		TooShortMarketplaceName,
		/// Marketplace name is too long.
		TooLongMarketplaceName,
		/// Series is not completed.
		SeriesNotCompleted,
		// Marketplace uri is too long.
		TooLongUri,
		// Marketplace uri is too short.
		TooShortUri,
		// Marketplace logo uri is too long.
		TooLongLogoUri,
		// Marketplace logo uri is too short.
		TooShortLogoUri,
		/// Nft is capsulized.
		CannotListCapsules,
		/// Marketplace description in too short.
		TooShortDescription,
		/// Marketplace description in too long.
		TooLongDescription,
		/// TODO!
		AlreadyListedForSale,
		/// TODO!
		UnknownNFT,
	}

	/// Nfts listed on the marketplace
	#[pallet::storage]
	#[pallet::getter(fn nft_for_sale)]
	pub type NFTsForSale<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		NFTId,
		SaleInformation<T::AccountId, BalanceOf<T>>,
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
	pub type MarketplaceMintFee<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub nfts_for_sale: Vec<(NFTId, SaleInformation<T::AccountId, BalanceOf<T>>)>,
		pub marketplaces: Vec<(MarketplaceId, MarketplaceInformation<T::AccountId>)>,
		pub marketplace_mint_fee: BalanceOf<T>,
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
			self.nfts_for_sale.clone().into_iter().for_each(|(nft_id, sale_information)| {
				NFTsForSale::<T>::insert(nft_id, sale_information);
			});

			self.marketplaces.clone().into_iter().for_each(|(market_id, market_info)| {
				Marketplaces::<T>::insert(market_id, market_info);
			});
			MarketplaceMintFee::<T>::put(self.marketplace_mint_fee);
		}
	}
}

impl<T: Config> MarketplaceTrait<T::AccountId> for Pallet<T> {
	// Return if an account is permitted to list on given marketplace
	fn is_allowed_to_list(
		marketplace_id: MarketplaceId,
		account_id: T::AccountId,
	) -> DispatchResult {
		let market =
			Marketplaces::<T>::get(marketplace_id).ok_or(Error::<T>::UnknownMarketplace)?;

		if market.kind == MarketplaceType::Private {
			let is_on_list = market.allow_list.contains(&account_id);
			ensure!(is_on_list, Error::<T>::NotAllowedToList);
			Ok(())
		} else {
			let is_on_list = market.disallow_list.contains(&account_id);
			ensure!(!is_on_list, Error::<T>::NotAllowedToList);
			Ok(())
		}
	}

	// Return the owner account and commision for marketplace with `marketplace_id`
	fn get_marketplace(
		marketplace_id: MarketplaceId,
	) -> Option<MarketplaceInformation<T::AccountId>> {
		match Marketplaces::<T>::get(marketplace_id) {
			Some(marketplace) => Some(marketplace),
			None => None,
		}
	}

	// create a new marketplace
	fn create(
		caller_id: <T as frame_system::Config>::AccountId,
		kind: MarketplaceType,
		commission_fee: u8,
		name: TextFormat,
		uri: Option<TextFormat>,
		logo_uri: Option<TextFormat>,
		description: Option<TextFormat>,
	) -> Result<MarketplaceId, DispatchErrorWithPostInfo> {
		Self::create(
			Origin::<T>::Signed(caller_id).into(),
			kind,
			commission_fee,
			name,
			uri,
			logo_uri,
			description,
		)?;

		Ok(MarketplaceIdGenerator::<T>::get())
	}
}
