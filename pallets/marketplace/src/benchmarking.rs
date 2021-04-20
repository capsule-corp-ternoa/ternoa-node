use crate::{Call, Config, Module, NFTIdOf, NFTsForSale};
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::{boxed::Box, prelude::*};
use ternoa_common::traits::NFTs;

fn create_nft<T: Config>(caller: &T::AccountId) -> NFTIdOf<T> {
    T::NFTs::create(
        caller,
        <<T::NFTs as NFTs>::NFTDetails as Default>::default(),
    )
    .expect("shall not fail with a clean state")
}

benchmarks! {
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
    use crate::tests::mock::{ExtBuilder, Test};
    use frame_support::assert_ok;

    #[test]
    fn list() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_list::<Test>());
        });
    }

    #[test]
    fn unlist() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_unlist::<Test>());
        });
    }

    #[test]
    fn buy() {
        ExtBuilder::default().build().execute_with(|| {
            assert_ok!(test_benchmark_buy::<Test>());
        });
    }
}
