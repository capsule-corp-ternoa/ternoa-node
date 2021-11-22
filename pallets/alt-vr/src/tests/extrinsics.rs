use super::mock::*;
use crate::tests::mock;
use crate::{AltvrUser, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn create_user_happy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        assert_eq!(Altvr::users(1), None);

        let user_name: Vec<u8> = "sub".into();
        let vchat_name: Vec<u8> = "strate".into();
        let altvr_user = AltvrUser::new(user_name.clone(), vchat_name.clone());
        assert_ok!(Altvr::create_user(
            alice.clone(),
            user_name.clone(),
            vchat_name.clone()
        ));
        assert_eq!(Altvr::users(&ALICE), Some(altvr_user));
    });
}

#[test]
fn create_user_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        let normal_name: Vec<u8> = "normal_name".into();
        let too_long_name: Vec<u8> = "this_is_a_too_long_name".into();

        // Unhappy too long Username
        let ok = Altvr::create_user(alice.clone(), too_long_name.clone(), normal_name.clone());
        assert_noop!(ok, Error::<Test>::TooLongUsername);

        // Unhappy too short Username
        let ok = Altvr::create_user(alice.clone(), vec![], normal_name.clone());
        assert_noop!(ok, Error::<Test>::TooShortUsername);

        // Unhappy too long Vchatname
        let ok = Altvr::create_user(alice.clone(), normal_name.clone(), too_long_name.clone());
        assert_noop!(ok, Error::<Test>::TooLongVchatname);

        // Unhappy too short Vchatname
        let ok = Altvr::create_user(alice.clone(), normal_name.clone(), vec![]);
        assert_noop!(ok, Error::<Test>::TooShortVchatname);

        // Unhappy user already exist
        let _ = Altvr::create_user(alice.clone(), normal_name.clone(), normal_name.clone());
        let ok = Altvr::create_user(alice.clone(), normal_name.clone(), normal_name.clone());
        assert_noop!(ok, Error::<Test>::UserAlreadyExist);
    });
}

#[test]
fn set_username_happy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        let initial_name: Vec<u8> = "initial_name".into();
        let updated_name: Vec<u8> = "updated_name".into();
        assert_ok!(Altvr::create_user(
            alice.clone(),
            initial_name.clone(),
            initial_name.clone()
        ));

        assert_ok!(Altvr::set_username(alice.clone(), updated_name.clone()));
    });
}

#[test]
fn set_username_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        let normal_name: Vec<u8> = "normal_name".into();
        let too_long_name: Vec<u8> = "this_is_a_too_long_name".into();

        assert_ok!(Altvr::create_user(
            alice.clone(),
            normal_name.clone(),
            normal_name.clone()
        ));

        // Unhappy too long Username
        let ok = Altvr::set_username(alice.clone(), too_long_name.clone());
        assert_noop!(ok, Error::<Test>::TooLongUsername);
    });
}

#[test]
fn set_vchatname_happy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        let initial_name: Vec<u8> = "initial_name".into();
        let updated_name: Vec<u8> = "updated_name".into();
        assert_ok!(Altvr::create_user(
            alice.clone(),
            initial_name.clone(),
            initial_name.clone()
        ));

        assert_ok!(Altvr::set_vchatname(alice.clone(), updated_name.clone()));

        assert_eq!(Altvr::users(&ALICE).unwrap().vchatname, updated_name);
    });
}

#[test]
fn set_vchatname_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
        let normal_name: Vec<u8> = "normal_name".into();
        let too_long_name: Vec<u8> = "this_is_a_too_long_name".into();

        assert_ok!(Altvr::create_user(
            alice.clone(),
            normal_name.clone(),
            normal_name.clone()
        ));

        // Unhappy too long Vchatname
        let ok = Altvr::set_vchatname(alice.clone(), too_long_name.clone());
        assert_noop!(ok, Error::<Test>::TooLongVchatname);
    });
}
