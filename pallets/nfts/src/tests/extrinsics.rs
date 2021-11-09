use super::mock::*;
use crate::tests::mock;
use crate::{Error, NFTData, NFTSeriesDetails};
use frame_support::error::BadOrigin;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;

#[test]
fn create_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1), (CHAD, 100)])
        .build()
        .execute_with(|| {
            // Initial state
            assert_eq!(NFTs::nft_id_generator(), 0);
            assert_eq!(NFTs::series_id_generator(), 0);

            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path NFT with series
            let series = NFTSeriesDetails::new(ALICE, true);
            let data = NFTData::new(ALICE, vec![1], vec![50], false);
            let alice_balance = Balances::free_balance(ALICE);

            let ok = NFTs::create(
                alice.clone(),
                data.ipfs_reference.clone(),
                Some(data.series_id.clone()),
            );
            assert_ok!(ok);

            assert_eq!(NFTs::nft_id_generator(), 1);
            assert_eq!(NFTs::series(&data.series_id), Some(series));
            assert_eq!(NFTs::data(0), Some(data));
            assert_eq!(NFTs::series_id_generator(), 0);
            assert_eq!(
                Balances::free_balance(ALICE),
                alice_balance - NFTs::nft_mint_fee()
            );

            // Happy path NFT without series
            let data = NFTData::new(ALICE, vec![0], vec![48], false);
            let series = NFTSeriesDetails::new(ALICE, true);

            let ok = NFTs::create(alice.clone(), vec![0], None);
            assert_ok!(ok);

            assert_eq!(NFTs::series(&data.series_id), Some(series));
            assert_eq!(NFTs::data(1), Some(data));
            assert_eq!(NFTs::series_id_generator(), 1);
        })
}

#[test]
fn create_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1), (BOB, 100), (CHAD, 100)])
        .build()
        .execute_with(|| {
            // Initial state
            assert_eq!(NFTs::nft_id_generator(), 0);
            assert_eq!(NFTs::series_id_generator(), 0);

            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let chad: mock::Origin = RawOrigin::Signed(CHAD).into();

            // Unhappy too short name
            let ok = NFTs::create(alice.clone(), vec![], None);
            assert_noop!(ok, Error::<Test>::TooShortIpfsReference);

            // Unhappy too long name
            let ok = NFTs::create(alice.clone(), vec![1, 2, 3, 4, 5, 6], None);
            assert_noop!(ok, Error::<Test>::TooLongIpfsReference);

            // Unhappy not enough caps to mint an NFT
            let ok = NFTs::create(alice.clone(), vec![1], None);
            assert_noop!(ok, BalanceError::<Test>::InsufficientBalance);

            // Unhappy not the owner of series
            let series_id = Some(vec![50]);
            help::create(chad.clone(), vec![50], series_id.clone());

            let ok = NFTs::create(bob.clone(), vec![1], series_id);
            assert_noop!(ok, Error::<Test>::NotSeriesOwner);
            assert_eq!(Balances::free_balance(BOB), 100);

            // Unhappy cannot create nfts with complete (locked) series
            let series_id = Some(vec![51]);
            help::create(bob.clone(), vec![50], series_id.clone());
            help::finish_series(bob.clone(), series_id.clone().unwrap());

            let ok = NFTs::create(bob.clone(), vec![1], series_id.clone());
            assert_noop!(ok, Error::<Test>::SeriesIsCompleted);
        })
}

#[test]
fn transfer_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path transfer
            let series_id = vec![2];
            let nft_id = help::create(alice.clone(), vec![1], Some(series_id.clone()));
            help::finish_series(alice.clone(), series_id);
            assert_eq!(NFTs::data(nft_id).unwrap().owner, ALICE);

            assert_ok!(NFTs::transfer(alice.clone(), nft_id, BOB));
            assert_eq!(NFTs::data(nft_id).unwrap().owner, BOB);
        })
}

#[test]
fn transfer_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Unhappy invalid NFT Id
            let ok = NFTs::transfer(alice.clone(), 1001, BOB);
            assert_noop!(ok, Error::<Test>::InvalidNFTId);

            // Unhappy locked NFT
            let nft_id = help::create(alice.clone(), vec![0], None);
            help::lock(nft_id);

            let ok = NFTs::transfer(alice.clone(), nft_id, BOB);
            assert_noop!(ok, Error::<Test>::Locked);

            // Unhappy draft(open) series
            let nft_id = help::create(alice.clone(), vec![0], None);
            let ok = NFTs::transfer(alice.clone(), nft_id, BOB);
            assert_noop!(ok, Error::<Test>::SeriesIsInDraft);

            // Unhappy is capsulized
            let nft_id = help::create(alice.clone(), vec![0], Some(vec![86]));
            help::finish_series(alice.clone(), vec![86]);
            help::capsulize(true);
            let ok = NFTs::transfer(alice.clone(), nft_id, BOB);
            assert_noop!(ok, Error::<Test>::NFTIsCapsulized);
            help::capsulize(false);
        })
}

#[test]
fn burn_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path transfer
            let nft_id = help::create(alice.clone(), vec![1], Some(vec![2]));
            assert_eq!(NFTs::data(nft_id).is_some(), true);

            assert_ok!(NFTs::burn(alice.clone(), nft_id));
            assert_eq!(NFTs::data(nft_id).is_some(), false);
        })
}

#[test]
fn burn_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            // Unhappy invalid NFT Id
            let ok = NFTs::burn(alice.clone(), 10001);
            assert_noop!(ok, Error::<Test>::InvalidNFTId);

            // Unhappy not the owner
            let nft_id = help::create(bob.clone(), vec![1], Some(vec![3]));
            let ok = NFTs::burn(alice.clone(), nft_id);
            assert_noop!(ok, Error::<Test>::NotOwner);

            // Unhappy locked
            let nft_id = help::create(alice.clone(), vec![1], Some(vec![2]));
            help::lock(nft_id);

            let ok = NFTs::burn(alice.clone(), nft_id);
            assert_noop!(ok, Error::<Test>::Locked);

            // Unhappy is capsulized
            let nft_id = help::create(alice.clone(), vec![1], Some(vec![2]));
            help::capsulize(true);
            let ok = NFTs::burn(alice.clone(), nft_id);
            assert_noop!(ok, Error::<Test>::NFTIsCapsulized);
            help::capsulize(false);
        })
}

#[test]
fn finish_series_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path transfer
            let series_id = vec![50];
            help::create(alice.clone(), vec![1], Some(series_id.clone()));
            assert_eq!(NFTs::series(&series_id).unwrap().draft, true);

            assert_ok!(NFTs::finish_series(alice.clone(), series_id.clone()));
            assert_eq!(NFTs::series(&series_id).unwrap().draft, false);
        })
}

#[test]
fn finish_series_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            // Unhappy series id not found
            let ok = NFTs::finish_series(alice.clone(), vec![123]);
            assert_noop!(ok, Error::<Test>::SeriesNotFound);

            // Unhappy not series owner
            let series_id = vec![3];
            help::create(bob.clone(), vec![1], Some(series_id.clone()));
            let ok = NFTs::finish_series(alice.clone(), series_id);
            assert_noop!(ok, Error::<Test>::NotSeriesOwner);

            // Unhappy series is already completed(locked)
            let series_id = vec![55];
            help::create(alice.clone(), vec![1], Some(series_id.clone()));

            assert_ok!(NFTs::finish_series(alice.clone(), series_id.clone()));
            let ok = NFTs::finish_series(alice.clone(), series_id);
            assert_noop!(ok, Error::<Test>::SeriesIsCompleted);
        })
}

#[test]
fn set_nft_mint_fee_happy() {
    ExtBuilder::default().build().execute_with(|| {
        // Happy path
        let old_mint_fee = NFTs::nft_mint_fee();
        let new_mint_fee = 654u64;
        assert_eq!(NFTs::nft_mint_fee(), old_mint_fee);

        let ok = NFTs::set_nft_mint_fee(mock::Origin::root(), new_mint_fee);
        assert_ok!(ok);
        assert_eq!(NFTs::nft_mint_fee(), new_mint_fee);
    })
}

#[test]
fn set_nft_mint_fee_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Unhappy non root user tries to modify the mint fee
            let ok = NFTs::set_nft_mint_fee(alice.clone(), 654);
            assert_noop!(ok, BadOrigin);
        })
}

#[test]
fn set_ipfs_reference_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            let ipfs_reference = vec![50];
            let nft_id = help::create(alice.clone(), ipfs_reference.clone(), None);
            assert_eq!(NFTs::data(&nft_id).unwrap().ipfs_reference, ipfs_reference);

            // Happy path transfer
            let ipfs_reference = vec![51];
            let ok = NFTs::set_ipfs_reference(alice.clone(), nft_id, ipfs_reference.clone());
            assert_ok!(ok);
            assert_eq!(NFTs::data(&nft_id).unwrap().ipfs_reference, ipfs_reference);
        })
}

#[test]
fn set_ipfs_reference_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            // Unhappy invalid nft ID
            let ok = NFTs::set_ipfs_reference(alice.clone(), 10001, vec![60]);
            assert_noop!(ok, Error::<Test>::InvalidNFTId);

            // Unhappy not nft owner
            let bob_nft_id = help::create(bob.clone(), vec![50], None);
            let ok = NFTs::set_ipfs_reference(alice.clone(), bob_nft_id, vec![60]);
            assert_noop!(ok, Error::<Test>::NotOwner);

            // Unhappy nft locked
            let nft_id = help::create(alice.clone(), vec![50], None);
            help::lock(nft_id);
            let ok = NFTs::set_ipfs_reference(alice.clone(), nft_id, vec![60]);
            assert_noop!(ok, Error::<Test>::Locked);

            // Unhappy series
            let series_id = vec![60];
            let nft_id = help::create(alice.clone(), vec![50], Some(series_id.clone()));
            assert_ok!(NFTs::finish_series(alice.clone(), series_id.clone()));
            let ok = NFTs::set_ipfs_reference(alice.clone(), nft_id, vec![60]);
            assert_noop!(ok, Error::<Test>::SeriesIsCompleted);
        })
}
