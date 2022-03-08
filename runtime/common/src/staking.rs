use frame_support::{parameter_types, traits::ConstU32};
use sp_runtime::Perbill;

use crate::elections::NposCompactSolution24;

parameter_types! {
	pub const SessionsPerEra: sp_staking::SessionIndex = 6;
	pub const BondingDuration: sp_staking::EraIndex = 28;
	pub const SlashDeferDuration: sp_staking::EraIndex = 27; // 1/4 the bonding duration.
	pub const MaxNominatorRewardedPerValidator: u32 = 256;
	// 1 hour session, 15 minutes unsigned phase, 8 offchain executions.
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
	pub const MaxIterations: u32 = 10;
	// 0.05%. The higher the value, the more strict solution acceptance becomes.
	pub MinSolutionScoreBump: Perbill = Perbill::from_rational(5u32, 10_000);
	pub const MaxNominations: u32 = <NposCompactSolution24 as sp_npos_elections::NposSolution>::LIMIT as u32;
}

/// A reasonable benchmarking config for staking pallet.
pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxValidators = ConstU32<1000>;
	type MaxNominators = ConstU32<1000>;
}

pub type EraPayout = ();
pub type GenesisElectionProvider<R> =
	frame_election_provider_support::onchain::OnChainSequentialPhragmen<R>;
