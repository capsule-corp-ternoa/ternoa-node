//! Autogenerated weights for `pallet_democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Ternoa-Recommended-Reference-Machine`, CPU: `AMD EPYC 7281 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/ternoa
// benchmark
// pallet
// --chain=alphanet-dev
// --steps=50
// --repeat=20
// --pallet=pallet_democracy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// ./output

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
                Weight::from_ref_time(305_505_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
        }
        // Storage: Democracy DepositOf (r:1 w:1)
        /// The range of component `s` is `[0, 100]`.
        fn second(s: u32, ) -> Weight {
                Weight::from_ref_time(157_030_000 as RefTimeWeight)
                        // Standard Error: 55_000
                        .saturating_add(Weight::from_ref_time(537_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Democracy ReferendumInfoOf (r:1 w:1)
        // Storage: Democracy VotingOf (r:1 w:1)
        // Storage: Balances Locks (r:1 w:1)
        /// The range of component `r` is `[1, 99]`.
        fn vote_new(r: u32, ) -> Weight {
                Weight::from_ref_time(154_721_000 as RefTimeWeight)
                        // Standard Error: 75_000
                        .saturating_add(Weight::from_ref_time(1_243_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
        }
        // Storage: Democracy ReferendumInfoOf (r:1 w:1)
        // Storage: Democracy VotingOf (r:1 w:1)
        // Storage: Balances Locks (r:1 w:1)
        /// The range of component `r` is `[1, 99]`.
        fn vote_existing(r: u32, ) -> Weight {
                Weight::from_ref_time(170_730_000 as RefTimeWeight)
                        // Standard Error: 70_000
                        .saturating_add(Weight::from_ref_time(910_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
        }
        // Storage: Democracy ReferendumInfoOf (r:1 w:1)
        // Storage: Democracy Cancellations (r:1 w:1)
        fn emergency_cancel() -> Weight {
                Weight::from_ref_time(57_048_000 as RefTimeWeight)
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
                Weight::from_ref_time(163_844_000 as RefTimeWeight)
                        // Standard Error: 70_000
                        .saturating_add(Weight::from_ref_time(2_133_000 as RefTimeWeight).scalar_saturating_mul(p as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(5 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(6 as RefTimeWeight))
        }
        // Storage: Democracy NextExternal (r:1 w:1)
        // Storage: Democracy Blacklist (r:1 w:0)
        /// The range of component `v` is `[1, 100]`.
        fn external_propose(_v: u32, ) -> Weight {
                Weight::from_ref_time(66_781_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Democracy NextExternal (r:0 w:1)
        fn external_propose_majority() -> Weight {
                Weight::from_ref_time(17_933_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Democracy NextExternal (r:0 w:1)
        fn external_propose_default() -> Weight {
                Weight::from_ref_time(10_039_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Democracy NextExternal (r:1 w:1)
        // Storage: Democracy ReferendumCount (r:1 w:1)
        // Storage: Democracy ReferendumInfoOf (r:0 w:1)
        fn fast_track() -> Weight {
                Weight::from_ref_time(57_098_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
        }
        // Storage: Democracy NextExternal (r:1 w:1)
        // Storage: Democracy Blacklist (r:1 w:1)
        /// The range of component `v` is `[0, 100]`.
        fn veto_external(v: u32, ) -> Weight {
                Weight::from_ref_time(63_593_000 as RefTimeWeight)
                        // Standard Error: 25_000
                        .saturating_add(Weight::from_ref_time(266_000 as RefTimeWeight).scalar_saturating_mul(v as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
        }
        // Storage: Democracy PublicProps (r:1 w:1)
        // Storage: Democracy DepositOf (r:1 w:1)
        // Storage: System Account (r:1 w:1)
        /// The range of component `p` is `[1, 100]`.
        fn cancel_proposal(p: u32, ) -> Weight {
                Weight::from_ref_time(146_831_000 as RefTimeWeight)
                        // Standard Error: 64_000
                        .saturating_add(Weight::from_ref_time(1_882_000 as RefTimeWeight).scalar_saturating_mul(p as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
        }
        // Storage: Democracy ReferendumInfoOf (r:0 w:1)
        fn cancel_referendum() -> Weight {
                Weight::from_ref_time(59_872_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Scheduler Lookup (r:1 w:1)
        // Storage: Scheduler Agenda (r:1 w:1)
        /// The range of component `r` is `[1, 99]`.
        fn cancel_queued(r: u32, ) -> Weight {
                Weight::from_ref_time(115_657_000 as RefTimeWeight)
                        // Standard Error: 62_000
                        .saturating_add(Weight::from_ref_time(2_381_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
        }
        // Storage: Democracy LowestUnbaked (r:1 w:1)
        // Storage: Democracy ReferendumCount (r:1 w:0)
        // Storage: Democracy ReferendumInfoOf (r:1 w:0)
        /// The range of component `r` is `[1, 99]`.
        fn on_initialize_base(r: u32, ) -> Weight {
                Weight::from_ref_time(4_707_000 as RefTimeWeight)
                        // Standard Error: 128_000
                        .saturating_add(Weight::from_ref_time(8_734_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
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
                Weight::from_ref_time(13_491_000 as RefTimeWeight)
                        // Standard Error: 162_000
                        .saturating_add(Weight::from_ref_time(8_712_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(5 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Democracy VotingOf (r:3 w:3)
        // Storage: Democracy ReferendumInfoOf (r:1 w:1)
        // Storage: Balances Locks (r:1 w:1)
        /// The range of component `r` is `[1, 99]`.
        fn delegate(r: u32, ) -> Weight {
                Weight::from_ref_time(58_515_000 as RefTimeWeight)
                        // Standard Error: 225_000
                        .saturating_add(Weight::from_ref_time(17_349_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(4 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
                        .saturating_add(T::DbWeight::get().writes(4 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
        }
        // Storage: Democracy VotingOf (r:2 w:2)
        // Storage: Democracy ReferendumInfoOf (r:1 w:1)
        /// The range of component `r` is `[1, 99]`.
        fn undelegate(r: u32, ) -> Weight {
                Weight::from_ref_time(187_008_000 as RefTimeWeight)
                        // Standard Error: 188_000
                        .saturating_add(Weight::from_ref_time(11_839_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(r as RefTimeWeight)))
        }
        // Storage: Democracy PublicProps (r:0 w:1)
        fn clear_public_proposals() -> Weight {
                Weight::from_ref_time(13_015_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Democracy Preimages (r:1 w:1)
        /// The range of component `b` is `[0, 16384]`.
        fn note_preimage(b: u32, ) -> Weight {
                Weight::from_ref_time(80_510_000 as RefTimeWeight)
                        // Standard Error: 0
                        .saturating_add(Weight::from_ref_time(5_000 as RefTimeWeight).scalar_saturating_mul(b as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Democracy Preimages (r:1 w:1)
        /// The range of component `b` is `[0, 16384]`.
        fn note_imminent_preimage(b: u32, ) -> Weight {
                Weight::from_ref_time(63_583_000 as RefTimeWeight)
                        // Standard Error: 0
                        .saturating_add(Weight::from_ref_time(5_000 as RefTimeWeight).scalar_saturating_mul(b as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Democracy Preimages (r:1 w:1)
        // Storage: System Account (r:1 w:1)
        /// The range of component `b` is `[0, 16384]`.
        fn reap_preimage(b: u32, ) -> Weight {
                Weight::from_ref_time(116_651_000 as RefTimeWeight)
                        // Standard Error: 0
                        .saturating_add(Weight::from_ref_time(4_000 as RefTimeWeight).scalar_saturating_mul(b as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
        }
        // Storage: Democracy VotingOf (r:1 w:1)
        // Storage: Balances Locks (r:1 w:1)
        // Storage: System Account (r:1 w:1)
        /// The range of component `r` is `[1, 99]`.
        fn unlock_remove(r: u32, ) -> Weight {
                Weight::from_ref_time(109_127_000 as RefTimeWeight)
                        // Standard Error: 39_000
                        .saturating_add(Weight::from_ref_time(785_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
        }
        // Storage: Democracy VotingOf (r:1 w:1)
        // Storage: Balances Locks (r:1 w:1)
        // Storage: System Account (r:1 w:1)
        /// The range of component `r` is `[1, 99]`.
        fn unlock_set(r: u32, ) -> Weight {
                Weight::from_ref_time(70_415_000 as RefTimeWeight)
                        // Standard Error: 51_000
                        .saturating_add(Weight::from_ref_time(1_912_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
        }
        // Storage: Democracy ReferendumInfoOf (r:1 w:1)
        // Storage: Democracy VotingOf (r:1 w:1)
        /// The range of component `r` is `[1, 99]`.
        fn remove_vote(r: u32, ) -> Weight {
                Weight::from_ref_time(95_968_000 as RefTimeWeight)
                        // Standard Error: 40_000
                        .saturating_add(Weight::from_ref_time(956_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
        }
        // Storage: Democracy ReferendumInfoOf (r:1 w:1)
        // Storage: Democracy VotingOf (r:1 w:1)
        /// The range of component `r` is `[1, 99]`.
        fn remove_other_vote(r: u32, ) -> Weight {
                Weight::from_ref_time(56_984_000 as RefTimeWeight)
                        // Standard Error: 39_000
                        .saturating_add(Weight::from_ref_time(1_248_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
        }
}
