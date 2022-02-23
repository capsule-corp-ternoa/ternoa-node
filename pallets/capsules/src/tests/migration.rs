use super::mock::*;
use frame_support::traits::OnRuntimeUpgrade;

mod version_1 {
	use super::*;

	frame_support::generate_storage_alias!(
		TernoaCapsules, CapsuleMintFee => Value<()>
	);

	#[test]
	fn set_to_version_1() {
		ExtBuilder::default().build().execute_with(|| {
			CapsuleMintFee::kill();

			let weight = <TernoaCapsules as OnRuntimeUpgrade>::on_runtime_upgrade();
			let mint_fee = TernoaCapsules::capsule_mint_fee();

			// Check NFT mint fee
			assert_eq!(weight, 1);
			assert_eq!(mint_fee, 1000000000000000000000);
		})
	}
}

#[test]
fn upgrade_from_latest_to_latest() {
	ExtBuilder::default().build().execute_with(|| {
		let weight = <TernoaCapsules as OnRuntimeUpgrade>::on_runtime_upgrade();
		assert_eq!(weight, 0);
	})
}
