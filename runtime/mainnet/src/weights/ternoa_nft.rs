
//! Autogenerated weights for `ternoa_nft`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Ternoa-Recommended-Reference-Machine`, CPU: `AMD EPYC 7281 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("mainnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/ternoa
// benchmark
// pallet
// --chain=mainnet-dev
// --steps=50
// --repeat=20
// --pallet=ternoa_nft
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

/// Weight functions for `ternoa_nft`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ternoa_nft::WeightInfo for WeightInfo<T> {
	// Storage: NFT NftMintFee (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: NFT Collections (r:1 w:1)
	// Storage: NFT NextNFTId (r:1 w:1)
	// Storage: NFT Nfts (r:0 w:1)
	/// The range of component `s` is `[0, 999999]`.
	fn create_nft(s: u32, ) -> Weight {
		Weight::from_ref_time(115_467_000 as u64)
			// Standard Error: 74
			.saturating_add(Weight::from_ref_time(18_063 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: NFT Collections (r:1 w:1)
	/// The range of component `s` is `[0, 999999]`.
	fn burn_nft(s: u32, ) -> Weight {
		Weight::from_ref_time(83_528_000 as u64)
			// Standard Error: 89
			.saturating_add(Weight::from_ref_time(19_906 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: NFT Nfts (r:1 w:1)
	fn transfer_nft() -> Weight {
		Weight::from_ref_time(83_436_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: NFT DelegatedNFTs (r:0 w:1)
	fn delegate_nft() -> Weight {
		Weight::from_ref_time(87_534_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: NFT Nfts (r:1 w:1)
	fn set_royalty() -> Weight {
		Weight::from_ref_time(91_223_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NFT NftMintFee (r:0 w:1)
	fn set_nft_mint_fee() -> Weight {
		Weight::from_ref_time(65_472_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NFT NextCollectionId (r:1 w:1)
	// Storage: NFT Collections (r:0 w:1)
	fn create_collection() -> Weight {
		Weight::from_ref_time(130_164_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: NFT Collections (r:1 w:1)
	fn burn_collection() -> Weight {
		Weight::from_ref_time(101_370_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NFT Collections (r:1 w:1)
	fn close_collection() -> Weight {
		Weight::from_ref_time(70_933_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NFT Collections (r:1 w:1)
	fn limit_collection() -> Weight {
		Weight::from_ref_time(87_805_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NFT Collections (r:1 w:1)
	// Storage: NFT Nfts (r:1 w:1)
	/// The range of component `s` is `[0, 999999]`.
	fn add_nft_to_collection(s: u32, ) -> Weight {
		Weight::from_ref_time(61_635_000 as u64)
			// Standard Error: 58
			.saturating_add(Weight::from_ref_time(17_429 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: NFT NftMintFee (r:1 w:0)
	// Storage: NFT SecretNftMintFee (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: NFT Collections (r:1 w:1)
	// Storage: NFT NextNFTId (r:1 w:1)
	// Storage: NFT SecretNftsOffchainData (r:0 w:1)
	// Storage: NFT Nfts (r:0 w:1)
	/// The range of component `s` is `[0, 999999]`.
	fn create_secret_nft(s: u32, ) -> Weight {
		Weight::from_ref_time(267_193_000 as u64)
			// Standard Error: 60
			.saturating_add(Weight::from_ref_time(17_859 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: NFT SecretNftMintFee (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: NFT SecretNftsOffchainData (r:0 w:1)
	fn add_secret() -> Weight {
		Weight::from_ref_time(209_585_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: TEE EnclaveAccountOperator (r:1 w:0)
	// Storage: TEE EnclaveClusterId (r:1 w:0)
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: NFT SecretNftsShardsCount (r:1 w:1)
	fn add_secret_shard() -> Weight {
		Weight::from_ref_time(147_668_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: NFT SecretNftMintFee (r:0 w:1)
	fn set_secret_nft_mint_fee() -> Weight {
		Weight::from_ref_time(38_874_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: NFT CapsuleMintFee (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: NFT CapsuleOffchainData (r:0 w:1)
	fn convert_to_capsule() -> Weight {
		Weight::from_ref_time(185_901_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: NFT NftMintFee (r:1 w:0)
	// Storage: NFT CapsuleMintFee (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: NFT Collections (r:1 w:1)
	// Storage: NFT NextNFTId (r:1 w:1)
	// Storage: NFT CapsuleOffchainData (r:0 w:1)
	// Storage: NFT Nfts (r:0 w:1)
	/// The range of component `s` is `[0, 999999]`.
	fn create_capsule(s: u32, ) -> Weight {
		Weight::from_ref_time(287_011_000 as u64)
			// Standard Error: 81
			.saturating_add(Weight::from_ref_time(18_083 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: NFT CapsuleOffchainData (r:0 w:1)
	fn set_capsule_offchaindata() -> Weight {
		Weight::from_ref_time(118_724_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: NFT CapsuleMintFee (r:0 w:1)
	fn set_capsule_mint_fee() -> Weight {
		Weight::from_ref_time(53_340_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TEE EnclaveAccountOperator (r:1 w:0)
	// Storage: TEE EnclaveClusterId (r:1 w:0)
	// Storage: NFT Nfts (r:1 w:1)
	// Storage: NFT CapsulesShardsCount (r:1 w:1)
	fn add_capsule_shard() -> Weight {
		Weight::from_ref_time(159_811_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: NFT Nfts (r:1 w:1)
	fn notify_enclave_key_update() -> Weight {
		Weight::from_ref_time(140_375_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
