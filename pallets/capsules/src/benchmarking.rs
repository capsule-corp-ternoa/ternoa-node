#![cfg(feature = "runtime-benchmarks")]

use crate::{Call, CapsuleData, Config, Pallet};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_std::prelude::*;

use crate::Pallet as TernoaCapsules;

benchmarks! {
    create {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let nft_reference = vec![50];
        let capsule_reference = vec![51];
        let nft_id = 1;
        let capsule = CapsuleData::new(alice.clone(), capsule_reference.clone());

    }: _(RawOrigin::Signed(alice.clone()), nft_reference, capsule_reference, None)
    verify {
        assert_eq!(TernoaCapsules::<T>::capsules(nft_id), Some(capsule));
    }

    create_from_nft {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let nft_id = 0;
        let capsule_reference = vec![51];
        let capsule = CapsuleData::new(alice.clone(), capsule_reference.clone());

    }: _(RawOrigin::Signed(alice.clone()), nft_id, capsule_reference.clone())
    verify {
        assert_eq!(TernoaCapsules::<T>::capsules(nft_id), Some(capsule));
    }

    remove {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let nft_id = 0;
        assert_ok!(TernoaCapsules::<T>::create_from_nft(RawOrigin::Signed(alice.clone()).into(), nft_id, vec![40]));
        assert!(TernoaCapsules::<T>::capsules(nft_id).is_some());
        assert!(TernoaCapsules::<T>::ledgers(&alice).is_some());

    }: _(RawOrigin::Signed(alice.clone()), nft_id)
    verify {
        assert!(TernoaCapsules::<T>::capsules(nft_id).is_none());
        assert!(TernoaCapsules::<T>::ledgers(&alice).is_none());
    }

    add_funds {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let nft_id = 0;
        assert_ok!(TernoaCapsules::<T>::create_from_nft(RawOrigin::Signed(alice.clone()).into(), nft_id, vec![40]));
        let fee = TernoaCapsules::<T>::capsule_mint_fee();
        assert_eq!(TernoaCapsules::<T>::ledgers(&alice).unwrap()[0].1, fee);

        let amount = 200u32;
    }: _(RawOrigin::Signed(alice.clone()), nft_id, amount.into())
    verify {
        assert_eq!(TernoaCapsules::<T>::ledgers(&alice).unwrap()[0].1, fee + amount.into());
    }

    set_ipfs_reference {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let nft_id = 0;
        assert_ok!(TernoaCapsules::<T>::create_from_nft(RawOrigin::Signed(alice.clone()).into(), nft_id, vec![40]));
        let old_reference = TernoaCapsules::<T>::capsules(nft_id).unwrap().ipfs_reference.clone();
        let new_reference = vec![67];
        assert_ne!(old_reference, new_reference);
    }: _(RawOrigin::Signed(alice.clone()), nft_id, new_reference.clone())
    verify {
        let reference = TernoaCapsules::<T>::capsules(nft_id).unwrap().ipfs_reference.clone();
        assert_eq!(reference, new_reference);
    }

    set_capsule_mint_fee {
        let old_mint_fee = TernoaCapsules::<T>::capsule_mint_fee();
        let new_mint_fee = 1234u32;
        assert_ne!(old_mint_fee, new_mint_fee.clone().into());

    }: _(RawOrigin::Root, new_mint_fee.clone().into())
    verify {
        assert_eq!(TernoaCapsules::<T>::capsule_mint_fee(), new_mint_fee.into());
    }
}

impl_benchmark_test_suite!(
    TernoaCapsules,
    crate::tests::mock::new_test_ext(),
    crate::tests::mock::Test
);
