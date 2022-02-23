#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{Auctions as AuctionsStorage, Claims, Pallet as TernoaAuctions};
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_support::assert_ok;
use frame_support::traits::{Currency, OnFinalize, OnInitialize};
use frame_system::pallet_prelude::OriginFor;
use frame_system::{Pallet as System, RawOrigin};
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;
use ternoa_common::traits::{MarketplaceTrait, NFTTrait};
use ternoa_primitives::marketplace::{MarketplaceId, MarketplaceType};
use ternoa_primitives::nfts::NFTId;

pub enum AuctionState {
	Before,
	InProgress,
	Extended,
}

pub struct BenchmarkData {
	pub alice_nft_id: NFTId,
	pub alice_market_id: MarketplaceId,
	pub bob_nft_id: NFTId,
}

pub fn prepare_benchmarks<T: Config>(state: Option<AuctionState>) -> BenchmarkData {
	// Get accounts
	let alice: T::AccountId = get_account::<T>("ALICE");
	let bob: T::AccountId = get_account::<T>("BOB");
	let charlie: T::AccountId = get_account::<T>("CHARLIE");
	let eve: T::AccountId = get_account::<T>("EVE");

	// Give them enough caps
	T::Currency::make_free_balance_be(&alice, BalanceOf::<T>::max_value() / 5u32.into()); // to avoid overflow for auction owner
	T::Currency::make_free_balance_be(&bob, BalanceOf::<T>::max_value() / 5u32.into());
	T::Currency::make_free_balance_be(&charlie, BalanceOf::<T>::max_value() / 5u32.into());
	T::Currency::make_free_balance_be(&eve, BalanceOf::<T>::max_value() / 5u32.into());

	// Create Alice's marketplace
	let market_id = T::MarketplaceHandler::create(
		alice.clone(),
		MarketplaceType::Public,
		10,
		vec![1],
		None,
		None,
		None,
	)
	.unwrap();

	// Create NFTs
	let alice_nft_id = T::NFTHandler::create_nft(alice.clone(), vec![10], None).unwrap();
	let bob_nft_id = T::NFTHandler::create_nft(bob.clone(), vec![10], None).unwrap();

	let alice_series = T::NFTHandler::get_nft(alice_nft_id).unwrap().series_id;
	let bob_series = T::NFTHandler::get_nft(bob_nft_id).unwrap().series_id;

	assert_ok!(T::NFTHandler::set_series_completion(&alice_series, true));
	assert_ok!(T::NFTHandler::set_series_completion(&bob_series, true));

	// Create auctions
	if let Some(state) = state {
		let (start_block, is_extended) = match state {
			AuctionState::Before => {
				(System::<T>::block_number() + T::MaxAuctionDelay::get(), false)
			},
			AuctionState::InProgress => (System::<T>::block_number(), false),
			AuctionState::Extended => (System::<T>::block_number(), true),
		};

		let end_block = start_block + T::MinAuctionDuration::get();
		let start_price = BalanceOf::<T>::max_value() / 1000u32.into();
		let buy_it_price = Some(start_price.saturating_mul(2u16.into()));

		let ok = TernoaAuctions::<T>::create_auction(
			origin::<T>("BOB"),
			bob_nft_id,
			market_id,
			start_block,
			end_block,
			start_price,
			buy_it_price,
		);
		assert_ok!(ok);

		AuctionsStorage::<T>::mutate(bob_nft_id, |x| {
			let mut x = x.as_mut().unwrap();
			x.is_extended = is_extended;
		});
	}

	BenchmarkData { alice_nft_id, alice_market_id: market_id, bob_nft_id }
}

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

pub fn origin<T: Config>(name: &'static str) -> OriginFor<T> {
	RawOrigin::Signed(get_account::<T>(name)).into()
}

#[allow(dead_code)]
pub fn run_to_block<T: Config>(n: T::BlockNumber) {
	while System::<T>::block_number() < n {
		<TernoaAuctions<T> as OnFinalize<T::BlockNumber>>::on_finalize(System::<T>::block_number());
		<System<T> as OnFinalize<T::BlockNumber>>::on_finalize(System::<T>::block_number());
		System::<T>::set_block_number(System::<T>::block_number() + 1u16.into());
		<System<T> as OnInitialize<T::BlockNumber>>::on_initialize(System::<T>::block_number());
		<TernoaAuctions<T> as OnInitialize<T::BlockNumber>>::on_initialize(
			System::<T>::block_number(),
		);
	}
}

/* pub fn prepare_create_auction<T: Config>(data: &BenchmarkData) {
	let alice: T::AccountId = get_account::<T>("ALICE");

	// Create 10 000 additional auctions
	for _i in 0..10_000 {
		let nft_id = T::NFTHandler::create_nft(alice.clone(), vec![10], None).unwrap();
		let series_id = T::NFTHandler::get_nft(nft_id).unwrap().series_id;
		assert_ok!(T::NFTHandler::set_series_completion(&series_id, true));

		let start_block = System::<T>::block_number() + T::MaxAuctionDelay::get() - 1u16.into();
		let end_block = start_block + T::MinAuctionDuration::get();
		let start_price = BalanceOf::<T>::max_value() / 100000u32.into();
		let buy_it_price = start_price.saturating_mul(2u16.into());

		let ok = TernoaAuctions::<T>::create_auction(
			origin::<T>("ALICE"),
			nft_id,
			data.alice_market_id,
			start_block,
			end_block,
			start_price,
			Some(buy_it_price),
		);
		assert_ok!(ok);
	}
} */

benchmarks! {
	create_auction {
		let bench_data = prepare_benchmarks::<T>(None);
		//prepare_create_auction::<T>(&bench_data);

		let alice: T::AccountId = get_account::<T>("ALICE");

		let nft_id = bench_data.alice_nft_id;
		let market_id = bench_data.alice_market_id;
		let start_block = System::<T>::block_number() + T::MaxAuctionDelay::get();
		let end_block = start_block + T::MinAuctionDuration::get();
		let start_price = BalanceOf::<T>::max_value() / 100u32.into();
		let buy_now_price = start_price.saturating_mul(2u16.into());

	}: _(RawOrigin::Signed(alice.clone()), nft_id, market_id, start_block, end_block, start_price, Some(buy_now_price))
	verify {
		assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(true));
	}

	 cancel_auction {
		let bench_data = prepare_benchmarks::<T>(Some(AuctionState::Before));
		let bob: T::AccountId = get_account::<T>("BOB");
		let nft_id = bench_data.bob_nft_id;

	}: _(RawOrigin::Signed(bob.clone()), nft_id)
	verify {
		assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(false));
	}

	end_auction {
		let bench_data = prepare_benchmarks::<T>(Some(AuctionState::Extended));
		let bob: T::AccountId = get_account::<T>("BOB");
		let nft_id = bench_data.bob_nft_id;

		let auction = AuctionsStorage::<T>::get(nft_id).unwrap();
		let charlie_bid = auction.buy_it_price.clone().unwrap();
		let eve_bid = charlie_bid.saturating_mul(2u16.into());

		assert_ok!(TernoaAuctions::<T>::add_bid(origin::<T>("CHARLIE"), nft_id, charlie_bid));
		assert_ok!(TernoaAuctions::<T>::add_bid(origin::<T>("EVE"), nft_id, eve_bid));

	}: _(RawOrigin::Signed(bob.clone()), nft_id)
	verify {
		let eve: T::AccountId = get_account::<T>("EVE");

		assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(false));
		assert_eq!(T::NFTHandler::owner(nft_id), Some(eve));
	}

	add_bid {
		let bench_data = prepare_benchmarks::<T>(Some(AuctionState::InProgress));
		let charlie: T::AccountId = get_account::<T>("CHARLIE");
		let nft_id = bench_data.bob_nft_id;

		let auction = AuctionsStorage::<T>::get(nft_id).unwrap();
		let charlie_bid =  auction.buy_it_price.clone().unwrap();

	}: _(RawOrigin::Signed(charlie.clone()), nft_id, charlie_bid)
	verify {
		let auction = AuctionsStorage::<T>::get(nft_id).unwrap();
		assert_eq!(auction.bidders.list, vec![(charlie, charlie_bid)]);
	}

	remove_bid {
		let bench_data = prepare_benchmarks::<T>(Some(AuctionState::InProgress));
		let charlie: T::AccountId = get_account::<T>("CHARLIE");
		let nft_id = bench_data.bob_nft_id;

		let auction = AuctionsStorage::<T>::get(nft_id).unwrap();
		let charlie_bid =  auction.buy_it_price.clone().unwrap();
		assert_ok!(TernoaAuctions::<T>::add_bid(origin::<T>("CHARLIE"), nft_id, charlie_bid));

	}: _(RawOrigin::Signed(charlie.clone()), nft_id)
	verify {
		assert_eq!(auction.bidders.list, vec![]);
	}

	buy_it_now {
		let bench_data = prepare_benchmarks::<T>(Some(AuctionState::InProgress));
		let charlie: T::AccountId = get_account::<T>("CHARLIE");
		let nft_id = bench_data.bob_nft_id;

	}: _(RawOrigin::Signed(charlie.clone()), nft_id)
	verify {
		assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(false));
		assert_eq!(T::NFTHandler::owner(nft_id), Some(charlie));
	}

	complete_auction {
		let bench_data = prepare_benchmarks::<T>(Some(AuctionState::InProgress));
		let bob: T::AccountId = get_account::<T>("BOB");
		let nft_id = bench_data.bob_nft_id;

		let auction = AuctionsStorage::<T>::get(nft_id).unwrap();
		let charlie_bid = auction.buy_it_price.clone().unwrap();
		let eve_bid = charlie_bid.saturating_mul(2u16.into());

		assert_ok!(TernoaAuctions::<T>::add_bid(origin::<T>("CHARLIE"), nft_id, charlie_bid));
		assert_ok!(TernoaAuctions::<T>::add_bid(origin::<T>("EVE"), nft_id, eve_bid));

	}: _(RawOrigin::Root, nft_id)
	verify {
		let eve: T::AccountId = get_account::<T>("EVE");

		assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(false));
		assert_eq!(T::NFTHandler::owner(nft_id), Some(eve));
	}

	claim {
		let bench_data = prepare_benchmarks::<T>(Some(AuctionState::InProgress));
		let charlie: T::AccountId = get_account::<T>("CHARLIE");
		let nft_id = bench_data.bob_nft_id;

		let auction = AuctionsStorage::<T>::get(nft_id).unwrap();
		let charlie_bid = auction.buy_it_price.clone().unwrap();
		let eve_bid = charlie_bid.saturating_mul(2u16.into());

		assert_ok!(TernoaAuctions::<T>::add_bid(origin::<T>("CHARLIE"), nft_id, charlie_bid));
		assert_ok!(TernoaAuctions::<T>::add_bid(origin::<T>("EVE"), nft_id, eve_bid));
		assert_ok!(TernoaAuctions::<T>::complete_auction(RawOrigin::Root.into(), nft_id));

	}: _(RawOrigin::Signed(charlie.clone()))
	verify {
		assert_eq!(Claims::<T>::get(charlie.clone()), None);
	}
}

impl_benchmark_test_suite!(
	TernoaAuctions,
	crate::tests::mock::new_test_ext(),
	crate::tests::mock::Test
);
