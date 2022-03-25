use frame_support::{
	parameter_types,
	traits::{ConstU32, Currency, Imbalance, OnUnbalanced},
	PalletId,
};
use pallet_balances::NegativeImbalance;
use sp_runtime::Perbill;

use crate::elections::NposCompactSolution24;

parameter_types! {
	// Six sessions in an era (6 * EPOCH, 6 hours).
	pub const SessionsPerEra: sp_staking::SessionIndex = 6;
	// 28 eras for unbonding (7 days).
	pub const BondingDuration: sp_staking::EraIndex = 28;
	// 27 eras in which slashes can be cancelled (slightly less than 7 days).
	pub const SlashDeferDuration: sp_staking::EraIndex = 27;
	pub const MaxNominatorRewardedPerValidator: u32 = 256;
	// 1 hour session, 15 minutes unsigned phase, 8 offchain executions.
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
	pub const MaxNominations: u32 = <NposCompactSolution24 as frame_election_provider_support::NposSolution>::LIMIT as u32;

	// Staking rewards
	pub const StakingRewardsPalletId: PalletId = PalletId(*b"terstare");
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

pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_authorship::Config,
{
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		if let Some(mut amount) = fees_then_tips.next() {
			// for fees, 100% to author
			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 100% to author
				tips.merge_into(&mut amount);
			}

			if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
				<pallet_balances::Pallet<R>>::resolve_creating(&author, amount);
			}
		}
	}
}
