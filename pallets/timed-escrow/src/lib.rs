#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use frame_support::{
    decl_error, decl_event, decl_module, ensure,
    traits::{
        schedule::{DispatchTime, Named as ScheduleNamed},
        LockIdentifier,
    },
    weights::Weight,
};
use frame_system::{ensure_root, ensure_signed, RawOrigin};
use sp_runtime::{traits::Dispatchable, traits::StaticLookup};
use ternoa_common::traits::{LockableNFTs, NFTs};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;
#[cfg(test)]
mod tests;

/// Used for derivating scheduled tasks IDs
const ESCROW_ID: LockIdentifier = *b"escrow  ";

pub trait WeightInfo {
    fn create() -> Weight;
    fn cancel() -> Weight;
    fn complete_transfer() -> Weight;
}

pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    /// Pallet managing NFTs.
    type NFTs: LockableNFTs<AccountId = Self::AccountId>
        + NFTs<AccountId = Self::AccountId, NFTId = NFTIdOf<Self>>;
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

type NFTIdOf<T> = <<T as Config>::NFTs as LockableNFTs>::NFTId;

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        NFTId = NFTIdOf<T>,
        BlockNumber = <T as frame_system::Config>::BlockNumber,
    {
        /// A transfer has been scheduled. \[capsule id, destination, block of transfer\]
        TransferScheduled(NFTId, AccountId, BlockNumber),
        /// A transfer has been canceled. \[capsule id\]
        TransferCanceled(NFTId),
        /// A transfer was executed and finalized. \[capsule id\]
        TransferCompleted(NFTId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        /// This function is reserved to the owner of a nft.
        NotNFTOwner,
        /// An unknown error happened which made the scheduling call fail.
        SchedulingFailed,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Create a timed transfer. This will lock the associated capsule until it gets
        /// transferred or canceled.
        #[weight = T::WeightInfo::create()]
        fn create(origin, nft_id: NFTIdOf<T>, to: <T::Lookup as StaticLookup>::Source, at: T::BlockNumber) {
            let who = ensure_signed(origin)?;
            ensure!(T::NFTs::owner(nft_id) == who, Error::<T>::NotNFTOwner);

            let to_unlookup = T::Lookup::lookup(to)?;
            T::NFTs::lock(nft_id)?;

            ensure!(T::Scheduler::schedule_named(
                (ESCROW_ID, nft_id).encode(),
                DispatchTime::At(at),
                None,
                // priority was chosen arbitrarily, we made sure it is lower than runtime
                // upgrades and democracy calls
                100,
                RawOrigin::Root.into(),
                Call::complete_transfer(to_unlookup.clone(), nft_id).into()
            ).is_ok(), Error::<T>::SchedulingFailed);

            Self::deposit_event(RawEvent::TransferScheduled(nft_id, to_unlookup, at));
        }

        /// Cancel a transfer that was previously created and unlocks the capsule.
        #[weight = T::WeightInfo::cancel()]
        fn cancel(origin, nft_id: NFTIdOf<T>) {
            let who = ensure_signed(origin)?;
            ensure!(T::NFTs::owner(nft_id) == who, Error::<T>::NotNFTOwner);

            ensure!(T::Scheduler::cancel_named((ESCROW_ID, nft_id).encode()).is_ok(), Error::<T>::SchedulingFailed);
            T::NFTs::unlock(nft_id);

            Self::deposit_event(RawEvent::TransferCanceled(nft_id));
        }

        /// System only. Execute a transfer, called by the scheduler.
        #[weight = T::WeightInfo::complete_transfer()]
        fn complete_transfer(origin, to: T::AccountId, nft_id: NFTIdOf<T>) {
            // We do not verify anything else as the only way for this function
            // to be called is if it was scheduled via either root action (trusted)
            // or the call to `create` which will verify NFT ownership and locking
            // status.
            ensure_root(origin)?;
            T::NFTs::unlock(nft_id);
            T::NFTs::set_owner(nft_id, &to)?;

            Self::deposit_event(RawEvent::TransferCompleted(nft_id));
        }
    }
}
