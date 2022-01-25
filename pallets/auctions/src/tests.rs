#[cfg(test)]
use super::mock::*;
use crate::types::{AuctionData, AuctionState, BidderList};
use crate::{mock, Auctions as AuctionsStorage, Error};
use frame_support::error::BadOrigin;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use ternoa_common::traits::{MarketplaceTrait, NFTTrait};
use ternoa_marketplace::Error as MarketplaceError;
use ternoa_primitives::{
    marketplace::{MarketplaceId, MarketplaceType},
    nfts::NFTId,
};

pub const MIN_AUCTION_DURATION: u64 = 14400;
// min auction buffer of 1 hour (1*60*60)/6
pub const MIN_AUCTION_BUFFER: u64 = 600;
// max auction duration of 30 days (30*24*60*60)/6
pub const MAX_AUCTION_DURATION: u64 = 432000;
// auction grace period of 10min (10*60)/6
pub const AUCTION_GRACE_PERIOD: u64 = 100;

fn get_marketplace(owner: u64) -> MarketplaceId {
    let owner_signed: mock::Origin = RawOrigin::Signed(owner).into();
    help::create_mkp(
        owner_signed,
        MarketplaceType::Public,
        10, // 10% commission
        vec![1],
        vec![],
    )
}

fn create_nft(owner: u64) -> NFTId {
    let series_id = vec![50];
    <NFTs as NFTTrait>::create_nft(owner, vec![50], Some(series_id.clone())).unwrap()
}

fn create_auction(owner: mock::Origin, marketplace_id: MarketplaceId, nft_id: NFTId) {
    assert_ok!(Auctions::create_auction(
        owner,
        nft_id,
        marketplace_id,
        MIN_AUCTION_BUFFER + 1,
        MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 1,
        100,
        Some(200)
    ));
}

#[test]
fn create_auction_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            // ensure nft is marked as listed for sale
            assert_eq!(NFTs::is_listed_for_sale(nft_id), Some(true));
            // ensure storage is populated correctly
            assert_eq!(
                AuctionsStorage::<Test>::get(nft_id).unwrap(),
                AuctionData {
                    creator: ALICE,
                    start_block: MIN_AUCTION_BUFFER + 1,
                    end_block: MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 1,
                    start_price: 100,
                    buy_it_price: Some(200),
                    bidders: BidderList::new(),
                    marketplace_id: mkp_id,
                    state: AuctionState::Pending
                }
            );
        })
}

#[test]
fn create_auction_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);

            // should fail since start block > end block
            assert_noop!(
                Auctions::create_auction(
                    alice.clone(),
                    nft_id,
                    mkp_id,
                    MIN_AUCTION_BUFFER + 1,
                    6,
                    100,
                    Some(200)
                ),
                Error::<Test>::AuctionCannotStartBeforeItHasEnded
            );

            // should fail since start block < current block
            assert_noop!(
                Auctions::create_auction(alice.clone(), nft_id, mkp_id, 1, 6, 100, Some(200)),
                Error::<Test>::AuctionMustStartInFuture
            );

            // should fail since auction period greater than max auction duration
            assert_noop!(
                Auctions::create_auction(
                    alice.clone(),
                    nft_id,
                    mkp_id,
                    MIN_AUCTION_BUFFER + 1,
                    MAX_AUCTION_DURATION + MIN_AUCTION_BUFFER + 2,
                    100,
                    Some(200)
                ),
                Error::<Test>::AuctionTimelineGreaterThanMaxDuration
            );

            // should fail since auction period lesser than min auction duration
            assert_noop!(
                Auctions::create_auction(
                    alice.clone(),
                    nft_id,
                    mkp_id,
                    MIN_AUCTION_BUFFER + 1,
                    MIN_AUCTION_BUFFER + 2,
                    100,
                    Some(200)
                ),
                Error::<Test>::AuctionTimelineLowerThanMinDuration
            );

            let ideal_start_block: u64 = MIN_AUCTION_BUFFER + 1;
            let ideal_end_block: u64 = MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 1;

            // should fail since buy it price < start price
            assert_noop!(
                Auctions::create_auction(
                    alice.clone(),
                    nft_id,
                    mkp_id,
                    ideal_start_block,
                    ideal_end_block,
                    100,
                    Some(50)
                ),
                Error::<Test>::StartPriceCannotBeLowerThanBuyItPrice
            );

            // should fail since the caller is not the owner of nft
            assert_noop!(
                Auctions::create_auction(
                    bob.clone(),
                    nft_id,
                    mkp_id,
                    ideal_start_block,
                    ideal_end_block,
                    100,
                    Some(150)
                ),
                Error::<Test>::NftNotOwned
            );

            // should fail since the nft is already listed for sale
            let _ = <NFTs as NFTTrait>::set_listed_for_sale(nft_id, true);
            assert_noop!(
                Auctions::create_auction(
                    alice.clone(),
                    nft_id,
                    mkp_id,
                    ideal_start_block,
                    ideal_end_block,
                    100,
                    Some(150)
                ),
                Error::<Test>::NFTAlreadyListedForSale
            );
            let _ = <NFTs as NFTTrait>::set_listed_for_sale(nft_id, false);

            // should fail since the nft is set in transmission
            let _ = <NFTs as NFTTrait>::set_in_transmission(nft_id, true);
            assert_noop!(
                Auctions::create_auction(
                    alice.clone(),
                    nft_id,
                    mkp_id,
                    ideal_start_block,
                    ideal_end_block,
                    100,
                    Some(150)
                ),
                Error::<Test>::NFTInTransmission
            );
            let _ = <NFTs as NFTTrait>::set_in_transmission(nft_id, false);

            // should fail when nft converted to capsule
            let _ = <NFTs as NFTTrait>::set_converted_to_capsule(nft_id, true);
            assert_noop!(
                Auctions::create_auction(
                    alice.clone(),
                    nft_id,
                    mkp_id,
                    ideal_start_block,
                    ideal_end_block,
                    100,
                    Some(150)
                ),
                Error::<Test>::NFTConvertedToCapsule
            );
            let _ = <NFTs as NFTTrait>::set_converted_to_capsule(nft_id, false);

            // should fail since the caller is not permitted to list on marketplace
            let restricted_mkp_id =
                help::create_mkp(bob.clone(), MarketplaceType::Private, 0, vec![1], vec![]);
            assert_noop!(
                Auctions::create_auction(
                    alice.clone(),
                    nft_id,
                    restricted_mkp_id,
                    ideal_start_block,
                    ideal_end_block,
                    100,
                    Some(150)
                ),
                MarketplaceError::<Test>::NotAllowedToList
            );
        })
}

#[test]
fn cancel_auction_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice.clone(), mkp_id, nft_id);

            assert_ok!(Auctions::cancel_auction(alice, nft_id,));

            // ensure auction is removed from storage
            assert_eq!(AuctionsStorage::<Test>::get(nft_id), None);
        })
}

#[test]
fn cancel_auction_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);

            // should fail since the auction does not exist
            assert_noop!(
                Auctions::cancel_auction(alice.clone(), nft_id,),
                Error::<Test>::AuctionDoesNotExist
            );

            create_auction(alice.clone(), mkp_id, nft_id);

            // should fail since the caller is not creator of auction
            assert_noop!(
                Auctions::cancel_auction(bob.clone(), nft_id,),
                Error::<Test>::OnlyAuctionCreatorCanCancel
            );

            // should fail since the auction has already started
            run_to_block(MIN_AUCTION_BUFFER + 2);
            assert_noop!(
                Auctions::cancel_auction(alice, nft_id,),
                Error::<Test>::CannotCancelInProcessAuction
            );
        })
}

#[test]
fn add_bid_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let charlie: mock::Origin = RawOrigin::Signed(CHARLIE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(MIN_AUCTION_BUFFER + 2);
            assert_ok!(Auctions::add_bid(bob, nft_id, 102));
            assert_eq!(Balances::free_balance(BOB), 898);
            // the end block should not be modified
            let end_block: u64 = MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 1;
            // ensure storage is populated correctly
            assert_eq!(
                AuctionsStorage::<Test>::get(nft_id).unwrap(),
                AuctionData {
                    creator: ALICE,
                    start_block: MIN_AUCTION_BUFFER + 1,
                    end_block,
                    start_price: 100,
                    buy_it_price: Some(200),
                    bidders: BidderList([(BOB, 102)].to_vec()),
                    marketplace_id: mkp_id,
                    state: AuctionState::Pending
                }
            );
            // adding a bid in ending period should extend the auction by grace period
            run_to_block(end_block - 50);
            assert_ok!(Auctions::add_bid(charlie, nft_id, 105));
            assert_eq!(Balances::free_balance(CHARLIE), 895);
            // ensure storage is populated correctly
            assert_eq!(
                AuctionsStorage::<Test>::get(nft_id).unwrap(),
                AuctionData {
                    creator: ALICE,
                    start_block: MIN_AUCTION_BUFFER + 1,
                    end_block: end_block + 50, // the auction time should be extended
                    start_price: 100,
                    buy_it_price: Some(200),
                    bidders: BidderList([(BOB, 102), (CHARLIE, 105)].to_vec()),
                    marketplace_id: mkp_id,
                    state: AuctionState::Extended // the auction state should be extended
                }
            );
        })
}

#[test]
fn add_bid_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let charlie: mock::Origin = RawOrigin::Signed(CHARLIE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);

            // should fail since the auction does not exist
            assert_noop!(
                Auctions::add_bid(bob.clone(), nft_id, 100),
                Error::<Test>::AuctionDoesNotExist
            );

            create_auction(alice.clone(), mkp_id, nft_id);

            // should fail since the owner cannot add a bid
            assert_noop!(
                Auctions::add_bid(alice, nft_id, 100),
                Error::<Test>::OwnerCannotCreateBid
            );

            // should fail since the auction has not started
            assert_noop!(
                Auctions::add_bid(bob.clone(), nft_id, 100),
                Error::<Test>::AuctionNotStarted
            );

            run_to_block(MIN_AUCTION_BUFFER + 2);
            // should fail since the amount is not greater than start price
            assert_noop!(
                Auctions::add_bid(bob.clone(), nft_id, 100),
                Error::<Test>::InvalidBidAmount
            );

            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 101));

            // Should fail since one user cannot create more than one bid
            assert_noop!(
                Auctions::add_bid(bob.clone(), nft_id, 103),
                Error::<Test>::UserBidAlreadyExists
            );

            // Should fail since the bid is not greater than last highest bid
            assert_noop!(
                Auctions::add_bid(charlie.clone(), nft_id, 101),
                Error::<Test>::InvalidBidAmount
            );

            run_to_block(MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 2);
            assert_noop!(
                Auctions::add_bid(charlie.clone(), nft_id, 105),
                Error::<Test>::AuctionEnded
            );
        })
}

#[test]
fn add_bid_bidderlist_overflow_works() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 5000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice.clone(), mkp_id, nft_id);

            run_to_block(MIN_AUCTION_BUFFER + 2);
            // add 10 bids in a row
            for n in 2..12 {
                assert_ok!(Balances::transfer(alice.clone(), n, 200));
                let account: mock::Origin = RawOrigin::Signed(n).into();
                assert_ok!(Auctions::add_bid(account, nft_id, (100 + n).into()));
            }

            // on insertion of 11th bid, first user should be refunded
            assert_ok!(Balances::transfer(alice.clone(), 13, 200));
            let account13: mock::Origin = RawOrigin::Signed(13).into();
            assert_ok!(Auctions::add_bid(account13, nft_id, 200));
            // first user should be refunded
            assert_eq!(Balances::free_balance(2), 200);

            // on insertion of 12th bid, second user should be refunded
            assert_ok!(Balances::transfer(alice.clone(), 14, 300));
            let account14: mock::Origin = RawOrigin::Signed(14).into();
            assert_ok!(Auctions::add_bid(account14, nft_id, 201));
            // first user should be refunded
            assert_eq!(Balances::free_balance(3), 200);
        })
}

#[test]
fn remove_bid_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(MIN_AUCTION_BUFFER + 2);
            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 102));
            assert_eq!(Balances::free_balance(BOB), 898);
            assert_ok!(Auctions::remove_bid(bob, nft_id));
            assert_eq!(Balances::free_balance(BOB), 1000);
        })
}

#[test]
fn remove_bid_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);

            // should fail since the auction does not exist
            assert_noop!(
                Auctions::remove_bid(bob.clone(), nft_id),
                Error::<Test>::AuctionDoesNotExist
            );

            create_auction(alice.clone(), mkp_id, nft_id);

            run_to_block(MIN_AUCTION_BUFFER + 2);

            // should fail since the user does not have a bid
            assert_noop!(
                Auctions::remove_bid(bob.clone(), nft_id),
                Error::<Test>::BidDoesNotExist
            );
        })
}

#[test]
fn increase_bid_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(MIN_AUCTION_BUFFER + 2);
            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));
            assert_eq!(Balances::free_balance(BOB), 800);
            assert_ok!(Auctions::increase_bid(bob, nft_id, 300));
            assert_eq!(Balances::free_balance(BOB), 700);

            // ensure storage is populated correctly
            assert_eq!(
                AuctionsStorage::<Test>::get(nft_id).unwrap(),
                AuctionData {
                    creator: ALICE,
                    start_block: MIN_AUCTION_BUFFER + 1,
                    end_block: MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 1,
                    start_price: 100,
                    buy_it_price: Some(200),
                    bidders: BidderList([(BOB, 300)].to_vec()),
                    marketplace_id: mkp_id,
                    state: AuctionState::Pending
                }
            );
        })
}

#[test]
fn increase_bid_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);

            // should fail since the auction does not exist
            assert_noop!(
                Auctions::add_bid(bob.clone(), nft_id, 100),
                Error::<Test>::AuctionDoesNotExist
            );

            create_auction(alice.clone(), mkp_id, nft_id);
            run_to_block(MIN_AUCTION_BUFFER + 2);

            // should fail since the user does not have a bid
            assert_noop!(
                Auctions::increase_bid(bob.clone(), nft_id, 100),
                Error::<Test>::BidDoesNotExist
            );

            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));

            // should fail since the amount is lower than current highest bid
            assert_noop!(
                Auctions::increase_bid(bob.clone(), nft_id, 100),
                Error::<Test>::InvalidBidAmount
            );

            run_to_block(MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 2);
            // should fail since the auction has ended
            assert_noop!(
                Auctions::increase_bid(bob.clone(), nft_id, 100),
                Error::<Test>::AuctionEnded
            );
        })
}

#[test]
fn buy_it_now_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(CHARLIE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(MIN_AUCTION_BUFFER + 2);
            assert_eq!(Balances::free_balance(ALICE), 990);
            assert_ok!(Auctions::buy_it_now(bob.clone(), nft_id, 200));
            assert_eq!(Balances::free_balance(BOB), 800);
            // Bob should be the owner of nft
            assert_eq!(<NFTs as NFTTrait>::owner(nft_id), Some(BOB));
            // alice should have received the amount - commission fee (10%)
            assert_eq!(Balances::free_balance(ALICE), 990 + 200 - 20);
            // marketplace account should have received commission fee
            let marketplace_account = Marketplace::get_marketplace_info(mkp_id).unwrap();
            assert_eq!(marketplace_account.1, 10);
            assert_eq!(Balances::free_balance(marketplace_account.0), 1020);
            assert_eq!(Balances::free_balance(Auctions::account_id()), 0);

            // ensure storage is populated correctly
            assert_eq!(
                AuctionsStorage::<Test>::get(nft_id).unwrap(),
                AuctionData {
                    creator: ALICE,
                    start_block: MIN_AUCTION_BUFFER + 1,
                    end_block: MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 1,
                    start_price: 100,
                    buy_it_price: Some(200),
                    bidders: BidderList::new(),
                    marketplace_id: mkp_id,
                    state: AuctionState::Completed
                }
            );
        })
}

#[test]
fn buy_it_now_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            // should fail since the auction does not exist
            assert_noop!(
                Auctions::buy_it_now(bob.clone(), nft_id, 100),
                Error::<Test>::AuctionDoesNotExist
            );

            create_auction(alice.clone(), mkp_id, nft_id);

            // should fail since the auction has not started
            assert_noop!(
                Auctions::buy_it_now(bob.clone(), nft_id, 100),
                Error::<Test>::AuctionNotStarted
            );
        })
}

#[test]
fn complete_auction_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(CHARLIE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(MIN_AUCTION_BUFFER + 2);
            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));
            assert_eq!(Balances::free_balance(BOB), 800);
            run_to_block(MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 2);
            assert_ok!(Auctions::complete_auction(RawOrigin::Root.into(), nft_id));
            // Bob should be the owner of nft
            assert_eq!(<NFTs as NFTTrait>::owner(nft_id), Some(BOB));
            // alice should have received the amount - commission fee (10%)
            assert_eq!(Balances::free_balance(ALICE), 990 + 200 - 20);
            // marketplace account should have received commission fee
            let marketplace_account = Marketplace::get_marketplace_info(mkp_id).unwrap();
            assert_eq!(marketplace_account.1, 10);
            assert_eq!(Balances::free_balance(marketplace_account.0), 20);
            assert_eq!(Balances::free_balance(Auctions::account_id()), 0);
            // ensure storage is populated correctly
            assert_eq!(
                AuctionsStorage::<Test>::get(nft_id).unwrap(),
                AuctionData {
                    creator: ALICE,
                    start_block: MIN_AUCTION_BUFFER + 1,
                    end_block: MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 1,
                    start_price: 100,
                    buy_it_price: Some(200),
                    bidders: BidderList::new(),
                    marketplace_id: mkp_id,
                    state: AuctionState::Completed
                }
            );
        })
}

#[test]
fn complete_auction_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(MIN_AUCTION_BUFFER + 2);
            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));
            assert_eq!(Balances::free_balance(BOB), 800);

            // should fail since owner is not root
            assert_noop!(Auctions::complete_auction(bob.clone(), nft_id), BadOrigin);
        })
}

#[test]
fn claim_bid_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let charlie: mock::Origin = RawOrigin::Signed(CHARLIE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(MIN_AUCTION_BUFFER + 2);
            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));
            assert_ok!(Auctions::add_bid(charlie.clone(), nft_id, 500));
            assert_eq!(Balances::free_balance(BOB), 800);
            assert_eq!(Balances::free_balance(CHARLIE), 500);
            run_to_block(MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 2);
            assert_ok!(Auctions::complete_auction(RawOrigin::Root.into(), nft_id));
            assert_eq!(<NFTs as NFTTrait>::owner(nft_id), Some(CHARLIE));
            assert_ok!(Auctions::claim_bid(bob.clone(), nft_id));
            assert_eq!(Balances::free_balance(BOB), 1000);
            // ensure storage is populated correctly
            assert_eq!(
                AuctionsStorage::<Test>::get(nft_id).unwrap(),
                AuctionData {
                    creator: ALICE,
                    start_block: MIN_AUCTION_BUFFER + 1,
                    end_block: MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 1,
                    start_price: 100,
                    buy_it_price: Some(200),
                    bidders: BidderList::new(),
                    marketplace_id: mkp_id,
                    state: AuctionState::Completed
                }
            );
        })
}

#[test]
fn claim_bid_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000), (CHARLIE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let charlie: mock::Origin = RawOrigin::Signed(CHARLIE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            // should fail since the auction does not exist
            assert_noop!(
                Auctions::claim_bid(bob.clone(), nft_id),
                Error::<Test>::AuctionDoesNotExist
            );

            create_auction(alice.clone(), mkp_id, nft_id);
            run_to_block(MIN_AUCTION_BUFFER + 2);

            // should fail since the auction is not completed
            assert_noop!(
                Auctions::claim_bid(bob.clone(), nft_id),
                Error::<Test>::AuctionNotCompleted
            );

            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));
            assert_ok!(Auctions::add_bid(charlie.clone(), nft_id, 500));
            assert_eq!(Balances::free_balance(BOB), 800);
            assert_eq!(Balances::free_balance(CHARLIE), 500);
            run_to_block(MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 2);
            assert_ok!(Auctions::complete_auction(RawOrigin::Root.into(), nft_id));
            assert_eq!(<NFTs as NFTTrait>::owner(nft_id), Some(CHARLIE));
            assert_ok!(Auctions::claim_bid(bob.clone(), nft_id));
            assert_eq!(Balances::free_balance(BOB), 1000);

            // should fail since the user has already claimed
            assert_noop!(
                Auctions::claim_bid(bob.clone(), nft_id),
                Error::<Test>::BidDoesNotExist
            );
        })
}

#[test]
fn test_auction_workflow() {
    ExtBuilder::default()
        .caps(vec![
            (TREASURY, 10000),
            (BOB, 1000),
            (CHARLIE, 1000),
            (ALICE, 1000),
        ])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let charlie: mock::Origin = RawOrigin::Signed(CHARLIE).into();
            let treasury: mock::Origin = RawOrigin::Signed(TREASURY).into();

            let mkp_id = get_marketplace(3000);
            let nft_id = create_nft(ALICE);
            create_auction(alice.clone(), mkp_id, nft_id);
            run_to_block(MIN_AUCTION_BUFFER + 2);

            // add 10 bids in a row
            for n in 5..15 {
                assert_ok!(Balances::transfer(treasury.clone(), n, 200));
                let account: mock::Origin = RawOrigin::Signed(n).into();
                assert_ok!(Auctions::add_bid(account, nft_id, (100 + n).into()));
            }

            // on insertion of 11th bid, first user should be refunded
            assert_ok!(Balances::transfer(treasury.clone(), 16, 200));
            let account11: mock::Origin = RawOrigin::Signed(16).into();
            assert_ok!(Auctions::add_bid(account11, nft_id, 200));
            // first user should be refunded
            assert_eq!(Balances::free_balance(5), 200);

            // on insertion of 12th bid, second user should be refunded
            assert_ok!(Balances::transfer(treasury.clone(), 17, 300));
            let account12: mock::Origin = RawOrigin::Signed(17).into();
            assert_ok!(Auctions::add_bid(account12.clone(), nft_id, 201));
            // first user should be refunded
            assert_eq!(Balances::free_balance(6), 200);

            // move inside the grace period
            let end_block: u64 = MIN_AUCTION_BUFFER + MIN_AUCTION_DURATION + 1;
            run_to_block(end_block - AUCTION_GRACE_PERIOD / 2);
            // charlie creates a bid, this should extend the auction by 2 blocks
            assert_ok!(Auctions::add_bid(charlie.clone(), nft_id, 300));
            assert_eq!(Balances::free_balance(CHARLIE), 700);
            // 3rd user should be refunded
            assert_eq!(Balances::free_balance(7), 200);
            assert_eq!(
                AuctionsStorage::<Test>::get(nft_id).unwrap().end_block,
                end_block + AUCTION_GRACE_PERIOD / 2
            );
            // bob outbids by huge margin
            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 1000));
            run_to_block(end_block + AUCTION_GRACE_PERIOD / 2 + 1);
            assert_eq!(Balances::free_balance(ALICE), 990);
            assert_ok!(Auctions::complete_auction(RawOrigin::Root.into(), nft_id));
            // Bob should be the owner of nft
            assert_eq!(<NFTs as NFTTrait>::owner(nft_id), Some(BOB));
            // alice should have received the amount - commission fee (10%)
            assert_eq!(Balances::free_balance(ALICE), 990 + 1000 - 100);
            // marketplace account should have received commission fee
            let marketplace_account = Marketplace::get_marketplace_info(mkp_id).unwrap();
            assert_eq!(marketplace_account.1, 10);
            assert_eq!(Balances::free_balance(marketplace_account.0), 100);

            // all the users who did not win the auction should be able to claim the bids
            for n in 9..15 {
                let account: mock::Origin = RawOrigin::Signed(n).into();
                assert_ok!(Auctions::claim_bid(account, nft_id));
            }

            // charlie claims back bid
            assert_ok!(Auctions::claim_bid(charlie.clone(), nft_id));
            // charlie can only claim once
            assert_noop!(
                Auctions::claim_bid(charlie.clone(), nft_id),
                Error::<Test>::BidDoesNotExist
            );
            // bob won the auction so should not be able to claim
            assert_noop!(
                Auctions::claim_bid(bob.clone(), nft_id),
                Error::<Test>::BidDoesNotExist
            );
        })
}
