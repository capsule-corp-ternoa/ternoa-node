use crate::{
	chain_spec,
	cli::{Cli, Subcommand},
	service,
};
use frame_benchmarking_cli::BenchmarkCmd;
use sc_cli::{
	ChainSpec, Error,
	Result, RevertCmd, RuntimeVersion, SubstrateCli,
};
use ternoa_node::service::IdentifyVariant;
use sc_service::error::Error as ServiceError;

#[cfg(feature = "alphanet-native")]
use ternoa_node::service::alphanet_runtime;
#[cfg(feature = "alphanet-native")]
use ternoa_node::service::AlphanetExecutorDispatch;

#[cfg(feature = "mainnet-native")]
use ternoa_node::service::mainnet_runtime;
#[cfg(feature = "mainnet-native")]
use ternoa_node::service::MainnetExecutorDispatch;


impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Substrate Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"alphanet" => Box::new(chain_spec::alphanet_config()?),
			#[cfg(feature = "alphanet-native")]
			"alphanet-dev" | "a-dev" | "dev" => Box::new(chain_spec::alphanet::development_config()),

			"mainnet" => Box::new(chain_spec::mainnet_config()?),
			#[cfg(feature = "mainnet-native")]
			"mainnet-dev" | "m-dev" => Box::new(chain_spec::mainnet::development_config()),

			"" => return Err("Please specify which chain you want to run!".into()),
			path => {
				let path = std::path::PathBuf::from(path);

				let chain_spec =
					Box::new(chain_spec::MainnetChainSpec::from_json_file(path.clone())?)
						as Box<dyn sc_service::ChainSpec>;

				if chain_spec.is_alphanet() {
					Box::new(chain_spec::AlphanetChainSpec::from_json_file(path)?)
				} else {
					chain_spec
				}
			},
		})
	}
	fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		#[cfg(feature = "alphanet-native")]
		if spec.is_alphanet() {
			return &alphanet_runtime::VERSION
		}

		#[cfg(feature = "mainnet-native")]
		{
			return &mainnet_runtime::VERSION
		}

		#[cfg(not(feature = "mainnet-native"))]
		panic!("No runtime feature (alphanet, mainnet) is enabled");
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let (client, _, import_queue, task_manager) = service::new_chain_ops(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => revert(&cli, cmd),
		Some(Subcommand::Benchmark(cmd)) => benchmark(&cli, cmd),
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;

			let registry = &runner.config().prometheus_config.as_ref().map(|cfg| &cfg.registry);
			let task_manager =
				 			sc_service::TaskManager::new(runner.config().tokio_handle.clone(), *registry)
						.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;

			#[cfg(feature = "alphanet-native")]
			if chain_spec.is_cere_dev() {
				return runner.async_run(|config| {
					Ok((cmd.run::<service::alphanet_runtime::Block, service::AlphanetExecutorDispatch>(config), task_manager))
				})
			}

			#[cfg(feature = "mainnet-native")]
			{
				return runner.async_run(|config| {
					Ok((cmd.run::<service::mainnet_runtime::Block, service::MainnetExecutorDispatch>(config), task_manager))
				})
			}

			#[cfg(not(feature = "mainnet-native"))]
			panic!("No runtime feature (mainnet, alphanet) is enabled")
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::ChainInfo(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run::<ternoa_core_primitives::Block>(&config))
		},
		None => {
			let runner = cli.create_runner(&cli.run.base)?;
			runner.run_node_until_exit(|config| async move {
				service::build_full(config, cli.run.no_hardware_benchmarks)
					.map(|full| full.task_manager)
					.map_err(Error::Service)
			})
		},
	}
}

fn ensure_dev(spec: &Box<dyn sc_service::ChainSpec>) -> Result<()> {
	if spec.is_dev() {
		Ok(())
	} else {
		panic!("Only Dev Specification Allowed!")
	}
}

macro_rules! with_runtime {
	($chain_spec:expr, $code:expr) => {
		#[cfg(feature = "alphanet-native")]
		if $chain_spec.is_alphanet() {
			#[allow(unused_imports)]
			use alphanet_runtime::Block;
			#[allow(unused_imports)]
			use alphanet_runtime::RuntimeApi;
			#[allow(unused_imports)]
			use AlphanetExecutorDispatch as ExecutorDispatch;

			return $code
		}

		#[cfg(feature = "mainnet-native")]
		{
			#[allow(unused_imports)]
			use mainnet_runtime::Block;
			#[allow(unused_imports)]
			use mainnet_runtime::RuntimeApi;
			#[allow(unused_imports)]
			use MainnetExecutorDispatch as ExecutorDispatch;

			return $code
		}

		#[cfg(not(feature = "mainnet-native"))]
		panic!("No runtime feature (alphanet, mainnet) is enabled");
	};
}

macro_rules! unwrap_client {
	(
		$client:ident,
		$code:expr
	) => {
		match $client.as_ref() {
			#[cfg(feature = "mainnet-native")]
			ternoa_client::Client::Mainnet($client) => $code,
			#[cfg(feature = "alphanet-native")]
			ternoa_client::Client::Alphanet($client) => $code,
			#[allow(unreachable_patterns)]
			_ => Err(Error::Service(ServiceError::Other(
				"No runtime feature  is enabled".to_string(),
			))),
		}
	};
}
fn benchmark(cli: &Cli, cmd: &BenchmarkCmd) -> Result<()> {
	if !cfg!(feature = "runtime-benchmarks") {
		return Err("Benchmarking wasn't enabled when building the node. \
					 You can enable it with `--features runtime-benchmarks`."
			.into())
	}

	let runner = cli.create_runner(cmd)?;
	let chain_spec = &runner.config().chain_spec.cloned_box();

	match cmd {
		BenchmarkCmd::Pallet(cmd) => {
			ensure_dev(chain_spec)?;
			with_runtime!(chain_spec, {
				runner.sync_run(|config| cmd.run::<Block, ExecutorDispatch>(config))
			});
		},
		#[cfg(not(feature = "runtime-benchmarks"))]
		BenchmarkCmd::Storage(_) =>
			Err("Storage benchmarking can be enabled with `--features runtime-benchmarks`.".into()),
		#[cfg(feature = "runtime-benchmarks")]
		BenchmarkCmd::Storage(_) => {
			todo!()
			// with_runtime!(chain_spec, {
			// 	runner.sync_run(|config| {
			// 		// ensure that we keep the task manager alive
			// 		let partial = new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
			// 		let db = partial.backend.expose_db();
			// 		let storage = partial.backend.expose_storage();

			// 		cmd.run(config, partial.client, db, storage)
			// 	})
			// });
		},
		BenchmarkCmd::Overhead(_cmd) => {
			print!("BenchmarkCmd::Overhead is not supported");
			unimplemented!()
		},
		BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
			let (client, _, _, _) = service::new_chain_ops(&config)?;
			unwrap_client!(client, cmd.run(client.clone()))
		}),
		BenchmarkCmd::Extrinsic(_cmd) => todo!(),
		_ => panic!("Benchmark Command not implement."),
	}
}

fn revert(cli: &Cli, cmd: &RevertCmd) -> Result<()> {
	let runner = cli.create_runner(cmd)?;
	runner.async_run(|config| {
		let (client, backend, _, task_manager) = service::new_chain_ops(&config)?;
		let aux_revert = Box::new(|client, backend, blocks| {
			service::revert_backend(client, backend, blocks)?;
			Ok(())
		});
		Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
	})
}