use crate::{Call, Config, Module, NFTIdOf};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::{Module as SystemModule, RawOrigin};
use sp_runtime::traits::StaticLookup;
use sp_std::{boxed::Box, prelude::*};
use ternoa_common::traits::{LockableNFTs, NFTs};

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

        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), nft_id, receiver_lookup, at));
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

        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), nft_id, receiver_lookup, at));
    }: _(RawOrigin::Root, receiver.clone(), nft_id)
    verify {
        assert_eq!(T::NFTs::owner(nft_id), receiver);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mock::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn create() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_create::<Test>());
        });
    }

    #[test]
    fn cancel() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_cancel::<Test>());
        });
    }

    #[test]
    fn complete_transfer() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_complete_transfer::<Test>());
        });
    }
}
