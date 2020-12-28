use crate::{Call, CapsuleIDOf, Module, Trait};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::{Module as SystemModule, RawOrigin};
use sp_runtime::traits::StaticLookup;
use sp_std::{boxed::Box, prelude::*};
use ternoa_common::traits::{
    CapsuleCreationEnabled, CapsuleDefaultBuilder, CapsuleTransferEnabled,
};

fn create_capsule<T: Trait>(caller: &T::AccountId) -> CapsuleIDOf<T> {
    <T::Capsules as CapsuleCreationEnabled>::create(caller, T::CapsuleData::new_with_owner(caller))
        .expect("shall not fail with a clean state")
}

benchmarks! {
    _ { }

    create {
        let at = SystemModule::<T>::block_number() + 10.into();
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        let capsule_id = create_capsule::<T>(&caller);
    }: _(RawOrigin::Signed(caller), capsule_id, receiver_lookup, at)
    verify {
        assert!(<T::Capsules as CapsuleTransferEnabled>::is_locked(capsule_id));
    }

    cancel {
        let at = SystemModule::<T>::block_number() + 10.into();
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        let capsule_id = create_capsule::<T>(&caller);

        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), capsule_id, receiver_lookup, at));
    }: _(RawOrigin::Signed(caller), capsule_id)
    verify {
        assert!(!<T::Capsules as CapsuleTransferEnabled>::is_locked(capsule_id));
    }

    complete_transfer {
        let at = SystemModule::<T>::block_number() + 10.into();
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        let capsule_id = create_capsule::<T>(&caller);

        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), capsule_id, receiver_lookup, at));
    }: _(RawOrigin::Root, caller, receiver.clone(), capsule_id)
    verify {
        assert!(<T::Capsules as CapsuleTransferEnabled>::is_owner(receiver, capsule_id));
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
