#![cfg(feature = "runtime-benchmarks")]

use crate::{Call, Config, Pallet};
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_std::prelude::*;

use crate::Pallet as Altvr;

benchmarks! {
    set_altvr_username {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let username: Vec<u8> = "jean".into();
    }: _(RawOrigin::Signed(alice.clone()), username.clone())
    verify {
        assert_eq!(Altvr::<T>::altvr_users(alice).unwrap(), username);
    }
}
