#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::sp_runtime::traits::CheckedSub;
    use frame_support::traits::Currency;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use ternoa_common::traits::NFTTrait;
    use ternoa_primitives::nfts::{NFTData, NFTId, NFTSeriesDetails, NFTSeriesId};

    pub type BalanceCaps<T> =
        <<T as Config>::CurrencyCaps as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Caps Currency
        type CurrencyCaps: Currency<Self::AccountId>;
        /// Get information on nfts
        type NFTHandler: NFTTrait<AccountId = Self::AccountId>;
        /// Minimum required length of auction
        #[pallet::constant]
        type MinAuctionDuration: Get<Self::BlockNumber>;
        /// Maximum permitted length of auction
        #[pallet::constant]
        type MaxAuctionDuration: Get<Self::BlockNumber>;
        /// Minimum buffer of blocks required before auction start
        #[pallet::constant]
        type MinAuctionBuffer: Get<Self::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://docs.substrate.io/v3/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        AuctionCreated {
            creator: T::AccountId,
            nft_id: NFTId,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// End block must be greater than start block for an auction
        AuctionTimelineInvalid,
        /// Only owner of NFT can list for auction
        NftNotOwned,
        /// buy_it_price should be greater then start_price
        AuctionPricingInvalid,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_auction(
            origin: OriginFor<T>,
            nft_id: NFTId,
            marketplace_id: u32, // TODO : move to commons and import marketplaceid here
            #[pallet::compact] start_block: T::BlockNumber,
            #[pallet::compact] end_block: T::BlockNumber,
            start_price: BalanceCaps<T>,
            buy_it_price: Option<BalanceCaps<T>>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();
            // ensure the auction timeline is valid
            ensure!(start_block < end_block, Error::<T>::AuctionTimelineInvalid);
            // ensure start block > current block
            ensure!(
                start_block > current_block,
                Error::<T>::AuctionTimelineInvalid
            );
            // ensure maximum auction duration is not exceeded
            ensure!(
                end_block.checked_sub(&start_block) <= Some(T::MaxAuctionDuration::get()),
                Error::<T>::AuctionTimelineInvalid
            );
            // ensure minimum auction duration is valid
            ensure!(
                end_block.checked_sub(&start_block) >= Some(T::MinAuctionDuration::get()),
                Error::<T>::AuctionTimelineInvalid
            );
            // ensure buffer period is valid
            ensure!(
                start_block.checked_sub(&current_block) >= Some(T::MinAuctionBuffer::get()),
                Error::<T>::AuctionTimelineInvalid
            );
            // ensure the caller is the owner of NFT
            ensure!(
                T::NFTHandler::owner(nft_id) == Some(who.clone()),
                Error::<T>::NftNotOwned
            );
            // ensure the caller is the owner of NFT
            match buy_it_price {
                Some(price) => ensure!(
                    buy_it_price > Some(start_price),
                    Error::<T>::AuctionPricingInvalid
                ),
                None => (),
            }

            // Emit an event.
            Self::deposit_event(Event::AuctionCreated {
                creator: who,
                nft_id,
            });
            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }
    }
}
