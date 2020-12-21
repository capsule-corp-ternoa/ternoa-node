use super::mock::{new_test_ext, Capsules, Test, ALICE};
use crate::{types::CapsuleData, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn create_increments_counter() {
    new_test_ext().execute_with(|| {
        assert_eq!(Capsules::total(), 0);
        assert_ok!(Capsules::create(
            RawOrigin::Signed(ALICE).into(),
            CapsuleData {
                owner: ALICE,
                creator: ALICE,
                ..Default::default()
            }
        ));
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
