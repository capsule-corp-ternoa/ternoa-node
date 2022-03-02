use sc_chain_spec::{ChainSpecExtension, GenericChainSpec};
use serde::{Deserialize, Serialize};
use sp_core::{Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use ternoa_core_primitives::{AccountId, Block, Signature};

#[cfg(feature = "alphanet-native")]
pub mod alphanet;
#[cfg(feature = "chaosnet-native")]
pub mod chaosnet;
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

#[cfg(feature = "chaosnet-native")]
pub type ChaosnetChainSpec = GenericChainSpec<chaosnet_runtime::GenesisConfig, Extensions>;

#[cfg(not(feature = "chaosnet-native"))]
pub type ChaosnetChainSpec = GenericChainSpec<DummyChainSpec, Extensions>;

#[cfg(feature = "alphanet-native")]
pub type AlphanetChainSpec = GenericChainSpec<alphanet_runtime::GenesisConfig, Extensions>;

#[cfg(not(feature = "alphanet-native"))]
pub type AlphanetChainSpec = GenericChainSpec<DummyChainSpec, Extensions>;

#[cfg(feature = "mainnet-native")]
pub type MainnetChainSpec = GenericChainSpec<mainnet_runtime::GenesisConfig, Extensions>;

#[cfg(not(feature = "mainnet-native"))]
pub type MainnetChainSpec = GenericChainSpec<DummyChainSpec, Extensions>;

pub fn chaosnet_config() -> Result<ChaosnetChainSpec, String> {
	ChaosnetChainSpec::from_json_bytes(&include_bytes!("../../../../specs/chaosnet/raw.json")[..])
}

pub fn alphanet_config() -> Result<AlphanetChainSpec, String> {
	AlphanetChainSpec::from_json_bytes(&include_bytes!("../../../../specs/alphanet/raw.json")[..])
}

pub fn mainnet_config() -> Result<MainnetChainSpec, String> {
	MainnetChainSpec::from_json_bytes(&include_bytes!("../../../../specs/mainnet/raw.json")[..])
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
