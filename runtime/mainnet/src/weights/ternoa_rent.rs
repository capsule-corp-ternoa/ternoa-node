
//! Autogenerated weights for `ternoa_rent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Ternoa-Recommended-Reference-Machine`, CPU: `AMD EPYC 7281 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/ternoa
// benchmark
// pallet
// --chain=alphanet-dev
// --steps=50
// --repeat=20
// --pallet=ternoa_rent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// ./output

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `ternoa_rent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ternoa_rent::WeightInfo for WeightInfo<T> {
	// Storage: Rent Queues (r:1 w:1)
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Rent Contracts (r:0 w:1)
	/// The range of component `s` is `[0, 1000000]`.
	fn create_contract(s: u32, ) -> Weight {
		Weight::from_ref_time(96_762_000 as u64)
			// Standard Error: 97
			.saturating_add(Weight::from_ref_time(35_803 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Rent Contracts (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Rent Queues (r:1 w:1)
	// Storage: NFT Nfts (r:1 w:1)
	/// The range of component `s` is `[0, 1000000]`.
	fn revoke_contract(s: u32, ) -> Weight {
		Weight::from_ref_time(177_685_000 as u64)
			// Standard Error: 58
			.saturating_add(Weight::from_ref_time(29_833 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Rent Contracts (r:1 w:1)
	// Storage: Rent Queues (r:1 w:1)
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Rent Offers (r:0 w:1)
	/// The range of component `s` is `[0, 1000000]`.
	fn cancel_contract(s: u32, ) -> Weight {
		Weight::from_ref_time(117_100_000 as u64)
			// Standard Error: 81
			.saturating_add(Weight::from_ref_time(33_946 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Rent Contracts (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Rent Queues (r:1 w:1)
	// Storage: Rent Offers (r:0 w:1)
	/// The range of component `s` is `[0, 1000000]`.
	fn rent(s: u32, ) -> Weight {
		Weight::from_ref_time(135_855_000 as u64)
			// Standard Error: 80
			.saturating_add(Weight::from_ref_time(34_145 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Rent Contracts (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Rent Offers (r:1 w:1)
	/// The range of component `s` is `[0, 1000000]`.
	fn make_rent_offer(s: u32, ) -> Weight {
		Weight::from_ref_time(76_494_000 as u64)
			// Standard Error: 3
			.saturating_add(Weight::from_ref_time(130 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Rent Contracts (r:1 w:1)
	// Storage: Rent Offers (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Rent Queues (r:1 w:1)
	/// The range of component `s` is `[0, 1000000]`.
	fn accept_rent_offer(s: u32, ) -> Weight {
		Weight::from_ref_time(100_100_000 as u64)
			// Standard Error: 64
			.saturating_add(Weight::from_ref_time(33_343 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Rent Offers (r:1 w:1)
	/// The range of component `s` is `[0, 10000]`.
	fn retract_rent_offer(s: u32, ) -> Weight {
		Weight::from_ref_time(56_746_000 as u64)
			// Standard Error: 433
			.saturating_add(Weight::from_ref_time(72_355 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Rent Contracts (r:1 w:1)
	// Storage: Rent Offers (r:0 w:1)
	/// The range of component `s` is `[0, 1000000]`.
	fn change_subscription_terms(s: u32, ) -> Weight {
		Weight::from_ref_time(56_276_000 as u64)
			// Standard Error: 2
			.saturating_add(Weight::from_ref_time(108 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Rent Contracts (r:1 w:1)
	/// The range of component `s` is `[0, 1000000]`.
	fn accept_subscription_terms(s: u32, ) -> Weight {
		Weight::from_ref_time(46_427_000 as u64)
			// Standard Error: 2
			.saturating_add(Weight::from_ref_time(116 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
