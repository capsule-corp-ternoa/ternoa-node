
//! Autogenerated weights for `frame_benchmarking::baseline`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-18, STEPS: `2`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `DESKTOP-PAL18UV`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// D:\TernoaCode\ternoa-node\target\production\ternoa.exe
// benchmark
// pallet
// --chain=alphanet-dev
// --steps=2
// --repeat=1
// --pallet=frame_benchmarking::baseline
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

/// Weight functions for `frame_benchmarking::baseline`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_benchmarking::baseline::WeightInfo for WeightInfo<T> {
	/// The range of component `i` is `[0, 1000000]`.
	fn addition(_i: u32, ) -> Weight {
		Weight::from_ref_time(700_000 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn subtraction(_i: u32, ) -> Weight {
		Weight::from_ref_time(800_000 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn multiplication(_i: u32, ) -> Weight {
		Weight::from_ref_time(1_200_000 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn division(_i: u32, ) -> Weight {
		Weight::from_ref_time(700_000 as u64)
	}
	/// The range of component `i` is `[0, 100]`.
	fn hashing(_i: u32, ) -> Weight {
		Weight::from_ref_time(29_543_600_000 as u64)
	}
	/// The range of component `i` is `[1, 100]`.
	fn sr25519_verification(_i: u32, ) -> Weight {
		Weight::from_ref_time(4_632_700_000 as u64)
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_read(_i: u32, ) -> Weight {
		Weight::from_ref_time(4_213_600_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1000 as u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_write(_i: u32, ) -> Weight {
		Weight::from_ref_time(460_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1000 as u64))
	}
}