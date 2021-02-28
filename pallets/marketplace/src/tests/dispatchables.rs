use super::mock::*;
use frame_support::{assert_ok};
use frame_system::RawOrigin;
use ternoa_common::traits;

#[test]
fn list_nft() {
    new_test_ext().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty);
        assert_ok!(Marketplace::list(
            RawOrigin::Signed(ALICE).into(),
            id,
            10
        ));
        assert_eq!(Marketplace::NFTsForSale, 1);
    })
}
