use frame_support::{parameter_types, PalletId};
use sp_runtime::Permill;
use ternoa_core_primitives::{Balance, BlockNumber, Moment};

use crate::{
	constants::{
		currency::{deposit, CENTS, EUROS},
		time::{DAYS, SLOT_DURATION},
	},
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
	pub const SpendPeriod: BlockNumber = 1 * DAYS;
	pub const Burn: Permill = Permill::from_percent(0);
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaxApprovals: u32 = 100;
	pub const ProposalBondMinimum: Balance = 1 * EUROS;
	pub const ProposalBondMaximum: Balance = 1000 * EUROS;

	// Balances
	pub const ExistentialDeposit: Balance = 5 * CENTS;
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}
