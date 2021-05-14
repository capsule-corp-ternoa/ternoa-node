use crate::{Call, Config, Module, NFTData, NFTDetails, NFTSeriesDetails, NFTSeriesId};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;
use sp_std::{boxed::Box, prelude::*};

benchmarks! {
    create {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), NFTDetails::default())
    verify {
        let series = NFTSeriesDetails::new(caller, sp_std::vec![T::NFTId::from(0)]);
        assert_eq!(Module::<T>::total(), T::NFTId::from(1));
    }

    create_with_series {
        let caller: T::AccountId = whitelisted_caller();
        let series_id = NFTSeriesId::from(1u32);
        let details = NFTDetails::new(vec![], series_id);
    }: create(RawOrigin::Signed(caller.clone()), details)
    verify {
        let series = NFTSeriesDetails::new(caller, sp_std::vec![T::NFTId::from(0)]);
        assert_eq!(Module::<T>::total(), T::NFTId::from(1));
        assert_eq!(Module::<T>::series(series_id), Some(series));
    }

    mutate {
        let caller: T::AccountId = whitelisted_caller();

        // There is no code to optimize when the new details are the same
        // than before so calling mutate with the same values (default) will
        // always do the same work and thus keep the benchmark relevant.

        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), NFTDetails::default()));
    }: _(RawOrigin::Signed(caller), T::NFTId::from(0), Default::default())
    verify {
        // Absence of error should be enough but we also check the
        // details values so that a unit test is generated.
        assert_eq!(Module::<T>::data(T::NFTId::from(0)).details, Default::default());
    }

    seal {
        let caller: T::AccountId = whitelisted_caller();
        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), NFTDetails::default()));
    }: _(RawOrigin::Signed(caller), T::NFTId::from(0))
    verify {
        assert_eq!(Module::<T>::data(T::NFTId::from(0)).sealed, true);
    }

    transfer {
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), NFTDetails::default()));
    }: _(RawOrigin::Signed(caller), T::NFTId::from(0), receiver_lookup)
    verify {
        assert_eq!(Module::<T>::data(T::NFTId::from(0)).owner, receiver);
    }

    burn {
        let caller: T::AccountId = whitelisted_caller();
        let series_id = NFTSeriesId::from(1u32);
        let details = NFTDetails::new(vec![], series_id);
        let nft_id = T::NFTId::from(0);
        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), details));
    }: _(RawOrigin::Signed(caller), nft_id)
    verify {
        assert_eq!(Module::<T>::data(nft_id), NFTData::default());
    }

    transfer_series {
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());


        let caller: T::AccountId = whitelisted_caller();
        let series_id = NFTSeriesId::from(1u32);
        let details = NFTDetails::new(vec![], series_id);
        drop(Module::<T>::create(RawOrigin::Signed(caller.clone()).into(), details));
    }: _(RawOrigin::Signed(caller), series_id, receiver_lookup)
    verify {
        let series = NFTSeriesDetails::new(receiver, sp_std::vec![T::NFTId::from(0)]);
        assert_eq!(Module::<T>::series(series_id), Some(series));
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
    fn seal() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_seal::<Test>());
        });
    }

    #[test]
    fn transfer() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_transfer::<Test>());
        });
    }

    #[test]
    fn burn() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_burn::<Test>());
        });
    }
}
