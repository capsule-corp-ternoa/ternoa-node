use super::mock::*;
use crate::{mock, Auctions as AuctionsStorage, types::AuctionData};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;
use ternoa_common::traits::NFTTrait;
use ternoa_marketplace::{Error, MarketplaceInformation, MarketplaceType};
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
            let mkp_id = help::create_mkp(bob.clone(), MarketplaceType::Private, 0, vec![1], vec![ALICE]);

            assert_ok!(Auctions::create_auction(
                alice.clone(),
                nft_id,
                mkp_id,
                6,
                10,
                100,
                Some(200)
            ));
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
                let mkp_id = help::create_mkp(bob.clone(), MarketplaceType::Private, 0, vec![1], vec![ALICE]);

            assert_ok!(Auctions::create_auction(
                alice.clone(),
                nft_id,
                mkp_id,
                6,
                10,
                100,
                Some(200)
            ));

            // skip to block of auction start

            assert_ok!(Auctions::cancel_auction(alice.clone(), nft_id,));
        })
}
