use crate::cli::{Cli, Subcommand};
use sc_cli::{ChainSpec, Result, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use ternoa_client::Block;
use ternoa_service::{chain_spec, new_partial, IdentifyVariant};

#[cfg(feature = "chaosnet-native")]
use chaosnet_runtime;

#[cfg(feature = "alphanet-native")]
use alphanet_runtime;

#[cfg(feature = "mainnet-native")]
use mainnet_runtime;

#[cfg(feature = "chaosnet-native")]
use ternoa_client::ChaosnetExecutorDispatch;

#[cfg(feature = "alphanet-native")]
use ternoa_client::AlphanetExecutorDispatch;

#[cfg(feature = "mainnet-native")]
use ternoa_client::MainnetExecutorDispatch;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Ternoa Node".into()
	}

	fn impl_version() -> String {
		env!("CARGO_PKG_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/capsule-corp-ternoa/chain/issues".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let spec = match id {
			"chaosnet" => Box::new(chain_spec::chaosnet_config()?),
			"alphanet" => Box::new(chain_spec::alphanet_config()?),
			"mainnet" => Box::new(chain_spec::mainnet_config()?),

			#[cfg(feature = "chaosnet-native")]
			"chaosnet-dev" | "dev" => Box::new(chain_spec::chaosnet::development_config()),
			#[cfg(feature = "alphanet-native")]
			"alphanet-dev" | "dev" => Box::new(chain_spec::alphanet::development_config()),
			#[cfg(feature = "mainnet-native")]
			"mainnet-dev" | "dev" => Box::new(chain_spec::mainnet::development_config()),

			"" => return Err("Please specify which chain you want to run!".into()),

			path => {
				let path = std::path::PathBuf::from(path);

				let chain_spec =
					Box::new(chain_spec::MainnetChainSpec::from_json_file(path.clone())?)
						as Box<dyn sc_service::ChainSpec>;

				if chain_spec.is_chaosnet() {
					Box::new(chain_spec::ChaosnetChainSpec::from_json_file(path.clone())?)
				} else if chain_spec.is_alphanet() {
					Box::new(chain_spec::AlphanetChainSpec::from_json_file(path.clone())?)
				} else {
					chain_spec
				}
			},
		};
		Ok(spec)
	}

	fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		#[cfg(feature = "chaosnet-native")]
		if spec.is_chaosnet() {
			return &chaosnet_runtime::VERSION
		}

		#[cfg(feature = "alphanet-native")]
		if spec.is_alphanet() {
			return &alphanet_runtime::VERSION
		}

		#[cfg(feature = "mainnet-native")]
		{
			return &mainnet_runtime::VERSION
		}

		#[cfg(not(feature = "mainnet-native"))]
		panic!("No runtime feature (chaosnet, alphanet, mainnet) is enabled");
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				service::new_full(config).map_err(sc_cli::Error::Service)
			})
		},
		Some(Subcommand::Inspect(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			#[cfg(feature = "chaosnet-native")]
			if chain_spec.is_chaosnet() {
				return runner.sync_run(|config| {
					cmd.run::<chaosnet_runtime::Block, chaosnet_runtime::RuntimeApi, ChaosnetExecutorDispatch>(config)
				})
			}

			#[cfg(feature = "alphanet-native")]
			if chain_spec.is_alphanet() {
				return runner.sync_run(|config| {
					cmd.run::<alphanet_runtime::Block, alphanet_runtime::RuntimeApi, AlphanetExecutorDispatch>(config)
				})
			}

			#[cfg(feature = "mainnet-native")]
			{
				return runner.sync_run(|config| {
					runner.sync_run(|config| {
						cmd.run::<mainnet_runtime::Block, mainnet_runtime::RuntimeApi, MainnetExecutorDispatch>(config)
					})
				})
			}

			#[cfg(not(feature = "mainnet-native"))]
			panic!("No runtime feature (chaosnet, alphanet, mainnet) is enabled")
		},
		Some(Subcommand::Benchmark(cmd)) => {
			if !cfg!(feature = "runtime-benchmarks") {
				return Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
					.into())
			}

			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			#[cfg(feature = "chaosnet-native")]
			if chain_spec.is_chaosnet() {
				return runner.sync_run(|config| {
					cmd.run::<chaosnet_runtime::Block, ChaosnetExecutorDispatch>(config)
				})
			}

			#[cfg(feature = "alphanet-native")]
			if chain_spec.is_alphanet() {
				return runner.sync_run(|config| {
					cmd.run::<alphanet_runtime::Block, AlphanetExecutorDispatch>(config)
				})
			}

			#[cfg(feature = "mainnet-native")]
			{
				return runner.sync_run(|config| {
					cmd.run::<mainnet_runtime::Block, MainnetExecutorDispatch>(config)
				})
			}

			#[cfg(not(feature = "mainnet-native"))]
			panic!("No runtime feature (chaosnet, alphanet, mainnet) is enabled")
		},
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, .. } = new_partial(&config)?;
				Ok((cmd.run(client, backend), task_manager))
			})
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
						.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;

				Ok((cmd.run::<Block, ExecutorDispatch>(config), task_manager))
			})
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
	}
}
