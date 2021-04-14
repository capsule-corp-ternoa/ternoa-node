use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use ternoa_common::traits;

#[test]
fn set_owner() {
    new_test_ext().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty).expect("creation failed");

        assert_ok!(<NFTs as traits::NFTs>::set_owner(id, &BOB));
        assert_eq!(<NFTs as traits::NFTs>::owner(id), BOB);
    })
}

#[test]
fn seal() {
    new_test_ext().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty).expect("creation failed");

        assert_ok!(<NFTs as traits::NFTs>::seal(id));
        assert_eq!(<NFTs as traits::NFTs>::sealed(id), true);
        assert_noop!(
            <NFTs as traits::NFTs>::mutate(id, |_o, _d| { Ok(()) }),
            Error::<Test>::Sealed
        );
    })
}

#[test]
fn lock_and_unlock() {
    new_test_ext().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty).expect("creation failed");

        assert_ok!(<NFTs as traits::LockableNFTs>::lock(id));
        assert_eq!(<NFTs as traits::LockableNFTs>::locked(id), true);

        <NFTs as traits::LockableNFTs>::unlock(id);
        assert_eq!(<NFTs as traits::LockableNFTs>::locked(id), false);
    })
}

#[test]
fn lock_prevent_transfers() {
    new_test_ext().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty).expect("creation failed");

        assert_ok!(<NFTs as traits::LockableNFTs>::lock(id));
        assert_noop!(
            NFTs::transfer(RawOrigin::Signed(ALICE).into(), id, BOB),
            Error::<Test>::Locked
        );
    })
}

#[test]
fn lock_prevent_set_owner() {
    new_test_ext().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty).expect("creation failed");

        assert_ok!(<NFTs as traits::LockableNFTs>::lock(id));
        assert_noop!(
            <NFTs as traits::NFTs>::set_owner(id, &BOB),
            Error::<Test>::Locked
        );
    })
}

#[test]
fn lock_double_fail() {
    new_test_ext().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty).expect("creation failed");

        assert_ok!(<NFTs as traits::LockableNFTs>::lock(id));
        assert_noop!(
            <NFTs as traits::LockableNFTs>::lock(id),
            Error::<Test>::Locked
        );
    })
}

#[test]
fn burn_nft() {
    new_test_ext().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty).expect("creation failed");

        assert_ne!(<NFTs as traits::NFTs>::owner(id), 0);
        assert_ok!(<NFTs as traits::NFTs>::burn(id));
        assert_eq!(<NFTs as traits::NFTs>::owner(id), 0);
    })
}
