use super::mock::*;
use crate::tests::mock;
use crate::Error;
use frame_support::{assert_noop, assert_ok, error::BadOrigin, traits::OnInitialize};
use frame_system::RawOrigin;
use pallet_scheduler::Agenda as SchedulerAgenda;
use ternoa_common::traits::NFTTrait;

#[test]
fn create_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path
            let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
            assert_ok!(TimedEscrow::create(alice.clone(), nft_id, BOB, 10));

            let nft = <NFTs as NFTTrait>::get_nft(nft_id).unwrap();
            assert_eq!(nft.in_transmission, true);

            // By default nothing is scheduled so checking if we have one element
            // inside the block's agenda should be enough to confirm that a transfer
            // was scheduled.
            assert_eq!(SchedulerAgenda::<Test>::get(10).len(), 1);
            assert!(SchedulerAgenda::<Test>::get(10)[0].is_some());

            // block 10
            Scheduler::on_initialize(10);
            let nft = <NFTs as NFTTrait>::get_nft(nft_id).unwrap();
            assert_eq!(nft.owner, BOB);
            assert_eq!(nft.in_transmission, false);
        });
}

#[test]
fn create_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Unhappy nft doesn't exist
            let ok = TimedEscrow::create(alice.clone(), 1001, BOB, 10);
            assert_noop!(ok, Error::<Test>::UnknownNFT);

            // Unhappy not nft owner
            let nft_id = <NFTs as NFTTrait>::create_nft(BOB, vec![0], None).unwrap();
            let ok = TimedEscrow::create(alice.clone(), nft_id, BOB, 10);
            assert_noop!(ok, Error::<Test>::NotNFTOwner);

            // Unhappy listed for sale
            let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
            <NFTs as NFTTrait>::set_listed_for_sale(nft_id, true).unwrap();

            let ok = TimedEscrow::create(alice.clone(), nft_id, BOB, 10);
            assert_noop!(ok, Error::<Test>::ListedForSale);

            // Unhappy already in transmission
            let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
            <NFTs as NFTTrait>::set_in_transmission(nft_id, true).unwrap();

            let ok = TimedEscrow::create(alice.clone(), nft_id, BOB, 10);
            assert_noop!(ok, Error::<Test>::AlreadyInTransmission);
        });
}

#[test]
fn cancel_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Happy path
            let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
            assert_ok!(TimedEscrow::create(alice.clone(), nft_id, BOB, 10));
            assert_ok!(TimedEscrow::cancel(alice.clone(), nft_id));
            assert_eq!(<NFTs as NFTTrait>::is_in_transmission(nft_id), Some(false));

            // We verified previously would fill the block's agenda. So canceling should
            // reset it to 0. However, due to how this is implemented in the scheduler
            // pallet it actually mutate the entry to `None` instead.
            assert_eq!(SchedulerAgenda::<Test>::get(10).len(), 1);
            assert!(SchedulerAgenda::<Test>::get(10)[0].is_none());
        });
}

#[test]
fn cancel_unhappy() {
    ExtBuilder::default()
        .caps(vec![(BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Unhappy nft id doesn't exists
            let ok = TimedEscrow::cancel(alice.clone(), 1001);
            assert_noop!(ok, Error::<Test>::UnknownNFT);

            // Unhappy not nft owner
            let nft_id = <NFTs as NFTTrait>::create_nft(BOB, vec![0], None).unwrap();
            let ok = TimedEscrow::cancel(alice.clone(), nft_id);
            assert_noop!(ok, Error::<Test>::NotNFTOwner);
        });
}

#[test]
fn complete_transfer_happy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let root: mock::Origin = RawOrigin::Root.into();

            // Happy path
            let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
            assert_ok!(TimedEscrow::create(alice.clone(), nft_id, BOB, 10));
            assert_ok!(TimedEscrow::complete_transfer(root, BOB, nft_id));

            let nft = <NFTs as NFTTrait>::get_nft(nft_id).unwrap();
            assert_eq!(nft.owner, BOB);
            assert_eq!(nft.in_transmission, false);
        });
}

#[test]
fn complete_transfer_unhappy() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let root: mock::Origin = RawOrigin::Root.into();

            let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
            assert_ok!(TimedEscrow::create(alice.clone(), nft_id, BOB, 10));

            // Unhappy not root
            let ok = TimedEscrow::complete_transfer(alice.clone(), BOB, nft_id);
            assert_noop!(ok, BadOrigin);

            // Unhappy failed to set owner because wrong id was given
            let ok = TimedEscrow::complete_transfer(root, BOB, 1001);
            assert_noop!(ok, ternoa_nfts::Error::<Test>::NFTNotFound);
        });
}
