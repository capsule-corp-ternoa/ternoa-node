use crate::{Call, Config, NFTIdOf, Pallet};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::{Pallet as SystemModule, RawOrigin};
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;
use ternoa_common::traits::{LockableNFTs, NFTs};

use crate::Pallet as TimedEscrow;

fn create_nft<T: Config>(caller: &T::AccountId) -> NFTIdOf<T> {
    T::NFTs::create(
        caller,
        <<T::NFTs as NFTs>::NFTDetails as Default>::default(),
    )
    .expect("shall not fail with a clean state")
}

benchmarks! {
    create {
        let at = SystemModule::<T>::block_number() + 10u32.into();
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        let nft_id = create_nft::<T>(&caller);
    }: _(RawOrigin::Signed(caller), nft_id, receiver_lookup, at)
    verify {
        assert!(T::NFTs::locked(nft_id));
    }

    cancel {
        let at = SystemModule::<T>::block_number() + 10u32.into();
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        let nft_id = create_nft::<T>(&caller);

        drop(TimedEscrow::<T>::create(RawOrigin::Signed(caller.clone()).into(), nft_id, receiver_lookup, at));
    }: _(RawOrigin::Signed(caller), nft_id)
    verify {
        assert!(!T::NFTs::locked(nft_id));
    }

    complete_transfer {
        let at = SystemModule::<T>::block_number() + 10u32.into();
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        let nft_id = create_nft::<T>(&caller);

        drop(TimedEscrow::<T>::create(RawOrigin::Signed(caller.clone()).into(), nft_id, receiver_lookup, at));
    }: _(RawOrigin::Root, receiver.clone(), nft_id)
    verify {
        assert_eq!(T::NFTs::owner(nft_id), receiver);
    }
}

impl_benchmark_test_suite!(
    TimedEscrow,
    crate::tests::mock::new_test_ext(),
    crate::tests::mock::Test
);
