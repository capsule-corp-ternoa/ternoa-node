// Copyright 2022 Capsule Corp (France) SAS.
// This file is part of Ternoa.

// Ternoa is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Ternoa is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Ternoa.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_election_provider_support::Weight;
use frame_support::{parameter_types, weights::constants::WEIGHT_PER_SECOND};
use frame_system::limits;
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
use sp_runtime::{FixedPointNumber, Perbill, Perquintill};
use static_assertions::const_assert;
use ternoa_core_primitives::BlockNumber;

pub mod authorship;
pub mod babe;
pub mod bags_list;
pub mod balances;
pub mod bridge;
pub mod constants;
pub mod council;
pub mod democracy;
pub mod election_provider_multi_phase;
pub mod election_provider_support;
pub mod identity;
pub mod imonline;
pub mod multisig;
pub mod phragmen_election;
pub mod preimage;
pub mod shared;
pub mod staking;
pub mod staking_rewards;
pub mod technical_collective;
pub mod timestamp;
pub mod transaction_payment;
pub mod treasury;
pub mod voter_bags;

#[macro_export]
macro_rules! prod_or_fast {
	($prod:expr, $test:expr) => {
		if cfg!(feature = "fast-runtime") {
			$test
		} else {
			$prod
		}
	};
	($prod:expr, $test:expr, $env:expr) => {
		if cfg!(feature = "fast-runtime") {
			core::option_env!($env).map(|s| s.parse().ok()).flatten().unwrap_or($test)
		} else {
			$prod
		}
	};
}

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	/// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
	/// than this will decrease the weight and more will increase.
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	/// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
	/// change the fees more rapidly.
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000); // TODO!
	/// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
	/// that combined with `AdjustmentVariable`, we can recover from the minimum.
	/// See `multiplier_can_grow_from_zero`.
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128); // TODO!
	pub BlockLength: limits::BlockLength =
	limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);

	pub const SS58Prefix: u8 = 42;
}

/// Parameterized slow adjusting fee updated based on
/// https://research.web3.foundation/en/latest/polkadot/overview/2-token-economics.html#-2.-slow-adjusting-mechanism
pub type SlowAdjustingFeeUpdate<R> =
	TargetedFeeAdjustment<R, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;

/// We assume that an on-initialize consumes 1% of the weight on average, hence a single extrinsic
/// will not be allowed to consume more than `AvailableBlockRatio - 1%`.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(1);
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND.scalar_saturating_mul(2);

const_assert!(NORMAL_DISPATCH_RATIO.deconstruct() >= AVERAGE_ON_INITIALIZE_RATIO.deconstruct());

/// Implements the weight types for a runtime.
/// It expects the passed runtime constants to contain a `weights` module.
/// The generated weight types were formerly part of the common
/// runtime but are now runtime dependant.
#[macro_export]
macro_rules! impl_runtime_weights {
	($runtime:ident) => {
		use frame_support::weights::{DispatchClass, Weight};
		use frame_system::limits;
		use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
		use sp_runtime::{FixedPointNumber, Perquintill};
		pub use ternoa_runtime_common::{
			impl_elections_weights, impl_multiplier_tests, MinimumMultiplier,
			SlowAdjustingFeeUpdate, TargetBlockFullness, AVERAGE_ON_INITIALIZE_RATIO,
			MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO,
		};

		// Implement the weight types of the elections module.
		impl_elections_weights!($runtime);
		// Implement tests for the weight multiplier.
		impl_multiplier_tests!();

		// Expose the weight from the runtime constants module.
		pub use $runtime::weights::{
			BlockExecutionWeight, ExtrinsicBaseWeight, ParityDbWeight, RocksDbWeight,
		};

		parameter_types! {
			/// Block weights base values and limits.
			pub BlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
				.base_block($runtime::weights::BlockExecutionWeight::get())
				.for_class(DispatchClass::all(), |weights| {
					weights.base_extrinsic = $runtime::weights::ExtrinsicBaseWeight::get();
				})
				.for_class(DispatchClass::Normal, |weights| {
					weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
				})
				.for_class(DispatchClass::Operational, |weights| {
					weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
					// Operational transactions have an extra reserved space, so that they
					// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
					weights.reserved = Some(
						MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT,
					);
				})
				.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
				.build_or_panic();
		}
	};
}

/// Generates tests that check that the different weight multiplier work together.
/// Should not be called directly, use [`impl_runtime_weights`] instead.
#[macro_export]
macro_rules! impl_multiplier_tests {
	() => {
	#[cfg(test)]
	mod multiplier_tests {
		use super::*;
		use frame_support::{parameter_types, weights::Weight};
		use sp_core::H256;
		use sp_runtime::{
			testing::Header,
			traits::{BlakeTwo256, One, Convert, IdentityLookup},
			Perbill,
		};

		type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
		type Block = frame_system::mocking::MockBlock<Runtime>;

		frame_support::construct_runtime!(
			pub enum Runtime where
				Block = Block,
				NodeBlock = Block,
				UncheckedExtrinsic = UncheckedExtrinsic,
			{
				System: frame_system::{Pallet, Call, Config, Storage, Event<T>}
			}
		);

		parameter_types! {
			pub const BlockHashCount: u64 = 250;
			pub const AvailableBlockRatio: Perbill = Perbill::one();
			pub BlockLength: frame_system::limits::BlockLength =
				frame_system::limits::BlockLength::max(2 * 1024);
			pub BlockWeights: frame_system::limits::BlockWeights =
				frame_system::limits::BlockWeights::simple_max(1024);
		}

		impl frame_system::Config for Runtime {
			type BaseCallFilter = frame_support::traits::Everything;
			type BlockWeights = BlockWeights;
			type BlockLength = ();
			type DbWeight = ();
			type Origin = Origin;
			type Index = u64;
			type BlockNumber = u64;
			type Call = Call;
			type Hash = H256;
			type Hashing = BlakeTwo256;
			type AccountId = u64;
			type Lookup = IdentityLookup<Self::AccountId>;
			type Header = Header;
			type Event = Event;
			type BlockHashCount = BlockHashCount;
			type Version = ();
			type PalletInfo = PalletInfo;
			type AccountData = ();
			type OnNewAccount = ();
			type OnKilledAccount = ();
			type SystemWeightInfo = ();
			type SS58Prefix = ();
			type OnSetCode = ();
			type MaxConsumers = frame_support::traits::ConstU32<16>;
		}

		fn run_with_system_weight<F>(w: Weight, mut assertions: F)
		where
			F: FnMut() -> (),
		{
			let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default()
				.build_storage::<Runtime>()
				.unwrap()
				.into();
			t.execute_with(|| {
				System::set_block_consumed_resources(w, 0);
				assertions()
			});
		}

		#[test]
		fn multiplier_can_grow_from_zero() {
			let minimum_multiplier = MinimumMultiplier::get();
			let target = TargetBlockFullness::get() *
				BlockWeights::get().get(DispatchClass::Normal).max_total.unwrap();
			// if the min is too small, then this will not change, and we are doomed forever.
			// the weight is 1/100th bigger than target.
			run_with_system_weight(target * 101 / 100, || {
				let next = SlowAdjustingFeeUpdate::<Runtime>::convert(minimum_multiplier);
				assert!(next > minimum_multiplier, "{:?} !>= {:?}", next, minimum_multiplier);
			})
		}

		#[test]
		#[ignore]
		fn multiplier_growth_simulator() {
			// assume the multiplier is initially set to its minimum. We update it with values twice the
			//target (target is 25%, thus 50%) and we see at which point it reaches 1.
			let mut multiplier = MinimumMultiplier::get();
			let block_weight = TargetBlockFullness::get() *
				BlockWeights::get().get(DispatchClass::Normal).max_total.unwrap() *
				2;
			let mut blocks = 0;
			while multiplier <= Multiplier::one() {
				run_with_system_weight(block_weight, || {
					let next = SlowAdjustingFeeUpdate::<Runtime>::convert(multiplier);
					// ensure that it is growing as well.
					assert!(next > multiplier, "{:?} !>= {:?}", next, multiplier);
					multiplier = next;
				});
				blocks += 1;
				println!("block = {} multiplier {:?}", blocks, multiplier);
			}
		}
	}
	}
}
