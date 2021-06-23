use super::mock::*;
use crate::tests::mock;
use crate::{Error, NFTCurrency, NFTCurrencyCombined, NFTCurrencyId, NFTsForSale};
use frame_support::instances::Instance1;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

const NFT_ID_1: u8 = 0;
const NFT_ID_2: u8 = 1;
const NFT_ID_3: u8 = 2;
const NFT_ID_4: u8 = 3;
const NFT_ID_5: u8 = 4;
const NFT_ID_6: u8 = 5;
const CAPS_ID: NFTCurrencyId = NFTCurrencyId::CAPS;
const TIIME_ID: NFTCurrencyId = NFTCurrencyId::TIIME;

#[test]
fn list_register_price() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let caps = NFTCurrency::CAPS(50);

            assert_ok!(Marketplace::list(alice, NFT_ID_1, caps));
            assert_eq!(Marketplace::nft_for_sale(0), (ALICE, caps));
        })
}

#[test]
fn buy_transfer_funds_to_owner() {
    ExtBuilder::default()
        .one_hundred_caps_for_alice_n_bob()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let caps = NFTCurrency::CAPS(50);

            assert_ok!(Marketplace::list(alice, NFT_ID_1, caps));
            assert_ok!(Marketplace::buy(bob, NFT_ID_1, CAPS_ID));

            assert_eq!(Balances::free_balance(ALICE), 150);
            assert_eq!(Balances::free_balance(BOB), 50);
        })
}

#[test]
fn buy_change_owner() {
    ExtBuilder::default()
        .one_hundred_caps_for_alice_n_bob()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let caps = NFTCurrency::CAPS(50);

            assert_ok!(Marketplace::list(alice, NFT_ID_1, caps));
            assert_ok!(Marketplace::buy(bob, NFT_ID_1, CAPS_ID));
            assert_eq!(NFTs::data(0).owner, BOB);
        })
}

#[test]
fn buy_unlock_nft() {
    ExtBuilder::default()
        .one_hundred_caps_for_alice_n_bob()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let caps = NFTCurrency::CAPS(50);

            assert_ok!(Marketplace::list(alice, NFT_ID_1, caps));
            assert_ok!(Marketplace::buy(bob, NFT_ID_1, CAPS_ID));
            assert_eq!(NFTs::data(NFT_ID_1).locked, false);
        })
}

#[test]
fn unlist_unlocks_nft() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let caps = NFTCurrency::CAPS(50);

            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_1, caps));
            assert_ok!(Marketplace::unlist(alice, NFT_ID_1));
            assert_eq!(NFTs::data(0).locked, false);
        })
}

#[test]
fn unlist_remove_from_for_sale() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let caps = NFTCurrency::CAPS(50);

            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_1, caps));
            assert_ok!(Marketplace::unlist(alice, NFT_ID_1));
            assert_eq!(NFTsForSale::<Test>::contains_key(NFT_ID_1), false);
        })
}

#[test]
fn bought_nft_is_not_listed_anymore() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .one_hundred_caps_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let seller: mock::Origin = RawOrigin::Signed(ALICE).into();
            let buyer: mock::Origin = RawOrigin::Signed(BOB).into();

            assert_ok!(Marketplace::list(seller, NFT_ID_1, NFTCurrency::CAPS(100)));
            assert_ok!(Marketplace::buy(buyer.clone(), NFT_ID_1, CAPS_ID));
            assert_noop!(
                Marketplace::buy(buyer, NFT_ID_1, CAPS_ID),
                Error::<Test>::NftNotForSale,
            );
        })
}

#[test]
fn list_nft() {
    ExtBuilder::default()
        .three_nfts_for_alice()
        .one_hundred_caps_for_alice_n_bob()
        .one_hundred_tiime_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let caps: NFTCurrency<Test> = NFTCurrency::CAPS(10);
            let alice_caller: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Alice should NOT be able to list an nft that she does not own.
            assert_noop!(
                Marketplace::list(alice_caller.clone(), 100, caps),
                Error::<Test>::NotNftOwner,
            );

            // Alice should be able list her own nft.
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_1, caps));

            // Alice should NOT be able to list her own nft again.
            assert_noop!(
                Marketplace::list(alice_caller.clone(), NFT_ID_1, caps),
                ternoa_nfts::Error::<Test>::Locked,
            );

            // Alice should be able to list her second nft with Tiime.
            let tiime = NFTCurrency::TIIME(10);
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_2, tiime));

            // Alice should be able to list her third nft with Combined currency.
            let combined = NFTCurrency::COMBINED(NFTCurrencyCombined::new(10, 10));
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_3, combined));
        })
}

#[test]
fn unlist_nft() {
    ExtBuilder::default()
        .three_nfts_for_alice()
        .one_hundred_caps_for_alice_n_bob()
        .one_hundred_tiime_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let nft_price: NFTCurrency<Test> = NFTCurrency::CAPS(10);
            let alice_caller: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob_caller: mock::Origin = RawOrigin::Signed(BOB).into();

            // Bob should NOT be able to unlist an nft that he does not own.
            assert_noop!(
                Marketplace::unlist(bob_caller.clone(), NFT_ID_1),
                Error::<Test>::NotNftOwner,
            );

            // Alice should NOT be able to unlist an nft that she owns but it's not listed.
            assert_noop!(
                Marketplace::unlist(alice_caller.clone(), NFT_ID_1),
                Error::<Test>::NftNotForSale,
            );

            // Alice should be able to unlist her own listed nft.
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_1, nft_price));
            assert_ok!(Marketplace::unlist(alice_caller.clone(), NFT_ID_1,));
        })
}

#[test]
fn buy_nft() {
    ExtBuilder::default()
        .n_nfts_for_alice(6)
        .one_hundred_caps_for_alice_n_bob()
        .one_hundred_tiime_for_alice_n_bob()
        .build()
        .execute_with(|| {
            let caps: NFTCurrency<Test> = NFTCurrency::CAPS(200);
            let tiime: NFTCurrency<Test> = NFTCurrency::TIIME(200);
            let alice_caller: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob_caller: mock::Origin = RawOrigin::Signed(BOB).into();

            // Bob should NOT be able to buy nfts that are not listed.
            assert_noop!(
                Marketplace::buy(bob_caller.clone(), NFT_ID_1, CAPS_ID),
                Error::<Test>::NftNotForSale,
            );

            // Alice should NOT be able to buy her own listed nfts.
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_1, caps));
            assert_noop!(
                Marketplace::buy(alice_caller.clone(), NFT_ID_1, CAPS_ID),
                Error::<Test>::NftAlreadyOwned,
            );

            // Bob should NOT be able to buy nfts (that are listed with caps) with tiime. And Vice versa.
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_2, tiime));
            assert_noop!(
                Marketplace::buy(bob_caller.clone(), NFT_ID_1, TIIME_ID),
                Error::<Test>::WrongCurrencyUsed,
            );
            assert_noop!(
                Marketplace::buy(bob_caller.clone(), NFT_ID_2, CAPS_ID),
                Error::<Test>::WrongCurrencyUsed,
            );

            // Bob should NOT be able to buy nfts that are too expensive.
            assert_noop!(
                Marketplace::buy(bob_caller.clone(), NFT_ID_1, CAPS_ID),
                pallet_balances::Error::<Test>::InsufficientBalance,
            );

            assert_noop!(
                Marketplace::buy(bob_caller.clone(), NFT_ID_2, TIIME_ID),
                pallet_balances::Error::<Test, Instance1>::InsufficientBalance,
            );

            // Bob should be able to buy nfts that are listed with either caps or time.
            let caps = NFTCurrency::CAPS(10);
            let time = NFTCurrency::TIIME(10);
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_3, caps));
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_4, time));
            assert_ok!(Marketplace::buy(bob_caller.clone(), NFT_ID_3, CAPS_ID));
            assert_ok!(Marketplace::buy(bob_caller.clone(), NFT_ID_4, TIIME_ID));

            // Bob should be able to buy nfts (that are listed with combined currency) with either caps or tiime.
            let combined = NFTCurrency::COMBINED(NFTCurrencyCombined::new(10, 10));
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_5, combined));
            assert_ok!(Marketplace::list(alice_caller.clone(), NFT_ID_6, combined));
            assert_ok!(Marketplace::buy(bob_caller.clone(), NFT_ID_5, CAPS_ID));
            assert_ok!(Marketplace::buy(bob_caller.clone(), NFT_ID_6, TIIME_ID));
        })
}
