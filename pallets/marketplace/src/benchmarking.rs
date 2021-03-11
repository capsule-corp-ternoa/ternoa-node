use crate::{Call, Module, Trait, NFTIdOf};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;
use sp_std::{boxed::Box, prelude::*};
use ternoa_common::traits::{NFTs};


fn create_nft<T: Trait>(caller: &T::AccountId) -> NFTIdOf<T> {
    T::NFTs::create(
        caller,
        <<T::NFTs as NFTs>::NFTDetails as Default>::default(),
    )
        .expect("shall not fail with a clean state")
}

fn list_nft<T: Trait>(caller: &T::AccountId) -> NFTIdOf<T> {
    T::NFTs::create(
        caller,
        <<T::NFTs as NFTs>::NFTDetails as Default>::default(),
    )
        .expect("shall not fail with a clean state")
}

benchmarks! {
    _ { }

    buy {
        let receiver: T::AccountId = account("receiver", 0, 0);
        let receiver_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(receiver.clone());

        let caller: T::AccountId = whitelisted_caller();
        let nft_id = create_nft::<T>(&caller);

        drop(Module::<T>::list(RawOrigin::Signed(caller.clone()).into(), nft_id, 100));
    }: _(RawOrigin::Root, receiver.clone(), nft_id)
    verify {
        assert_eq!(T::NFTs::owner(nft_id), receiver);
        assert_eq!(Module::<T>::NFTsForSale<T>::contains_key(nft_id),true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mock::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn list() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_list::<Test>());
        });
    }

    #[test]
    fn unlist() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_unlist::<Test>());
        });
    }

    #[test]
    fn buy() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_unlist::<Test>());
        });
    }
}
