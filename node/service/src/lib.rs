pub mod chain_spec;
mod rpc;

use futures::StreamExt;
use sc_chain_spec::ChainSpec;
use sc_client_api::{BlockBackend, ExecutorProvider};
use sc_consensus_babe::{self, SlotProportion};
use sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch};
use sc_network::{Event, NetworkService};
use sc_service::{
	config::Configuration, error::Error as ServiceError, RpcHandlers, TFullClient, TaskManager,
};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sp_api::ConstructRuntimeApi;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;
use ternoa_client::RuntimeApiCollection;
use ternoa_core_primitives::Block;

#[cfg(feature = "chaosnet-native")]
pub use ternoa_client::ChaosnetExecutorDispatch;

#[cfg(feature = "alphanet-native")]
pub use ternoa_client::AlphanetExecutorDispatch;

#[cfg(feature = "mainnet-native")]
pub use ternoa_client::MainnetExecutorDispatch;

#[cfg(feature = "chaosnet-native")]
pub use chaosnet_runtime;

#[cfg(feature = "alphanet-native")]
pub use alphanet_runtime;

#[cfg(feature = "mainnet-native")]
pub use mainnet_runtime;

//pub use client::*;

/// The full client type definition.
type FullClient<RuntimeApi, Executor> =
	TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullGrandpaBlockImport<RuntimeApi, Executor> = sc_finality_grandpa::GrandpaBlockImport<
	FullBackend,
	Block,
	FullClient<RuntimeApi, Executor>,
	FullSelectChain,
>;
/// The transaction pool type defintion.
pub type TransactionPool<RuntimeApi, Executor> =
	sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, Executor>>;

/// Can be called for a `Configuration` to identify which network the configuration targets.
pub trait IdentifyVariant {
	/// Returns `true` if this is a configuration for the `Chaosnet` network.
	fn is_chaosnet(&self) -> bool;

	/// Returns `true` if this is a configuration for the `Alphanet` network.
	fn is_alphanet(&self) -> bool;

	/// Returns `true` if this is a configuration for the `Mainnet` network.
	fn is_mainnet(&self) -> bool;

	/// Returns `true` if this is a configuration for a dev network.
	fn is_dev(&self) -> bool;
}

impl IdentifyVariant for Box<dyn ChainSpec> {
	fn is_chaosnet(&self) -> bool {
		self.id().starts_with("chaosnet") || self.id().starts_with("chaos")
	}

	fn is_alphanet(&self) -> bool {
		self.id().starts_with("alphanet") || self.id().starts_with("alpha")
	}

	fn is_mainnet(&self) -> bool {
		self.id().starts_with("mainnet") || self.id().starts_with("main")
	}

	fn is_dev(&self) -> bool {
		self.id().ends_with("dev")
	}
}

pub fn new_partial<RuntimeApi, ExecutorDispatch>(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient<RuntimeApi, ExecutorDispatch>,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient<RuntimeApi, ExecutorDispatch>>,
		sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, ExecutorDispatch>>,
		(
			impl Fn(
				rpc::DenyUnsafe,
				sc_rpc::SubscriptionTaskExecutor,
			) -> Result<rpc::IoHandler, sc_service::Error>,
			(
				sc_consensus_babe::BabeBlockImport<
					Block,
					FullClient<RuntimeApi, ExecutorDispatch>,
					FullGrandpaBlockImport<RuntimeApi, ExecutorDispatch>,
				>,
				sc_finality_grandpa::LinkHalf<
					Block,
					FullClient<RuntimeApi, ExecutorDispatch>,
					FullSelectChain,
				>,
				sc_consensus_babe::BabeLink<Block>,
			),
			sc_finality_grandpa::SharedVoterState,
			Option<Telemetry>,
		),
	>,
	ServiceError,
>
where
	RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi:
		RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
	ExecutorDispatch: NativeExecutionDispatch + 'static,
{
	// First we will initialize some basic node objects and tasks.

	// We take the endpoints that we read from the configuration (specification file) and convert
	// them to a pair of (Telemetry worker  and Telemetry instance) object.
	//
	// The Telemetry worker is a background task which is going to be passed to the task manager
	// object. It's a libp2p web-socked like object. Low level stuff.
	// The Telemetry instance is a object that can be used to send telemetry messages.
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	// Object that is used to execute runtime functions specified by it's name. The runtime can
	// either but native or wasm.
	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	// This creates some basic objects that we need in order to have a node.
	// Client is the substrate client that glues everything together
	// Backend is the DB database where we store data
	// Keystore Container contains our secret and public SR25519, ECDSA or ED25519 keys
	// Task managers manages async tasks
	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	// The Telemetry worker is now being spawn as a tokio async task. If we don't call this the
	// worker will not do anything.
	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	// The basic node initialization is done. We have the basic node objects initialized and the
	// telemetry worker is ready to do some work.

	// The SelectChain trait defines the strategy upon which the head is chosen
	// if multiple forks are present for an opaque definition of "best" in the
	// specific chain build.
	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	// The transaction pool is here to receive and validate incoming transactions. It spawns two
	// validation tasks that will be used by the transaction pool to validate transactions.
	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	// Grandpa Block Importer: This scans each imported block for signals of changing authority set.
	// If the block being imported enacts an authority set change then:
	// - If the current authority set is still live: we import the block
	// - Otherwise, the block must include a valid justification.
	//
	// LinkHalf: Link between the block importer and the background voter.
	//
	// Additional documentation:
	// First, create a block-import wrapper with the block_import function. The GRANDPA worker needs
	// to be linked together with this block import object, so a LinkHalf is returned as well. All
	// blocks imported (from network or consensus or otherwise) must pass through this wrapper,
	// otherwise consensus is likely to break in unexpected ways.
	let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;
	let justification_import = grandpa_block_import.clone();

	// Babe Block Importer: This scans each imported block for epoch change signals. The signals are
	// tracked in a tree (of all forks), and the import logic validates all epoch change
	// transitions, i.e. whether a given epoch change is expected or whether it is missing.
	//
	// BabeLink: State that must be shared between the import queue and the authoring logic.
	let (block_import, babe_link) = sc_consensus_babe::block_import(
		sc_consensus_babe::Config::get(&*client)?,
		grandpa_block_import,
		client.clone(),
	)?;

	let slot_duration = babe_link.config().slot_duration();
	// Checks if the node can author blocks.
	let can_author_with = sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());
	// TODO? I think it's data that it's provided to the runtime from the native environment.
	let create_inherent_data_providers = move |_, ()| async move {
		let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

		let slot = sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
			*timestamp,
			slot_duration,
		);

		let uncles =
			sp_authorship::InherentDataProvider::<<Block as BlockT>::Header>::check_inherents();

		Ok((timestamp, slot, uncles))
	};
	// Starts an import queue for the BABE consensus algorithm.
	// Instantiate a new basic queue, with given verifier.
	//
	// This method returns the import queue, some data that needs to be passed to the block
	// authoring logic (`BabeLink`), and a future that must be run to
	// completion and is responsible for listening to finality notifications and
	// pruning the epoch changes tree.
	//
	// This creates a background task, and calls `on_start` on the justification importer.
	// TODO?
	let import_queue = sc_consensus_babe::import_queue(
		babe_link.clone(),
		block_import.clone(),
		Some(Box::new(justification_import)),
		client.clone(),
		select_chain.clone(),
		create_inherent_data_providers,
		&task_manager.spawn_essential_handle(),
		config.prometheus_registry(),
		can_author_with,
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let import_setup = (block_import, grandpa_link, babe_link);

	// We want to extended the current RPC interface functionality with additional RPC endpoints.
	let (rpc_extensions_builder, rpc_setup) = {
		let (_, grandpa_link, babe_link) = &import_setup;
		// General Node RPC Config
		let client = client.clone();
		let pool = transaction_pool.clone();
		let select_chain = select_chain.clone();
		let chain_spec = config.chain_spec.cloned_box();
		let keystore = keystore_container.sync_keystore();

		// Babe RPC Config
		let babe_config = babe_link.config().clone();
		let shared_epoch_changes = babe_link.epoch_changes().clone();

		// Grandpa RPC Config
		let shared_voter_state = sc_finality_grandpa::SharedVoterState::empty();
		let shared_authority_set = grandpa_link.shared_authority_set().clone();
		let justification_stream = grandpa_link.justification_stream();
		let finality_proof_provider = sc_finality_grandpa::FinalityProofProvider::new_for_service(
			backend.clone(),
			Some(shared_authority_set.clone()),
		);

		// Rest
		let rpc_setup = shared_voter_state.clone();

		// Calling this function will extend the IO interface with additional RPC Pallet specific
		// calls.
		let rpc_extensions_builder = move |deny_unsafe, subscription_executor| {
			let deps = rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				select_chain: select_chain.clone(),
				chain_spec: chain_spec.cloned_box(),
				deny_unsafe,
				babe: rpc::BabeDeps {
					babe_config: babe_config.clone(),
					shared_epoch_changes: shared_epoch_changes.clone(),
					keystore: keystore.clone(),
				},
				grandpa: rpc::GrandpaDeps {
					shared_voter_state: shared_voter_state.clone(),
					shared_authority_set: shared_authority_set.clone(),
					justification_stream: justification_stream.clone(),
					subscription_executor,
					finality_provider: finality_proof_provider.clone(),
				},
			};

			rpc::create_full(deps).map_err(Into::into)
		};

		(rpc_extensions_builder, rpc_setup)
	};

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool,
		other: (rpc_extensions_builder, import_setup, rpc_setup, telemetry),
	})
}

/// Result of [`new_full_base`].
pub struct NewFullBase<RuntimeApi, ExecutorDispatch>
where
	RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi:
		RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
	ExecutorDispatch: NativeExecutionDispatch + 'static,
{
	/// The task manager of the node.
	pub task_manager: TaskManager,
	/// The client instance of the node.
	pub client: Arc<FullClient<RuntimeApi, ExecutorDispatch>>,
	/// The networking service of the node.
	pub network: Arc<NetworkService<Block, <Block as BlockT>::Hash>>,
	/// The transaction pool of the node.
	pub transaction_pool: Arc<TransactionPool<RuntimeApi, ExecutorDispatch>>,
	/// The rpc handlers of the node.
	pub rpc_handlers: RpcHandlers,
}

/// Creates a full service from the configuration.
pub fn new_full_base<RuntimeApi, ExecutorDispatch>(
	mut config: Configuration,
	with_startup_data: impl FnOnce(
		&sc_consensus_babe::BabeBlockImport<
			Block,
			FullClient<RuntimeApi, ExecutorDispatch>,
			FullGrandpaBlockImport<RuntimeApi, ExecutorDispatch>,
		>,
		&sc_consensus_babe::BabeLink<Block>,
	),
) -> Result<NewFullBase<RuntimeApi, ExecutorDispatch>, ServiceError>
where
	RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi:
		RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
	ExecutorDispatch: NativeExecutionDispatch + 'static,
{
	// Initialize the core part of the node
	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (rpc_extensions_builder, import_setup, rpc_setup, mut telemetry),
	} = new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;

	let shared_voter_state = rpc_setup;
	let auth_disc_publish_non_global_ips = config.network.allow_non_globals_in_dht;

	let grandpa_protocol_name = sc_finality_grandpa::protocol_standard_name(
		&client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
		&config.chain_spec,
	);

	config
		.network
		.extra_sets
		.push(sc_finality_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));
	let warp_sync = Arc::new(sc_finality_grandpa::warp_proof::NetworkProvider::new(
		backend.clone(),
		import_setup.1.shared_authority_set().clone(),
		Vec::default(),
	));

	let (network, system_rpc_tx, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync: Some(warp_sync),
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks =
		Some(sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging::default());
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();

	let rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		config,
		backend,
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		network: network.clone(),
		rpc_extensions_builder: Box::new(rpc_extensions_builder),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		system_rpc_tx,
		telemetry: telemetry.as_mut(),
	})?;

	let (block_import, grandpa_link, babe_link) = import_setup;

	(with_startup_data)(&block_import, &babe_link);

	if let sc_service::config::Role::Authority { .. } = &role {
		let proposer = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let can_author_with =
			sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		let client_clone = client.clone();
		let slot_duration = babe_link.config().slot_duration();
		let babe_config = sc_consensus_babe::BabeParams {
			keystore: keystore_container.sync_keystore(),
			client: client.clone(),
			select_chain,
			env: proposer,
			block_import,
			sync_oracle: network.clone(),
			justification_sync_link: network.clone(),
			create_inherent_data_providers: move |parent, ()| {
				let client_clone = client_clone.clone();
				async move {
					let uncles = sc_consensus_uncles::create_uncles_inherent_data_provider(
						&*client_clone,
						parent,
					)?;

					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot =
					sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
						*timestamp,
						slot_duration,
					);

					let storage_proof =
						sp_transaction_storage_proof::registration::new_data_provider(
							&*client_clone,
							&parent,
						)?;

					Ok((timestamp, slot, uncles, storage_proof))
				}
			},
			force_authoring,
			backoff_authoring_blocks,
			babe_link,
			can_author_with,
			block_proposal_slot_portion: SlotProportion::new(0.5),
			max_block_proposal_slot_portion: None,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		};

		let babe = sc_consensus_babe::start_babe(babe_config)?;
		task_manager.spawn_essential_handle().spawn_blocking(
			"babe-proposer",
			Some("block-authoring"),
			babe,
		);
	}

	// Spawn authority discovery module.
	if role.is_authority() {
		let authority_discovery_role =
			sc_authority_discovery::Role::PublishAndDiscover(keystore_container.keystore());
		let dht_event_stream =
			network.event_stream("authority-discovery").filter_map(|e| async move {
				match e {
					Event::Dht(e) => Some(e),
					_ => None,
				}
			});
		let (authority_discovery_worker, _service) =
			sc_authority_discovery::new_worker_and_service_with_config(
				sc_authority_discovery::WorkerConfig {
					publish_non_global_ips: auth_disc_publish_non_global_ips,
					..Default::default()
				},
				client.clone(),
				network.clone(),
				Box::pin(dht_event_stream),
				authority_discovery_role,
				prometheus_registry.clone(),
			);

		task_manager.spawn_handle().spawn(
			"authority-discovery-worker",
			Some("networking"),
			authority_discovery_worker.run(),
		);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore =
		if role.is_authority() { Some(keystore_container.sync_keystore()) } else { None };

	let config = sc_finality_grandpa::Config {
		// FIXME #1578 make this available through chainspec
		gossip_duration: std::time::Duration::from_millis(333),
		justification_period: 512,
		name: Some(name),
		observer_enabled: false,
		keystore,
		local_role: role,
		telemetry: telemetry.as_ref().map(|x| x.handle()),
		protocol_name: grandpa_protocol_name,
	};

	if enable_grandpa {
		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_config = sc_finality_grandpa::GrandpaParams {
			config,
			link: grandpa_link,
			network: network.clone(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state,
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			None,
			sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
		);
	}

	network_starter.start_network();
	Ok(NewFullBase { task_manager, client, network, transaction_pool, rpc_handlers })
}

/// Builds a new service for a full client.
pub fn new_full<RuntimeApi, ExecutorDispatch>(
	config: Configuration,
) -> Result<TaskManager, ServiceError>
where
	RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi:
		RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
	ExecutorDispatch: NativeExecutionDispatch + 'static,
{
	new_full_base(config, |_, _| ())
		.map(|NewFullBase::<RuntimeApi, ExecutorDispatch> { task_manager, .. }| task_manager)
}
