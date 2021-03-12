use crate::NFTsForSale;
use crate::{Call, Module, NFTIdOf, Trait};
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::StorageMap;
use frame_system::RawOrigin;
use sp_std::{boxed::Box, prelude::*};
use ternoa_common::traits::NFTs;

fn create_nft<T: Trait>(caller: &T::AccountId) -> NFTIdOf<T> {
    T::NFTs::create(
        caller,
        <<T::NFTs as NFTs>::NFTDetails as Default>::default(),
    )
    .expect("shall not fail with a clean state")
}

benchmarks! {
    _ { }

    buy {
        let caller: T::AccountId = whitelisted_caller();
        let nft_id = create_nft::<T>(&caller);

        drop(Module::<T>::list(RawOrigin::Signed(caller.clone()).into(), nft_id, 100u32.into()));
    }: _(RawOrigin::Signed(caller.clone().into()), nft_id)
    verify {
        assert_eq!(T::NFTs::owner(nft_id), caller);
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), true);
    }

    list {
        let caller: T::AccountId = whitelisted_caller();
        let nft_id = create_nft::<T>(&caller);

    }: _(RawOrigin::Signed(caller.clone().into()), nft_id, 100u32.into())
    verify {
        assert_eq!(T::NFTs::owner(nft_id), caller);
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), true);
    }

    unlist {
        let caller: T::AccountId = whitelisted_caller();
        let nft_id = create_nft::<T>(&caller);

        drop(Module::<T>::list(RawOrigin::Signed(caller.clone()).into(), nft_id, 100u32.into()));
    }: _(RawOrigin::Signed(caller.clone().into()), nft_id)
    verify {
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), false);
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
            assert_ok!(test_benchmark_buy::<Test>());
        });
    }
}
