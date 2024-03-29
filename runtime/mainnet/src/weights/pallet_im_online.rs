
//! Autogenerated weights for `pallet_im_online`
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
// --pallet=pallet_im_online
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

/// Weight functions for `pallet_im_online`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_im_online::WeightInfo for WeightInfo<T> {
	/// Storage: Session Validators (r:1 w:0)
	/// Proof Skipped: Session Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Session CurrentIndex (r:1 w:0)
	/// Proof Skipped: Session CurrentIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ImOnline Keys (r:1 w:0)
	/// Proof: ImOnline Keys (max_values: Some(1), max_size: Some(320002), added: 320497, mode: MaxEncodedLen)
	/// Storage: ImOnline ReceivedHeartbeats (r:1 w:1)
	/// Proof: ImOnline ReceivedHeartbeats (max_values: None, max_size: Some(10021032), added: 10023507, mode: MaxEncodedLen)
	/// Storage: ImOnline AuthoredBlocks (r:1 w:0)
	/// Proof: ImOnline AuthoredBlocks (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// The range of component `k` is `[1, 1000]`.
	/// The range of component `e` is `[1, 100]`.
	fn validate_unsigned_and_then_heartbeat(k: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `361 + k * (32 ±0)`
		//  Estimated: `10349676 + e * (35 ±0) + k * (64 ±0)`
		// Minimum execution time: 91_159_000 picoseconds.
		Weight::from_parts(74_157_856, 0)
			.saturating_add(Weight::from_parts(0, 10349676))
			// Standard Error: 275
			.saturating_add(Weight::from_parts(20_308, 0).saturating_mul(k.into()))
			// Standard Error: 2_777
			.saturating_add(Weight::from_parts(387_445, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 35).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(k.into()))
	}
}
