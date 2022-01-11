#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;

#[frame_support::pallet]
pub mod pallet {
    use crate::types::AuctionData;
    use frame_support::sp_runtime::traits::CheckedSub;
    use frame_support::traits::Currency;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use ternoa_common::traits::NFTTrait;
    use ternoa_primitives::{nfts::NFTId, MarketplaceId};

    pub type BalanceOf<T> =
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
    pub type Auctions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        NFTId,
        AuctionData<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        OptionQuery,
    >;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new auction was created
        AuctionCreated {
            nft_id: NFTId,
            marketplace_id: MarketplaceId,
            creator: T::AccountId,
        },
        /// An existing auction was cancelled
        AuctionCancelled {
            nft_id: NFTId,
            marketplace_id: MarketplaceId,
            creator: T::AccountId,
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
        /// NFT has not been listed for auction
        NFTNotListedForAuction,
        /// NFT is already listed for sale
        NFTAlreadyListedForSale,
        /// The given NFTID is invalid
        NFTIdInvalid,
        /// The nft is in transmission
        NFTInTransmission,
        /// The nft is not currently listed for sale
        NFTNotListedForSale,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_auction(
            origin: OriginFor<T>,
            nft_id: NFTId,
            marketplace_id: MarketplaceId,
            #[pallet::compact] start_block: T::BlockNumber,
            #[pallet::compact] end_block: T::BlockNumber,
            start_price: BalanceOf<T>,
            buy_it_price: Option<BalanceOf<T>>,
        ) -> DispatchResultWithPostInfo {
            let creator = ensure_signed(origin)?;
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

            // ensure the buy_it_price is greater than start_price
            match buy_it_price {
                Some(price) => ensure!(price > start_price, Error::<T>::AuctionPricingInvalid),
                None => (),
            }

            // fetch the data of given nftId
            let nft_data = T::NFTHandler::get_nft(nft_id).ok_or(Error::<T>::NFTIdInvalid)?;

            // ensure the caller is the owner of NFT
            ensure!(
                nft_data.owner == creator.clone(),
                Error::<T>::NftNotOwned
            );

            // ensure the nft is not listed for sale
            ensure!(
                nft_data.listed_for_sale == false,
                Error::<T>::NFTAlreadyListedForSale
            );

            // ensure the nft is not in transmission
            ensure!(
                nft_data.in_transmission == false,
                Error::<T>::NFTInTransmission
            );

            // TODO : Ensure origin is allowed to sell nft on given marketplace
            // TODO : Implement trait to accesss data from marketplace pallet

            // Mark NFT as listed for sale
            T::NFTHandler::set_listed_for_sale(nft_id, true)?;

            // Add auction to storage
            Auctions::<T>::insert(
                nft_id,
                AuctionData {
                    creator: creator.clone(),
                    start_block,
                    end_block,
                    start_price,
                    buy_it_price,
                    top_bidder: None,
                    marketplace_id,
                },
            );

            // Emit AuctionCreated event
            Self::deposit_event(Event::AuctionCreated {
                nft_id,
                marketplace_id,
                creator,
            });
            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn cancel_auction(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();
            let current_auction = Auctions::<T>::take(nft_id).unwrap(); // TODO: Handle failure and reject

            // ensure the caller is the owner of NFT
            ensure!(
                T::NFTHandler::owner(nft_id) == Some(who.clone()),
                Error::<T>::NftNotOwned
            );

            // ensure the nft is in listed for sale state
            ensure!(
                T::NFTHandler::is_listed_for_sale(nft_id) == Some(true),
                Error::<T>::NFTNotListedForSale
            );

            // ensure start block > current block ie auction already started
            // TODO : is this check necessary
            ensure!(
                current_auction.start_block > current_block,
                Error::<T>::AuctionTimelineInvalid
            );

            // TODO : Refund any reserved bids

            // List nft as not for sale
            T::NFTHandler::set_listed_for_sale(nft_id, false)?;

            // Remove auction from storage
            Auctions::<T>::remove(nft_id);

            // Emit auction canceled event
            Self::deposit_event(Event::AuctionCancelled {
                nft_id,
                marketplace_id: current_auction.marketplace_id,
                creator: who,
            });

            Ok(().into())
        }
    }
}
