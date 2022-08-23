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

use frame_support::{parameter_types, traits::LockIdentifier};
use ternoa_core_primitives::{Balance, BlockNumber};

use crate::{
	constants::{
		currency::{deposit, CENTS},
		time::{DAYS, MINUTES},
	},
	prod_or_fast,
};

parameter_types! {
	pub const PhragmenCandidacyBond: Balance = 100 * CENTS;
	// 1 storage item created, key size is 32 bytes, value size is 16+16.
	pub const PhragmenVotingBondBase: Balance = deposit(1, 64);
	// additional data per vote is 32 bytes (account id).
	pub const PhragmenVotingBondFactor: Balance = deposit(0, 32);
	/// Daily council elections
	pub PhragmenTermDuration: BlockNumber = prod_or_fast!(7 * DAYS, 2 * MINUTES);
	pub const PhragmenDesiredMembers: u32 = 7;
	pub const PhragmenDesiredRunnersUp: u32 = 7;
	pub const PhragmenElectionPalletId: LockIdentifier = *b"phrelect";
	pub const MaxVoters: u32 = 10 * 1000;
	pub const MaxCandidates: u32 = 1000;
}
