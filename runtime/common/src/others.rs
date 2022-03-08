use frame_support::parameter_types;
use ternoa_core_primitives::{Balance, Moment};

use crate::{
	constants::{currency::deposit, time::SLOT_DURATION},
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
}
