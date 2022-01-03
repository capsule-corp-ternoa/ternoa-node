#![cfg(feature = "runtime-benchmarks")]

use crate::{Call, Config, Pallet};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::{Pallet as SystemModule, RawOrigin};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;
use ternoa_common::traits::NFTTrait;

use crate::Pallet as TimedEscrow;

benchmarks! {
    create {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let bob: T::AccountId = account("BOB", 0, 0);
        let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

        let nft_id = 100;
        let at = SystemModule::<T>::block_number() + 10u32.into();

    }: _(RawOrigin::Signed(alice), nft_id, bob_lookup, at)
    verify {
        assert_eq!(T::NFTs::is_in_transmission(nft_id), Some(true));
    }

    cancel {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let bob: T::AccountId = account("BOB", 0, 0);
        let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

        let nft_id = 100;
        let at = SystemModule::<T>::block_number() + 10u32.into();

        drop(TimedEscrow::<T>::create(RawOrigin::Signed(alice.clone()).into(), nft_id, bob_lookup, at));
    }: _(RawOrigin::Signed(alice), nft_id)
    verify {
        assert_eq!(T::NFTs::is_in_transmission(nft_id), Some(false));
    }

    complete_transfer {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let bob: T::AccountId = account("BOB", 0, 0);
        let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

        let nft_id = 100;
        let at = SystemModule::<T>::block_number() + 10u32.into();

        drop(TimedEscrow::<T>::create(RawOrigin::Signed(alice).into(), nft_id, bob_lookup, at));
    }: _(RawOrigin::Root, bob.clone(), nft_id)
    verify {
        assert_eq!(T::NFTs::owner(nft_id), Some(bob));
    }
}

impl_benchmark_test_suite!(
    TimedEscrow,
    crate::tests::mock::new_test_ext(),
    crate::tests::mock::Test
);
