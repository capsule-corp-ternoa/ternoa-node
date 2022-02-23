#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;
mod types;

pub use default_weights::WeightInfo;
use frame_support::dispatch::DispatchResult;
pub use pallet::*;
pub use types::*;

use frame_support::{
	traits::{
		Currency, ExistenceRequirement,
		ExistenceRequirement::{AllowDeath, KeepAlive},
		Get, StorageVersion, WithdrawReasons,
	},
	PalletId,
};
use sp_runtime::traits::AccountIdConversion;
use sp_std::vec;
use ternoa_primitives::{
	nfts::{NFTId, NFTSeriesId},
	TextFormat,
};

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{ensure, pallet_prelude::*, transactional};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::CheckedAdd;
	use sp_std::convert::TryInto;
	use ternoa_common::{helpers::check_bounds, traits::NFTTrait};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: WeightInfo;

		/// Currency used to bill minting fees
		type Currency: Currency<Self::AccountId>;

		/// TODO!
		type NFTTrait: NFTTrait<AccountId = Self::AccountId>;

		/// Min Ipfs len
		#[pallet::constant]
		type MinIpfsLen: Get<u16>;

		/// Max Uri len
		#[pallet::constant]
		type MaxIpfsLen: Get<u16>;

		/// The treasury's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			if !CapsuleMintFee::<T>::exists() {
				let fee: BalanceOf<T> = 1000000000000000000000u128.try_into().ok().unwrap();
				CapsuleMintFee::<T>::put(fee);

				return 1
			}

			0
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates an NFT and coverts it into a capsule.
		#[pallet::weight(T::WeightInfo::create())]
		#[transactional]
		pub fn create(
			origin: OriginFor<T>,
			nft_ipfs_reference: TextFormat,
			capsule_ipfs_reference: TextFormat,
			series_id: Option<NFTSeriesId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			check_bounds(
				capsule_ipfs_reference.len(),
				(T::MinIpfsLen::get(), Error::<T>::TooShortIpfsReference),
				(T::MaxIpfsLen::get(), Error::<T>::TooLongIpfsReference),
			)?;

			// Reserve funds
			let amount = CapsuleMintFee::<T>::get();
			Self::send_funds(&who, &Self::account_id(), amount, KeepAlive)?;

			// Create NFT and capsule
			let nft_id = T::NFTTrait::create_nft(who.clone(), nft_ipfs_reference, series_id)?;
			T::NFTTrait::set_converted_to_capsule(nft_id, true)?;
			Self::new_capsule(&who, nft_id, capsule_ipfs_reference.clone(), amount);

			Self::deposit_event(Event::CapsuleDeposit { balance: amount });
			let event = Event::CapsuleCreated { owner: who, nft_id, frozen_balance: amount };
			Self::deposit_event(event);

			Ok(().into())
		}

		/// Converts an existing NFT into a capsule.
		#[pallet::weight(T::WeightInfo::create_from_nft())]
		#[transactional]
		pub fn create_from_nft(
			origin: OriginFor<T>,
			nft_id: NFTId,
			ipfs_reference: TextFormat,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			check_bounds(
				ipfs_reference.len(),
				(T::MinIpfsLen::get(), Error::<T>::TooShortIpfsReference),
				(T::MaxIpfsLen::get(), Error::<T>::TooLongIpfsReference),
			)?;

			let nft = T::NFTTrait::get_nft(nft_id).ok_or(Error::<T>::UnknownNFT)?;
			ensure!(nft.owner == who, Error::<T>::NotOwner);
			ensure!(!nft.listed_for_sale, Error::<T>::ListedForSale);
			ensure!(!nft.in_transmission, Error::<T>::InTransmission);
			ensure!(!nft.converted_to_capsule, Error::<T>::CapsuleAlreadyExists);
			ensure!(nft.viewer.is_none(), Error::<T>::CannotCreateCapsuleFromLentNFTs);

			let exists = Capsules::<T>::contains_key(nft_id);
			ensure!(!exists, Error::<T>::CapsuleAlreadyExists);

			// Reserve funds
			let amount = CapsuleMintFee::<T>::get();
			Self::send_funds(&who, &Self::account_id(), amount, KeepAlive)?;

			// Create capsule
			T::NFTTrait::set_converted_to_capsule(nft_id, true)?;
			Self::new_capsule(&who, nft_id, ipfs_reference.clone(), amount);

			Self::deposit_event(Event::CapsuleDeposit { balance: amount });
			let event = Event::CapsuleCreated { owner: who, nft_id, frozen_balance: amount };
			Self::deposit_event(event);

			Ok(().into())
		}

		/// Converts a capsule into an NFT.
		#[pallet::weight(T::WeightInfo::remove())]
		#[transactional]
		pub fn remove(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut unused_funds = Default::default();

			Ledgers::<T>::mutate(&who, |x| -> DispatchResult {
				let data = x.as_mut().ok_or(Error::<T>::NotOwner)?;

				let error = Error::<T>::NotOwner;
				let index = data.iter().position(|x| x.0 == nft_id).ok_or(error)?;

				unused_funds = data[index].1;
				Self::send_funds(&Self::account_id(), &who, data[index].1, AllowDeath)?;

				data.swap_remove(index);
				if data.is_empty() {
					*x = None;
				}

				Capsules::<T>::take(nft_id).ok_or(Error::<T>::InternalError)?;

				Ok(())
			})?;

			let event = Event::CapsuleRemoved { nft_id, unfrozen_balance: unused_funds };
			Self::deposit_event(event);

			Ok(().into())
		}

		/// Adds additional funds to a capsule.
		#[pallet::weight(T::WeightInfo::add_funds())]
		#[transactional]
		pub fn add_funds(
			origin: OriginFor<T>,
			nft_id: NFTId,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Ledgers::<T>::mutate(&who, |x| -> DispatchResult {
				let data = x.as_mut().ok_or(Error::<T>::NotOwner)?;
				let error = Error::<T>::NotOwner;
				let index = data.iter().position(|x| x.0 == nft_id).ok_or(error)?;

				Self::send_funds(&who, &Self::account_id(), amount, KeepAlive)?;

				let error = Error::<T>::ArithmeticError;
				data[index].1 = data[index].1.checked_add(&amount).ok_or(error)?;

				Ok(())
			})?;

			let event = Event::CapsuleFundsAdded { nft_id, balance: amount };
			Self::deposit_event(event);

			Ok(().into())
		}

		/// Changes the capsule ipfs reference.
		#[pallet::weight(T::WeightInfo::set_ipfs_reference())]
		pub fn set_ipfs_reference(
			origin: OriginFor<T>,
			nft_id: NFTId,
			ipfs_reference: TextFormat,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			check_bounds(
				ipfs_reference.len(),
				(T::MinIpfsLen::get(), Error::<T>::TooShortIpfsReference),
				(T::MaxIpfsLen::get(), Error::<T>::TooLongIpfsReference),
			)?;

			Capsules::<T>::mutate(nft_id, |x| -> DispatchResult {
				let data = x.as_mut().ok_or(Error::<T>::UnknownNFT)?;
				ensure!(data.owner == who, Error::<T>::NotOwner);

				data.ipfs_reference = ipfs_reference.clone();
				Ok(())
			})?;

			let event = Event::CapsuleIpfsReferenceChanged { nft_id, ipfs_reference };
			Self::deposit_event(event);

			Ok(().into())
		}

		/// Sets the Capsule Mint Fee.
		#[pallet::weight(T::WeightInfo::set_capsule_mint_fee())]
		pub fn set_capsule_mint_fee(
			origin: OriginFor<T>,
			capsule_fee: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			CapsuleMintFee::<T>::put(capsule_fee);

			Self::deposit_event(Event::CapsuleMintFeeChanged { fee: capsule_fee });

			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A capsule ipfs reference was changed.
		CapsuleIpfsReferenceChanged { nft_id: NFTId, ipfs_reference: TextFormat },
		/// Additional funds were added to a capsule.
		CapsuleFundsAdded { nft_id: NFTId, balance: BalanceOf<T> },
		/// A capsule was convert into an NFT.
		CapsuleRemoved { nft_id: NFTId, unfrozen_balance: BalanceOf<T> },
		/// A capsule was created.
		CapsuleCreated { owner: T::AccountId, nft_id: NFTId, frozen_balance: BalanceOf<T> },
		/// Capsule mint fee has been changed.
		CapsuleMintFeeChanged { fee: BalanceOf<T> },
		/// Some funds have been deposited.
		CapsuleDeposit { balance: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// TODO!
		CannotCreateCapsuleFromLentNFTs,

		/// This should never happen.
		ArithmeticError,
		/// Callers is not the NFT owner.
		NotOwner,
		/// Ipfs reference is too short.
		TooShortIpfsReference,
		/// Ipfs reference is too long.
		TooLongIpfsReference,
		/// Capsule already exists.
		CapsuleAlreadyExists,
		/// This should never happen.
		InternalError,
		/// NFT is locked.
		ListedForSale,
		/// TODO!
		AlreadyACapsule,
		/// TODO!
		UnknownNFT,
		/// TODO!
		InTransmission,
	}

	/// Current capsule mint fee.
	#[pallet::storage]
	#[pallet::getter(fn capsule_mint_fee)]
	pub type CapsuleMintFee<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// List of NFTs that are capsulized.
	#[pallet::storage]
	#[pallet::getter(fn capsules)]
	pub type Capsules<T: Config> =
		StorageMap<_, Blake2_128Concat, NFTId, CapsuleData<T::AccountId>, OptionQuery>;

	/// List of accounts that hold capsulized NFTs.
	#[pallet::storage]
	#[pallet::getter(fn ledgers)]
	pub type Ledgers<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, CapsuleLedger<BalanceOf<T>>, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub capsule_mint_fee: BalanceOf<T>,
		pub capsules: Vec<(NFTId, T::AccountId, TextFormat)>,
		pub ledgers: Vec<(T::AccountId, Vec<(NFTId, BalanceOf<T>)>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				capsule_mint_fee: Default::default(),
				capsules: Default::default(),
				ledgers: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			self.capsules.clone().into_iter().for_each(|(nft_id, account, reference)| {
				Capsules::<T>::insert(nft_id, CapsuleData::new(account, reference));
			});

			self.ledgers.clone().into_iter().for_each(|(account, data)| {
				Ledgers::<T>::insert(account, data);
			});

			CapsuleMintFee::<T>::put(self.capsule_mint_fee);
		}
	}
}

impl<T: Config> Pallet<T> {
	fn new_capsule(
		owner: &T::AccountId,
		nft_id: NFTId,
		ipfs_reference: TextFormat,
		funds: BalanceOf<T>,
	) {
		let data = CapsuleData::new(owner.clone(), ipfs_reference.clone());
		Capsules::<T>::insert(nft_id, data);

		Ledgers::<T>::mutate(&owner, |x| {
			if let Some(data) = x {
				data.push((nft_id, funds));
			} else {
				*x = Some(vec![(nft_id, funds)]);
			}
		});
	}

	fn account_id() -> T::AccountId {
		T::PalletId::get().into_account()
	}

	fn send_funds(
		sender: &T::AccountId,
		receiver: &T::AccountId,
		amount: BalanceOf<T>,
		liveness: ExistenceRequirement,
	) -> DispatchResult {
		let imbalance = T::Currency::withdraw(&sender, amount, WithdrawReasons::FEE, liveness)?;
		T::Currency::resolve_creating(receiver, imbalance);

		Ok(())
	}
}
