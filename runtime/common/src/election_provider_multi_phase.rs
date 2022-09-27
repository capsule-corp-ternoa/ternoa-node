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

use frame_support::{dispatch::TransactionPriority, parameter_types, sp_runtime::Perbill};
use sp_std::vec;
use ternoa_core_primitives::{AccountId, Balance};

use crate::constants::currency::{deposit, UNITS};

/// Implements the weight types for the elections module and a specific
/// runtime.
/// This macro should not be called directly; use [`impl_runtime_weights`] instead.
#[macro_export]
macro_rules! impl_elections_weights {
	($runtime:ident) => {
		parameter_types! {
			/// A limit for off-chain phragmen unsigned solution submission.
			///
			/// We want to keep it as high as possible, but can't risk having it reject,
			/// so we always subtract the base block execution weight.
			pub OffchainSolutionWeightLimit: Weight = BlockWeights::get()
				.get(DispatchClass::Normal)
				.max_extrinsic
				.expect("Normal extrinsics have weight limit configured by default; qed")
				.saturating_sub($runtime::weights::BlockExecutionWeight::get());

			/// A limit for off-chain phragmen unsigned solution length.
			///
			/// We allow up to 90% of the block's size to be consumed by the solution.
			pub OffchainSolutionLengthLimit: u32 = Perbill::from_rational(90_u32, 100) *
				*BlockLength::get()
				.max
				.get(DispatchClass::Normal);
		}
	};
}

parameter_types! {
	// signed config
	pub const SignedMaxSubmissions: u32 = 16;
	// Each good submission will get 1/10 CAPS as reward
	pub const SignedRewardBase: Balance = UNITS / 10;
	pub const SignedDepositBase: Balance = deposit(2, 0);
	pub const SignedDepositByte: Balance = deposit(0, 10) / 1024;
	pub const SignedMaxRefunds: u32 = 16 / 4;

	/// Whilst `UseNominatorsAndUpdateBagsList` or `UseNominatorsMap` is in use, this can still be a
	/// very large value. Once the `BagsList` is in full motion, staking might open its door to many
	/// more nominators, and this value should instead be what is a "safe" number (e.g. 22500).
	pub const VoterSnapshotPerBlock: u32 = 22_500;

	pub NposSolutionPriority: TransactionPriority =
		Perbill::from_percent(90) * TransactionPriority::max_value();
	/// We take the top 12500 nominators as electing voters..
	pub const MaxElectingVoters: u32 = 12_500;
	/// ... and all of the validators as electable targets. Whilst this is the case, we cannot and
	/// shall not increase the size of the validator intentions.
	pub const MaxElectableTargets: u16 = u16::MAX;

	pub BetterUnsignedThreshold: Perbill = Perbill::from_rational(5u32, 10_000);
}

frame_election_provider_support::generate_solution_type!(
	#[compact]
	pub struct NposCompactSolution24::<
		VoterIndex = u32,
		TargetIndex = u16,
		Accuracy = sp_runtime::PerU16,
		MaxVoters = MaxElectingVoters,
	>(24)
);

/// The numbers configured here should always be more than the the maximum limits of staking pallet
/// to ensure election snapshot will not run out of memory.
pub struct BenchmarkConfig;
impl pallet_election_provider_multi_phase::BenchmarkingConfig for BenchmarkConfig {
	const VOTERS: [u32; 2] = [1000, 2000];
	const TARGETS: [u32; 2] = [500, 1000];
	const ACTIVE_VOTERS: [u32; 2] = [500, 800];
	const DESIRED_TARGETS: [u32; 2] = [200, 400];
	const SNAPSHOT_MAXIMUM_VOTERS: u32 = 1000;
	const MINER_MAXIMUM_VOTERS: u32 = 1000;
	const MAXIMUM_TARGETS: u32 = 300;
}

pub type Solver<R> = frame_election_provider_support::SequentialPhragmen<
	AccountId,
	pallet_election_provider_multi_phase::SolutionAccuracyOf<R>,
	(),
>;
