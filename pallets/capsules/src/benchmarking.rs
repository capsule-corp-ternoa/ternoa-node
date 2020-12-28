use crate::{Call, CapsuleData, Module, Trait};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;
use sp_std::{boxed::Box, prelude::*};

fn mock_capsule_data<T: Trait>(owner: T::AccountId) -> CapsuleData<T::AccountId, T::Hash> {
    CapsuleData {
        offchain_uri: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        pk_hash: Default::default(),
        creator: owner.clone(),
        owner: owner,
        locked: false,
    }
}

benchmarks! {
    _ { }

    create {
        let caller: T::AccountId = whitelisted_caller();
        let capsule = mock_capsule_data::<T>(caller.clone());
    }: _(RawOrigin::Signed(caller), capsule.clone())
    verify {
        assert_eq!(Module::<T>::metadata(1), capsule);
    }

    mutate {
        let new_uri = vec![42];

        let caller: T::AccountId = whitelisted_caller();
        let mut capsule = mock_capsule_data::<T>(caller.clone());
        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), capsule.clone()));
        capsule.offchain_uri = new_uri.clone();
    }: _(RawOrigin::Signed(caller), 1, capsule)
    verify {
        assert_eq!(Module::<T>::metadata(1).offchain_uri, new_uri);
    }

    transfer {
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        let capsule = mock_capsule_data::<T>(caller.clone());
        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), capsule.clone()));
    }: _(RawOrigin::Signed(caller), receiver_lookup, 1)
    verify {
        assert_eq!(Module::<T>::metadata(1).owner, receiver);
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
    fn mutate() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_mutate::<Test>());
        });
    }

    #[test]
    fn transfer() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_transfer::<Test>());
        });
    }
}
