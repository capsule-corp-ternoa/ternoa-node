#[cfg(test)]
use super::mock::*;
use crate::{mock, types::AuctionData, Auctions as AuctionsStorage, Error};
use frame_support::error::BadOrigin;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;
use ternoa_common::traits::NFTTrait;
use ternoa_marketplace::{Error as MarketplaceError, MarketplaceInformation, MarketplaceType};
use ternoa_primitives::{marketplace::MarketplaceId, nfts::NFTId, AccountId};

fn get_marketplace(owner: u64) -> MarketplaceId {
    let owner_signed: mock::Origin = RawOrigin::Signed(owner).into();
    help::create_mkp(
        owner_signed,
        MarketplaceType::Private,
        0,
        vec![1],
        vec![owner],
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
        6,
        17,
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
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            // ensure nft is marked as listed for sale
            assert_eq!(NFTs::is_listed_for_sale(nft_id), Some(true));
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
                Auctions::create_auction(alice.clone(), nft_id, mkp_id, 10, 6, 100, Some(200)),
                Error::<Test>::AuctionStartBlockLesserThanEndBlock
            );

            // should fail since start block > end block
            assert_noop!(
                Auctions::create_auction(alice.clone(), nft_id, mkp_id, 1, 6, 100, Some(200)),
                Error::<Test>::AuctionStartLowerThanCurrentBlock
            );

            // should fail since auction period greater than max auction duration
            assert_noop!(
                Auctions::create_auction(alice.clone(), nft_id, mkp_id, 5, 26, 100, Some(200)),
                Error::<Test>::AuctionTimelineGreaterThanMaxDuration
            );

            // should fail since auction period lesser than min auction duration
            assert_noop!(
                Auctions::create_auction(alice.clone(), nft_id, mkp_id, 5, 6, 100, Some(200)),
                Error::<Test>::AuctionTimelineLowerThanMinDuration
            );

            // should fail since buy it price < start price
            assert_noop!(
                Auctions::create_auction(alice.clone(), nft_id, mkp_id, 6, 16, 100, Some(50)),
                Error::<Test>::AuctionPricingInvalid
            );

            // should fail since the caller is not the owner of nft
            assert_noop!(
                Auctions::create_auction(bob.clone(), nft_id, mkp_id, 6, 16, 100, Some(150)),
                Error::<Test>::NftNotOwned
            );

            // should fail since the nft is already listed for sale
            let _ = <NFTs as NFTTrait>::set_listed_for_sale(nft_id, true);
            assert_noop!(
                Auctions::create_auction(alice.clone(), nft_id, mkp_id, 6, 16, 100, Some(150)),
                Error::<Test>::NFTAlreadyListedForSale
            );
            let _ = <NFTs as NFTTrait>::set_listed_for_sale(nft_id, false);

            // should fail since the nft is set in transmission
            let _ = <NFTs as NFTTrait>::set_in_transmission(nft_id, true);
            assert_noop!(
                Auctions::create_auction(alice.clone(), nft_id, mkp_id, 6, 16, 100, Some(150)),
                Error::<Test>::NFTInTransmission
            );
            let _ = <NFTs as NFTTrait>::set_in_transmission(nft_id, false);

            // should fail since the caller is not permitted to list on marketplace
            let restricted_mkp_id =
                help::create_mkp(bob.clone(), MarketplaceType::Private, 0, vec![1], vec![]);
            assert_noop!(
                Auctions::create_auction(
                    alice.clone(),
                    nft_id,
                    restricted_mkp_id,
                    6,
                    16,
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
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice.clone(), mkp_id, nft_id);

            assert_ok!(Auctions::cancel_auction(alice, nft_id,));
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

            // should fail since the nft_id does not exist
            assert_noop!(
                Auctions::cancel_auction(alice.clone(), 2021u32,),
                Error::<Test>::NFTIdInvalid
            );

            // should fail since the auction does not exist
            assert_noop!(
                Auctions::cancel_auction(alice.clone(), nft_id,),
                Error::<Test>::AuctionDoesNotExist
            );

            create_auction(alice.clone(), mkp_id, nft_id);

            // should fail since the caller is not owner of nft
            assert_noop!(
                Auctions::cancel_auction(bob.clone(), nft_id,),
                Error::<Test>::NftNotOwned
            );

            // should fail since the auction has already started
            run_to_block(7);
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

            run_to_block(7);
            assert_ok!(Auctions::add_bid(bob, nft_id, 102));
            // the end block should not be modified
            assert_eq!(AuctionsStorage::<Test>::get(nft_id).unwrap().end_block, 17);
            // adding a bid in auction period should extend the auction by grace period
            run_to_block(15);
            assert_ok!(Auctions::add_bid(charlie, nft_id, 105));
            assert_eq!(AuctionsStorage::<Test>::get(nft_id).unwrap().end_block, 19);
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

            // should fail since the nft_id does not exist
            assert_noop!(
                Auctions::add_bid(bob.clone(), 2021u32, 100),
                Error::<Test>::NFTIdInvalid
            );

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

            run_to_block(7);
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

            run_to_block(18);
            assert_noop!(
                Auctions::add_bid(charlie.clone(), nft_id, 105),
                Error::<Test>::AuctionEnded
            );

            // TODO : add a test for extreme case of add bid where a user is ejected from queue
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
            let charlie: mock::Origin = RawOrigin::Signed(CHARLIE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(7);
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
            let charlie: mock::Origin = RawOrigin::Signed(CHARLIE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);

            // should fail since the auction does not exist
            assert_noop!(
                Auctions::remove_bid(bob.clone(), nft_id),
                Error::<Test>::AuctionDoesNotExist
            );

            create_auction(alice.clone(), mkp_id, nft_id);

            run_to_block(7);

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
            let charlie: mock::Origin = RawOrigin::Signed(CHARLIE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(7);
            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));
            assert_eq!(Balances::free_balance(BOB), 800);
            assert_ok!(Auctions::increase_bid(bob, nft_id, 300));
            assert_eq!(Balances::free_balance(BOB), 700);
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
            let charlie: mock::Origin = RawOrigin::Signed(CHARLIE).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);

            // should fail since the auction does not exist
            assert_noop!(
                Auctions::add_bid(bob.clone(), nft_id, 100),
                Error::<Test>::AuctionDoesNotExist
            );

            create_auction(alice.clone(), mkp_id, nft_id);
            run_to_block(7);

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

            run_to_block(18);
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
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(7);
            assert_ok!(Auctions::buy_it_now(bob.clone(), nft_id, 200));
            assert_eq!(Balances::free_balance(BOB), 800);
            assert_eq!(<NFTs as NFTTrait>::owner(nft_id), Some(BOB));
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

            let mkp_id = get_marketplace(ALICE);
            let nft_id = create_nft(ALICE);
            create_auction(alice, mkp_id, nft_id);

            run_to_block(7);
            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));
            assert_eq!(Balances::free_balance(BOB), 800);
            assert_ok!(Auctions::complete_auction(RawOrigin::Root.into(), nft_id));
            assert_eq!(<NFTs as NFTTrait>::owner(nft_id), Some(BOB));
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

            run_to_block(7);
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

            run_to_block(7);
            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));
            assert_ok!(Auctions::add_bid(charlie.clone(), nft_id, 500));
            assert_eq!(Balances::free_balance(BOB), 800);
            assert_eq!(Balances::free_balance(CHARLIE), 500);
            assert_ok!(Auctions::complete_auction(RawOrigin::Root.into(), nft_id));
            assert_eq!(<NFTs as NFTTrait>::owner(nft_id), Some(CHARLIE));
            assert_ok!(Auctions::claim_bid(bob.clone(), nft_id));
            assert_eq!(Balances::free_balance(BOB), 1000);
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
            run_to_block(7);

            // should fail since the auction is not completed
            assert_noop!(
                Auctions::claim_bid(bob.clone(), nft_id),
                Error::<Test>::AuctionNotCompleted
            );

            assert_ok!(Auctions::add_bid(bob.clone(), nft_id, 200));
            assert_ok!(Auctions::add_bid(charlie.clone(), nft_id, 500));
            assert_eq!(Balances::free_balance(BOB), 800);
            assert_eq!(Balances::free_balance(CHARLIE), 500);
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
