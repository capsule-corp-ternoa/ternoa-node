fn main() -> sc_cli::Result<()> {
	#[cfg(feature = "mainnet-native")]
	panic!("Mainnet-Native");

	#[cfg(feature = "chaosnet-native")]
	panic!("Chaosnet-Native");

	#[cfg(feature = "alphanet-native")]
	panic!("Alphanet-Native");

	ternoa_cli::run()
}
