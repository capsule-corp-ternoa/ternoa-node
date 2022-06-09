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

use frame_support::parameter_types;
use ternoa_core_primitives::{Balance, BlockNumber};

use crate::{
	constants::{
		currency::UNITS,
		time::{DAYS, HOURS, MINUTES},
	},
	prod_or_fast,
};

parameter_types! {
	pub const LaunchPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1 * MINUTES);
	pub const VotingPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1 * MINUTES);
	pub const FastTrackVotingPeriod: BlockNumber = prod_or_fast!(3 * HOURS, 1 * MINUTES);
	pub const MinimumDeposit: Balance = 100 * UNITS;
	pub const EnactmentPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1 * MINUTES);
	pub const VoteLockingPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1 * MINUTES); // Should be same as EnactmentPeriod
	pub const CooloffPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES);
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
}
