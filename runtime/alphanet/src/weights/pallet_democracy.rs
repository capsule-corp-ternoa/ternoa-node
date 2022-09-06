
//! Autogenerated weights for `pallet_democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `marko-MS-7B85`, CPU: `AMD Ryzen 7 5800X 8-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

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
// --pallet=pallet_democracy

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		Weight::from_ref_time(53_091_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	/// The range of component `s` is `[0, 100]`.
	fn second(s: u32, ) -> Weight {
		Weight::from_ref_time(36_768_000 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(97_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn vote_new(r: u32, ) -> Weight {
		Weight::from_ref_time(47_789_000 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(150_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn vote_existing(r: u32, ) -> Weight {
		Weight::from_ref_time(47_612_000 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(154_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	fn emergency_cancel() -> Weight {
		Weight::from_ref_time(23_791_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn blacklist(p: u32, ) -> Weight {
		Weight::from_ref_time(63_366_000 as RefTimeWeight)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(271_000 as RefTimeWeight).scalar_saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(5 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(6 as RefTimeWeight))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	/// The range of component `v` is `[1, 100]`.
	fn external_propose(v: u32, ) -> Weight {
		Weight::from_ref_time(17_067_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(4_000 as RefTimeWeight).scalar_saturating_mul(v as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		Weight::from_ref_time(5_420_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		Weight::from_ref_time(5_370_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		Weight::from_ref_time(23_190_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	/// The range of component `v` is `[0, 100]`.
	fn veto_external(v: u32, ) -> Weight {
		Weight::from_ref_time(24_755_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(26_000 as RefTimeWeight).scalar_saturating_mul(v as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn cancel_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(51_650_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(245_000 as RefTimeWeight).scalar_saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		Weight::from_ref_time(16_320_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn cancel_queued(r: u32, ) -> Weight {
		Weight::from_ref_time(28_571_000 as RefTimeWeight)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(515_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	/// The range of component `r` is `[1, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		Weight::from_ref_time(9_411_000 as RefTimeWeight)
			// Standard Error: 12_000
			.saturating_add(Weight::from_ref_time(2_521_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	/// The range of component `r` is `[1, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		Weight::from_ref_time(13_171_000 as RefTimeWeight)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(2_504_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(5 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn delegate(r: u32, ) -> Weight {
		Weight::from_ref_time(50_025_000 as RefTimeWeight)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(3_719_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(4 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		Weight::from_ref_time(26_969_000 as RefTimeWeight)
			// Standard Error: 14_000
			.saturating_add(Weight::from_ref_time(3_748_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		Weight::from_ref_time(6_410_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	/// The range of component `b` is `[0, 16384]`.
	fn note_preimage(b: u32, ) -> Weight {
		Weight::from_ref_time(34_171_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as RefTimeWeight).scalar_saturating_mul(b as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	/// The range of component `b` is `[0, 16384]`.
	fn note_imminent_preimage(b: u32, ) -> Weight {
		Weight::from_ref_time(25_841_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as RefTimeWeight).scalar_saturating_mul(b as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `b` is `[0, 16384]`.
	fn reap_preimage(b: u32, ) -> Weight {
		Weight::from_ref_time(43_010_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as RefTimeWeight).scalar_saturating_mul(b as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		Weight::from_ref_time(35_589_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(104_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		Weight::from_ref_time(34_637_000 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(145_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn remove_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(20_168_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(148_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 99]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(20_355_000 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(145_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
}