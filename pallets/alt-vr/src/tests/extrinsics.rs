use super::mock::*;
use crate::tests::mock;
use crate::{AltvrData, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn create_altvr_happy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        assert_eq!(Altvr::altvrs(1), None);

        let user_name: Vec<u8> = "sub".into();
        let vchat_name: Vec<u8> = "strate".into();
        let altvr_data = AltvrData::new(user_name.clone(), vchat_name.clone());
        assert_ok!(Altvr::create_altvr(
            alice.clone(),
            user_name.clone(),
            vchat_name.clone()
        ));
        assert_eq!(Altvr::altvrs(&ALICE), Some(altvr_data));
    });
}

#[test]
fn create_altvr_unhappy() {
    ExtBuilder::default()
        //.caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            let normal_name: Vec<u8> = "normal_name".into();
            let too_long_name: Vec<u8> = "this_is_a_too_long_name".into();

            // Unhappy too long Username
            let ok = Altvr::create_altvr(alice.clone(), too_long_name.clone(), normal_name.clone());
            assert_noop!(ok, Error::<Test>::TooLongUsername);

            // Unhappy too short Username
            let ok = Altvr::create_altvr(alice.clone(), vec![], normal_name.clone());
            assert_noop!(ok, Error::<Test>::TooShortUsername);

            // Unhappy too long Vchatname
            let ok = Altvr::create_altvr(alice.clone(), normal_name.clone(), too_long_name.clone());
            assert_noop!(ok, Error::<Test>::TooLongVchatname);

            // Unhappy too short Vchatname
            let ok = Altvr::create_altvr(alice.clone(), normal_name.clone(), vec![]);
            assert_noop!(ok, Error::<Test>::TooShortVchatname);
        });
}

#[test]
fn update_username_happy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        let initial_name: Vec<u8> = "initial_name".into();
        let updated_name: Vec<u8> = "updated_name".into();
        assert_ok!(Altvr::create_altvr(
            alice.clone(),
            initial_name.clone(),
            initial_name.clone()
        ));

        assert_ok!(Altvr::update_username(alice.clone(), updated_name.clone()));

        assert_eq!(Altvr::altvrs(&ALICE).unwrap().username, updated_name);
    });
}

#[test]
fn update_username_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        let normal_name: Vec<u8> = "normal_name".into();
        let too_long_name: Vec<u8> = "this_is_a_too_long_name".into();

        assert_ok!(Altvr::create_altvr(
            alice.clone(),
            normal_name.clone(),
            normal_name.clone()
        ));

        // Unhappy too long Username
        let ok = Altvr::update_username(alice.clone(), too_long_name.clone());
        assert_noop!(ok, Error::<Test>::TooLongUsername);
    });
}

#[test]
fn update_vchatname_happy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

        let initial_name: Vec<u8> = "initial_name".into();
        let updated_name: Vec<u8> = "updated_name".into();
        assert_ok!(Altvr::create_altvr(
            alice.clone(),
            initial_name.clone(),
            initial_name.clone()
        ));

        assert_ok!(Altvr::update_vchatname(alice.clone(), updated_name.clone()));

        assert_eq!(Altvr::altvrs(&ALICE).unwrap().vchatname, updated_name);
    });
}

#[test]
fn update_vchatname_unhappy() {
    ExtBuilder::default().build().execute_with(|| {
        let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
        let normal_name: Vec<u8> = "normal_name".into();
        let too_long_name: Vec<u8> = "this_is_a_too_long_name".into();

        assert_ok!(Altvr::create_altvr(
            alice.clone(),
            normal_name.clone(),
            normal_name.clone()
        ));

        // Unhappy too long Vchatname
        let ok = Altvr::update_vchatname(alice.clone(), too_long_name.clone());
        assert_noop!(ok, Error::<Test>::TooLongVchatname);
    });
}
