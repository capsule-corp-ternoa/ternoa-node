use super::mock::*;
use crate::{Data, Error};
use frame_support::{assert_noop, assert_ok, StorageMap};
use frame_system::RawOrigin;

#[test]
fn create_increment_id() {
    new_test_ext().execute_with(|| {
        assert_eq!(NFTs::total(), 0);
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_eq!(NFTs::total(), 1);
    })
}

#[test]
fn create_register_details() {
    new_test_ext().execute_with(|| {
        let mock_details = MockNFTDetails::WithU8(42);
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            mock_details.clone()
        ));
        assert_eq!(NFTs::data(0).details, mock_details);
    })
}

#[test]
fn create_register_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_eq!(NFTs::data(0).owner, ALICE);
    })
}

#[test]
fn create_is_unsealed() {
    new_test_ext().execute_with(|| {
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_eq!(NFTs::data(0).sealed, false);
    })
}

#[test]
fn mutate_update_details() {
    new_test_ext().execute_with(|| {
        let mock_details = MockNFTDetails::WithU8(42);
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_ok!(NFTs::mutate(
            RawOrigin::Signed(ALICE).into(),
            0,
            mock_details.clone(),
        ));
        assert_eq!(NFTs::data(0).details, mock_details);
    })
}

#[test]
fn mutate_not_the_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_noop!(
            NFTs::mutate(RawOrigin::Signed(BOB).into(), 0, MockNFTDetails::WithU8(42),),
            Error::<Test>::NotOwner
        );
    })
}

#[test]
fn mutate_sealed() {
    new_test_ext().execute_with(|| {
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        Data::<Test>::mutate(0, |d| d.sealed = true);
        assert_noop!(
            NFTs::mutate(
                RawOrigin::Signed(ALICE).into(),
                0,
                MockNFTDetails::WithU8(42),
            ),
            Error::<Test>::Sealed
        );
    })
}

#[test]
fn transfer_update_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_ok!(NFTs::transfer(RawOrigin::Signed(ALICE).into(), 0, BOB));
        assert_eq!(NFTs::data(0).owner, BOB);
    })
}

#[test]
fn transfer_not_the_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_noop!(
            NFTs::transfer(RawOrigin::Signed(BOB).into(), 0, BOB),
            Error::<Test>::NotOwner
        );
    })
}

#[test]
fn seal_mutate_seal_flag() {
    new_test_ext().execute_with(|| {
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_ok!(NFTs::seal(RawOrigin::Signed(ALICE).into(), 0));
        assert_eq!(NFTs::data(0).sealed, true);
    })
}

#[test]
fn seal_not_the_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_noop!(
            NFTs::seal(RawOrigin::Signed(BOB).into(), 0),
            Error::<Test>::NotOwner
        );
    })
}

#[test]
fn seal_already_sealed() {
    new_test_ext().execute_with(|| {
        assert_ok!(NFTs::create(
            RawOrigin::Signed(ALICE).into(),
            MockNFTDetails::Empty
        ));
        assert_ok!(NFTs::seal(RawOrigin::Signed(ALICE).into(), 0));
        assert_noop!(
            NFTs::seal(RawOrigin::Signed(ALICE).into(), 0),
            Error::<Test>::Sealed
        );
    })
}
