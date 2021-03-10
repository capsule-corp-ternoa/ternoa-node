use super::mock::*;
use crate::{Error, NFTsForSale};
use frame_support::{assert_noop, assert_ok, StorageMap};
use frame_system::RawOrigin;

#[test]
fn cannot_list_nft_if_not_owner() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_noop!(
                Marketplace::list(RawOrigin::Signed(BOB).into(), 0, 1),
                Error::<Test>::NotNftOwner
            );
        })
}

#[test]
fn cannot_list_the_same_nft_twice() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_ok!(Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 1));
            assert_noop!(
                Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 1),
                ternoa_nfts::Error::<Test>::Locked
            );
        })
}

#[test]
fn list_register_price() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_ok!(Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 1));
            assert_eq!(Marketplace::nft_for_sale(0), (ALICE, 1));
        })
}

#[test]
fn cannot_buy_if_not_for_sale() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Marketplace::buy(RawOrigin::Signed(ALICE).into(), 0),
            Error::<Test>::NftNotForSale
        );
    })
}

#[test]
fn cannot_buy_if_not_enough_money() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_ok!(Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 1));
            assert_noop!(
                Marketplace::buy(RawOrigin::Signed(BOB).into(), 0),
                pallet_balances::Error::<Test, _>::InsufficientBalance
            );
        })
}

#[test]
fn buy_transfer_funds_to_owner() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_ok!(Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 50));
            assert_ok!(Marketplace::buy(RawOrigin::Signed(BOB).into(), 0));

            assert_eq!(Balances::free_balance(ALICE), 150);
            assert_eq!(Balances::free_balance(BOB), 50);
        })
}

#[test]
fn buy_change_owner() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_ok!(Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 50));
            assert_ok!(Marketplace::buy(RawOrigin::Signed(BOB).into(), 0));

            assert_eq!(NFTs::data(0).owner, BOB);
        })
}

#[test]
fn buy_unlock_nft() {
    ExtBuilder::default()
        .one_hundred_for_alice_n_bob()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_ok!(Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 50));
            assert_ok!(Marketplace::buy(RawOrigin::Signed(BOB).into(), 0));

            assert_eq!(NFTs::data(0).locked, false);
        })
}

#[test]
fn cannot_unlist_if_not_listed() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_noop!(
                Marketplace::unlist(RawOrigin::Signed(ALICE).into(), 0),
                Error::<Test>::NftNotForSale
            );
        })
}

#[test]
fn cannot_unlist_if_not_owner() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_ok!(Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 50));
            assert_noop!(
                Marketplace::unlist(RawOrigin::Signed(BOB).into(), 0),
                Error::<Test>::NotNftOwner
            );
        })
}

#[test]
fn unlist_unlocks_nft() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_ok!(Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 50));
            assert_ok!(Marketplace::unlist(RawOrigin::Signed(ALICE).into(), 0));

            assert_eq!(NFTs::data(0).locked, false);
        })
}

#[test]
fn unlist_remove_from_for_sale() {
    ExtBuilder::default()
        .one_nft_for_alice()
        .build()
        .execute_with(|| {
            assert_ok!(Marketplace::list(RawOrigin::Signed(ALICE).into(), 0, 50));
            assert_ok!(Marketplace::unlist(RawOrigin::Signed(ALICE).into(), 0));

            assert_eq!(NFTsForSale::<Test>::contains_key(0), false);
        })
}
