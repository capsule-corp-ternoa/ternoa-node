
//! Autogenerated weights for `ternoa_bridge`
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
// --pallet=ternoa_bridge
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

/// Weight functions for `ternoa_bridge`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ternoa_bridge::WeightInfo for WeightInfo<T> {
	// Storage: Bridge RelayerVoteThreshold (r:0 w:1)
	fn set_threshold() -> Weight {
		Weight::from_parts(34_063_000 , 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Bridge ChainNonces (r:1 w:1)
	fn add_chain() -> Weight {
		Weight::from_parts(52_018_000 , 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Bridge Relayers (r:0 w:1)
	fn set_relayers() -> Weight {
		Weight::from_parts(44_643_000 , 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Bridge Relayers (r:1 w:0)
	// Storage: Bridge ChainNonces (r:1 w:0)
	// Storage: Bridge RelayerVoteThreshold (r:1 w:0)
	// Storage: Bridge Votes (r:1 w:1)
	fn vote_for_proposal() -> Weight {
		Weight::from_parts(57_488_000 , 0)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Bridge BridgeFee (r:1 w:0)
	// Storage: Bridge ChainNonces (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn deposit() -> Weight {
		Weight::from_parts(178_516_000 , 0)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Bridge BridgeFee (r:0 w:1)
	fn set_bridge_fee() -> Weight {
		Weight::from_parts(34_004_000 , 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Bridge ChainNonces (r:1 w:1)
	fn set_deposit_nonce() -> Weight {
		Weight::from_parts(66_546_000 , 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
