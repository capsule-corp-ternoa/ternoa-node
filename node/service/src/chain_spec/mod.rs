pub use ternoa_core_primitives::AccountId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use serde::{Deserialize, Serialize};

#[cfg(feature = "alphanet-native")]
pub mod alphanet;
#[cfg(feature = "chaosnet-native")]
pub mod chaosnet;
#[cfg(feature = "mainnet-native")]
pub mod mainnet;

#[cfg(not(feature = "alphanet-native"))]
pub mod alphanet {
	pub type ChainSpec = crate::chain_spec::fake_spec::FakeSpec;
	pub fn chain_spec_from_json_file(_: std::path::PathBuf) -> Result<ChainSpec, String> {
		panic!("alphanet runtime not enabled")
	}
	pub fn development_chain_spec(_: Option<String>, _: Option<u32>) -> ChainSpec {
		panic!("alphanet runtime not enabled")
	}
}
#[cfg(not(feature = "chaosnet-native"))]
pub mod chaosnet {
	pub type ChainSpec = crate::chain_spec::fake_spec::FakeSpec;
	pub fn chain_spec_from_json_file(_: std::path::PathBuf) -> Result<ChainSpec, String> {
		panic!("chaosnet runtime not enabled")
	}
	pub fn development_chain_spec(_: Option<String>, _: Option<u32>) -> ChainSpec {
		panic!("chaosnet runtime not enabled")
	}
}
#[cfg(not(feature = "mainnet-native"))]
pub mod mainnet {
	pub type ChainSpec = crate::chain_spec::fake_spec::FakeSpec;
	pub fn chain_spec_from_json_file(_: std::path::PathBuf) -> Result<ChainSpec, String> {
		panic!("mainnet runtime not enabled")
	}
	pub fn development_chain_spec(_: Option<String>, _: Option<u32>) -> ChainSpec {
		panic!("mainnet runtime not enabled")
	}
}

pub type RawChainSpec = sc_service::GenericChainSpec<(), Extensions>;

#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension, ChainSpecGroup)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}