#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_support::assert_ok;
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_runtime::traits::{Bounded, StaticLookup};
use sp_std::prelude::*;

use crate::Pallet as NFTs;

const SERIES_ID: u8 = 20;
const NFT_ID: u32 = 0;

pub fn prepare_benchmarks<T: Config>() {
	let alice: T::AccountId = get_account::<T>("ALICE");
	let bob: T::AccountId = get_account::<T>("BOB");

	// Give them enough caps
	T::Currency::make_free_balance_be(&alice, BalanceOf::<T>::max_value());
	T::Currency::make_free_balance_be(&bob, BalanceOf::<T>::max_value());

	// Create default NFT and series
	let series_id = vec![SERIES_ID];
	assert_ok!(NFTs::<T>::create(
		RawOrigin::Signed(alice.clone()).into(),
		vec![1],
		Some(series_id.clone()),
	));
}

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

pub fn origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
	RawOrigin::Signed(get_account::<T>(name))
}

benchmarks! {
	create {
		prepare_benchmarks::<T>();
		let alice: T::AccountId = get_account::<T>("ALICE");
		let nft_id = NFTs::<T>::nft_id_generator();

	}: _(RawOrigin::Signed(alice.clone()), vec![55], None)
	verify {
		assert_eq!(NFTs::<T>::data(nft_id).unwrap().owner, alice);
	}

	transfer {
		prepare_benchmarks::<T>();

		let alice = origin::<T>("ALICE");
		let bob: T::AccountId = get_account::<T>("BOB");
		let bob_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(bob.clone());

		assert_ok!(NFTs::<T>::finish_series(alice.clone().into(), vec![SERIES_ID]));
	}: _(alice.clone(), NFT_ID, bob_lookup)
	verify {
		assert_eq!(NFTs::<T>::data(NFT_ID).unwrap().owner, bob);
	}

	burn {
		prepare_benchmarks::<T>();

	}: _(origin::<T>("ALICE"), NFT_ID)
	verify {
		assert_eq!(NFTs::<T>::data(NFT_ID), None);
	}

	finish_series {
		prepare_benchmarks::<T>();

		let series_id: Vec<u8> = vec![SERIES_ID];

	}: _(origin::<T>("ALICE"), series_id.clone())
	verify {
		assert_eq!(NFTs::<T>::series(&series_id).unwrap().draft, false);
	}

	set_nft_mint_fee {
		prepare_benchmarks::<T>();

		let old_mint_fee = NFTs::<T>::nft_mint_fee();
		let new_mint_fee = 1000u32;

	}: _(RawOrigin::Root, new_mint_fee.clone().into())
	verify {
		assert_ne!(old_mint_fee, new_mint_fee.clone().into());
		assert_eq!(NFTs::<T>::nft_mint_fee(), new_mint_fee.into());
	}

	lend {
		prepare_benchmarks::<T>();

		let bob: T::AccountId = get_account::<T>("BOB");

	}: _(origin::<T>("ALICE"), NFT_ID, Some(bob.clone()))
	verify {
		assert_eq!(NFTs::<T>::data(NFT_ID).unwrap().viewer, Some(bob));
	}
}

impl_benchmark_test_suite!(NFTs, crate::tests::mock::new_test_ext(), crate::tests::mock::Test);
