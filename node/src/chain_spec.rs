use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde_json::json;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::Perbill;
use ternoa_marketplace::{MarketplaceInformation, MarketplaceType};
use ternoa_primitives::{AccountId, Balance, Signature};
use ternoa_runtime::constants::currency::UNIT;
use ternoa_runtime::{
    wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, GenesisConfig,
    GrandpaConfig, ImOnlineConfig, MarketplaceConfig, SessionConfig, SessionKeys, SgxConfig,
    StakerStatus, StakingConfig, SystemConfig, TechnicalMembershipConfig,
};

type AccountPublic = <Signature as Verify>::Signer;
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
const VALIDATOR_TEST_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

/// Helper function to generate a crypto pair from seed
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

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    endowed_accounts: Option<Vec<AccountId>>,
    root: Option<AccountId>,
) -> GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
        ]
    });

    const ENDOWMENT: Balance = UNIT * 1_000_000;
    const STASH: Balance = UNIT * 10_000;

    GenesisConfig {
        // Core
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        },
        tiime_balances: Default::default(),

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
        babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(ternoa_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        im_online: ImOnlineConfig { keys: vec![] },
        authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        staking: StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        },
        treasury: Default::default(),

        // Governance
        technical_committee: Default::default(),
        technical_membership: TechnicalMembershipConfig {
            members: vec![root.unwrap_or(get_account_id_from_seed::<sr25519::Public>("Alice"))],
            phantom: Default::default(),
        },

        // Ternoa
        nfts: Default::default(),
        marketplace: MarketplaceConfig {
            nfts_for_sale: Default::default(),
            marketplaces: vec![(
                0,
                MarketplaceInformation::new(
                    MarketplaceType::Public,
                    0,
                    // Alice
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    Default::default(),
                    "Ternoa Marketplace".into(),
                ),
            )],
        },
        sgx: SgxConfig {
            clusters: Default::default(),
            enclaves: Default::default(),
        },
    }
}

fn build_local_properties() -> Properties {
    let mut props = Properties::new();
    props.insert("tokenDecimals".to_string(), json!(18));
    props.insert("tokenSymbol".to_string(), json!("CAPS"));

    props
}

pub fn staging_net_config() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../chain-specifications/staging-net.json")[..])
        .unwrap()
}

pub fn test_net_config() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../chain-specifications/test-net.json")[..])
        .unwrap()
}

pub fn main_net_config() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../chain-specifications/main-net.json")[..])
        .unwrap()
}
fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(vec![get_authority_keys_from_seed("Alice")], None, None)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        Some("ternoa"),
        Some(build_local_properties()),
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
            get_authority_keys_from_seed("Bob"),
        ],
        None,
        None,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        local_testnet_genesis,
        vec![],
        None,
        Some("ternoa"),
        Some(build_local_properties()),
        Default::default(),
    )
}

fn local_validator_testnet_genesis() -> GenesisConfig {
    let mut genesis = testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
            get_authority_keys_from_seed("Bob"),
            get_authority_keys_from_seed("Charlie"),
            get_authority_keys_from_seed("Dave"),
            get_authority_keys_from_seed("Eve"),
        ],
        None,
        None,
    );
    genesis.staking.minimum_validator_count = 1;
    genesis.staking.invulnerables.clear();

    genesis
}

/// Local Validator testnet config
pub fn local_validator_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Validator Testnet",
        "local_validator_testnet",
        ChainType::Local,
        local_validator_testnet_genesis,
        vec![
            "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWDEjYH18aJ67pxnyPnSumDCyaPvqZFyGCpURnTYo8jtyU"
                .parse()
                .unwrap(),
        ],
        Some(
            TelemetryEndpoints::new(vec![(VALIDATOR_TEST_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid"),
        ),
        Some("ternoa"),
        Some(build_local_properties()),
        Default::default(),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use sp_runtime::BuildStorage;

    #[test]
    fn create_chain_specs() {
        let configs = vec![
            development_config(),
            local_testnet_config(),
            local_validator_testnet_config(),
            staging_net_config(),
            test_net_config(),
            main_net_config(),
        ];
        for conf in configs {
            assert!(conf.build_storage().is_ok());
        }
    }
}
