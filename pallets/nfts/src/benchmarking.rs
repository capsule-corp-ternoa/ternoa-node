#![cfg(feature = "runtime-benchmarks")]

use crate::{Call, Config, Pallet};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

use crate::Pallet as NFTs;

benchmarks! {
    create {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let nft_id = NFTs::<T>::nft_id_generator();

    }: _(RawOrigin::Signed(alice.clone()), vec![55], None)
    verify {
        assert_eq!(NFTs::<T>::data(nft_id).unwrap().owner, alice);
    }

    transfer {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let bob: T::AccountId = account("BOB", 0, 0);
        let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());
        let nft_id = 100;

    }: _(RawOrigin::Signed(alice.clone()), nft_id, bob_lookup)
    verify {
        assert_eq!(NFTs::<T>::data(nft_id).unwrap().owner, bob);
    }

    burn {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let nft_id = 100;

    }: _(RawOrigin::Signed(alice), nft_id)
    verify {
        assert_eq!(NFTs::<T>::data(nft_id), None);
    }

    finish_series {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let series_id: Vec<u8> = vec![51];

        drop(NFTs::<T>::create(RawOrigin::Signed(alice.clone()).into(), vec![], Some(series_id.clone())));
    }: _(RawOrigin::Signed(alice.clone()), series_id.clone())
    verify {
        assert_eq!(NFTs::<T>::series(&series_id).unwrap().draft, false);
        assert_eq!(NFTs::<T>::series(&series_id).unwrap().owner, alice);
    }
}

impl_benchmark_test_suite!(
    NFTs,
    crate::tests::mock::new_test_ext(),
    crate::tests::mock::Test
);
