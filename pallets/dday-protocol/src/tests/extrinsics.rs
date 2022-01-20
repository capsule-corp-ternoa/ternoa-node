use super::mock::*;
use crate::{tests::mock, Error};
use frame_support::{assert_ok, assert_noop};
use frame_system::RawOrigin;
use ternoa_common::traits::NFTTrait;

#[test]
fn dday_transmission_ok() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000), (BOB, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            // create a nft and encapsulate it
            let nft_id = help::create_encapsulated_nft(alice.clone(), vec![1], vec![2]);
            // store capsule in transmissions
            assert_ok!(DdayProtocol::dday_transmission(alice, nft_id, BOB, 10));
            let transmission = DdayProtocol::transmissions(nft_id);
            // check if transmission has been stored
            assert_eq!(DdayProtocol::transmissions(nft_id), transmission);
            // check if nft flag has been set to true
            assert_eq!(<NFTs as NFTTrait>::is_in_transmission(nft_id), Some(true));
            // move to block 10
            help::run_to_block(10);
            // check if transmission has been removed
            assert!(DdayProtocol::transmissions(nft_id).is_none());
            // check if nft flag has been set to false
            assert_eq!(<NFTs as NFTTrait>::is_in_transmission(nft_id), Some(false));
            // check capsule owner
            assert_eq!(Capsules::capsules(nft_id).unwrap().owner, BOB);
            // check nft owner
            assert_eq!(NFTs::get_nft(nft_id).unwrap().owner, BOB);
        });
}

#[test]
fn dday_transmission_unknown_capsule_error() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000), (BOB, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            // create a nft
            let nft_id = NFTs::create_nft(ALICE, vec![1], None);
            // should fail because nft has not been encapsulated so this capsule does not exist
            assert_noop!(DdayProtocol::dday_transmission(alice, nft_id.unwrap(), BOB, 10), Error::<Test>::UnknownCapsule);
        });
}

#[test]
fn dday_transmission_not_owner_error() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000), (BOB, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            // create a nft and encapsulate it
            let nft_id = help::create_encapsulated_nft(alice.clone(), vec![1], vec![2]);
            // store capsule in transmissions
            // should fail because bob is not the owner of the capsule
            assert_noop!(DdayProtocol::dday_transmission(bob, nft_id, BOB, 10), Error::<Test>::NotCapsuleOwner);
        });
}

#[test]
fn dday_transmission_is_listed_for_sale_error() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000), (BOB, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            // create a nft and encapsulate it
            let nft_id = help::create_encapsulated_nft(alice.clone(), vec![1], vec![2]);
            // set nft as listed for sale
            assert_ok!(NFTs::set_listed_for_sale(nft_id, true));
            // store capsule in transmissions
            // should fail because NFT is listed for sale
            assert_noop!(DdayProtocol::dday_transmission(alice.clone(), nft_id, BOB, 10), Error::<Test>::IsListedForSale);
        });
}

#[test]
fn dday_transmission_already_in_transmission_error() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000), (BOB, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            // create a nft and encapsulate it
            let nft_id = help::create_encapsulated_nft(alice.clone(), vec![1], vec![2]);
            // store capsule in transmissions
            assert_ok!(DdayProtocol::dday_transmission(alice.clone(), nft_id, BOB, 10));
            // store same capsule in transmissions
            // should fail because it's already in transmissions 
            assert_noop!(DdayProtocol::dday_transmission(alice.clone(), nft_id, BOB, 10), Error::<Test>::AlreadyInTransmission);
        });
}

#[test]
fn cancel_ok() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000), (BOB, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            // create a nft and encapsulate it
            let nft_id = help::create_encapsulated_nft(alice.clone(), vec![1], vec![2]);
            // store capsule in transmissions
            assert_ok!(DdayProtocol::dday_transmission(alice.clone(), nft_id, BOB, 10));
            let transmission = DdayProtocol::transmissions(nft_id);
            // check if transmission has been stored
            assert_eq!(DdayProtocol::transmissions(nft_id), transmission);
            // check if nft flag has been set to true
            assert_eq!(<NFTs as NFTTrait>::is_in_transmission(nft_id), Some(true));
            // cancel transmission
            assert_ok!(DdayProtocol::cancel(alice.clone(), nft_id));
            // check if transmission has been removed
            assert!(DdayProtocol::transmissions(nft_id).is_none());
            // check if nft flag has been set to false
            assert_eq!(<NFTs as NFTTrait>::is_in_transmission(nft_id), Some(false));
        });
}

#[test]
fn cancel_unknown_capsule_error() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000), (BOB, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            // create a nft
            let nft_id = NFTs::create_nft(ALICE, vec![1], None);
            // should fail because nft has not been encapsulated so this capsule does not exist
            assert_noop!(DdayProtocol::cancel(alice.clone(), nft_id.unwrap()), Error::<Test>::UnknownCapsule);
        });
}

#[test]
fn cancel_not_owner_error() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000), (BOB, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            // create a nft and encapsulate it
            let nft_id = help::create_encapsulated_nft(alice.clone(), vec![1], vec![2]);
            // store capsule in transmissions
            // should fail because bob is not the owner of the capsule
            assert_noop!(DdayProtocol::cancel(bob, nft_id), Error::<Test>::NotCapsuleOwner);
        });
}

#[test]
fn cancel_not_in_transmission_error() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 10000), (BOB, 10000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            // create a nft and encapsulate it
            let nft_id = help::create_encapsulated_nft(alice.clone(), vec![1], vec![2]);
            // store same capsule in transmissions
            // should fail because capsule is not in transmissions
            assert_noop!(DdayProtocol::cancel(alice.clone(), nft_id), Error::<Test>::NotInTransmission);
        });
}