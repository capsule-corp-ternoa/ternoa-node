#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod migrations;
pub mod weights;

use frame_support::{
	dispatch::{DispatchErrorWithPostInfo, DispatchResult},
	pallet_prelude::ensure,
	traits::StorageVersion,
};
use frame_system::Origin;
pub use pallet::*;
use sp_std::{vec, vec::Vec};
use ternoa_common::traits;
use ternoa_primitives::{
	nfts::{NFTData, NFTId, NFTSeriesDetails, NFTSeriesId},
	TextFormat,
};
pub use weights::WeightInfo;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement::KeepAlive, OnUnbalanced, WithdrawReasons},
		transactional,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::StaticLookup;
	use ternoa_common::helpers::check_bounds;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: WeightInfo;

		/// Currency used to bill minting fees
		type Currency: Currency<Self::AccountId>;

		/// What we do with additional fees
		type FeesCollector: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// Min Ipfs len
		#[pallet::constant]
		type MinIpfsLen: Get<u16>;

		/// Max Uri len
		#[pallet::constant]
		type MaxIpfsLen: Get<u16>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

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
		/// Create a new NFT with the provided details. An ID will be auto
		/// generated and logged as an event, The caller of this function
		/// will become the owner of the new NFT.
		#[pallet::weight(T::WeightInfo::create())]
		// have to be transactional otherwise we could make people pay the mint
		// even if the creation fails.
		#[transactional]
		pub fn create(
			origin: OriginFor<T>,
			ipfs_reference: TextFormat,
			series_id: Option<NFTSeriesId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			check_bounds(
				ipfs_reference.len(),
				(T::MinIpfsLen::get(), Error::<T>::IPFSReferenceIsTooShort),
				(T::MaxIpfsLen::get(), Error::<T>::IPFSReferenceIsTooLong),
			)?;

			// Checks
			// The Caller needs to pay the NFT Mint fee.
			let mint_fee = NftMintFee::<T>::get();
			let reason = WithdrawReasons::FEE;
			let imbalance = T::Currency::withdraw(&who, mint_fee, reason, KeepAlive)?;
			T::FeesCollector::on_unbalanced(imbalance);

			// Check if the series exists. If it exists and the caller is not the owner throw error.
			let mut series_exists = false;
			if let Some(id) = &series_id {
				if let Some(series) = Series::<T>::get(id) {
					ensure!(series.owner == who, Error::<T>::NotTheSeriesOwner);
					ensure!(series.draft, Error::<T>::CannotCreateNFTsWithCompletedSeries);
					series_exists = true;
				}
			}

			// Execute
			let nft_id = Self::generate_nft_id();
			let series_id = series_id.unwrap_or_else(|| Self::generate_series_id());

			let value =
				NFTData::new_default(who.clone(), ipfs_reference.clone(), series_id.clone());

			// Save
			Data::<T>::insert(nft_id, value);
			if !series_exists {
				Series::<T>::insert(series_id.clone(), NFTSeriesDetails::new(who.clone(), true));
			}

			let event =
				Event::NFTCreated { nft_id, owner: who, series_id, ipfs_reference, mint_fee };
			Self::deposit_event(event);

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

			let mut data = Data::<T>::get(id).ok_or(Error::<T>::NFTNotFound)?;
			let series = Series::<T>::get(&data.series_id).ok_or(Error::<T>::SeriesNotFound)?;

			ensure!(data.owner == who, Error::<T>::NotTheNFTOwner);
			ensure!(!data.listed_for_sale, Error::<T>::CannotTransferNFTsListedForSale);
			ensure!(!data.converted_to_capsule, Error::<T>::CannotTransferCapsules);
			ensure!(!data.in_transmission, Error::<T>::CannotTransferNFTsInTransmission);
			ensure!(data.viewer.is_none(), Error::<T>::CannotTransferLentNFTs);
			ensure!(!series.draft, Error::<T>::CannotTransferNFTsInUncompletedSeries);

			data.owner = to.clone();
			Data::<T>::insert(id, data);

			let event = Event::NFTTransferred { nft_id: id, old_owner: who, new_owner: to };
			Self::deposit_event(event);

			Ok(().into())
		}

		/// Remove an NFT from the storage. This operation is irreversible which means
		/// once the NFT is removed (burned) from the storage there is no way to
		/// get it back.
		/// Must be called by the owner of the NFT.
		#[pallet::weight(T::WeightInfo::burn())]
		pub fn burn(origin: OriginFor<T>, id: NFTId) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let data = Data::<T>::get(id).ok_or(Error::<T>::NFTNotFound)?;

			ensure!(data.owner == who, Error::<T>::NotTheNFTOwner);
			ensure!(!data.listed_for_sale, Error::<T>::CannotBurnNFTsListedForSale);
			ensure!(!data.converted_to_capsule, Error::<T>::CannotBurnCapsules);
			ensure!(!data.in_transmission, Error::<T>::CannotBurnNFTsInTransmission);
			ensure!(data.viewer.is_none(), Error::<T>::CannotBurnLentNFTs);

			Data::<T>::remove(id);
			Self::deposit_event(Event::NFTBurned { nft_id: id });

			Ok(().into())
		}

		/// Makes the series completed. This means that is not anymore
		/// possible to add new NFTs to the series.
		#[pallet::weight(T::WeightInfo::finish_series())]
		pub fn finish_series(
			origin: OriginFor<T>,
			series_id: NFTSeriesId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Series::<T>::mutate(&series_id, |x| -> DispatchResult {
				let series = x.as_mut().ok_or(Error::<T>::SeriesNotFound)?;
				ensure!(series.owner == who, Error::<T>::NotTheSeriesOwner);

				series.draft = false;

				Ok(())
			})?;

			Self::deposit_event(Event::SeriesFinished { series_id });

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::set_nft_mint_fee())]
		pub fn set_nft_mint_fee(
			origin: OriginFor<T>,
			mint_fee: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			NftMintFee::<T>::put(mint_fee);

			Self::deposit_event(Event::NFTMintFeeUpdated { fee: mint_fee });

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::lend())]
		pub fn lend(
			origin: OriginFor<T>,
			nft_id: NFTId,
			viewer: Option<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Data::<T>::try_mutate(nft_id, |maybe_data| -> DispatchResult {
				let data = maybe_data.as_mut().ok_or(Error::<T>::NFTNotFound)?;

				ensure!(data.owner == who, Error::<T>::NotTheNFTOwner);
				ensure!(!data.listed_for_sale, Error::<T>::CannotLendNFTsListedForSale);
				ensure!(!data.converted_to_capsule, Error::<T>::CannotLendCapsules);
				ensure!(!data.in_transmission, Error::<T>::CannotLendNFTsInTransmission);

				if let Some(viewer) = &viewer {
					ensure!(who != *viewer, Error::<T>::CannotLendNFTsToYourself);
				}

				data.viewer = viewer.clone();

				Ok(().into())
			})?;

			let event = Event::NFTLent { nft_id, viewer };
			Self::deposit_event(event);

			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new NFT was created.
		NFTCreated {
			nft_id: NFTId,
			owner: T::AccountId,
			series_id: NFTSeriesId,
			ipfs_reference: TextFormat,
			mint_fee: BalanceOf<T>,
		},
		/// An NFT was transferred to someone else.
		NFTTransferred { nft_id: NFTId, old_owner: T::AccountId, new_owner: T::AccountId },
		/// An NFT was burned.
		NFTBurned { nft_id: NFTId },
		/// A series has been completed.
		SeriesFinished { series_id: NFTSeriesId },
		/// Nft mint fee changed.
		NFTMintFeeUpdated { fee: BalanceOf<T> },
		/// An NFT was lent to someone else or it was returned.
		NFTLent { nft_id: NFTId, viewer: Option<T::AccountId> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Operation not allowed because the NFT is a capsule.
		CannotTransferCapsules,
		/// Operation not allowed because the NFT is a capsule.
		CannotBurnCapsules,
		/// Operation not allowed because the NFT is a capsule.
		CannotLendCapsules,

		/// Operation not allowed because the NFT is listed for sale.
		CannotTransferNFTsListedForSale,
		/// Operation not allowed because the NFT is listed for sale.
		CannotBurnNFTsListedForSale,
		/// Operation not allowed because the NFT is listed for sale.
		CannotLendNFTsListedForSale,

		/// Operation not allowed because the NFT is in transmission.
		CannotTransferNFTsInTransmission,
		/// Operation not allowed because the NFT is in transmission.
		CannotBurnNFTsInTransmission,
		/// Operation not allowed because the NFT is in transmission.
		CannotLendNFTsInTransmission,

		/// Operation is not allowed because the NFT is lent.
		CannotTransferLentNFTs,
		/// Operation is not allowed because the NFT is lent.
		CannotBurnLentNFTs,

		/// Operation is not allowed because the series is in draft.
		CannotTransferNFTsInUncompletedSeries,
		/// Operation is not allowed because the NFT is inside a completed series.
		CannotCreateNFTsWithCompletedSeries,
		/// Cannot lends NFTs to yourself.
		CannotLendNFTsToYourself,
		/// Ipfs reference is too short.
		IPFSReferenceIsTooShort,
		/// Ipfs reference is too long.
		IPFSReferenceIsTooLong,
		/// No NFT was found with that NFT id.
		NFTNotFound,
		/// This function can only be called by the owner of the NFT.
		NotTheNFTOwner,
		/// Cannot add NFTs to a series that is not owned.
		NotTheSeriesOwner,
		/// Series not Found.
		SeriesNotFound,
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

	/// Host much does it cost to mint a NFT (extra fee on top of the tx fees)
	#[pallet::storage]
	#[pallet::getter(fn nft_mint_fee)]
	pub type NftMintFee<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub nfts: Vec<(NFTId, NFTData<T::AccountId>)>,
		pub series: Vec<(NFTSeriesId, NFTSeriesDetails<T::AccountId>)>,
		pub nft_mint_fee: BalanceOf<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				nfts: Default::default(),
				series: Default::default(),
				nft_mint_fee: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			self.series.clone().into_iter().for_each(|(series_id, series)| {
				Series::<T>::insert(series_id, series);
			});

			let mut current_nft_id: NFTId = 0;
			self.nfts.clone().into_iter().for_each(|(nft_id, data)| {
				Data::<T>::insert(nft_id, data);
				current_nft_id = current_nft_id.max(nft_id);
			});

			if !self.nfts.is_empty() {
				current_nft_id += 1;
			}

			NftIdGenerator::<T>::put(current_nft_id);
			SeriesIdGenerator::<T>::put(0);
			NftMintFee::<T>::put(self.nft_mint_fee);
		}
	}
}

impl<T: Config> traits::NFTTrait for Pallet<T> {
	type AccountId = T::AccountId;

	fn set_owner(id: NFTId, owner: &Self::AccountId) -> DispatchResult {
		Data::<T>::try_mutate(id, |data| -> DispatchResult {
			let data = data.as_mut().ok_or(Error::<T>::NFTNotFound)?;
			data.owner = owner.clone();
			Ok(())
		})?;

		Ok(())
	}

	fn owner(id: NFTId) -> Option<Self::AccountId> {
		Some(Data::<T>::get(id)?.owner)
	}

	fn is_nft_in_completed_series(id: NFTId) -> Option<bool> {
		let series_id = Data::<T>::get(id)?.series_id;
		Some(!Series::<T>::get(series_id)?.draft)
	}

	fn create_nft(
		owner: Self::AccountId,
		ipfs_reference: TextFormat,
		series_id: Option<NFTSeriesId>,
	) -> Result<NFTId, DispatchErrorWithPostInfo> {
		Self::create(Origin::<T>::Signed(owner).into(), ipfs_reference, series_id)?;
		return Ok(Self::nft_id_generator() - 1)
	}

	fn get_nft(id: NFTId) -> Option<NFTData<Self::AccountId>> {
		Data::<T>::get(id)
	}

	fn benchmark_lock_series(series_id: NFTSeriesId) {
		Series::<T>::mutate(&series_id, |x| {
			x.as_mut().unwrap().draft = false;
		});
	}

	fn set_listed_for_sale(id: NFTId, value: bool) -> DispatchResult {
		Data::<T>::try_mutate(id, |data| -> DispatchResult {
			let data = data.as_mut().ok_or(Error::<T>::NFTNotFound)?;
			data.listed_for_sale = value;
			Ok(())
		})?;

		Ok(())
	}

	fn is_listed_for_sale(id: NFTId) -> Option<bool> {
		let nft = Data::<T>::get(id);
		if let Some(nft) = nft {
			return Some(nft.listed_for_sale)
		}

		return None
	}

	fn set_in_transmission(id: NFTId, value: bool) -> DispatchResult {
		Data::<T>::try_mutate(id, |data| -> DispatchResult {
			let data = data.as_mut().ok_or(Error::<T>::NFTNotFound)?;
			data.in_transmission = value;
			Ok(())
		})?;

		Ok(())
	}

	fn is_in_transmission(id: NFTId) -> Option<bool> {
		let nft = Data::<T>::get(id);
		if let Some(nft) = nft {
			return Some(nft.in_transmission)
		}

		return None
	}

	fn set_converted_to_capsule(id: NFTId, value: bool) -> DispatchResult {
		Data::<T>::try_mutate(id, |d| -> DispatchResult {
			let data = d.as_mut().ok_or(Error::<T>::NFTNotFound)?;
			data.converted_to_capsule = value;
			Ok(())
		})?;

		Ok(())
	}

	fn is_converted_to_capsule(id: NFTId) -> Option<bool> {
		let nft = Data::<T>::get(id);
		if let Some(nft) = nft {
			return Some(nft.converted_to_capsule)
		}

		return None
	}

	fn set_series_completion(series_id: &NFTSeriesId, value: bool) -> DispatchResult {
		Series::<T>::try_mutate(series_id, |x| -> DispatchResult {
			let series = x.as_mut().ok_or(Error::<T>::SeriesNotFound)?;
			series.draft = !value;
			Ok(())
		})?;

		Ok(())
	}

	fn set_viewer(id: NFTId, value: Option<Self::AccountId>) -> DispatchResult {
		Data::<T>::try_mutate(id, |maybe_data| -> DispatchResult {
			let data = maybe_data.as_mut().ok_or(Error::<T>::NFTNotFound)?;
			data.viewer = value;
			Ok(())
		})?;

		Ok(())
	}
}

impl<T: Config> Pallet<T> {
	fn generate_nft_id() -> NFTId {
		let nft_id = NftIdGenerator::<T>::get();
		let next_id = nft_id
			.checked_add(1)
			.expect("If u32 is not enough we should crash for safety; qed.");
		NftIdGenerator::<T>::put(next_id);

		return nft_id
	}

	fn generate_series_id() -> NFTSeriesId {
		let mut id = SeriesIdGenerator::<T>::get();
		loop {
			let id_vec = u32_to_text(id);
			if !Series::<T>::contains_key(&id_vec) {
				break
			}
			id = id
				.checked_add(1)
				.expect("If u32 is not enough we should crash for safety; qed.");
		}
		SeriesIdGenerator::<T>::put(
			id.checked_add(1)
				.expect("If u32 is not enough we should crash for safety; qed."),
		);

		return u32_to_text(id)
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
	return num + 48
}
