use frame_support::{
	dispatch::{TransactionPriority, Weight},
	parameter_types,
	sp_runtime::Perbill,
	weights::{constants::BlockExecutionWeight, DispatchClass},
};
use sp_std::vec;
use ternoa_core_primitives::{AccountId, Balance, BlockNumber};

use crate::{
	constants::{
		currency::{deposit, UNITS},
		time::EPOCH_DURATION_IN_SLOTS,
	},
	system::{RuntimeBlockLength, RuntimeBlockWeights},
};

parameter_types! {
	// phase durations. 1/4 of the last session for each.
	pub const SignedPhase: u32 = EPOCH_DURATION_IN_SLOTS / 4;
	pub const UnsignedPhase: u32 = EPOCH_DURATION_IN_SLOTS / 4;

	// signed config
	pub const SignedMaxSubmissions: u32 = 16;
	// Each good submission will get 1/10 CAPS as reward
	pub const SignedRewardBase: Balance = UNITS / 10;
	pub const SignedDepositBase: Balance = deposit(2, 0);
	pub const SignedDepositByte: Balance = deposit(0, 10) / 1024;

	pub SolutionImprovementThreshold: Perbill = Perbill::from_rational(5u32, 10_000);

	pub OffchainRepeat: BlockNumber = UnsignedPhase::get() / 8;

	// miner configs
	pub const MinerMaxIterations: u32 = 10;
	pub MinerMaxWeight: Weight = RuntimeBlockWeights::get()
		.get(DispatchClass::Normal)
		.max_extrinsic.expect("Normal extrinsics have a weight limit configured; qed")
		.saturating_sub(BlockExecutionWeight::get());
	// Solution can occupy 90% of normal block size
	pub MinerMaxLength: u32 = Perbill::from_rational(90u32, 100) *
		*RuntimeBlockLength::get()
		.max
		.get(DispatchClass::Normal);

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

pub type Fallback<R> = pallet_election_provider_multi_phase::NoFallback<R>;
pub type GovernanceFallback<R> =
	frame_election_provider_support::onchain::OnChainSequentialPhragmen<R>;
pub type Solver<R> = frame_election_provider_support::SequentialPhragmen<
	AccountId,
	pallet_election_provider_multi_phase::SolutionAccuracyOf<R>,
	(),
>;

pub type OnChainAccuracy = Perbill;
