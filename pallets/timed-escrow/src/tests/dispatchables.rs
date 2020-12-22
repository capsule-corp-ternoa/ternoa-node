use super::mock::{
    create_one_capsule, new_test_ext, Capsules, Scheduler, Test, TimedEscrow, ALICE, BOB,
};
use crate::Error;
use frame_support::{assert_noop, assert_ok, error::BadOrigin, traits::OnInitialize, StorageMap};
use frame_system::RawOrigin;
use pallet_scheduler::Agenda as SchedulerAgenda;
use ternoa_common::traits::CapsuleTransferEnabled;

#[test]
fn create_locks_capsule() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_ok!(TimedEscrow::create(
            RawOrigin::Signed(ALICE).into(),
            1,
            BOB,
            10
        ));
        assert!(Capsules::metadata(1).locked);
    })
}

#[test]
fn create_schedule_transfer() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_ok!(TimedEscrow::create(
            RawOrigin::Signed(ALICE).into(),
            1,
            BOB,
            10
        ));
        // By default nothing is scheduled so checking if we have one element
        // inside the block's agenda should be enough to confirm that a transfer
        // was scheduled.
        assert_eq!(SchedulerAgenda::<Test>::get(10).len(), 1);
        assert!(SchedulerAgenda::<Test>::get(10)[0].is_some());
    })
}

#[test]
fn create_fail_if_not_owner() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TimedEscrow::create(RawOrigin::Signed(BOB).into(), 1, BOB, 10),
            Error::<Test>::NotCapsuleOwner
        );
    })
}

#[test]
fn cancel_unlocks_capsule() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_ok!(TimedEscrow::create(
            RawOrigin::Signed(ALICE).into(),
            1,
            BOB,
            10
        ));
        assert_ok!(TimedEscrow::cancel(RawOrigin::Signed(ALICE).into(), 1));
        assert!(!Capsules::metadata(1).locked);
    })
}

#[test]
fn cancel_cancel_transfer() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_ok!(TimedEscrow::create(
            RawOrigin::Signed(ALICE).into(),
            1,
            BOB,
            10
        ));
        assert_ok!(TimedEscrow::cancel(RawOrigin::Signed(ALICE).into(), 1));
        // We verified previously would fill the block's agenda. So canceling should
        // reset it to 0. However, due to how this is implemented in the scheduler
        // pallet it actually mutate the entry to `None` instead.
        assert_eq!(SchedulerAgenda::<Test>::get(10).len(), 1);
        assert!(SchedulerAgenda::<Test>::get(10)[0].is_none());
    })
}

#[test]
fn cancel_fail_if_not_owner() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TimedEscrow::cancel(RawOrigin::Signed(BOB).into(), 1),
            Error::<Test>::NotCapsuleOwner
        );
    })
}

#[test]
fn transfer_trigger() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_ok!(TimedEscrow::create(
            RawOrigin::Signed(ALICE).into(),
            1,
            BOB,
            10
        ));

        Scheduler::on_initialize(10);

        // Capsule unlocked
        assert!(!Capsules::metadata(1).locked);
        // New owner
        assert_eq!(Capsules::metadata(1).owner, BOB);
    })
}

#[test]
fn manual_complete_transfer() {
    new_test_ext().execute_with(|| {
        create_one_capsule();
        assert_ok!(Capsules::lock(1));
        assert_ok!(TimedEscrow::complete_transfer(
            RawOrigin::Root.into(),
            ALICE,
            BOB,
            1
        ));

        // Capsule unlocked
        assert!(!Capsules::metadata(1).locked);
        // New owner
        assert_eq!(Capsules::metadata(1).owner, BOB);
    })
}

#[test]
fn complete_transfer_can_only_be_called_by_root() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TimedEscrow::complete_transfer(RawOrigin::Signed(ALICE).into(), ALICE, BOB, 1),
            BadOrigin
        );
    })
}
