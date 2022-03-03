#[derive(Debug, clap::Parser)]
pub struct Cli {
	/// Possible subcommand with parameters.
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub run: sc_cli::RunCmd,
}

/// Possible subcommands of the main binary.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// The custom benchmark subcommmand benchmarking runtime pallets.
	#[clap(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// The custom inspect subcommmand for decoding blocks and extrinsics.
	#[clap(
		name = "inspect",
		about = "Decode given block or extrinsic using current native runtime."
	)]
	Inspect(node_inspect::cli::InspectCmd),

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,

	/// Key management cli utilities
	#[clap(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
	Verify(sc_cli::VerifyCmd),

	/// Generate a seed that provides a vanity address.
	Vanity(sc_cli::VanityCmd),

	/// Sign a message, with a given (secret) key.
	Sign(sc_cli::SignCmd),
}
