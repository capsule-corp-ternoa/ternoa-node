use super::mock::*;
use crate::{Error, NFTSeriesDetails};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use ternoa_common::traits;

#[test]
fn set_owner() {
    new_test_ext().execute_with(|| {
        let id = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, Default::default())
            .expect("creation failed");

        assert_ok!(<NFTs as traits::NFTs>::set_owner(id, &BOB));
        assert_eq!(<NFTs as traits::NFTs>::owner(id), BOB);
    })
}

#[test]
fn seal() {
    new_test_ext().execute_with(|| {
        let id = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, Default::default())
            .expect("creation failed");

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
        let id = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, Default::default())
            .expect("creation failed");

        assert_ok!(<NFTs as traits::LockableNFTs>::lock(id));
        assert_eq!(<NFTs as traits::LockableNFTs>::locked(id), true);

        <NFTs as traits::LockableNFTs>::unlock(id);
        assert_eq!(<NFTs as traits::LockableNFTs>::locked(id), false);
    })
}

#[test]
fn lock_prevent_transfers() {
    new_test_ext().execute_with(|| {
        let id = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, Default::default())
            .expect("creation failed");

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
        let id = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, Default::default())
            .expect("creation failed");

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
        let id = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, Default::default())
            .expect("creation failed");

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
        let id = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, Default::default())
            .expect("creation failed");

        assert_ne!(<NFTs as traits::NFTs>::owner(id), 0);
        assert_ok!(<NFTs as traits::NFTs>::burn(id));
        assert_eq!(<NFTs as traits::NFTs>::owner(id), 0);
    })
}

#[test]
fn add_nft_to_a_series() {
    new_test_ext().execute_with(|| {
        let series_id = <NFTs as traits::NFTs>::NFTSeriesId::from(1u32);
        let nft_id = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, series_id)
            .expect("creation failed");

        assert_eq!(<NFTs as traits::NFTs>::series_id(nft_id), series_id);
        assert_eq!(<NFTs as traits::NFTs>::series_length(series_id), 1);
    })
}

#[test]
fn nfts_with_default_series_id_are_part_of_the_default_series() {
    new_test_ext().execute_with(|| {
        let series_id = <NFTs as traits::NFTs>::NFTSeriesId::default();
        let id = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, series_id)
            .expect("creation failed");

        assert_eq!(<NFTs as traits::NFTs>::series_id(id), series_id);
    })
}

#[test]
fn adding_nfts_to_a_series_increases_series_length() {
    new_test_ext().execute_with(|| {
        let series_id = <NFTs as traits::NFTs>::NFTSeriesId::from(1u32);
        let count = 3;
        for _ in 0..count {
            let _ = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, series_id)
                .expect("creation failed");
        }
        assert_eq!(<NFTs as traits::NFTs>::series_length(series_id), count);
    })
}

#[test]
fn adding_an_nft_to_a_non_owned_nft_series_returns_error() {
    new_test_ext().execute_with(|| {
        let series_id = <NFTs as traits::NFTs>::NFTSeriesId::from(1u32);
        let _ = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, series_id)
            .expect("creation failed");

        assert_noop!(
            <NFTs as traits::NFTs>::create(&BOB, MockNFTDetails::Empty, series_id),
            Error::<Test>::NotSeriesOwner,
        );
    })
}

#[test]
fn series_owner() {
    new_test_ext().execute_with(|| {
        let valid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(1u32);
        let invalid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(2u32);
        let default_id = <NFTs as traits::NFTs>::NFTSeriesId::default();

        let _ = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, valid_id)
            .expect("creation failed");

        // Existing id should return the creator of the series as owner.
        assert_eq!(<NFTs as traits::NFTs>::series_owner(valid_id), Some(ALICE));
        // Non existing id should return None as the series owner.
        assert_eq!(<NFTs as traits::NFTs>::series_owner(invalid_id), None);
        // Default id should return None as the series owner.
        assert_eq!(<NFTs as traits::NFTs>::series_owner(default_id), None);
    })
}

#[test]
fn transfer_series() {
    new_test_ext().execute_with(|| {
        let valid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(1u32);
        let invalid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(2u32);
        let default_id = <NFTs as traits::NFTs>::NFTSeriesId::default();

        let _ = <NFTs as traits::NFTs>::create(&ALICE, MockNFTDetails::Empty, valid_id)
            .expect("creation failed");

        assert_ok!(<NFTs as traits::NFTs>::set_series_owner(valid_id, &BOB));
        assert_eq!(<NFTs as traits::NFTs>::series_owner(valid_id), Some(BOB));

        assert_ok!(<NFTs as traits::NFTs>::set_series_owner(invalid_id, &BOB));
        assert_eq!(<NFTs as traits::NFTs>::series_owner(invalid_id), Some(BOB));

        assert_noop!(
            <NFTs as traits::NFTs>::set_series_owner(default_id, &BOB),
            Error::<Test>::NFTSeriesLocked,
        );
    })
}
