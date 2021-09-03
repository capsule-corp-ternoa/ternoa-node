use crate::{Call, Config, NFTData, NFTDetails, NFTId, NFTSeriesDetails, NFTSeriesId, Pallet};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::{Currency, Get};
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

use crate::Pallet as NFTs;

benchmarks! {
    create {
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, T::MintFee::get() + T::Currency::minimum_balance());
    }: _(RawOrigin::Signed(caller.clone()), NFTDetails::default())
    verify {
        let series = NFTSeriesDetails::new(caller, sp_std::vec![NFTId::from(0u32)]);
        assert_eq!(NFTs::<T>::nft_id_generator(), NFTId::from(1u32));
    }

    create_with_series {
        let caller: T::AccountId = whitelisted_caller();
        let series_id = NFTSeriesId::from(1u32);
        let details = NFTDetails::new(vec![], series_id, false);

        T::Currency::make_free_balance_be(&caller, T::MintFee::get() + T::Currency::minimum_balance());
    }: create(RawOrigin::Signed(caller.clone()), details)
    verify {
        let series = NFTSeriesDetails::new(caller, sp_std::vec![NFTId::from(0u32)]);
        assert_eq!(NFTs::<T>::nft_id_generator(), NFTId::from(1u32));
        assert_eq!(NFTs::<T>::series(series_id), Some(series));
    }

    mutate {
        let caller: T::AccountId = whitelisted_caller();

        // There is no code to optimize when the new details are the same
        // than before so calling mutate with the same values (default) will
        // always do the same work and thus keep the benchmark relevant.

        T::Currency::make_free_balance_be(&caller, T::MintFee::get() + T::Currency::minimum_balance());
        drop(NFTs::<T>::create(RawOrigin::Signed(caller.clone()).into(), NFTDetails::default()));
    }: _(RawOrigin::Signed(caller), NFTId::from(0u32), Default::default())
    verify {
        // Absence of error should be enough but we also check the
        // details values so that a unit test is generated.
        assert_eq!(NFTs::<T>::data(NFTId::from(0u32)).details, Default::default());
    }

    seal {
        let caller: T::AccountId = whitelisted_caller();

        T::Currency::make_free_balance_be(&caller, T::MintFee::get() + T::Currency::minimum_balance());
        drop(NFTs::<T>::create(RawOrigin::Signed(caller.clone()).into(), NFTDetails::default()));
    }: _(RawOrigin::Signed(caller), NFTId::from(0u32))
    verify {
        assert_eq!(NFTs::<T>::data(NFTId::from(0u32)).sealed, true);
    }

    transfer {
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();

        T::Currency::make_free_balance_be(&caller, T::MintFee::get() + T::Currency::minimum_balance());
        drop(NFTs::<T>::create(RawOrigin::Signed(caller.clone()).into(), NFTDetails::default()));
    }: _(RawOrigin::Signed(caller), NFTId::from(0u32), receiver_lookup)
    verify {
        assert_eq!(NFTs::<T>::data(NFTId::from(0u32)).owner, receiver);
    }

    burn {
        let caller: T::AccountId = whitelisted_caller();
        let series_id = NFTSeriesId::from(1u32);
        let details = NFTDetails::new(vec![], series_id, false);
        let nft_id = NFTId::from(0u32);

        T::Currency::make_free_balance_be(&caller, T::MintFee::get() + T::Currency::minimum_balance());
        drop(NFTs::<T>::create(RawOrigin::Signed(caller.clone()).into(), details));
    }: _(RawOrigin::Signed(caller), nft_id)
    verify {
        assert_eq!(NFTs::<T>::data(nft_id), NFTData::default());
    }

    transfer_series {
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        let series_id = NFTSeriesId::from(1u32);
        let details = NFTDetails::new(vec![], series_id, false);

        T::Currency::make_free_balance_be(&caller, T::MintFee::get() + T::Currency::minimum_balance());
        drop(NFTs::<T>::create(RawOrigin::Signed(caller.clone()).into(), details));
    }: _(RawOrigin::Signed(caller), series_id, receiver_lookup)
    verify {
        let series = NFTSeriesDetails::new(receiver, sp_std::vec![NFTId::from(0u32)]);
        assert_eq!(NFTs::<T>::series(series_id), Some(series));
    }
}

impl_benchmark_test_suite!(
    NFTs,
    crate::tests::mock::new_test_ext(),
    crate::tests::mock::Test
);
