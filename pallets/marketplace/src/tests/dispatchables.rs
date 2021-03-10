use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};
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
