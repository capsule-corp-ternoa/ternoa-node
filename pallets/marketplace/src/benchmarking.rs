#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Marketplace;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_support::{assert_ok, traits::Currency};
use frame_system::RawOrigin;
use sp_runtime::traits::{Bounded, StaticLookup};
use sp_std::prelude::*;
use ternoa_common::traits::NFTTrait;

const SERIES_ID: u8 = 20;

pub fn prepare_benchmarks<T: Config>() -> (MarketplaceId, MarketplaceId, NFTId) {
	let alice: T::AccountId = get_account::<T>("ALICE");
	let bob: T::AccountId = get_account::<T>("BOB");

	// Give them enough caps
	T::Currency::make_free_balance_be(&alice, BalanceOf::<T>::max_value());
	T::Currency::make_free_balance_be(&bob, BalanceOf::<T>::max_value());

	// Create default NFT and series
	let series_id = vec![SERIES_ID];
	let nft_id = T::NFTs::create_nft(alice.clone(), vec![1], Some(series_id.clone())).unwrap();

	// Lock series
	T::NFTs::benchmark_lock_series(series_id.clone());

	// Create Public Marketplace for Alice
	assert_ok!(Marketplace::<T>::create(
		get_origin::<T>("ALICE").into(),
		MarketplaceType::Public,
		0,
		vec![50],
		None,
		None,
		None,
	));
	let public_id = Marketplace::<T>::marketplace_id_generator();

	// Create Private Marketplace for Alice
	assert_ok!(Marketplace::<T>::create(
		get_origin::<T>("ALICE").into(),
		MarketplaceType::Private,
		0,
		vec![51],
		None,
		None,
		None,
	));
	let private_id = Marketplace::<T>::marketplace_id_generator();

	(public_id, private_id, nft_id)
}

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
	RawOrigin::Signed(get_account::<T>(name))
}

benchmarks! {
	list {
		let (mkp_id, _, nft_id) = prepare_benchmarks::<T>();

		let alice: T::AccountId = get_account::<T>("ALICE");
		let price: BalanceOf<T> = 100u32.into();

	}: _(RawOrigin::Signed(alice.clone()), nft_id, price, Some(mkp_id))
	verify {
		assert_eq!(T::NFTs::owner(nft_id), Some(alice));
		assert_eq!(NFTsForSale::<T>::contains_key(nft_id), true);
	}

	unlist {
		let (mkp_id, _, nft_id) = prepare_benchmarks::<T>();

		let alice = get_origin::<T>("ALICE");
		let price: BalanceOf<T> = 100u32.into();
		drop(Marketplace::<T>::list(alice.clone().into(), nft_id, price, Some(mkp_id)));

	}: _(alice.clone(), nft_id)
	verify {
		assert_eq!(NFTsForSale::<T>::contains_key(nft_id), false);
	}

	buy {
		let (mkp_id, _, nft_id) = prepare_benchmarks::<T>();

		let bob: T::AccountId = get_account::<T>("BOB");
		let price: BalanceOf<T> = 0u32.into();

		drop(Marketplace::<T>::list(get_origin::<T>("ALICE").into(), nft_id, price, Some(mkp_id)));
	}: _(RawOrigin::Signed(bob.clone().into()), nft_id)
	verify {
		assert_eq!(T::NFTs::owner(nft_id), Some(bob));
		assert_eq!(NFTsForSale::<T>::contains_key(nft_id), false);
	}

	create {
		prepare_benchmarks::<T>();

		let alice: T::AccountId = get_account::<T>("ALICE");
		let mkp_id = Marketplace::<T>::marketplace_id_generator() + 1;
	}: _(RawOrigin::Signed(alice.clone().into()), MarketplaceType::Public, 0, "Hop".into(), None, None, None)
	verify {
		assert_eq!(Marketplaces::<T>::contains_key(mkp_id), true);
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().owner, alice);
		assert_eq!(MarketplaceIdGenerator::<T>::get(), mkp_id);
	}

	add_account_to_allow_list {
		let (_, mkp_id, _) = prepare_benchmarks::<T>();

		let bob: T::AccountId = get_account::<T>("BOB");
		let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

	}: _(get_origin::<T>("ALICE"), mkp_id, bob_lookup.into())
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().allow_list, vec![bob]);
	}

	remove_account_from_allow_list {
		let (_, mkp_id, _) = prepare_benchmarks::<T>();

		let alice = get_origin::<T>("ALICE");
		let bob: T::AccountId = get_account::<T>("BOB");
		let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());
		drop(Marketplace::<T>::add_account_to_allow_list(alice.clone().into(), mkp_id, bob_lookup.clone()));

	}: _(alice.clone(), mkp_id, bob_lookup)
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().allow_list, vec![]);
	}

	set_owner {
		let (mkp_id, ..) = prepare_benchmarks::<T>();

		let bob: T::AccountId = get_account::<T>("BOB");
		let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

	}: _(get_origin::<T>("ALICE"), mkp_id, bob_lookup)
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().owner, bob);
	}

	set_market_type {
		let (mkp_id, ..) = prepare_benchmarks::<T>();

	}: _(get_origin::<T>("ALICE"), mkp_id, MarketplaceType::Private)
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().kind, MarketplaceType::Private);
	}

	set_name {
		let (mkp_id, ..) = prepare_benchmarks::<T>();

		let new_name: Vec<u8> = "poH".into();
	}: _(get_origin::<T>("ALICE"), mkp_id, new_name.clone())
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().name, new_name);
	}

	set_marketplace_mint_fee {
		prepare_benchmarks::<T>();

		let old_mint_fee = Marketplace::<T>::marketplace_mint_fee();
		let new_mint_fee = 1000u32;

	}: _(RawOrigin::Root, new_mint_fee.clone().into())
	verify {
		assert_ne!(old_mint_fee, new_mint_fee.clone().into());
		assert_eq!(Marketplace::<T>::marketplace_mint_fee(), new_mint_fee.into());
	}

	set_commission_fee {
		let (mkp_id, ..) = prepare_benchmarks::<T>();

		let commission_fee = 67;
	}: _(get_origin::<T>("ALICE"), mkp_id, commission_fee)
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().commission_fee, commission_fee);
	}

	set_uri {
		let (mkp_id, ..) = prepare_benchmarks::<T>();

		let uri: TextFormat = "test".as_bytes().to_vec();
	}: _(get_origin::<T>("ALICE"), mkp_id, uri.clone())
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().uri, Some(uri));
	}

	set_logo_uri {
		let (mkp_id, ..) = prepare_benchmarks::<T>();

		let uri: TextFormat = "test".as_bytes().to_vec();
	}: _(get_origin::<T>("ALICE"), mkp_id, uri.clone())
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().logo_uri, Some(uri));
	}

	add_account_to_disallow_list {
		let (mkp_id, ..) = prepare_benchmarks::<T>();

		let bob: T::AccountId = get_account::<T>("BOB");
		let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

	}: _(get_origin::<T>("ALICE"), mkp_id, bob_lookup.into())
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().disallow_list, vec![bob]);
	}

	remove_account_from_disallow_list {
		let (mkp_id, ..) = prepare_benchmarks::<T>();

		let alice = get_origin::<T>("ALICE");
		let bob: T::AccountId = get_account::<T>("BOB");
		let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

		drop(Marketplace::<T>::add_account_to_disallow_list(alice.clone().into(), mkp_id, bob_lookup.clone()));

	}: _(alice.clone(), 1, bob_lookup.into())
	verify {
		assert_eq!(Marketplaces::<T>::get(mkp_id).unwrap().disallow_list, vec![]);
	}
}

impl_benchmark_test_suite!(
	Marketplace,
	crate::tests::mock::new_test_ext(),
	crate::tests::mock::Test
);
