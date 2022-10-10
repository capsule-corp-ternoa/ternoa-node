
//! Autogenerated weights for `ternoa_nft`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-01, STEPS: `5`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `marko-MS-7B85`, CPU: `AMD Ryzen 7 5800X 8-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ternoa
// benchmark
// pallet
// --chain
// alphanet-dev
// --steps=5
// --repeat=2
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/
// --pallet=ternoa_rent

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `ternoa_rent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ternoa_rent::WeightInfo for WeightInfo<T> {
	// Storage: Rent NumberOfCurrentContracts (r:1 w:1)
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Rent AvailableQueue (r:1 w:1)
	// Storage: Rent Contracts (r:0 w:1)
	/// The range of component `s` is `[0, 8]`.
	fn create_contract(_s: u32) -> Weight {
		Weight::from_ref_time(10_000_000 as RefTimeWeight)
	}
	// Storage: Rent Contracts (r:1 w:1)
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Rent SubscriptionQueue (r:1 w:1)
	// Storage: Rent NumberOfCurrentContracts (r:1 w:1)
	/// The range of component `s` is `[0, 9]`.
	fn revoke_contract(_s: u32) -> Weight {
		Weight::from_ref_time(10_000_000 as RefTimeWeight)
	}
	fn cancel_contract(_s: u32) -> Weight {
		Weight::from_ref_time(10_000_000 as RefTimeWeight)
	}
	// Storage: Rent Contracts (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Rent SubscriptionQueue (r:1 w:1)
	// Storage: Rent AvailableQueue (r:1 w:1)
	// Storage: Rent Offers (r:0 w:1)
	/// The range of component `s` is `[0, 8]`.
	/// The range of component `t` is `[0, 9]`.
	fn rent(_s: u32) -> Weight {
		Weight::from_ref_time(10_000_000 as RefTimeWeight)
	}
	fn make_rent_offer(_s: u32) -> Weight {
		Weight::from_ref_time(10_000_000 as RefTimeWeight)
	}
	// Storage: Rent Contracts (r:1 w:1)
	// Storage: Rent Offers (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Rent SubscriptionQueue (r:1 w:1)
	// Storage: Rent AvailableQueue (r:1 w:1)
	/// The range of component `s` is `[0, 8]`.
	/// The range of component `t` is `[0, 9]`.
	/// The range of component `u` is `[0, 2]`.
	fn accept_rent_offer(_s: u32) -> Weight {
		Weight::from_ref_time(10_000_000 as RefTimeWeight)
	}
	// Storage: Rent Contracts (r:1 w:0)
	// Storage: Rent Offers (r:1 w:1)
	/// The range of component `s` is `[0, 2]`.
	fn retract_rent_offer(_s: u32) -> Weight {
		Weight::from_ref_time(10_000_000 as RefTimeWeight)
	}
	// Storage: Rent Contracts (r:1 w:1)
	fn change_subscription_terms(_s: u32) -> Weight {
		Weight::from_ref_time(10_000_000 as RefTimeWeight)
	}
	// Storage: Rent Contracts (r:1 w:1)
	fn accept_subscription_terms(_s: u32) -> Weight {
		Weight::from_ref_time(10_000_000 as RefTimeWeight)
	}
}
