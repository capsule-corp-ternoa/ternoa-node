#![cfg(feature = "runtime-benchmarks")]

use crate::{
    Call, Config, MarketplaceIdGenerator, MarketplaceType, Marketplaces, NFTCurrency,
    NFTCurrencyId, NFTsForSale, Pallet,
};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;
use ternoa_nfts::traits::NFTs;

use crate::Pallet as Marketplace;

benchmarks! {
    list {
        let alice: T::AccountId = frame_benchmarking::account("ALICE", 0, 0);

        let nft_id = 100;
        let price = NFTCurrency::Caps(100u32.into());
    }: _(RawOrigin::Signed(alice.clone()), nft_id, price, None)
    verify {
        assert_eq!(T::NFTs::owner(nft_id), Some(alice));
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), true);
    }

    unlist {
        let alice: T::AccountId = frame_benchmarking::account("ALICE", 0, 0);

        let nft_id = 100;
        let price = NFTCurrency::Caps(100u32.into());
        drop(Marketplace::<T>::list(RawOrigin::Signed(alice.clone()).into(), nft_id, price, None));

    }: _(RawOrigin::Signed(alice.clone().into()), nft_id)
    verify {
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), false);
    }

    buy {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let bob: T::AccountId = account("BOB", 0, 0);

        let nft_id = 100;
        let price = NFTCurrency::Caps(100u32.into());

        drop(Marketplace::<T>::list(RawOrigin::Signed(alice.clone()).into(), nft_id, price, None));
    }: _(RawOrigin::Signed(bob.clone().into()), nft_id, NFTCurrencyId::Caps)
    verify {
        assert_eq!(T::NFTs::owner(nft_id), Some(bob));
        assert_eq!(NFTsForSale::<T>::contains_key(nft_id), false);
    }

    create {
        let alice: T::AccountId = account("ALICE", 0, 0);

    }: _(RawOrigin::Signed(alice.clone().into()), MarketplaceType::Public, 0, "Hop".into())
    verify {
        assert_eq!(Marketplaces::<T>::contains_key(1), true);
        assert_eq!(Marketplaces::<T>::get(1).unwrap().owner, alice);
        assert_eq!(MarketplaceIdGenerator::<T>::get(), 1);
    }

    add_account_to_allow_list {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let bob: T::AccountId = account("BOB", 0, 0);
        let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

        drop(Marketplace::<T>::create(RawOrigin::Signed(alice.clone()).into(), MarketplaceType::Private, 0, "Hop".into()));

    }: _(RawOrigin::Signed(alice.clone().into()), 1, bob_lookup.into())
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().allow_list, vec![bob]);
    }

    remove_account_from_allow_list {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let bob: T::AccountId = account("BOB", 0, 0);
        let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

        drop(Marketplace::<T>::create(RawOrigin::Signed(alice.clone()).into(), MarketplaceType::Private, 0, "Hop".into()));
        drop(Marketplace::<T>::add_account_to_allow_list(RawOrigin::Signed(alice.clone()).into(), 1, bob_lookup.clone()));

    }: _(RawOrigin::Signed(alice.clone().into()), 1, bob_lookup)
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().allow_list, vec![]);
    }

    set_owner {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let bob: T::AccountId = account("BOB", 0, 0);
        let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

        drop(Marketplace::<T>::create(RawOrigin::Signed(alice.clone()).into(), MarketplaceType::Private, 0, "Hop".into()));

    }: _(RawOrigin::Signed(alice.clone().into()), 1, bob_lookup)
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().owner, bob);
    }

    set_market_type {
        let alice: T::AccountId = account("ALICE", 0, 0);
        drop(Marketplace::<T>::create(RawOrigin::Signed(alice.clone()).into(), MarketplaceType::Public, 0, "Hop".into()));

    }: _(RawOrigin::Signed(alice.clone().into()), 1, MarketplaceType::Private)
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().kind, MarketplaceType::Private);
    }

    set_name {
        let alice: T::AccountId = account("ALICE", 0, 0);
        drop(Marketplace::<T>::create(RawOrigin::Signed(alice.clone()).into(), MarketplaceType::Public, 0, "Hop".into()));

        let new_name: Vec<u8> = "poH".into();
    }: _(RawOrigin::Signed(alice.clone().into()), 1, new_name.clone())
    verify {
        assert_eq!(Marketplaces::<T>::get(1).unwrap().name, new_name);
    }

    set_marketplace_mint_fee {
        let old_mint_fee = Marketplace::<T>::marketplace_mint_fee();
        let new_mint_fee = 1000u32;

    }: _(RawOrigin::Root, new_mint_fee.clone().into())
    verify {
        assert_ne!(old_mint_fee, new_mint_fee.clone().into());
        assert_eq!(Marketplace::<T>::marketplace_mint_fee(), new_mint_fee.into());
    }

    set_commission_fee {
        let alice: T::AccountId = account("ALICE", 0, 0);
        let commission_fee = 15;
        let mkp_id = 1;
        drop(Marketplace::<T>::create(RawOrigin::Signed(alice.clone()).into(), MarketplaceType::Public, 0, "Hop".into()));
        assert_ne!(Marketplaces::<T>::get(mkp_id).unwrap().commission_fee, commission_fee);

    }: _(RawOrigin::Signed(alice.clone().into()), mkp_id, commission_fee)
    verify {
        assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().commission_fee, commission_fee);
    }
}

impl_benchmark_test_suite!(
    Marketplace,
    crate::tests::mock::new_test_ext(),
    crate::tests::mock::Test
);
