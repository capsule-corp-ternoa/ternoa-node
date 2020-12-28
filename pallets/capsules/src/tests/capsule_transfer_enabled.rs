use super::mock::{create_one_capsule, new_test_ext, Capsules, Test, ALICE, BOB};
use crate::Error;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use ternoa_common::traits::CapsuleTransferEnabled;

#[test]
fn locking_prevent_transfer_and_transfer_from() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_ok!(Capsules::lock(1));
        assert_noop!(
            Capsules::transfer_from(ALICE, BOB, 1),
            Error::<Test>::CapsuleLocked
        );
        assert_noop!(
            Capsules::transfer(RawOrigin::Signed(BOB).into(), ALICE, 1),
            Error::<Test>::CapsuleLocked
        );
    })
}

#[test]
fn unlocking_re_enable_transfers() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_ok!(Capsules::lock(1));
        assert_ok!(Capsules::unlock(1));
        assert_ok!(Capsules::transfer_from(ALICE, BOB, 1));
        assert_ok!(Capsules::transfer(RawOrigin::Signed(BOB).into(), ALICE, 1));
    })
}

#[test]
fn transfer_from_fail_if_wrong_owner() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_noop!(
            Capsules::transfer_from(BOB, BOB, 1),
            Error::<Test>::NotCapsuleOwner
        );
    })
}

#[test]
fn transfer_from() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_ok!(Capsules::transfer_from(ALICE, BOB, 1));
        assert_eq!(Capsules::metadata(1).owner, BOB);
    })
}

#[test]
fn is_locked() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert!(!Capsules::is_locked(1));
        assert_ok!(Capsules::lock(1));
        assert!(Capsules::is_locked(1));
    })
}

#[test]
fn is_owner() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert!(Capsules::is_owner(ALICE, 1));
        assert!(!Capsules::is_owner(BOB, 1));
    })
}
