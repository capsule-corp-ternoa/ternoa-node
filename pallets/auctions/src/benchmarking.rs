#![cfg(feature = "runtime-benchmarks")]
use super::*;
use crate::Pallet as TernoaAuctions;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_support::assert_ok;
use frame_support::traits::{Currency, OnFinalize, OnInitialize};
use frame_system::RawOrigin;
use sp_runtime::traits::{Bounded, One};
use sp_std::prelude::*;
use ternoa_common::traits::{MarketplaceTrait, NFTTrait};
use ternoa_primitives::{
    marketplace::{MarketplaceId, MarketplaceType},
    nfts::NFTId,
};

const SERIES_ID: u8 = 20;
const AUCTION_BUFFER: u32 = 600;

pub fn prepare_benchmarks<T: Config>() -> (NFTId, MarketplaceId) {
    let alice: T::AccountId = get_account::<T>("ALICE");
    let bob: T::AccountId = get_account::<T>("BOB");
    let charlie: T::AccountId = get_account::<T>("CHARLIE");

    // Give them enough caps
    T::Currency::make_free_balance_be(&alice, BalanceOf::<T>::max_value() / 10u32.into()); // to avoid overflow for auction owner
    T::Currency::make_free_balance_be(&bob, BalanceOf::<T>::max_value());
    T::Currency::make_free_balance_be(&charlie, BalanceOf::<T>::max_value());

    // Create default NFT and series
    let series_id = vec![SERIES_ID];
    let nft_id =
        T::NFTHandler::create_nft(alice.clone(), vec![10], Some(series_id.clone())).unwrap();

    assert_ok!(T::MarketplaceHandler::create(
        alice.clone(),
        MarketplaceType::Public,
        10,
        vec![1],
        None,
        None,
        None,
    ));

    (nft_id, 1)
}

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    let account: T::AccountId = benchmark_account(name, 0, 0);
    account
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

pub fn get_start_price<T: Config>() -> BalanceOf<T> {
    BalanceOf::<T>::max_value() / 100u32.into()
}

pub fn get_bid_price<T: Config>(add: u32) -> BalanceOf<T> {
    get_start_price::<T>() + add.into()
}

pub fn run_to_block<T: Config>(n: T::BlockNumber) {
    while frame_system::Pallet::<T>::block_number() < n {
        crate::Pallet::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
        frame_system::Pallet::<T>::on_finalize(frame_system::Pallet::<T>::block_number());
        frame_system::Pallet::<T>::set_block_number(
            frame_system::Pallet::<T>::block_number() + One::one(),
        );
        frame_system::Pallet::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
        crate::Pallet::<T>::on_initialize(frame_system::Pallet::<T>::block_number());
    }
}

benchmarks! {
    create_auction {
        let (nft_id, mkp_id) = prepare_benchmarks::<T>();

        let alice: T::AccountId = get_account::<T>("ALICE");
        let start_block = 610u32.into();
        let end_block = start_block + 14401u32.into();
    }: _(RawOrigin::Signed(alice.clone()), nft_id, mkp_id, start_block, end_block, get_start_price::<T>(), None)
    verify {
        assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(true));
    }

    cancel_auction {
        let (nft_id, mkp_id) = prepare_benchmarks::<T>();
        let alice: T::AccountId = get_account::<T>("ALICE");
        let start_block = 610u32.into();
        let end_block = start_block + 14401u32.into();
        TernoaAuctions::<T>::create_auction(get_origin::<T>("ALICE").into(), nft_id, mkp_id, start_block, end_block, get_start_price::<T>(), None)?;
    }: _(RawOrigin::Signed(alice.clone()), nft_id)
    verify {
        assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(false));
    }

    add_bid {
        let (nft_id, mkp_id) = prepare_benchmarks::<T>();
        let alice: T::AccountId = get_account::<T>("ALICE");
        let bob: T::AccountId = get_account::<T>("BOB");
        let start_block = 610u32.into();
        let end_block = start_block + 14401u32.into();
        TernoaAuctions::<T>::create_auction(get_origin::<T>("ALICE").into(), nft_id, mkp_id, start_block, end_block, get_start_price::<T>(), None)?;
        run_to_block::<T>(start_block + One::one());
    }: _(RawOrigin::Signed(bob.clone()), nft_id, get_bid_price::<T>(100u32))
    verify {
        assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(true));
    }

    remove_bid {
        let (nft_id, mkp_id) = prepare_benchmarks::<T>();
        let alice: T::AccountId = get_account::<T>("ALICE");
        let bob: T::AccountId = get_account::<T>("BOB");
        let start_block = frame_system::Pallet::<T>::block_number() + AUCTION_BUFFER.into();
        let end_block = start_block + 14401u32.into();
        TernoaAuctions::<T>::create_auction(get_origin::<T>("ALICE").into(), nft_id, mkp_id, start_block, end_block, get_start_price::<T>(), None)?;
        run_to_block::<T>(start_block + 1u32.into());
        TernoaAuctions::<T>::add_bid(get_origin::<T>("BOB").into(), nft_id, get_bid_price::<T>(101u32))?;
    }: _(RawOrigin::Signed(bob.clone()), nft_id)
    verify {
        assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(true));
    }

    increase_bid {
        let (nft_id, mkp_id) = prepare_benchmarks::<T>();
        let alice: T::AccountId = get_account::<T>("ALICE");
        let bob: T::AccountId = get_account::<T>("BOB");
        let start_block = frame_system::Pallet::<T>::block_number() + AUCTION_BUFFER.into();
        let end_block = start_block + 14401u32.into();
        TernoaAuctions::<T>::create_auction(get_origin::<T>("ALICE").into(), nft_id, mkp_id, start_block, end_block, get_start_price::<T>(), None)?;
        run_to_block::<T>(start_block + 1u32.into());
        TernoaAuctions::<T>::add_bid(get_origin::<T>("BOB").into(), nft_id, get_bid_price::<T>(101u32))?;
    }: _(RawOrigin::Signed(bob.clone()), nft_id, get_bid_price::<T>(102u32))
    verify {
        assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(true));
    }

    buy_it_now {
        let (nft_id, mkp_id) = prepare_benchmarks::<T>();
        let alice: T::AccountId = get_account::<T>("ALICE");
        let bob: T::AccountId = get_account::<T>("BOB");
        let start_block = frame_system::Pallet::<T>::block_number() + AUCTION_BUFFER.into();
        let end_block = start_block + 14401u32.into();
        TernoaAuctions::<T>::create_auction(get_origin::<T>("ALICE").into(), nft_id, mkp_id, start_block, end_block, get_start_price::<T>(), Some(get_bid_price::<T>(200u32)))?;
        run_to_block::<T>(start_block + 1u32.into());
    }: _(RawOrigin::Signed(bob.clone()), nft_id, get_bid_price::<T>(200u32))
    verify {
        assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(false));
    }

    complete_auction {
        let (nft_id, mkp_id) = prepare_benchmarks::<T>();
        let alice: T::AccountId = get_account::<T>("ALICE");
        let bob: T::AccountId = get_account::<T>("BOB");
        let start_block = frame_system::Pallet::<T>::block_number() + AUCTION_BUFFER.into();
        let end_block = start_block + 14401u32.into();
        TernoaAuctions::<T>::create_auction(get_origin::<T>("ALICE").into(), nft_id, mkp_id, start_block, end_block, get_start_price::<T>(), None)?;
        run_to_block::<T>(start_block + 1u32.into());
        TernoaAuctions::<T>::add_bid(get_origin::<T>("BOB").into(), nft_id, get_bid_price::<T>(101u32))?;
        run_to_block::<T>(end_block + 1u32.into());
    }: _(RawOrigin::Root, nft_id)
    verify {
        assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(false));
    }

    claim_bid {
        let (nft_id, mkp_id) = prepare_benchmarks::<T>();
        let alice: T::AccountId = get_account::<T>("ALICE");
        let bob: T::AccountId = get_account::<T>("BOB");
        let start_block = frame_system::Pallet::<T>::block_number() + AUCTION_BUFFER.into();
        let end_block = start_block + 14401u32.into();
        TernoaAuctions::<T>::create_auction(get_origin::<T>("ALICE").into(), nft_id, mkp_id, start_block, end_block, get_start_price::<T>(), Some(get_bid_price::<T>(200u32)))?;
        run_to_block::<T>(start_block + 1u32.into());
        TernoaAuctions::<T>::add_bid(get_origin::<T>("BOB").into(), nft_id, get_bid_price::<T>(101u32))?;
        TernoaAuctions::<T>::buy_it_now(get_origin::<T>("CHARLIE").into(), nft_id, get_bid_price::<T>(200u32))?;
    }: _(RawOrigin::Signed(bob.clone()), nft_id)
    verify {
        assert_eq!(T::NFTHandler::is_listed_for_sale(nft_id), Some(false));
    }
}

impl_benchmark_test_suite!(
    TernoaAuctions,
    crate::mock::new_test_ext(),
    crate::mock::Test
);
