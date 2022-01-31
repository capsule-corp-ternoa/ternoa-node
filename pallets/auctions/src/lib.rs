#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod default_weights;
mod types;

pub use default_weights::WeightInfo;
use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::CheckedSub;
use frame_support::traits::ExistenceRequirement::{AllowDeath, KeepAlive};
use frame_support::traits::{Currency, Get, StorageVersion};
use frame_support::PalletId;
use sp_runtime::traits::{AccountIdConversion, Saturating};
use ternoa_common::traits::{MarketplaceTrait, NFTTrait};
use ternoa_primitives::nfts::NFTId;
use types::{AuctionData, BidderList, DeadlineList};

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_support::transactional;
    use frame_system::pallet_prelude::*;
    use frame_system::{ensure_root, RawOrigin};
    use ternoa_common::traits::{MarketplaceTrait, NFTTrait};
    use ternoa_primitives::marketplace::MarketplaceId;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Caps Currency
        type Currency: Currency<Self::AccountId>;

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

        /// TODO!
        #[pallet::constant]
        type MaxAuctionBuffer: Get<Self::BlockNumber>;

        /// Grace period to extend auction by if new bid received
        #[pallet::constant]
        type AuctionGracePeriod: Get<Self::BlockNumber>;

        /// Ending period during which an auction can be extended
        #[pallet::constant]
        type AuctionEndingPeriod: Get<Self::BlockNumber>;

        /// The auctions pallet id - will be used to generate account id
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        // weight information for pallet
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Weight: see `begin_block`
        fn on_initialize(now: T::BlockNumber) -> Weight {
            let deadline = Deadlines::<T>::get();

            while let Some(nft_id) = deadline.next(now) {
                let ok = Self::complete_auction(RawOrigin::Root.into(), nft_id);
                debug_assert!(ok.is_ok());
            }

            0
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(T::WeightInfo::create_auction())]
        #[transactional]
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

            ensure!(
                start_block >= current_block,
                Error::<T>::AuctionCannotStartInThePast
            );

            ensure!(
                start_block < end_block,
                Error::<T>::AuctionCannotEndBeforeItHasStarted
            );

            let duration = end_block.saturating_sub(start_block);
            let buffer = start_block.saturating_sub(current_block);

            ensure!(
                duration <= T::MaxAuctionDuration::get(),
                Error::<T>::AuctionDurationIsTooLong
            );

            ensure!(
                duration >= T::MinAuctionDuration::get(),
                Error::<T>::AuctionDurationIsTooShort
            );

            ensure!(
                buffer <= T::MaxAuctionBuffer::get(),
                Error::<T>::AuctionIsTooFarAway
            );

            if let Some(price) = buy_it_price {
                ensure!(
                    price > start_price,
                    Error::<T>::BuyItPriceCannotBeLowerThanStartPrice
                );
            }

            // fetch the data of given nftId
            let nft_data = T::NFTHandler::get_nft(nft_id).ok_or(Error::<T>::NFTDoesNotExit)?;

            // ensure the caller is the owner of NFT
            ensure!(
                nft_data.owner == creator.clone(),
                Error::<T>::NotTheNftOwner
            );

            // ensure the nft is not listed for sale
            ensure!(
                nft_data.listed_for_sale == false,
                Error::<T>::CannotAuctionNFTsListedForSale
            );

            // ensure the nft is not in transmission
            ensure!(
                nft_data.in_transmission == false,
                Error::<T>::CannotAuctionNFTsInTransmission
            );

            // ensure the nft is not converted to capsule
            ensure!(
                nft_data.converted_to_capsule == false,
                Error::<T>::CannotAuctionCapsules
            );

            // Ensure origin is allowed to sell nft on given marketplace
            T::MarketplaceHandler::is_allowed_to_list(marketplace_id, creator.clone())?;

            // Mark NFT as listed for sale
            T::NFTHandler::set_listed_for_sale(nft_id, true)?;

            let bidders: BidderList<T::AccountId, BalanceOf<T>> = BidderList::new();
            let auction_data = AuctionData {
                creator: creator.clone(),
                start_block,
                end_block,
                start_price,
                buy_it_price,
                bidders,
                marketplace_id,
                is_extended: false,
            };

            // Add auction to storage and insert an entry to deadlines
            Auctions::<T>::insert(nft_id, auction_data);
            Deadlines::<T>::mutate(|x| x.insert(nft_id, end_block));

            // Emit AuctionCreated event
            Self::deposit_event(Event::AuctionCreated {
                nft_id,
                marketplace_id,
                creator,
                start_price,
                buy_it_price,
                start_block,
                end_block,
            });
            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::cancel_auction())]
        #[transactional]
        pub fn cancel_auction(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();

            let auction = Auctions::<T>::get(nft_id).ok_or(Error::<T>::AuctionDoesNotExist)?;

            // ensure auction creator is the caller
            ensure!(auction.creator == who, Error::<T>::NotTheAuctionCreator);

            // ensure auction has not started
            ensure!(
                !Self::has_started(current_block, auction.start_block),
                Error::<T>::CannotCancelAuctionInProgress
            );

            // List nft as not for sale
            T::NFTHandler::set_listed_for_sale(nft_id, false)?;

            Self::remove_auction(nft_id, &auction);

            // Emit auction canceled event
            Self::deposit_event(Event::AuctionCancelled { nft_id });

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::cancel_auction())]
        #[transactional]
        pub fn end_auction(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let auction = Auctions::<T>::get(nft_id).ok_or(Error::<T>::AuctionDoesNotExist)?;

            // ensure auction creator is the caller
            ensure!(auction.creator == who, Error::<T>::NotTheAuctionCreator);
            ensure!(
                auction.is_extended,
                Error::<T>::CannotEndAuctionThatWasNotExtended
            );

            Self::complete_auction(RawOrigin::Root.into(), nft_id)?;

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::add_bid())]
        #[transactional]
        pub fn add_bid(
            origin: OriginFor<T>,
            nft_id: NFTId,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();

            // add bid to storage
            Auctions::<T>::try_mutate(nft_id, |maybe_auction| -> DispatchResult {
                let auction = maybe_auction
                    .as_mut()
                    .ok_or(Error::<T>::AuctionDoesNotExist)?;

                // ensure the caller is not the owner of NFT
                ensure!(
                    auction.creator != who.clone(),
                    Error::<T>::OwnerNotAllowedToBid
                );

                // ensure the auction period has commenced
                ensure!(
                    Self::has_started(current_block, auction.start_block),
                    Error::<T>::AuctionNotStarted
                );

                // ensure the bid is larger than the current highest bid
                if let Some(highest_bid) = auction.bidders.get_highest_bid() {
                    ensure!(
                        amount > highest_bid.1,
                        Error::<T>::CannotBidLessThanTheHighestBid
                    );
                    // reject if user already has a bid
                    ensure!(
                        auction.bidders.find_bid(who.clone()).is_none(),
                        Error::<T>::UserBidAlreadyExists
                    );
                } else {
                    // ensure the bid amount is greater than start price
                    ensure!(
                        auction.start_price < amount,
                        Error::<T>::CannotBidLessThanTheHighestBid
                    );
                }

                // transfer funds from caller
                T::Currency::transfer(&who, &Self::account_id(), amount, KeepAlive)?;

                // replace top bidder with caller
                // if bidder has been removed, refund removed user
                if let Some(bid) = auction.bidders.insert_new_bid(who.clone(), amount) {
                    Self::add_claim(&bid.0, bid.1);
                }

                let grace_period = T::AuctionGracePeriod::get();
                let remaining_blocks = auction
                    .end_block
                    .checked_sub(&current_block)
                    .ok_or(Error::<T>::UnexpectedError)?;

                // extend auction by grace period if in ending period
                if remaining_blocks < grace_period {
                    let blocks_to_add = grace_period.saturating_sub(remaining_blocks);

                    auction.end_block = auction.end_block.saturating_add(blocks_to_add);
                    auction.is_extended = true;

                    // Update deadline
                    Deadlines::<T>::mutate(|x| x.update(nft_id, auction.end_block));
                }

                Ok(())
            })?;

            // emit bid created event
            Self::deposit_event(Event::BidCreated {
                nft_id,
                bidder: who,
                amount,
            });

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::remove_bid())]
        #[transactional]
        pub fn remove_bid(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();

            // remove bid from storage
            Auctions::<T>::try_mutate(nft_id, |maybe_auction| -> DispatchResult {
                // should not panic when unwrap since already checked above
                let auction = maybe_auction
                    .as_mut()
                    .ok_or(Error::<T>::AuctionDoesNotExist)?;

                let remaining_blocks = auction
                    .end_block
                    .checked_sub(&current_block)
                    .ok_or(Error::<T>::UnexpectedError)?;

                let bid = auction
                    .bidders
                    .find_bid(who.clone())
                    .ok_or(Error::<T>::BidDoesNotExist)?
                    .clone();

                // ensure the auction period has not ended
                ensure!(
                    remaining_blocks > T::AuctionEndingPeriod::get(),
                    Error::<T>::CannotRevokeBid
                );

                T::Currency::transfer(&Self::account_id(), &bid.0, bid.1, AllowDeath)?;

                auction.bidders.remove_bid(who.clone());

                // emit bid removed event
                Self::deposit_event(Event::BidRemoved {
                    nft_id,
                    bidder: who,
                    amount: bid.1,
                });

                Ok(())
            })?;

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::buy_it_now())]
        #[transactional]
        pub fn buy_it_now(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block = frame_system::Pallet::<T>::block_number();

            let auction = Auctions::<T>::get(nft_id).ok_or(Error::<T>::AuctionDoesNotExist)?;
            let amount = auction
                .buy_it_price
                .ok_or(Error::<T>::AuctionDoesNotSupportBuyItNow)?;

            // ensure the auction period has commenced
            ensure!(
                Self::has_started(current_block, auction.start_block),
                Error::<T>::AuctionNotStarted
            );

            if let Some(highest_bid) = auction.bidders.get_highest_bid() {
                ensure!(amount < highest_bid.1, Error::<T>::AuctionNotStarted);
            }

            Self::close_auction(nft_id, &auction, &who, amount)?;
            Self::remove_auction(nft_id, &auction);

            // emit bid created event
            Self::deposit_event(Event::AuctionCompleted {
                nft_id,
                new_owner: Some(who),
                amount: Some(amount),
            });

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::complete_auction())]
        #[transactional]
        pub fn complete_auction(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResultWithPostInfo {
            let _who = ensure_root(origin)?;

            let mut auction = Auctions::<T>::get(nft_id).ok_or(Error::<T>::AuctionDoesNotExist)?;

            let mut new_owner = None;
            let mut amount = None;
            // assign to highest bidder if exists
            if let Some(bidder) = auction.bidders.remove_highest_bid() {
                new_owner = Some(bidder.0.clone());
                amount = Some(bidder.1.clone());

                Self::close_auction(nft_id, &auction, &bidder.0, bidder.1)?;
            }

            Self::remove_auction(nft_id, &auction);

            // emit bid created event
            Self::deposit_event(Event::AuctionCompleted {
                nft_id,
                new_owner,
                amount,
            });

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::claim_bid())]
        #[transactional]
        pub fn claim(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let claim = Claims::<T>::get(&who).ok_or(Error::<T>::ClaimDoesNotExist)?;

            // transfer amount to user
            T::Currency::transfer(&Self::account_id(), &who, claim, AllowDeath)?;
            Claims::<T>::remove(&who);

            Self::deposit_event(Event::BalanceClaimed {
                account: who,
                amount: claim,
            });

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new auction was created
        AuctionCreated {
            nft_id: NFTId,
            marketplace_id: MarketplaceId,
            creator: T::AccountId,
            start_price: BalanceOf<T>,
            buy_it_price: Option<BalanceOf<T>>,
            start_block: T::BlockNumber,
            end_block: T::BlockNumber,
        },
        /// An existing auction was cancelled
        AuctionCancelled { nft_id: NFTId },
        /// An auction has completed and no more bids can be placed
        AuctionCompleted {
            nft_id: NFTId,
            new_owner: Option<T::AccountId>,
            amount: Option<BalanceOf<T>>,
        },
        /// A new bid was created
        BidCreated {
            nft_id: NFTId,
            bidder: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// An exising bid was removed
        BidRemoved {
            nft_id: NFTId,
            bidder: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// An exising bid was updated
        BidUpdated {
            nft_id: NFTId,
            bidder: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// Balance claimed
        BalanceClaimed {
            account: T::AccountId,
            amount: BalanceOf<T>,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Auction start block must be 'buffer' blocks away from current block
        AuctionTimelineBeforeBuffer,
        /// buy_it_price should be greater then start_price
        BuyItPriceCannotBeLowerThanStartPrice,
        /// The specified auction does not exist
        AuctionDoesNotExist,
        /// Current owner cannot bid on NFT
        OwnerNotAllowedToBid,
        /// The auction has not started
        AuctionNotStarted,
        /// The auction has already ended
        AuctionEnded,
        /// Unexpected error occured
        UnexpectedError,
        /// Auction start should be greater than end block
        AuctionCannotStartBeforeItHasEnded,
        /// Auction time should be higher than min auction duration
        AuctionTimelineLowerThanMinDuration,
        /// Auction time should be lower than max auction duration
        AuctionTimelineGreaterThanMaxDuration,
        /// The specified bid does not exist
        BidDoesNotExist,
        /// Cannot cancel an auction that has started
        CannotCancelInProcessAuction,
        /// User cannot create more than one bid
        CannotCreateABidTwice,
        /// The auction does not have a buy it now price
        AuctionDoesNotSupportBuyItNow,
        /// The auction has not yet completed
        AuctionNotCompleted,
        /// Only auction creator can cancel the auction
        OnlyAuctionCreatorCanCancel,
        /// NFT has been converted to capsule
        NFTConvertedToCapsule,
        /// The specified auction does not exist
        ClaimDoesNotExist,
        /// TODO!
        CannotRevokeBid,
        /// TODO!
        CannotEndAuctionThatWasNotExtended,
        /// TODO!
        AuctionCannotStartInThePast,
        /// TODO!
        AuctionCannotEndBeforeItHasStarted,
        /// TODO!
        AuctionDurationIsTooLong,
        /// TODO!
        AuctionDurationIsTooShort,
        /// TODO!
        AuctionIsTooFarAway,
        /// TODO!
        CannotAuctionNFTsListedForSale,
        /// TODO!
        CannotAuctionNFTsInTransmission,
        /// TODO!
        CannotAuctionCapsules,
        /// TODO!
        CannotAuctionNotOwnedNFTs,
        /// TODO!
        NFTDoesNotExit,
        /// TODO!
        NotTheNftOwner,
        /// TODO!
        NotTheAuctionCreator,
        /// TODO!
        CannotCancelAuctionInProgress,
        /// TODO
        CannotBidLessThanTheHighestBid,
        /// TODO
        CannotBidLessThanTheStartingPrice,
    }

    #[pallet::storage]
    #[pallet::getter(fn auctions)]
    pub type Auctions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        NFTId,
        AuctionData<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn deadlines)]
    pub type Deadlines<T: Config> = StorageValue<_, DeadlineList<T::BlockNumber>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn claims)]
    pub type Claims<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, OptionQuery>;
}

#[allow(dead_code)]
impl<T: Config> Pallet<T> {
    /// The account ID of the auctions pot.
    pub fn account_id() -> T::AccountId {
        T::PalletId::get().into_account()
    }

    pub fn close_auction(
        nft_id: NFTId,
        auction: &AuctionData<T::AccountId, T::BlockNumber, BalanceOf<T>>,
        new_owner: &T::AccountId,
        price: BalanceOf<T>,
    ) -> DispatchResult {
        // Handle marketplace fees
        let marketplace = T::MarketplaceHandler::get_marketplace(auction.marketplace_id)
            .ok_or(Error::<T>::UnexpectedError)?;

        let to_marketplace =
            price.saturating_mul(marketplace.commission_fee.into()) / 100u32.into();
        let to_auctioneer = price
            .checked_sub(&to_marketplace)
            .ok_or(Error::<T>::UnexpectedError)?;

        // Transfer fee to marketplace
        T::Currency::transfer(
            &Self::account_id(),
            &marketplace.owner,
            to_marketplace,
            AllowDeath,
        )?;

        // Transfer remaining to auction creator
        T::Currency::transfer(
            &Self::account_id(),
            &auction.creator,
            to_auctioneer,
            AllowDeath,
        )?;

        // transfer NFT to highest bidder
        T::NFTHandler::set_owner(nft_id, new_owner)?;

        // Mark NFT as not listed for sale
        T::NFTHandler::set_listed_for_sale(nft_id, false)?;

        Ok(())
    }

    pub fn remove_auction(
        nft_id: NFTId,
        auction: &AuctionData<T::AccountId, T::BlockNumber, BalanceOf<T>>,
    ) {
        // Remove it from deadlines
        Deadlines::<T>::mutate(|x| x.remove(nft_id));

        // If there are claims handle it
        for bidder in &auction.bidders.0 {
            Self::add_claim(&bidder.0, bidder.1);
        }

        Auctions::<T>::remove(nft_id);
    }

    pub fn add_auction(
        nft_id: NFTId,
        auction: &AuctionData<T::AccountId, T::BlockNumber, BalanceOf<T>>,
    ) {
        // Remove it from deadlines
        Deadlines::<T>::mutate(|x| x.remove(nft_id));

        // If there are claims handle it
        for bidder in &auction.bidders.0 {
            Self::add_claim(&bidder.0, bidder.1);
        }

        Auctions::<T>::remove(nft_id);
    }

    pub fn add_claim(account: &T::AccountId, amount: BalanceOf<T>) {
        Claims::<T>::mutate(account, |x| {
            if let Some(claim) = x {
                claim.saturating_add(amount);
            } else {
                *x = Some(amount);
            }
        })
    }

    pub fn has_started(now: T::BlockNumber, start_block: T::BlockNumber) -> bool {
        now >= start_block
    }

    pub fn has_ended(now: T::BlockNumber, end_block: T::BlockNumber) -> bool {
        now >= end_block
    }
}
