use frame_support::{dispatch::TransactionPriority, parameter_types};
use ternoa_core_primitives::Moment;

use crate::{
	constants::time::{EPOCH_DURATION_IN_SLOTS, MILLISECS_PER_BLOCK, PRIMARY_PROBABILITY},
	staking::{BondingDuration, SessionsPerEra},
};

parameter_types! {
	// Babe
	pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS as u64;
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();


	// I am Online
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	/// We prioritize im-online heartbeats over election solution submission.
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
	pub const MaxPeerDataEncodingSize: u32 = 1_000;

	// All
	pub const MaxAuthorities: u32 = 100_000;
}

pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
	sp_consensus_babe::BabeEpochConfiguration {
		c: PRIMARY_PROBABILITY,
		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
	};
