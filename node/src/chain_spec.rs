use jsonrpc_core::serde_json::{json, map::Map as SerdeMap};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::{config::TelemetryEndpoints, ChainType};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use ternoa_primitives::{AccountId, Balance, Signature};
use ternoa_runtime::{
    constants::currency::UNIT, wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
    BalancesConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig, SessionConfig, SessionKeys,
    StakerStatus, StakingConfig, SudoConfig, SystemConfig,
};

type AccountPublic = <Signature as Verify>::Signer;
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

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

    const ENDOWMENT: Balance = 1_000_000 * UNIT;
    const STASH: Balance = ENDOWMENT / 1_000;

    GenesisConfig {
        // Core
        frame_system: Some(SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        }),

        // Consensus
        pallet_session: Some(SessionConfig {
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
        }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),

        // Governance
        pallet_sudo: Some(SudoConfig {
            key: root.unwrap_or(get_account_id_from_seed::<sr25519::Public>("Alice")),
        }),
    }
}

fn chaos_config_genesis() -> GenesisConfig {
    let root: AccountId =
        hex_literal::hex!["d8f50455fa71e47aa6ed41670393ebba9f7083cd1d90f3f9f3f7a6bf921b6209"]
            .into();
    let authority_1: AccountId =
        hex_literal::hex!["cec787c2835cdb1b300fbce578368f052c0c68a546061cd3526d82ae6dccd772"]
            .into();
    let authority_1_ed25519 =
        hex_literal::hex!("0b8dd85cb7cfad9957c7b065ced7d9b332ce4e893041bf43cb68eb03f7081c52")
            .to_vec();
    let authority_1_sr25519 =
        hex_literal::hex!("b6eeee5ff904f44f0de4c1ba0e82ea6bbd0780f98eb8ccd45e142f8a2754ec32")
            .to_vec();
    let authority_2: AccountId =
        hex_literal::hex!["e4b0d5b34be3a82fdc643eb94085cbf8bd5c3280c7d55ff2a203aee6ab464745"]
            .into();
    let authority_2_ed25519 =
        hex_literal::hex!("300456bdf5e461ed39c76f9df8ccabf9a17f4f7d15dd458e66050610502c40e4")
            .to_vec();
    let authority_2_sr25519 =
        hex_literal::hex!("dac6ac8217639698d807b519f5d946ecc8d0323c6d956f336259b7e06e907304")
            .to_vec();

    testnet_genesis(
        vec![
            (
                authority_1.clone(),
                authority_1.clone(),
                GrandpaId::from_slice(authority_1_ed25519.as_slice()),
                BabeId::from_slice(authority_1_sr25519.as_slice()),
                ImOnlineId::from_slice(authority_1_sr25519.as_slice()),
                AuthorityDiscoveryId::from_slice(authority_1_sr25519.as_slice()),
            ),
            (
                authority_2.clone(),
                authority_2.clone(),
                GrandpaId::from_slice(authority_2_ed25519.as_slice()),
                BabeId::from_slice(authority_2_sr25519.as_slice()),
                ImOnlineId::from_slice(authority_2_sr25519.as_slice()),
                AuthorityDiscoveryId::from_slice(authority_2_sr25519.as_slice()),
            ),
        ],
        Some(vec![root.clone(), authority_1, authority_2]),
        Some(root),
    )
}

/// Chaos config (single validator Alice)
pub fn chaos_config() -> ChainSpec {
    let mut properties = SerdeMap::new();
    properties.insert(String::from("tokenSymbol"), json!("CACO"));
    properties.insert(String::from("tokenDecimals"), json!(18));

    ChainSpec::from_genesis(
        "Ternoa Chaos Net",
        "ternoa_chaos",
        ChainType::Live,
        chaos_config_genesis,
        vec![],
        Some(
            TelemetryEndpoints::new(vec![(
                String::from("/dns/telemetry.polkadot.io/tcp/443/x-parity-wss/%2Fsubmit%2F"),
                0,
            )])
            .unwrap(),
        ),
        Some("ternoa"),
        Some(properties),
        Default::default(),
    )
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
        None,
        None,
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
        None,
        None,
        Default::default(),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use sp_runtime::BuildStorage;

    #[test]
    fn create_chain_specs() {
        let configs = vec![development_config(), local_testnet_config()];
        for conf in configs {
            assert!(conf.build_storage().is_ok());
        }
    }
}
