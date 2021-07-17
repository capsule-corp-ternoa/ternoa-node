use super::mock::*;
use crate::Error;
use crate::NFTDetails;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use ternoa_common::traits;

#[test]
fn set_owner() {
    ExtBuilder::default().build().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::default()).expect("creation failed");

        assert_ok!(<NFTs as traits::NFTs>::set_owner(id, &BOB));
        assert_eq!(<NFTs as traits::NFTs>::owner(id), BOB);
    })
}

#[test]
fn seal() {
    ExtBuilder::default().build().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::default()).expect("creation failed");

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
    ExtBuilder::default().build().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::default()).expect("creation failed");

        assert_ok!(<NFTs as traits::LockableNFTs>::lock(id));
        assert_eq!(<NFTs as traits::LockableNFTs>::locked(id), true);

        <NFTs as traits::LockableNFTs>::unlock(id);
        assert_eq!(<NFTs as traits::LockableNFTs>::locked(id), false);
    })
}

#[test]
fn lock_prevent_transfers() {
    ExtBuilder::default().build().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::default()).expect("creation failed");

        assert_ok!(<NFTs as traits::LockableNFTs>::lock(id));
        assert_noop!(
            NFTs::transfer(RawOrigin::Signed(ALICE).into(), id, BOB),
            Error::<Test>::Locked
        );
    })
}

#[test]
fn lock_prevent_set_owner() {
    ExtBuilder::default().build().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::default()).expect("creation failed");

        assert_ok!(<NFTs as traits::LockableNFTs>::lock(id));
        assert_noop!(
            <NFTs as traits::NFTs>::set_owner(id, &BOB),
            Error::<Test>::Locked
        );
    })
}

#[test]
fn lock_double_fail() {
    ExtBuilder::default().build().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::default()).expect("creation failed");

        assert_ok!(<NFTs as traits::LockableNFTs>::lock(id));
        assert_noop!(
            <NFTs as traits::LockableNFTs>::lock(id),
            Error::<Test>::Locked
        );
    })
}

#[test]
fn burn_nft() {
    ExtBuilder::default().build().execute_with(|| {
        let id =
            <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::default()).expect("creation failed");

        assert_ne!(<NFTs as traits::NFTs>::owner(id), 0);
        assert_ok!(<NFTs as traits::NFTs>::burn(id));
        assert_eq!(<NFTs as traits::NFTs>::owner(id), 0);
    })
}

#[test]
fn series_length() {
    ExtBuilder::default().build().execute_with(|| {
        let valid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(1u32);
        let invalid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(2u32);
        let default_id = <NFTs as traits::NFTs>::NFTSeriesId::default();

        let count = 3;
        for _ in 0..count {
            let _ =
                <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::new(vec![], valid_id, false))
                    .expect("creation failed");
        }

        // Existing ids should return valid length values.
        assert_eq!(<NFTs as traits::NFTs>::series_length(valid_id), Some(count));

        // Non existing ids should return None as the length.
        assert_eq!(<NFTs as traits::NFTs>::series_length(invalid_id), None);

        // The Default id should return None as the length.
        assert_eq!(<NFTs as traits::NFTs>::series_length(default_id), None);
    })
}

#[test]
fn series_id() {
    ExtBuilder::default().build().execute_with(|| {
        let valid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(1u32);
        let default_id = <NFTs as traits::NFTs>::NFTSeriesId::default();

        let valid_nft_id =
            <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::new(vec![], valid_id, false))
                .expect("creation failed");
        let invalid_nft_id = <NFTs as traits::NFTs>::NFTId::from(100u32);
        let default_nft_id =
            <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::new(vec![], default_id, false))
                .expect("creation failed");

        // Existing nft ids should return valid non default series ids.
        assert_eq!(
            <NFTs as traits::NFTs>::series_id(valid_nft_id),
            Some(valid_id)
        );

        // None should be returned when looking for the series id of non existing nfts.
        assert_eq!(<NFTs as traits::NFTs>::series_id(invalid_nft_id), None);

        // The default series id should be returned for nfts that belong to the default series.
        assert_eq!(
            <NFTs as traits::NFTs>::series_id(default_nft_id),
            Some(<NFTs as traits::NFTs>::NFTSeriesId::default())
        );
    })
}

#[test]
fn series_owner() {
    ExtBuilder::default().build().execute_with(|| {
        let valid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(1u32);
        let invalid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(2u32);
        let default_id = <NFTs as traits::NFTs>::NFTSeriesId::default();

        let _ = <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::new(vec![], valid_id, false))
            .expect("creation failed");

        // Existing ids should return the creator of the series as owner.
        assert_eq!(<NFTs as traits::NFTs>::series_owner(valid_id), Some(ALICE));

        // Non existing ids should return None as the series owner.
        assert_eq!(<NFTs as traits::NFTs>::series_owner(invalid_id), None);

        // The Default id should return None as the series owner.
        assert_eq!(<NFTs as traits::NFTs>::series_owner(default_id), None);
    })
}

#[test]
fn set_series_owner() {
    ExtBuilder::default().build().execute_with(|| {
        let valid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(1u32);
        let invalid_id = <NFTs as traits::NFTs>::NFTSeriesId::from(2u32);
        let default_id = <NFTs as traits::NFTs>::NFTSeriesId::default();

        let _ = <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::new(vec![], valid_id, false))
            .expect("creation failed");

        // It is possible to change owners of existing series.
        assert_ok!(<NFTs as traits::NFTs>::set_series_owner(valid_id, &BOB));
        assert_eq!(<NFTs as traits::NFTs>::series_owner(valid_id), Some(BOB));

        // It is possible to claim ownership of unoccupied series.
        assert_ok!(<NFTs as traits::NFTs>::set_series_owner(invalid_id, &BOB));
        assert_eq!(<NFTs as traits::NFTs>::series_owner(invalid_id), Some(BOB));

        // It is not possible to claim ownership of the default series.
        assert_noop!(
            <NFTs as traits::NFTs>::set_series_owner(default_id, &BOB),
            Error::<Test>::NFTSeriesLocked,
        );
    })
}

#[test]
fn create_capsule() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            // Valid values
            let details = NFTDetails::new(vec![], 0, true);
            let id = <NFTs as traits::NFTs>::create(&ALICE, details).expect("creation failed");
            assert_eq!(<NFTs as traits::NFTs>::is_capsule(id), true);

            // The default values
            let id = <NFTs as traits::NFTs>::create(&ALICE, NFTDetails::default())
                .expect("creation failed");
            assert_eq!(<NFTs as traits::NFTs>::is_capsule(id), false);

            // Unknown nft id value
            assert_eq!(<NFTs as traits::NFTs>::is_capsule(23), false);
        })
}
