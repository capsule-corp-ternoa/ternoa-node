use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};
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
