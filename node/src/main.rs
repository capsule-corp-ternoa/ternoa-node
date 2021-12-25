mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod node_rpc;

fn main() -> sc_cli::Result<()> {
    command::run()
}
