
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-01, STEPS: `10`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `marko-MS-7B85`, CPU: `AMD Ryzen 7 5800X 8-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ternoa
// benchmark
// pallet
// --chain
// alphanet-dev
// --steps=10
// --repeat=5
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/
// --pallet=pallet_identity

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		Weight::from_ref_time(19_408_000 as RefTimeWeight)
			// Standard Error: 51_000
			.saturating_add(Weight::from_ref_time(206_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[1, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(36_416_000 as RefTimeWeight)
			// Standard Error: 89_000
			.saturating_add(Weight::from_ref_time(85_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			// Standard Error: 17_000
			.saturating_add(Weight::from_ref_time(438_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		Weight::from_ref_time(31_953_000 as RefTimeWeight)
			// Standard Error: 25_000
			.saturating_add(Weight::from_ref_time(3_119_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(s as RefTimeWeight)))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(s as RefTimeWeight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		Weight::from_ref_time(32_447_000 as RefTimeWeight)
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(1_299_000 as RefTimeWeight).scalar_saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(p as RefTimeWeight)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `x` is `[1, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(36_578_000 as RefTimeWeight)
			// Standard Error: 75_000
			.saturating_add(Weight::from_ref_time(211_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			// Standard Error: 14_000
			.saturating_add(Weight::from_ref_time(1_324_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			// Standard Error: 14_000
			.saturating_add(Weight::from_ref_time(209_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(s as RefTimeWeight)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[1, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(38_920_000 as RefTimeWeight)
			// Standard Error: 51_000
			.saturating_add(Weight::from_ref_time(186_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(405_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[1, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(37_177_000 as RefTimeWeight)
			// Standard Error: 39_000
			.saturating_add(Weight::from_ref_time(95_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(397_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		Weight::from_ref_time(11_982_000 as RefTimeWeight)
			// Standard Error: 57_000
			.saturating_add(Weight::from_ref_time(118_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		Weight::from_ref_time(11_016_000 as RefTimeWeight)
			// Standard Error: 27_000
			.saturating_add(Weight::from_ref_time(186_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		Weight::from_ref_time(11_192_000 as RefTimeWeight)
			// Standard Error: 31_000
			.saturating_add(Weight::from_ref_time(149_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[1, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(25_595_000 as RefTimeWeight)
			// Standard Error: 61_000
			.saturating_add(Weight::from_ref_time(242_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(408_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `x` is `[1, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(48_997_000 as RefTimeWeight)
			// Standard Error: 80_000
			.saturating_add(Weight::from_ref_time(188_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
			// Standard Error: 15_000
			.saturating_add(Weight::from_ref_time(1_335_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			// Standard Error: 15_000
			.saturating_add(Weight::from_ref_time(17_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(s as RefTimeWeight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(42_770_000 as RefTimeWeight)
			// Standard Error: 20_000
			.saturating_add(Weight::from_ref_time(105_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(17_165_000 as RefTimeWeight)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(72_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(42_685_000 as RefTimeWeight)
			// Standard Error: 12_000
			.saturating_add(Weight::from_ref_time(115_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(34_990_000 as RefTimeWeight)
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(38_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
}
