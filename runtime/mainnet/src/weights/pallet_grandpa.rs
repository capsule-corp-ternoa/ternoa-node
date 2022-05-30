
//! Autogenerated weights for `pallet_grandpa`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("mainnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ternoa
// benchmark
// --chain
// mainnet-dev
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/
// --pallet=pallet_grandpa

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_grandpa`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_grandpa::WeightInfo for WeightInfo<T> {
	fn check_equivocation_proof(x: u32, ) -> Weight {
		(314_795_000 as Weight)
			// Standard Error: 12_390_000
			.saturating_add((32_697_000 as Weight).saturating_mul(x as Weight))
	}
	// Storage: Grandpa Stalled (r:0 w:1)
	fn note_stalled() -> Weight {
		(3_366_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
