use crate::{
    BalanceCaps, Call, Config, MarketplaceIdGenerator, MarketplaceType, Marketplaces, NFTCurrency,
    NFTCurrencyId, NFTsForSale, Pallet,
};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_runtime::traits::{Bounded, StaticLookup};
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

        T::CurrencyCaps::make_free_balance_be(&buyer, BalanceCaps::<T>::max_value());
        T::CurrencyCaps::make_free_balance_be(&seller, T::CurrencyCaps::minimum_balance());

        let id = create_nft::<T>(&seller);
        let price = NFTCurrency::Caps(1u32.into());

        drop(Marketplace::<T>::list(RawOrigin::Signed(seller.clone()).into(), id, price, None));
    }: _(RawOrigin::Signed(buyer.clone().into()), id, NFTCurrencyId::Caps)
    verify {
        assert_eq!(T::NFTs::owner(id), buyer);
        assert_eq!(NFTsForSale::<T>::contains_key(id), false);
    }

    list {
        let caller: T::AccountId = whitelisted_caller();
        T::CurrencyCaps::make_free_balance_be(&caller, BalanceCaps::<T>::max_value());

        let nft_id = create_nft::<T>(&caller);

        let price = NFTCurrency::Caps(100u32.into());

    }: _(RawOrigin::Signed(caller.clone().into()), nft_id, price, None)
    verify {
        assert_eq!(T::NFTs::owner(nft_id), caller);
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), true);
    }

    unlist {
        let caller: T::AccountId = whitelisted_caller();
        T::CurrencyCaps::make_free_balance_be(&caller, BalanceCaps::<T>::max_value());

        let nft_id = create_nft::<T>(&caller);

        let price = NFTCurrency::Caps(100u32.into());

        drop(Marketplace::<T>::list(RawOrigin::Signed(caller.clone()).into(), nft_id, price, None));
    }: _(RawOrigin::Signed(caller.clone().into()), nft_id)
    verify {
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), false);
    }

    create {
        let caller: T::AccountId = whitelisted_caller();
        T::CurrencyCaps::make_free_balance_be(&caller, BalanceCaps::<T>::max_value());

    }: _(RawOrigin::Signed(caller.clone().into()), MarketplaceType::Public, 0, "".into())
    verify {
        assert_eq!(Marketplaces::<T>::contains_key(1), true);
        assert_eq!(Marketplaces::<T>::get(1).unwrap().owner, caller);
        assert_eq!(MarketplaceIdGenerator::<T>::get(), 1);
    }

    add_account_to_allow_list {
        let owner: T::AccountId = account("owner", 0, 0);
        let account: T::AccountId = account("account", 0, 0);
        let account_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(account.clone());
        T::CurrencyCaps::make_free_balance_be(&owner, BalanceCaps::<T>::max_value());

        drop(Marketplace::<T>::create(RawOrigin::Signed(owner.clone()).into(), MarketplaceType::Private, 0, "".into()));

    }: _(RawOrigin::Signed(owner.clone().into()), 1, account_lookup.into())
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().allow_list, vec![account]);
    }

    remove_account_from_allow_list {
        let owner: T::AccountId = account("owner", 0, 0);
        let account: T::AccountId = account("account", 0, 0);
        let account_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(account.clone());
        T::CurrencyCaps::make_free_balance_be(&owner, BalanceCaps::<T>::max_value());

        drop(Marketplace::<T>::create(RawOrigin::Signed(owner.clone()).into(), MarketplaceType::Private, 0, "".into()));
        drop(Marketplace::<T>::add_account_to_allow_list(RawOrigin::Signed(owner.clone()).into(), 1, account_lookup.clone()));

    }: _(RawOrigin::Signed(owner.clone().into()), 1, account_lookup)
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().allow_list, vec![]);
    }

    change_owner {
        let owner: T::AccountId = account("owner", 0, 0);
        let account: T::AccountId = account("account", 0, 0);
        let account_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(account.clone());
        T::CurrencyCaps::make_free_balance_be(&owner, BalanceCaps::<T>::max_value());

        drop(Marketplace::<T>::create(RawOrigin::Signed(owner.clone()).into(), MarketplaceType::Private, 0, "".into()));

    }: _(RawOrigin::Signed(owner.clone().into()), 1, account_lookup)
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().owner, account);
    }

    change_market_type {
        let owner: T::AccountId = account("owner", 0, 0);
        T::CurrencyCaps::make_free_balance_be(&owner, BalanceCaps::<T>::max_value());
        drop(Marketplace::<T>::create(RawOrigin::Signed(owner.clone()).into(), MarketplaceType::Public, 0, "".into()));

    }: _(RawOrigin::Signed(owner.clone().into()), 1, MarketplaceType::Private)
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().kind, MarketplaceType::Private);
    }

    set_name {
        let owner: T::AccountId = account("owner", 0, 0);
        T::CurrencyCaps::make_free_balance_be(&owner, BalanceCaps::<T>::max_value());
        drop(Marketplace::<T>::create(RawOrigin::Signed(owner.clone()).into(), MarketplaceType::Public, 0, "".into()));

        let new_name: Vec<u8> = "What is love baby dont hurt me".into();
    }: _(RawOrigin::Signed(owner.clone().into()), 1, new_name.clone())
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().name, new_name);
    }
}

impl_benchmark_test_suite!(
    Marketplace,
    crate::tests::mock::new_test_ext(),
    crate::tests::mock::Test
);
