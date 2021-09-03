use super::mock::*;
use crate::{Data, Error, NFTData, NFTDetails, NFTSeriesDetails, NFTSeriesId};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use ternoa_common::traits::LockableNFTs;

#[test]
fn create_increment_id() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_eq!(NFTs::nft_id_generator(), 0);
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_eq!(NFTs::nft_id_generator(), 1);
        })
}

#[test]
fn create_register_details() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            let details = NFTDetails::new(vec![42], 1, false);

            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                details.clone(),
            ));
            assert_eq!(NFTs::data(0).details, details);
        })
}

#[test]
fn create_register_owner() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_eq!(NFTs::data(0).owner, ALICE);
        })
}

#[test]
fn create_is_unsealed() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_eq!(NFTs::data(0).sealed, false);
        })
}

#[test]
fn mutate_update_details() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            let details = NFTDetails::new(vec![42], 1, false);
            let nft_id = 0;

            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_ok!(NFTs::mutate(
                RawOrigin::Signed(ALICE).into(),
                nft_id,
                details.clone(),
            ));
            assert_eq!(NFTs::data(0).details, details);
        })
}

#[test]
fn mutate_not_the_owner() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            let details = NFTDetails::new(vec![42], 1, false);
            let nft_id = 0;

            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_noop!(
                NFTs::mutate(RawOrigin::Signed(BOB).into(), nft_id, details),
                Error::<Test>::NotOwner
            );
        })
}

#[test]
fn mutate_sealed() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            let details = NFTDetails::new(vec![42], 1, false);
            let nft_id = 0;

            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            Data::<Test>::mutate(0, |d| d.sealed = true);
            assert_noop!(
                NFTs::mutate(RawOrigin::Signed(ALICE).into(), nft_id, details),
                Error::<Test>::Sealed
            );
        })
}

#[test]
fn transfer_update_owner() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_ok!(NFTs::transfer(RawOrigin::Signed(ALICE).into(), 0, BOB));
            assert_eq!(NFTs::data(0).owner, BOB);
        })
}

#[test]
fn transfer_not_the_owner() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_noop!(
                NFTs::transfer(RawOrigin::Signed(BOB).into(), 0, BOB),
                Error::<Test>::NotOwner
            );
        })
}

#[test]
fn seal_mutate_seal_flag() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_ok!(NFTs::seal(RawOrigin::Signed(ALICE).into(), 0));
            assert_eq!(NFTs::data(0).sealed, true);
        })
}

#[test]
fn seal_not_the_owner() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_noop!(
                NFTs::seal(RawOrigin::Signed(BOB).into(), 0),
                Error::<Test>::NotOwner
            );
        })
}

#[test]
fn seal_already_sealed() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));
            assert_ok!(NFTs::seal(RawOrigin::Signed(ALICE).into(), 0));
            assert_noop!(
                NFTs::seal(RawOrigin::Signed(ALICE).into(), 0),
                Error::<Test>::Sealed
            );
        })
}

#[test]
fn burn_owned_nft() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            let series_id = NFTSeriesId::from(1u32);
            let nft_id = NFTs::nft_id_generator();

            let before_details = NFTSeriesDetails::new(ALICE, sp_std::vec![nft_id]);
            let after_details = NFTSeriesDetails::new(ALICE, sp_std::vec![]);
            let details = NFTDetails::new(vec![], series_id, false);
            let alice: Origin = RawOrigin::Signed(ALICE).into();

            assert_ok!(NFTs::create(alice.clone(), details.clone()));
            assert_eq!(NFTs::series(series_id), Some(before_details));

            assert_ok!(NFTs::burn(alice.clone(), nft_id));
            assert_eq!(NFTs::data(nft_id), NFTData::default());
            assert_eq!(NFTs::series(series_id), Some(after_details));

            let id = NFTs::nft_id_generator();
            assert_ok!(NFTs::create(alice.clone(), details));
            assert_ok!(<NFTs as LockableNFTs>::lock(id));
            assert_noop!(NFTs::burn(alice, id), Error::<Test>::Locked);
        })
}

#[test]
fn burn_not_owned_nft() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::default(),
            ));

            let id = NFTs::nft_id_generator() - 1;

            assert_eq!(id, 0);
            assert_noop!(
                NFTs::burn(RawOrigin::Signed(BOB).into(), 0),
                Error::<Test>::NotOwner
            );
            assert_eq!(NFTs::data(id).owner, ALICE);
        })
}

#[test]
fn burn_none_existent_nft() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            assert_noop!(
                NFTs::burn(RawOrigin::Signed(ALICE).into(), 100),
                Error::<Test>::NotOwner
            );
        })
}

#[test]
fn series_create() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            let alice = RawOrigin::Signed(ALICE);

            let valid_id = NFTSeriesId::from(1u32);
            let default_id = NFTSeriesId::default();

            let details = NFTSeriesDetails::new(ALICE, sp_std::vec![1u32, 2u32]);
            let valid_nft_details = NFTDetails::new(vec![], valid_id, false);
            let default_nft_details = NFTDetails::new(vec![], default_id, false);

            // Alice can create an nft that belongs to the default series.
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                default_nft_details,
            ));

            // Alice can create a new nft series by creating an nft with a unused series id.
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                valid_nft_details.clone(),
            ));
            assert_eq!(NFTs::series(valid_id).unwrap().owner, ALICE);

            // Since Alice is now the owner of the series, she can add as many nfts as she
            // wants.
            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                valid_nft_details.clone(),
            ));
            assert_eq!(NFTs::series(valid_id), Some(details.clone()));

            // Bob cannot create an nft under a series that he does not own.
            assert_noop!(
                NFTs::create(RawOrigin::Signed(BOB).into(), valid_nft_details),
                Error::<Test>::NotSeriesOwner
            );

            // Alice stays the owner of the series even if all the nfts do not belong to her
            // anymore.
            for nft_id in details.nfts {
                assert_ok!(NFTs::transfer(alice.clone().into(), nft_id, BOB));
            }
            assert_eq!(NFTs::series(valid_id).unwrap().owner, ALICE);
        })
}

#[test]
fn transfer_series() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            let alice = RawOrigin::Signed(ALICE);

            let valid_id = NFTSeriesId::from(1u32);
            let invalid_id = NFTSeriesId::from(10u32);
            let default_id = NFTSeriesId::default();

            let bob_details = NFTSeriesDetails::new(BOB, sp_std::vec![0u32]);

            assert_ok!(NFTs::create(
                RawOrigin::Signed(ALICE).into(),
                NFTDetails::new(vec![], valid_id, false)
            ));

            // Since Alice owns the series she can transfer it to Bob.
            assert_ok!(NFTs::transfer_series(alice.clone().into(), valid_id, BOB));
            assert_eq!(NFTs::series(valid_id), Some(bob_details));

            // Sadly, Alice is no longer the series owner so she is unable to
            // transfer the same series to Chad.
            assert_noop!(
                NFTs::transfer_series(alice.clone().into(), valid_id, CHAD),
                Error::<Test>::NotSeriesOwner
            );

            // Alice cannot transfer series ownership to Bob if the series
            // does not exists.
            assert_noop!(
                NFTs::transfer_series(alice.clone().into(), invalid_id, BOB),
                Error::<Test>::NFTSeriesNotFound
            );

            // Alice cannot transfer ownership of the default series to anyone.
            assert_noop!(
                NFTs::transfer_series(alice.clone().into(), default_id, BOB),
                Error::<Test>::NotSeriesOwner
            );
        })
}

#[test]
fn mint_fees() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            // Setting up the stage
            const INITIAL_FUNDS: u64 = 100u64;
            const FEE: u64 = MintFee::get();
            const NEW_FUNDS_1: u64 = INITIAL_FUNDS - FEE;
            const NEW_FUNDS_2: u64 = NEW_FUNDS_1 - FEE;

            let alice: Origin = RawOrigin::Signed(ALICE).into();
            let bob: Origin = RawOrigin::Signed(BOB).into();

            const SERIES_ID: u32 = 1u32;
            assert_ok!(NFTs::create(bob, NFTDetails::new(vec![], SERIES_ID, false)));
            assert_eq!(Balances::free_balance(&COLLECTOR), FEE);

            // Alice will pay additional mint fees if she wants to create a normal nft.
            assert_ok!(NFTs::create(alice.clone(), NFTDetails::default()));
            assert_eq!(Balances::free_balance(&COLLECTOR), FEE * 2);
            assert_eq!(Balances::free_balance(&ALICE), NEW_FUNDS_1);

            // Alice will pay additional mint fees if she wants to create a capsule.
            assert_ok!(NFTs::create(
                alice.clone(),
                NFTDetails::new(vec![], 0, true)
            ));
            assert_eq!(Balances::free_balance(&COLLECTOR), FEE * 3);
            assert_eq!(Balances::free_balance(&ALICE), NEW_FUNDS_2);

            // Alice will not pay any fees if the create function fails.
            assert!(NFTs::create(alice.clone(), NFTDetails::new(vec![], SERIES_ID, true)).is_err());
            assert_eq!(Balances::free_balance(&ALICE), NEW_FUNDS_2);
        })
}

#[test]
fn create_capsule() {
    ExtBuilder::default()
        .one_hundred_for_everyone()
        .build()
        .execute_with(|| {
            // Setting up the stage
            let alice: Origin = RawOrigin::Signed(ALICE).into();

            assert_ok!(NFTs::create(
                alice.clone(),
                NFTDetails::new(vec![], 0, true)
            ));

            // Alice cannot create a capsule if she doesn't have enough money.
            let funds = Balances::free_balance(ALICE);
            assert_ok!(Balances::transfer(alice.clone(), BOB, funds));
            assert_noop!(
                NFTs::create(alice.clone(), NFTDetails::new(vec![], 0, true)),
                pallet_balances::Error::<Test>::InsufficientBalance
            );
        })
}
