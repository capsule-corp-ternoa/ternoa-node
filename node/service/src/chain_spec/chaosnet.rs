use super::{get_account_id_from_seed, get_from_seed, ChaosnetChainSpec as ChainSpec};
use chaosnet_runtime::{
	constants::currency::UNITS, wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
	BalancesConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig, SessionConfig, SessionKeys,
	StakingConfig, SystemConfig, TechnicalMembershipConfig, BABE_GENESIS_EPOCH_CONFIG,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::Forcing;
use sc_chain_spec::ChainType;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::sr25519;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::Perbill;
use ternoa_core_primitives::{AccountId, Balance};

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

pub type AuthorityKeys =
	(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId);

pub fn sr25519_account_from_seed(seed: &str) -> AccountId {
	get_account_id_from_seed::<sr25519::Public>(seed)
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> AuthorityKeys {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

pub struct GenesisInput {
	pub initial_authorities: Vec<AuthorityKeys>,
	pub endowed_accounts: Vec<(AccountId, Balance)>,
	pub stake_bond_amount: Balance,
	pub committee_members: Vec<AccountId>,
	pub invulnerables: Vec<AccountId>,
}

fn development_accounts() -> Vec<(AccountId, Balance)> {
	vec![
		(sr25519_account_from_seed("Alice"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Bob"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Charlie"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Dave"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Eve"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Ferdie"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Alice//stash"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Bob//stash"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Charlie//stash"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Dave//stash"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Eve//stash"), 1_000_000 * UNITS),
		(sr25519_account_from_seed("Ferdie//stash"), 1_000_000 * UNITS),
	]
}

fn development_config_genesis() -> GenesisConfig {
	let initial_authorities = vec![authority_keys_from_seed("Alice")];
	let endowed_accounts = development_accounts();
	let committee_members = vec![sr25519_account_from_seed("Alice")];
	let stake_bond_amount: Balance = 150_000 * UNITS;
	let invulnerables = vec![initial_authorities[0].0.clone()];

	let input = GenesisInput {
		initial_authorities,
		endowed_accounts,
		stake_bond_amount,
		committee_members,
		invulnerables,
	};

	genesis(input)
}

fn staging_accounts() -> Vec<(AccountId, Balance)> {
	vec![
		(sr25519_account_from_seed("boss"), 100_000_000 * UNITS),
		(sr25519_account_from_seed("bootnode1"), 1 * UNITS),
		(sr25519_account_from_seed("bootnode2"), 1 * UNITS),
		(sr25519_account_from_seed("bootnode1//stash"), 150_005 * UNITS),
		(sr25519_account_from_seed("bootnode2//stash"), 150_005 * UNITS),
	]
}

fn staging_config_genesis() -> GenesisConfig {
	let endowed_accounts = staging_accounts();
	let initial_authorities =
		vec![authority_keys_from_seed("bootnode1"), authority_keys_from_seed("bootnode2")];
	let committee_members = vec![sr25519_account_from_seed("boss")];
	let stake_bond_amount: Balance = 150_000 * UNITS;
	let invulnerables = vec![];

	let input = GenesisInput {
		initial_authorities,
		endowed_accounts,
		stake_bond_amount,
		committee_members,
		invulnerables,
	};

	genesis(input)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "CAPS".into());
	properties.insert("tokenDecimals".into(), 18.into());

	ChainSpec::from_genesis(
		"Ternoa Chaosnet Development",
		"chaosnet-dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		Some("ternoa"),
		None,
		Some(properties),
		Default::default(),
	)
}

/// Development config (single validator Alice)
pub fn staging_config() -> ChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "CAPS".into());
	properties.insert("tokenDecimals".into(), 18.into());

	ChainSpec::from_genesis(
		"Ternoa Chaosnet Staging",
		"chaosnet-sta",
		ChainType::Local,
		staging_config_genesis,
		vec![],
		None,
		Some("ternoa"),
		None,
		Some(properties),
		Default::default(),
	)
}

/// Helper function to create GenesisConfig for dev testo
pub fn genesis(input: GenesisInput) -> GenesisConfig {
	let GenesisInput {
		initial_authorities,
		endowed_accounts,
		stake_bond_amount,
		committee_members,
		invulnerables,
	} = input;

	GenesisConfig {
		// Core
		system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x.0, x.1)).collect(),
		},

		// Consensus
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		staking: StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.1.clone(),
						stake_bond_amount,
						alphanet_runtime::StakerStatus::Validator,
					)
				})
				.collect(),
			invulnerables,
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			min_validator_bond: 150_000 * UNITS,
			..Default::default()
		},
		treasury: Default::default(),
		transaction_payment: Default::default(),
		technical_committee: Default::default(),
		technical_membership: TechnicalMembershipConfig {
			phantom: Default::default(),
			members: committee_members,
		},
	}
}
