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

use frame_support::{dispatch::TransactionPriority, parameter_types};
use ternoa_core_primitives::{BlockNumber, Moment};

use crate::constants::time::{MILLISECS_PER_BLOCK, PRIMARY_PROBABILITY};

parameter_types! {
	// Babe
	pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;

	// I am Online
	pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	/// We prioritize im-online heartbeats over election solution submission.
	pub const MaxKeys: u32 = 10_000;
	pub const MaxPeerInHeartbeats: u32 = 10_000;
	pub const MaxPeerDataEncodingSize: u32 = 1_000;

	// Authorship
	pub const UncleGenerations: BlockNumber = 0;

	// All
	pub const MaxAuthorities: u32 = 100_000;
}

// Babe
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
	sp_consensus_babe::BabeEpochConfiguration {
		c: PRIMARY_PROBABILITY,
		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
	};
pub type EpochChangeTrigger = pallet_babe::ExternalTrigger;
