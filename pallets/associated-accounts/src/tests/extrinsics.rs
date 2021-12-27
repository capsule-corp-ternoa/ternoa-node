use super::mock::*;
use crate::tests::mock;
use crate::Error;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn set_altvr_username_happy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        assert_eq!(TernoaAssociatedAccounts::altvr_users(&ALICE), None);

        // Happy path set the altvr username for the first time
        let user_name: Vec<u8> = "sub".into();
        assert_ok!(TernoaAssociatedAccounts::set_altvr_username(
            alice.clone(),
            user_name.clone()
        ));
        assert_eq!(
            TernoaAssociatedAccounts::altvr_users(&ALICE),
            Some(user_name)
        );

        // Happy path update the existing username
        let user_name: Vec<u8> = "sub".into();
        assert_ok!(TernoaAssociatedAccounts::set_altvr_username(
            alice.clone(),
            user_name.clone()
        ));
        assert_eq!(
            TernoaAssociatedAccounts::altvr_users(&ALICE),
            Some(user_name)
        );
    });
}

#[test]
fn set_altvr_username_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        let too_long_name: Vec<u8> = "this_is_a_too_long_name".into();

        // Unhappy too long Username
        let ok = TernoaAssociatedAccounts::set_altvr_username(alice.clone(), too_long_name);
        assert_noop!(ok, Error::<Test>::TooLongAltvrUsername);

        // Unhappy too short Username
        let ok = TernoaAssociatedAccounts::set_altvr_username(alice.clone(), vec![]);
        assert_noop!(ok, Error::<Test>::TooShortAltvrUsername);
    });
}
