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

use sc_chain_spec::{ChainSpecExtension, GenericChainSpec};
use serde::{Deserialize, Serialize};
use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use ternoa_core_primitives::{AccountId, Block, Signature};

#[cfg(feature = "alphanet-native")]
pub mod alphanet;
#[cfg(feature = "mainnet-native")]
pub mod mainnet;

type AccountPublic = <Signature as Verify>::Signer;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state.
	///
	/// This value will be set by the `sync-state rpc` implementation.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

// Dummy chain spec, in case when we don't have the native runtime.
pub type DummyChainSpec = GenericChainSpec<(), Extensions>;

#[cfg(feature = "alphanet-native")]
pub type AlphanetChainSpec = GenericChainSpec<alphanet_runtime::GenesisConfig, Extensions>;

#[cfg(not(feature = "alphanet-native"))]
pub type AlphanetChainSpec = GenericChainSpec<DummyChainSpec, Extensions>;

#[cfg(feature = "mainnet-native")]
pub type MainnetChainSpec = GenericChainSpec<mainnet_runtime::GenesisConfig, Extensions>;

#[cfg(not(feature = "mainnet-native"))]
pub type MainnetChainSpec = GenericChainSpec<DummyChainSpec, Extensions>;

pub fn alphanet_config() -> Result<AlphanetChainSpec, String> {
	AlphanetChainSpec::from_json_bytes(
		&include_bytes!("../../../../specs/alphanet/alphanet_raw.json")[..],
	)
}

pub fn mainnet_config() -> Result<MainnetChainSpec, String> {
	MainnetChainSpec::from_json_bytes(
		&include_bytes!("../../../../specs/mainnet/mainnet_raw.json")[..],
	)
}

/// Helper function to generate a crypto pair from seeds
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}
