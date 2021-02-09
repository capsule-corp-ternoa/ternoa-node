use frame_support::assert_ok;
use frame_system::RawOrigin;

use super::mock::*;

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
