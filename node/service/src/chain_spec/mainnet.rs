use mainnet_runtime::{SessionKeys, BABE_GENESIS_EPOCH_CONFIG};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainType;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::sr25519;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use ternoa_core_primitives::AccountId;

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

/* use super::{DummyChainSpec, Extensions};

#[cfg(feature = "mainnet-native")]
use mainnet_runtime;

#[cfg(feature = "mainnet-native")]
pub type ChainSpec = sc_chain_spec::GenericChainSpec<mainnet_runtime::GenericConfig, Extensions>;

// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "mainnet-native"))]
pub type ChainSpec = sc_chain_spec::GenericChainSpec<DummyChainSpec, Extensions>;
 */
