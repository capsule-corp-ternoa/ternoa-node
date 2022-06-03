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

use frame_support::{parameter_types, PalletId as FramePalletId};

parameter_types! {
	pub const ChainId: u8 = 0;
	pub const PalletId: FramePalletId = FramePalletId(*b"terbirdg");
	pub const RelayerCountLimit: u32 = 10;
	pub const RelayerVoteThreshold: u32 = 2;
}
