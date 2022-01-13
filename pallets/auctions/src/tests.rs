use super::mock::*;
use crate::{mock, types::AuctionData, Auctions as AuctionsStorage, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;
use ternoa_common::traits::NFTTrait;
use ternoa_marketplace::{Error as MarketplaceError, MarketplaceInformation, MarketplaceType};
use ternoa_primitives::TextFormat;

#[test]
fn create_auction_happy() {
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
fn create_auction_unhappy() {
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

            assert_ok!(Auctions::cancel_auction(alice.clone(), nft_id,));
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

            assert_ok!(Auctions::create_auction(
                alice.clone(),
                nft_id,
                mkp_id,
                6,
                17,
                100,
                Some(200)
            ));

            // should fail since the caller is not owner of nft
            assert_noop!(
                Auctions::cancel_auction(bob.clone(), nft_id,),
                Error::<Test>::NftNotOwned
            );
        })
}
