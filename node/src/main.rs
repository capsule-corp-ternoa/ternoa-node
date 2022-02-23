mod chain_spec;
#[macro_use]
mod service;
mod command;

fn main() -> sc_cli::Result<()> {
	ternoa_cli::run()
}
