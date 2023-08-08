
//! Autogenerated weights for `ternoa_tee`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Ternoa-Recommended-Reference-Machine`, CPU: `AMD EPYC 7281 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("mainnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ternoa
// benchmark
// pallet
// --chain
// alphanet-dev
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/
// --pallet=ternoa_tee

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
    // Storage: TEE StakingAmount (r:1 w:0)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: TEE StakingLedger (r:0 w:1)
    fn register_enclave() -> Weight {
        Weight::from_ref_time(99_281_000 as u64)
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: TEE StakingAmount (r:1 w:0)
    // Storage: TEE EnclaveData (r:1 w:0)
    // Storage: TEE EnclaveUnregistrations (r:1 w:1)
    // Storage: TEE StakingLedger (r:0 w:1)
    fn unregister_enclave() -> Weight {
        Weight::from_ref_time(57_321_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: TEE EnclaveData (r:1 w:0)
    // Storage: TEE EnclaveUpdates (r:1 w:1)
    // Storage: TEE EnclaveAccountOperator (r:1 w:0)
    fn update_enclave() -> Weight {
        Weight::from_ref_time(52_191_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
	// Storage: TEE EnclaveUpdates (r:1 w:1)
	fn cancel_update() -> Weight {
        Weight::from_ref_time(47_110_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: TEE EnclaveRegistrations (r:1 w:1)
    // Storage: TEE ClusterData (r:1 w:1)
    // Storage: TEE EnclaveAccountOperator (r:1 w:1)
    // Storage: TEE EnclaveData (r:1 w:1)
    // Storage: TEE EnclaveClusterId (r:0 w:1)
    fn assign_enclave() -> Weight {
        Weight::from_ref_time(55_650_000 as u64)
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    // Storage: TEE EnclaveRegistrations (r:1 w:1)
    fn remove_registration() -> Weight {
        Weight::from_ref_time(32_590_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: TEE EnclaveUpdates (r:1 w:1)
    fn remove_update() -> Weight {
        Weight::from_ref_time(35_541_000 as u64)
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
        Weight::from_ref_time(162_732_000 as u64)
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(6 as u64))
    }
	// Storage: TEE EnclaveData (r:1 w:1)
    // Storage: TEE EnclaveAccountOperator (r:1 w:1)
    // Storage: TEE EnclaveUpdates (r:1 w:1)
    fn force_update_enclave() -> Weight {
        Weight::from_ref_time(124_002_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: TEE NextClusterId (r:1 w:1)
    // Storage: TEE ClusterData (r:0 w:1)
    fn create_cluster() -> Weight {
        Weight::from_ref_time(40_682_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: TEE ClusterData (r:1 w:1)
    fn update_cluster() -> Weight {
        Weight::from_ref_time(41_321_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: TEE ClusterData (r:1 w:1)
    fn remove_cluster() -> Weight {
        Weight::from_ref_time(65_551_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: TEE StakingLedger (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: TEE StakingAmount (r:1 w:0)
    fn withdraw_unbonded() -> Weight {
        Weight::from_ref_time(130_681_000 as u64)
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: TEE MetricsServers (r:1 w:1)
    fn register_metrics_server() -> Weight {
        Weight::from_ref_time(42_050_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: TEE MetricsServers (r:1 w:0)
    // Storage: TEE EnclaveData (r:1 w:0)
    // Storage: TEE MetricsReports (r:1 w:1)
    fn submit_metrics_server_report() -> Weight {
        Weight::from_ref_time(99_321_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: TEE ReportParamsWeightages (r:0 w:1)
    fn set_report_params_weightage() -> Weight {
        Weight::from_ref_time(32_340_000 as u64)
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
}

