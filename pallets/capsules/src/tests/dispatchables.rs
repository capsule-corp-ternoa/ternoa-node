use super::mock::{create_one_capsule, new_test_ext, Capsules, Test, ALICE, BOB};
use crate::{types::CapsuleData, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn create_increments_counter() {
    new_test_ext().execute_with(|| {
        assert_eq!(Capsules::total(), 0);
        create_one_capsule();
        assert_eq!(Capsules::total(), 1);
    })
}

#[test]
fn create_save_metadata() {
    new_test_ext().execute_with(|| {
        let data = CapsuleData {
            owner: ALICE,
            creator: ALICE,
            ..Default::default()
        };
        assert_ok!(Capsules::create(
            RawOrigin::Signed(ALICE).into(),
            data.clone()
        ));
        assert_eq!(Capsules::metadata(1), data);
    })
}

#[test]
fn create_rejects_malformed_metadata() {
    new_test_ext().execute_with(|| {
        // Creator is wrong
        assert_noop!(
            Capsules::create(
                RawOrigin::Signed(ALICE).into(),
                CapsuleData {
                    owner: ALICE,
                    ..Default::default()
                }
            ),
            Error::<Test>::MalformedMetadata
        );

        // Owner is wrong
        assert_noop!(
            Capsules::create(
                RawOrigin::Signed(ALICE).into(),
                CapsuleData {
                    creator: ALICE,
                    ..Default::default()
                }
            ),
            Error::<Test>::MalformedMetadata
        );
    })
}

#[test]
fn transfer_update_owner() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_eq!(Capsules::metadata(1).owner, ALICE);
        assert_ok!(Capsules::transfer(RawOrigin::Signed(ALICE).into(), BOB, 1));
        assert_eq!(Capsules::metadata(1).owner, BOB);
    })
}

#[test]
fn transfer_fail_if_not_owner() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_noop!(
            Capsules::transfer(RawOrigin::Signed(BOB).into(), ALICE, 1),
            Error::<Test>::NotCapsuleOwner
        );
    })
}

#[test]
fn transfer_fail_if_capsule_does_not_exists() {
    new_test_ext().execute_with(|| {
        // The capsule is not being created
        assert_noop!(
            Capsules::transfer(RawOrigin::Signed(BOB).into(), ALICE, 1),
            Error::<Test>::NotCapsuleOwner
        );
    })
}

#[test]
fn mutate_fails_if_not_owner() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Capsules::mutate(RawOrigin::Signed(BOB).into(), 1, CapsuleData::default()),
            Error::<Test>::NotCapsuleOwner
        );
    })
}

#[test]
fn mutate_fails_if_owner_change() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_noop!(
            Capsules::mutate(
                RawOrigin::Signed(ALICE).into(),
                1,
                CapsuleData {
                    owner: BOB,
                    creator: ALICE,
                    ..Default::default()
                }
            ),
            Error::<Test>::MalformedMetadata
        );
    })
}

#[test]
fn mutate_fails_if_creator_change() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_noop!(
            Capsules::mutate(
                RawOrigin::Signed(ALICE).into(),
                1,
                CapsuleData {
                    owner: ALICE,
                    creator: BOB,
                    ..Default::default()
                }
            ),
            Error::<Test>::MalformedMetadata
        );
    })
}

#[test]
fn mutate() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        let marker = vec![1, 2, 3, 4, 5];
        assert_ok!(Capsules::mutate(
            RawOrigin::Signed(ALICE).into(),
            1,
            CapsuleData {
                owner: ALICE,
                creator: ALICE,
                offchain_uri: marker.clone(),
                ..Default::default()
            }
        ));
        assert_eq!(Capsules::metadata(1).offchain_uri, marker);
    })
}
