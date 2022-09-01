
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
// --pallet=ternoa_nft

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `ternoa_nft`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ternoa_nft::WeightInfo for WeightInfo<T> {
	// Storage: NFT NftMintFee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: NFT Collections (r:1 w:1)
	// Storage: NFT NextNFTId (r:1 w:1)
	// Storage: NFT Nfts (r:0 w:1)
	/// The range of component `s` is `[0, 999999]`.
	fn create_nft(s: u32, ) -> Weight {
		Weight::from_ref_time(5_693_566_000 as RefTimeWeight)
			// Standard Error: 20_000
			.saturating_add(Weight::from_ref_time(43_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(4 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: NFT Collections (r:1 w:1)
	/// The range of component `s` is `[0, 999999]`.
	fn burn_nft(s: u32, ) -> Weight {
		Weight::from_ref_time(4_155_510_000 as RefTimeWeight)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(18_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: NFT Nfts (r:1 w:1)
	fn transfer_nft() -> Weight {
		Weight::from_ref_time(28_630_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: NFT DelegatedNFTs (r:0 w:1)
	fn delegate_nft() -> Weight {
		Weight::from_ref_time(29_721_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: NFT Nfts (r:1 w:1)
	fn set_royalty() -> Weight {
		Weight::from_ref_time(27_190_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: NFT NftMintFee (r:0 w:1)
	fn set_nft_mint_fee() -> Weight {
		Weight::from_ref_time(20_700_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: NFT NextCollectionId (r:1 w:1)
	// Storage: NFT Collections (r:0 w:1)
	fn create_collection() -> Weight {
		Weight::from_ref_time(29_551_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: NFT Collections (r:1 w:1)
	fn burn_collection() -> Weight {
		Weight::from_ref_time(29_080_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: NFT Collections (r:1 w:1)
	fn close_collection() -> Weight {
		Weight::from_ref_time(27_400_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: NFT Collections (r:1 w:1)
	fn limit_collection() -> Weight {
		Weight::from_ref_time(27_590_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: NFT Collections (r:1 w:1)
	// Storage: NFT Nfts (r:1 w:1)
	/// The range of component `s` is `[0, 999999]`.
	fn add_nft_to_collection(s: u32, ) -> Weight {
		Weight::from_ref_time(4_650_623_000 as RefTimeWeight)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(17_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
}
