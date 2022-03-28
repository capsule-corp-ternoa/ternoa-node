use frame_support::{dispatch::Weight, parameter_types, PalletId};
use sp_runtime::{Perbill, Permill};
use ternoa_core_primitives::{Balance, BlockNumber, Moment};

use crate::{
	constants::{
		currency::{deposit, EXISTENTIAL_DEPOSIT, UNITS},
		time::{DAYS, MINUTES, SLOT_DURATION},
	},
	prod_or_fast,
	system::RuntimeBlockWeights,
	voter_bags,
};

parameter_types! {
	// Preimage
	pub const PreimageMaxSize: u32 = 4096 * 1024;
	pub const PreimageBaseDeposit: Balance = deposit(2, 64);
	pub const PreimageByteDeposit: Balance = deposit(0, 1);

	// Bags
	pub const BagThresholds: &'static [u64] = &voter_bags::THRESHOLDS;

	// Timestamp
	pub const TimestampMinimumPeriod: Moment = SLOT_DURATION / 2;

	// Treasury
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const SpendPeriod: BlockNumber = 6 * DAYS;
	pub const Burn: Permill = Permill::from_percent(0);
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaxApprovals: u32 = 100;
	pub const ProposalBondMinimum: Balance = 1 * UNITS;
	pub const ProposalBondMaximum: Balance = 1000 * UNITS;

	// Balances
	pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;

	// Technical committee
	pub TechnicalMotionDuration: BlockNumber = prod_or_fast!(3 * DAYS, 10 * MINUTES);
	pub const TechnicalMaxProposals: u32 = 100;
	pub const TechnicalMaxMembers: u32 = 100;

	// Scheduler
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
	RuntimeBlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
	pub const NoPreimagePostponement: Option<u32> = Some(10);
}
