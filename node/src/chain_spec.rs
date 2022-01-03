use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::{ChainSpecExtension, Properties};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::Perbill;
use ternoa_marketplace::{MarketplaceInformation, MarketplaceType};
use ternoa_primitives::{AccountId, Balance, Signature};
use ternoa_runtime::constants::currency::UNITS;
use ternoa_runtime::{
    wasm_binary_unwrap, AssociatedAccountsConfig, AuthorityDiscoveryConfig, BabeConfig,
    BalancesConfig, Block, CapsulesConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig,
    MarketplaceConfig, NftsConfig, SessionConfig, SessionKeys, SgxConfig, StakerStatus,
    StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig, TeerexConfig,
    MAX_NOMINATIONS,
};

type AccountPublic = <Signature as Verify>::Signer;
const VALIDATOR_TEST_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

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
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

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
pub fn authority_keys_from_seed(
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
    initial_nominators: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ]
    });

    // endow all authorities and nominators.
    initial_authorities
        .iter()
        .map(|x| &x.0)
        .chain(initial_nominators.iter())
        .for_each(|x| {
            if !endowed_accounts.contains(x) {
                endowed_accounts.push(x.clone())
            }
        });

    // stakers: all validators and nominators.
    let mut rng = rand::thread_rng();
    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
        .chain(initial_nominators.iter().map(|x| {
            use rand::{seq::SliceRandom, Rng};
            let limit = (MAX_NOMINATIONS as usize).min(initial_authorities.len());
            let count = rng.gen::<usize>() % limit;
            let nominations = initial_authorities
                .as_slice()
                .choose_multiple(&mut rng, count)
                .into_iter()
                .map(|choice| choice.0.clone())
                .collect::<Vec<_>>();
            (
                x.clone(),
                x.clone(),
                STASH,
                StakerStatus::Nominator(nominations),
            )
        }))
        .collect::<Vec<_>>();

    let num_endowed_accounts = endowed_accounts.len();
    let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

    const ENDOWMENT: Balance = UNITS * 1_000_000;
    const STASH: Balance = UNITS * 10_000;

    GenesisConfig {
        // Core
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|x| (x, ENDOWMENT))
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
            validator_count: initial_authorities.len() as u32,
            minimum_validator_count: initial_authorities.len() as u32,
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            stakers,
            ..Default::default()
        },
        treasury: Default::default(),

        // Governance
        technical_committee: TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        },
        technical_membership: Default::default(),

        // Ternoa
        nfts: NftsConfig {
            nfts: Default::default(),
            series: Default::default(),
            nft_mint_fee: 10000000000000000000,
        },
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
                    Default::default(),
                    "Ternoa Marketplace".into(),
                    None,
                    None,
                    None,
                ),
            )],
            marketplace_mint_fee: 10000000000000000000000,
        },
        sgx: SgxConfig {
            clusters: Default::default(),
            enclaves: Default::default(),
        },
        capsules: CapsulesConfig {
            capsule_mint_fee: 1000000000000000000000,
            ..Default::default()
        },
        associated_accounts: AssociatedAccountsConfig {
            ..Default::default()
        },
        elections: Default::default(),
        indices: Default::default(),
        sudo: SudoConfig { key: root_key },
        scheduler: Default::default(),
        transaction_payment: Default::default(),
        teerex: TeerexConfig {
            allow_sgx_debug_mode: true,
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
    testnet_genesis(vec![authority_keys_from_seed("Alice")], vec![], None)
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
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        vec![],
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
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
            authority_keys_from_seed("Charlie"),
            authority_keys_from_seed("Dave"),
            authority_keys_from_seed("Eve"),
        ],
        vec![],
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
