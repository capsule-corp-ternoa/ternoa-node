use super::mock::*;
use crate::mock;
use frame_support::error::BadOrigin;
use frame_support::instances::Instance1;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;
use ternoa_common::traits::NFTTrait;
use ternoa_marketplace::{
    Error, MarketplaceInformation, MarketplaceType, NFTCurrency, NFTCurrencyCombined,
    NFTCurrencyId, SaleInformation,
};
use ternoa_primitives::TextFormat;

const CAPS_ID: NFTCurrencyId = NFTCurrencyId::Caps;

type MPT = MarketplaceType;

#[test]
fn list_happy() {
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
            //let sale_info = SaleInformation::new(ALICE, price.clone(), 0);

            help::finish_series(alice.clone(), series_id);
            assert_ok!(Auctions::create_auction(
                alice.clone(),
                nft_id,
                1,
                6,
                10,
                100,
                Some(200)
            ));
        })
}
