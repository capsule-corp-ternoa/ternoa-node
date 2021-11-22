
//! Autogenerated weights for `ternoa_nfts`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/debug/ternoa
// benchmark
// --chain
// dev
// --execution=wasm
// --extrinsic=*
// --pallet=ternoa_nfts
// --steps=50
// --repeat=20
// --heap-pages=4096
// --output
// .


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for ternoa_nfts.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ternoa_nfts::WeightInfo for WeightInfo<T> {
	// Storage: Nfts NftMintFee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Nfts NftIdGenerator (r:1 w:1)
	// Storage: Nfts SeriesIdGenerator (r:1 w:1)
	// Storage: Nfts Series (r:1 w:1)
	// Storage: Nfts Data (r:0 w:1)
	fn create() -> Weight {
		(1_771_738_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Nfts Series (r:1 w:0)
	// Storage: Capsules Capsules (r:1 w:0)
	fn transfer() -> Weight {
		(761_213_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Capsules Capsules (r:1 w:0)
	fn burn() -> Weight {
		(646_343_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Series (r:1 w:1)
	fn finish_series() -> Weight {
		(531_762_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts NftMintFee (r:0 w:1)
	fn set_nft_mint_fee() -> Weight {
		(400_472_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Nfts Series (r:1 w:0)
	fn set_ipfs_reference() -> Weight {
		(667_113_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
