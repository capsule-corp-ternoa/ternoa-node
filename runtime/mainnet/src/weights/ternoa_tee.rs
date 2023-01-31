
//! Autogenerated weights for `ternoa_tee`
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
// --pallet=ternoa_tee
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

/// Weight functions for `ternoa_tee`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> ternoa_tee::WeightInfo for WeightInfo<T> {
	// Storage: TEE EnclaveRegistrations (r:1 w:1)
	// Storage: TEE EnclaveData (r:1 w:0)
	// Storage: TEE EnclaveAccountOperator (r:1 w:0)
	fn register_enclave() -> Weight {
		Weight::from_ref_time(98_184_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TEE EnclaveData (r:1 w:0)
	// Storage: TEE EnclaveUnregistrations (r:1 w:1)
	fn unregister_enclave() -> Weight {
		Weight::from_ref_time(91_443_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TEE EnclaveData (r:1 w:0)
	// Storage: TEE EnclaveUpdates (r:1 w:1)
	// Storage: TEE EnclaveAccountOperator (r:1 w:0)
	fn update_enclave() -> Weight {
		Weight::from_ref_time(133_793_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TEE EnclaveUpdates (r:1 w:1)
	fn cancel_update() -> Weight {
		Weight::from_ref_time(71_405_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TEE EnclaveRegistrations (r:1 w:1)
	// Storage: TEE ClusterData (r:1 w:1)
	// Storage: TEE EnclaveAccountOperator (r:1 w:1)
	// Storage: TEE EnclaveData (r:1 w:1)
	// Storage: TEE EnclaveClusterId (r:0 w:1)
	fn assign_enclave() -> Weight {
		Weight::from_ref_time(141_175_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: TEE EnclaveRegistrations (r:1 w:1)
	fn remove_registration() -> Weight {
		Weight::from_ref_time(80_362_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TEE EnclaveUpdates (r:1 w:1)
	fn remove_update() -> Weight {
		Weight::from_ref_time(151_134_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TEE EnclaveData (r:1 w:1)
	// Storage: TEE EnclaveAccountOperator (r:1 w:1)
	// Storage: TEE EnclaveClusterId (r:1 w:1)
	// Storage: TEE ClusterData (r:1 w:1)
	// Storage: TEE EnclaveUnregistrations (r:1 w:1)
	// Storage: TEE EnclaveUpdates (r:1 w:1)
	fn remove_enclave() -> Weight {
		Weight::from_ref_time(161_545_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: TEE EnclaveData (r:1 w:1)
	// Storage: TEE EnclaveAccountOperator (r:1 w:1)
	// Storage: TEE EnclaveUpdates (r:1 w:1)
	fn force_update_enclave() -> Weight {
		Weight::from_ref_time(175_701_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TEE NextClusterId (r:1 w:1)
	// Storage: TEE ClusterData (r:0 w:1)
	fn create_cluster() -> Weight {
		Weight::from_ref_time(71_855_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: TEE ClusterData (r:1 w:1)
	fn remove_cluster() -> Weight {
		Weight::from_ref_time(71_444_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
