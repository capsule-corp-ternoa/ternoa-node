#![cfg_attr(not(feature = "std"), no_std)]

pub mod authority;
pub mod constants;
pub mod elections;
pub mod other;
pub mod staking;
pub mod system;
pub mod transactions;
pub mod voter_bags;

#[macro_export]
macro_rules! prod_or_fast {
	($prod:expr, $test:expr) => {
		if cfg!(feature = "fast-runtime") {
			$test
		} else {
			$prod
		}
	};
	($prod:expr, $test:expr, $env:expr) => {
		if cfg!(feature = "fast-runtime") {
			core::option_env!($env).map(|s| s.parse().ok()).flatten().unwrap_or($test)
		} else {
			$prod
		}
	};
}
