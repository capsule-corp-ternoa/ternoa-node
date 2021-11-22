#![cfg(feature = "runtime-benchmarks")]

use crate::{Call, Config, Pallet};
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_std::prelude::*;

use crate::Pallet as Altvr;

benchmarks! {
    create_user {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let user_name: Vec<u8> = "jean".into();
        let vchat_name: Vec<u8> = "misterj".into();

    }: _(RawOrigin::Signed(alice.clone()), user_name.clone(), vchat_name)
    verify {
        assert_eq!(Altvr::<T>::users(alice).unwrap().username, user_name);
    }

    set_username {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let user_name: Vec<u8> = "jean".into();
        let vchat_name: Vec<u8> = "misterj".into();
        let updated_user_name: Vec<u8> = "paul".into();

        drop(Altvr::<T>::create_user(RawOrigin::Signed(alice.clone()).into(),
        user_name, vchat_name));

    }: _(RawOrigin::Signed(alice.clone()), updated_user_name.clone())
    verify {
        assert_eq!(Altvr::<T>::users(alice).unwrap().username, updated_user_name);
    }

    set_vchatname {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let user_name: Vec<u8> = "jean".into();
        let vchat_name: Vec<u8> = "misterj".into();
        let updated_vchat_name: Vec<u8> = "mrjean".into();

        drop(Altvr::<T>::create_user(RawOrigin::Signed(alice.clone()).into(),
        user_name, vchat_name));

    }: _(RawOrigin::Signed(alice.clone()), updated_vchat_name.clone())
    verify {
        assert_eq!(Altvr::<T>::users(alice).unwrap().vchatname, updated_vchat_name);
    }
}
