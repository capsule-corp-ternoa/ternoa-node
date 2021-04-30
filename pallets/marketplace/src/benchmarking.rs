use crate::{Call, Config, Module, NFTIdOf, NFTsForSale};
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_std::{boxed::Box, prelude::*};
use ternoa_common::traits::NFTs;

fn create_nft<T: Config>(caller: &T::AccountId) -> NFTIdOf<T> {
    T::NFTs::create(
        caller,
        <<T::NFTs as NFTs>::NFTDetails as Default>::default(),
        None,
    )
    .expect("shall not fail with a clean state")
}

benchmarks! {
    buy {
        let buyer: T::AccountId = account("buyer", 0, 0);
        let seller: T::AccountId = account("seller", 0, 0);

        let balance: u32 = 1_000_000;
        T::Currency::make_free_balance_be(&buyer, balance.into());
        T::Currency::make_free_balance_be(&seller, balance.into());

        let id = create_nft::<T>(&seller);
        let price: u32 = 0;

        drop(Module::<T>::list(RawOrigin::Signed(seller.clone()).into(), id, price.into()));
    }: _(RawOrigin::Signed(buyer.clone().into()), id)
    verify {
        assert_eq!(T::NFTs::owner(id), buyer);
        assert_eq!(NFTsForSale::<T>::contains_key(id), false);
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
