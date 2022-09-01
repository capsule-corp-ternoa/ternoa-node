
//! Autogenerated weights for `ternoa_bridge`
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
// --pallet=ternoa_bridge

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
		Weight::one()
	}
	// Storage: Bridge ChainNonces (r:1 w:1)
	fn add_chain() -> Weight {
		Weight::one()
	}
	// Storage: Bridge Relayers (r:0 w:1)
	fn set_relayers() -> Weight {
		Weight::one()
	}
	// Storage: Bridge Relayers (r:1 w:0)
	// Storage: Bridge ChainNonces (r:1 w:0)
	// Storage: Bridge RelayerVoteThreshold (r:1 w:0)
	// Storage: Bridge Votes (r:1 w:1)
	fn vote_for_proposal() -> Weight {
		Weight::one()
	}
	// Storage: Bridge BridgeFee (r:1 w:0)
	// Storage: Bridge ChainNonces (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn deposit() -> Weight {
		Weight::one()
	}
	// Storage: Bridge BridgeFee (r:0 w:1)
	fn set_bridge_fee() -> Weight {
		Weight::one()
	}
	// Storage: Bridge ChainNonces (r:1 w:1)
	fn set_deposit_nonce() -> Weight {
		Weight::one()
	}
}
