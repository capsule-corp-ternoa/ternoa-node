use super::mock::{create_one_capsule, new_test_ext, Capsules, Test, TimedEscrow, ALICE, BOB};
use frame_support::{assert_ok, StorageMap};
use frame_system::RawOrigin;
use pallet_scheduler::Agenda as SchedulerAgenda;

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
    })
}

#[test]
fn cancel_unlocks_capsule() {}

#[test]
fn cancel_cancel_transfer() {}

#[test]
fn transfer_trigger() {}

#[test]
fn complete_transfer() {}

#[test]
fn complete_transfer_can_only_be_called_by_root() {}
