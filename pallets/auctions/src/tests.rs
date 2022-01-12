use super::mock::*;
use crate::{mock, types::AuctionData, Auctions as AuctionsStorage, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;
use ternoa_common::traits::NFTTrait;
use ternoa_marketplace::{Error as MarketplaceError, MarketplaceInformation, MarketplaceType};
use ternoa_primitives::TextFormat;

#[test]
fn test_create_auction_works() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            // Happy path Public marketplace
            //let price = NFTCurrency::Caps(50);
            let series_id = vec![50];
            let nft_id =
                <NFTs as NFTTrait>::create_nft(ALICE, vec![50], Some(series_id.clone())).unwrap();
            let mkp_id = help::create_mkp(
                bob.clone(),
                MarketplaceType::Private,
                0,
                vec![1],
                vec![ALICE],
            );

            assert_ok!(Auctions::create_auction(
                alice.clone(),
                nft_id,
                mkp_id,
                6,
                17,
                100,
                Some(200)
            ));
        })
}

#[test]
fn test_create_auction_fails_if_timeline_invalid() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            let series_id = vec![50];
            let nft_id =
                <NFTs as NFTTrait>::create_nft(ALICE, vec![50], Some(series_id.clone())).unwrap();
            let mkp_id = help::create_mkp(
                bob.clone(),
                MarketplaceType::Private,
                0,
                vec![1],
                vec![ALICE],
            );

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
        })
}

#[test]
fn test_cancel_auction_works() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            // Happy path Public marketplace
            //let price = NFTCurrency::Caps(50);
            let series_id = vec![50];
            let nft_id =
                <NFTs as NFTTrait>::create_nft(ALICE, vec![50], Some(series_id.clone())).unwrap();
            let mkp_id = help::create_mkp(
                bob.clone(),
                MarketplaceType::Private,
                0,
                vec![1],
                vec![ALICE],
            );

            assert_ok!(Auctions::create_auction(
                alice.clone(),
                nft_id,
                mkp_id,
                6,
                17,
                100,
                Some(200)
            ));

            // skip to block of auction start

            assert_ok!(Auctions::cancel_auction(alice.clone(), nft_id,));
        })
}
