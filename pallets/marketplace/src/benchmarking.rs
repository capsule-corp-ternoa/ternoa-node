use crate::{
    Call, Config, MarketplaceIdGenerator, MarketplaceType, Marketplaces, NFTCurrency,
    NFTCurrencyId, NFTsForSale, Pallet,
};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_std::prelude::*;
use ternoa_common::traits::NFTs;
use ternoa_primitives::nfts::NFTId;

use crate::Pallet as Marketplace;

fn create_nft<T: Config>(caller: &T::AccountId) -> NFTId {
    T::NFTs::create(
        caller,
        <<T::NFTs as NFTs>::NFTDetails as Default>::default(),
    )
    .expect("shall not fail with a clean state")
}

benchmarks! {
    buy {
        let buyer: T::AccountId = account("buyer", 0, 0);
        let seller: T::AccountId = account("seller", 0, 0);

        let balance: u32 = 1_000_000;
        T::CurrencyCaps::make_free_balance_be(&buyer, balance.into());
        T::CurrencyCaps::make_free_balance_be(&seller, balance.into());

        let id = create_nft::<T>(&seller);
        let price = NFTCurrency::CAPS(0u32.into());

        drop(Pallet::<T>::list(RawOrigin::Signed(seller.clone()).into(), id, price, None));
    }: _(RawOrigin::Signed(buyer.clone().into()), id, NFTCurrencyId::CAPS)
    verify {
        assert_eq!(T::NFTs::owner(id), buyer);
        assert_eq!(NFTsForSale::<T>::contains_key(id), false);
    }

    list {
        let caller: T::AccountId = whitelisted_caller();
        let nft_id = create_nft::<T>(&caller);

        let price = NFTCurrency::CAPS(100u32.into());

    }: _(RawOrigin::Signed(caller.clone().into()), nft_id, price, None)
    verify {
        assert_eq!(T::NFTs::owner(nft_id), caller);
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), true);
    }

    unlist {
        let caller: T::AccountId = whitelisted_caller();
        let nft_id = create_nft::<T>(&caller);

        let price = NFTCurrency::CAPS(100u32.into());

        drop(Pallet::<T>::list(RawOrigin::Signed(caller.clone()).into(), nft_id, price, None));
    }: _(RawOrigin::Signed(caller.clone().into()), nft_id)
    verify {
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), false);
    }

    create {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone().into()), MarketplaceType::Public, 0)
    verify {
        assert_eq!(Marketplaces::<T>::contains_key(1), true);
        assert_eq!(Marketplaces::<T>::get(1).unwrap().owner, caller);
        assert_eq!(MarketplaceIdGenerator::<T>::get(), 1);
    }
}

/* impl_benchmark_test_suite!(
    Marketplace,
    crate::tests::mock::new_test_ext(),
    crate::tests::mock::Test
); */
