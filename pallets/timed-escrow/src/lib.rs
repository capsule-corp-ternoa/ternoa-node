/* #![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;
#[cfg(test)]
mod tests;

pub use pallet::*;

use frame_support::traits::LockIdentifier;
use frame_support::weights::Weight;
use ternoa_primitives::nfts::NFTId;

/// Used for derivating scheduled tasks IDs
const ESCROW_ID: LockIdentifier = *b"escrow  ";

pub trait WeightInfo {
	fn create() -> Weight;
	fn cancel() -> Weight;
	fn complete_transfer() -> Weight;
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::schedule::{DispatchTime, Named as ScheduleNamed};
	use frame_system::pallet_prelude::*;
	use frame_system::RawOrigin;
	use sp_runtime::traits::{Dispatchable, StaticLookup};
	use ternoa_common::traits::NFTTrait;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Pallet managing NFTs.
		type NFTs: NFTTrait<AccountId = Self::AccountId>;
		/// Scheduler instance which we use to schedule actual transfer calls. This way, we have
		/// all scheduled calls accross all pallets in one place.
		type Scheduler: ScheduleNamed<Self::BlockNumber, Self::PalletsCall, Self::PalletsOrigin>;
		/// Overarching type of all pallets origins. Used with the scheduler.
		type PalletsOrigin: From<RawOrigin<Self::AccountId>>;
		/// Overarching type of all pallets calls. Used by the scheduler.
		type PalletsCall: Dispatchable<Origin = Self::Origin> + From<Call<Self>>;
		/// Weight values for this pallet
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a timed transfer. This will lock the associated capsule until it gets
		/// transferred or canceled.
		#[pallet::weight(T::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			nft_id: NFTId,
			to: <T::Lookup as StaticLookup>::Source,
			at: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let to_unlookup = T::Lookup::lookup(to)?;

			let nft = T::NFTs::get_nft(nft_id).ok_or(Error::<T>::UnknownNFT)?;
			ensure!(nft.owner == who, Error::<T>::NotNFTOwner);
			ensure!(!nft.listed_for_sale, Error::<T>::ListedForSale);
			ensure!(!nft.in_transmission, Error::<T>::AlreadyInTransmission);

			T::NFTs::set_in_transmission(nft_id, true)?;

			ensure!(
				T::Scheduler::schedule_named(
					(ESCROW_ID, nft_id).encode(),
					DispatchTime::At(at),
					None,
					// priority was chosen arbitrarily, we made sure it is lower than runtime
					// upgrades and democracy calls
					100,
					RawOrigin::Root.into(),
					Call::complete_transfer {
						to: to_unlookup.clone(),
						nft_id: nft_id
					}
					.into()
				)
				.is_ok(),
				Error::<T>::SchedulingFailed
			);

			Self::deposit_event(Event::TransferScheduled {
				nft_id,
				destination: to_unlookup,
				block_number: at,
			});

			Ok(().into())
		}

		/// Cancel a transfer that was previously created and unlocks the capsule.
		#[pallet::weight(T::WeightInfo::cancel())]
		pub fn cancel(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let nft = T::NFTs::get_nft(nft_id).ok_or(Error::<T>::UnknownNFT)?;
			ensure!(nft.owner == who, Error::<T>::NotNFTOwner);

			let ok = T::Scheduler::cancel_named((ESCROW_ID, nft_id).encode()).is_ok();
			ensure!(ok, Error::<T>::SchedulingFailed);

			T::NFTs::set_in_transmission(nft_id, false)?;

			Self::deposit_event(Event::TransferCanceled { nft_id });

			Ok(().into())
		}

		/// System only. Execute a transfer, called by the scheduler.
		#[pallet::weight(T::WeightInfo::complete_transfer())]
		pub fn complete_transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			nft_id: NFTId,
		) -> DispatchResultWithPostInfo {
			// We do not verify anything else as the only way for this function
			// to be called is if it was scheduled via either root action (trusted)
			// or the call to `create` which will verify NFT ownership and locking
			// status.
			ensure_root(origin)?;

			T::NFTs::set_in_transmission(nft_id, false)?;
			T::NFTs::set_owner(nft_id, &to)?;

			Self::deposit_event(Event::TransferCompleted { nft_id });

			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A transfer has been scheduled.
		TransferScheduled {
			nft_id: NFTId,
			destination: T::AccountId,
			block_number: T::BlockNumber,
		},
		/// A transfer has been canceled.
		TransferCanceled { nft_id: NFTId },
		/// A transfer was executed and finalized.
		TransferCompleted { nft_id: NFTId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// This function is reserved to the owner of a nft.
		NotNFTOwner,
		/// An unknown error happened which made the scheduling call fail.
		SchedulingFailed,
		/// Unknown NFT
		UnknownNFT,
		/// TODO!
		ListedForSale,
		/// TODO!
		AlreadyInTransmission,
	}
}
 */
