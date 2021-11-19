use super::mock::*;
use crate::tests::mock;
use crate::{AltvrData, Error};
use frame_support::error::BadOrigin;
use frame_support::instances::Instance1;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;

#[test]
fn create_altvr_happy() {
    ExtBuilder::default()
        //.caps(vec![(ALICE, 1000), (BOB, 1000)])
        .build()
        .execute_with(|| {
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
            //assert_eq!(Altvr::altvrs(alice), Some(altvr_data));
        });
}
