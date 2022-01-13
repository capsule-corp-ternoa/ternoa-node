#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;
use frame_support::traits::{Currency, Get, StorageVersion};
use frame_support::PalletId;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::traits::Saturating;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
    use crate::types::AuctionData;
    use crate::*;
    use frame_support::sp_runtime::traits::{CheckedAdd, CheckedSub};
    use frame_support::traits::ExistenceRequirement::KeepAlive;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use ternoa_common::traits::{MarketplaceTrait, NFTTrait};
    use ternoa_primitives::{marketplace::MarketplaceId, nfts::NFTId};

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
        /// Get information on marketplace
        type MarketplaceHandler: MarketplaceTrait<Self::AccountId>;
        /// Minimum required length of auction
        #[pallet::constant]
        type MinAuctionDuration: Get<Self::BlockNumber>;
        /// Maximum permitted length of auction
        #[pallet::constant]
        type MaxAuctionDuration: Get<Self::BlockNumber>;
        /// Minimum buffer of blocks required before auction start
        #[pallet::constant]
        type MinAuctionBuffer: Get<Self::BlockNumber>;
        /// Grace period to extend auction by if new bid received
        #[pallet::constant]
        type AuctionGracePeriod: Get<Self::BlockNumber>;
        /// Ending period during which an auction can be extended
        #[pallet::constant]
        type AuctionEndingPeriod: Get<Self::BlockNumber>;
        /// The auctions pallet id - will be used to generate account id
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://docs.substrate.io/v3/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn auctions)]
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
        /// A new bid was created
        BidCreated {
            nft_id: NFTId,
            marketplace_id: MarketplaceId,
            creator: T::AccountId,
        },
        /// An exising bid was removed
        BidRemoved {
            nft_id: NFTId,
            marketplace_id: MarketplaceId,
            creator: T::AccountId,
        },
        /// An exising bid was updated
        BidUpdated {
            nft_id: NFTId,
            marketplace_id: MarketplaceId,
            creator: T::AccountId,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Auction start block must be 'buffer' blocks away from current block
        AuctionTimelineBeforeBuffer,
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
        /// The specified auction does not exist
        AuctionDoesNotExist,
        /// Current owner cannot bid on NFT
        OwnerCannotCreateBid,
        /// The auction has not started
        AuctionNotStarted,
        /// The auction has already ended
        AuctionEnded,
        /// The bid amount is lower than current highest bid
        InvalidBidAmount,
        /// Unexpected error occured
        UnexpectedError,
        /// Auction start should be greater than end block
        AuctionStartBlockLesserThanEndBlock,
        /// Auction start block must be higher than current block
        AuctionStartLowerThanCurrentBlock,
        /// Auction time should be higher than min auction duration
        AuctionTimelineLowerThanMinDuration,
        /// Auction time should be lower than max auction duration
        AuctionTimelineGreaterThanMaxDuration,
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
            ensure!(
                start_block < end_block,
                Error::<T>::AuctionStartBlockLesserThanEndBlock
            );
            // ensure start block > current block
            ensure!(
                start_block > current_block,
                Error::<T>::AuctionStartLowerThanCurrentBlock
            );
            // ensure maximum auction duration is not exceeded
            ensure!(
                end_block.checked_sub(&start_block) <= Some(T::MaxAuctionDuration::get()),
                Error::<T>::AuctionTimelineGreaterThanMaxDuration
            );
            // ensure minimum auction duration is valid
            ensure!(
                end_block.checked_sub(&start_block) >= Some(T::MinAuctionDuration::get()),
                Error::<T>::AuctionTimelineLowerThanMinDuration
            );
            // ensure buffer period is valid
            ensure!(
                start_block.checked_sub(&current_block) >= Some(T::MinAuctionBuffer::get()),
                Error::<T>::AuctionTimelineBeforeBuffer
            );

            // ensure the buy_it_price is greater than start_price
            match buy_it_price {
                Some(price) => ensure!(price > start_price, Error::<T>::AuctionPricingInvalid),
                None => (),
            }

            // fetch the data of given nftId
            let nft_data = T::NFTHandler::get_nft(nft_id).ok_or(Error::<T>::NFTIdInvalid)?;

            // ensure the caller is the owner of NFT
            ensure!(nft_data.owner == creator.clone(), Error::<T>::NftNotOwned);

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

            // Ensure origin is allowed to sell nft on given marketplace
            T::MarketplaceHandler::is_allowed_to_list_on_marketplace(
                marketplace_id,
                creator.clone(),
            )?;

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

            // fetch the data of given nftId
            let nft_data = T::NFTHandler::get_nft(nft_id).ok_or(Error::<T>::NFTIdInvalid)?;

            // ensure the caller is the owner of NFT
            ensure!(nft_data.owner == who.clone(), Error::<T>::NftNotOwned);

            let current_auction =
                Auctions::<T>::get(nft_id).ok_or(Error::<T>::AuctionDoesNotExist)?;

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

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn add_bid(
            origin: OriginFor<T>,
            nft_id: NFTId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();

            // fetch the data of given nftId
            let nft_data = T::NFTHandler::get_nft(nft_id).ok_or(Error::<T>::NFTIdInvalid)?;

            // fetch data of auction that lists NFT
            let current_auction =
                Auctions::<T>::get(nft_id).ok_or(Error::<T>::AuctionDoesNotExist)?;

            // ensure the nft is in listed for sale state
            ensure!(
                nft_data.listed_for_sale == false,
                Error::<T>::NFTNotListedForSale
            );

            // ensure the caller is not the owner of NFT
            ensure!(
                nft_data.owner != who.clone(),
                Error::<T>::OwnerCannotCreateBid
            );

            // ensure the auction period has commenced
            ensure!(
                current_auction.start_block < current_block,
                Error::<T>::AuctionNotStarted
            );

            // ensure the auction period has not ended
            ensure!(
                current_auction.end_block > current_block,
                Error::<T>::AuctionEnded
            );

            // ensure the bid is larger than the current highest bid
            if let Some(current_highest_bid) = current_auction.top_bidder {
                ensure!(amount > current_highest_bid.1, Error::<T>::InvalidBidAmount);
            }

            // transfer funds from caller
            T::CurrencyCaps::transfer(&who, &Self::account_id(), amount, KeepAlive)?;

            // TODO : Return previous top bidder amount??

            // add bid as top bidder in storage
            Auctions::<T>::try_mutate(nft_id, |maybe_auction| -> DispatchResult {
                // should not panic when unwrap since already checked above
                let auction = maybe_auction.as_mut().unwrap();
                // replace top bidder with caller
                auction.top_bidder = Some((who.clone(), amount));
                // extend auction by grace period
                // TODO : Can lead to infinite auction??
                if auction.end_block.checked_sub(&current_block)
                    < Some(T::AuctionGracePeriod::get())
                {
                    auction.end_block = auction
                        .end_block
                        .checked_add(&T::AuctionGracePeriod::get())
                        .ok_or(Error::<T>::UnexpectedError)?;
                }
                Ok(())
            })?;

            // emit bid created event
            Self::deposit_event(Event::BidCreated {
                nft_id,
                marketplace_id: current_auction.marketplace_id,
                creator: who,
            });

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn remove_bid(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();

            // fetch the data of given nftId
            let nft_data = T::NFTHandler::get_nft(nft_id).ok_or(Error::<T>::NFTIdInvalid)?;

            // fetch data of auction that lists NFT
            let current_auction =
                Auctions::<T>::get(nft_id).ok_or(Error::<T>::AuctionDoesNotExist)?;

            // ensure the auction period has not ended
            ensure!(
                current_auction.end_block > current_block,
                Error::<T>::AuctionEnded
            );

            // transfer funds to caller
            // TODO : is it better to unreserve here to save gas??
            // TODO : replace amount by finding bid from bids map
            T::CurrencyCaps::transfer(
                &Self::account_id(),
                &who,
                current_auction.top_bidder.unwrap().1,
                KeepAlive,
            )?;

            // TODO : Return previous top bidder amount??

            // remove bidder as top bidder
            // TODO : possible attack to create bid increase time and then remove bid??
            // TODO : how to replace top bidder?

            // emit bid created event
            Self::deposit_event(Event::BidRemoved {
                nft_id,
                marketplace_id: current_auction.marketplace_id,
                creator: who,
            });

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn increase_bid(origin: OriginFor<T>, nft_id: NFTId, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();

            // fetch the data of given nftId
            let nft_data = T::NFTHandler::get_nft(nft_id).ok_or(Error::<T>::NFTIdInvalid)?;

            // fetch data of auction that lists NFT
            let current_auction =
                Auctions::<T>::get(nft_id).ok_or(Error::<T>::AuctionDoesNotExist)?;

            // ensure the auction period has not ended
            ensure!(
                current_auction.end_block > current_block,
                Error::<T>::AuctionEnded
            );

            // ensure the bid is larger than the current highest bid
            if let Some(current_highest_bid) = current_auction.top_bidder {
                ensure!(amount > current_highest_bid.1, Error::<T>::InvalidBidAmount);
            }

            // transfer funds from caller (subtracting amount from previous bid)
            // TODO : fetch the users current bid and subtract the amount to get amount to transfer;
            let amount_to_transfer = amount;
            T::CurrencyCaps::transfer(&who, &Self::account_id(), amount, KeepAlive)?;


            // emit bid created event
            Self::deposit_event(Event::BidUpdated {
                nft_id,
                marketplace_id: current_auction.marketplace_id,
                creator: who,
            });

            Ok(().into())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn buy_it_now(origin: OriginFor<T>, nft_id: NFTId, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();

            // fetch the data of given nftId
            let nft_data = T::NFTHandler::get_nft(nft_id).ok_or(Error::<T>::NFTIdInvalid)?;

            // fetch data of auction that lists NFT
            let current_auction =
                Auctions::<T>::get(nft_id).ok_or(Error::<T>::AuctionDoesNotExist)?;

            // ensure the auction period has commenced
            ensure!(
                current_auction.start_block < current_block,
                Error::<T>::AuctionNotStarted
            );

            // ensure the auction period has not ended
            ensure!(
                current_auction.end_block > current_block,
                Error::<T>::AuctionEnded
            );

            // ensure the bid is larger than the current highest bid
            if let Some(current_highest_bid) = current_auction.top_bidder {
                ensure!(amount > current_highest_bid.1, Error::<T>::InvalidBidAmount);
            }

            // transfer funds from caller (subtracting amount from previous bid)
            // TODO : fetch the users current bid and subtract the amount to get amount to transfer;
            let amount_to_transfer = amount;
            T::CurrencyCaps::transfer(&who, &Self::account_id(), amount, KeepAlive)?;


            // emit bid created event
            Self::deposit_event(Event::BidUpdated {
                nft_id,
                marketplace_id: current_auction.marketplace_id,
                creator: who,
            });

            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// The account ID of the auctions pot.
    pub fn account_id() -> T::AccountId {
        T::PalletId::get().into_account()
    }

    /// Return the pot account and amount of money in the pot.
    /// The existential deposit is not part of the pot so auctions account never gets deleted.
    fn pot() -> (T::AccountId, BalanceOf<T>) {
        let account_id = Self::account_id();
        let balance = T::CurrencyCaps::free_balance(&account_id)
            .saturating_sub(T::CurrencyCaps::minimum_balance());

        (account_id, balance)
    }
}
