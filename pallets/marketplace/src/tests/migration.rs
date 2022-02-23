/* use super::mock::*;
use crate::migrations::v6::v6;
use crate::migrations::v7::v7;
use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};

pub type MPT6 = v6::MarketplaceType;
pub type MPT7 = v7::MarketplaceType;

mod version_7 {
	use super::*;

	fn create_mp(
		id: v6::MarketplaceId,
		owner: u64,
		kind: v6::MarketplaceType,
		fee: u8,
		list: Vec<u64>,
		name: Vec<u8>,
	) {
		v6::insert_marketplace::<Test>(id, owner, kind, fee, list, name);
	}

	fn check_mp(
		id: v6::MarketplaceId,
		owner: u64,
		kind: v7::MarketplaceType,
		fee: u8,
		list: Vec<u64>,
		name: Vec<u8>,
		map: &v7::StorageMarketplaces<Test>,
	) {
		let empty: Vec<u64> = vec![];

		// Old
		assert_eq!(map.get(&id).unwrap().owner, owner);
		assert_eq!(map.get(&id).unwrap().kind, kind);
		assert_eq!(map.get(&id).unwrap().commission_fee, fee);
		assert_eq!(map.get(&id).unwrap().allow_list, list);
		assert_eq!(map.get(&id).unwrap().name, name);

		// New
		assert_eq!(map.get(&id).unwrap().disallow_list, empty);
		assert_eq!(map.get(&id).unwrap().uri, None);
		assert_eq!(map.get(&id).unwrap().logo_uri, None);
	}

	#[test]
	fn upgrade_from_v6_to_v7() {
		ExtBuilder::default().build().execute_with(|| {
			create_mp(0, ALICE, MPT6::Private, 0, vec![], "Charma".into());
			create_mp(1, ALICE, MPT6::Public, 1, vec![1], "Charme".into());
			create_mp(2, ALICE, MPT6::Private, 2, vec![2], "Chari".into());
			create_mp(3, BOB, MPT6::Public, 3, vec![3], "Squirt".into());
			create_mp(4, BOB, MPT6::Private, 4, vec![4], "Wartor".into());

			StorageVersion::put::<Marketplace>(&StorageVersion::new(6));
			let weight = <Marketplace as OnRuntimeUpgrade>::on_runtime_upgrade();
			assert_ne!(weight, 0);

			let mps = v7::get_marketplaces::<Test>();
			let mp_mint_fee = v7::get_nft_mint_fee::<Test>();

			// Check Marketplaces
			assert_eq!(mps.len(), 5);
			check_mp(0, ALICE, MPT7::Private, 0, vec![], "Charma".into(), &mps);
			check_mp(1, ALICE, MPT7::Public, 1, vec![1], "Charme".into(), &mps);
			check_mp(2, ALICE, MPT7::Private, 2, vec![2], "Chari".into(), &mps);
			check_mp(3, BOB, MPT7::Public, 3, vec![3], "Squirt".into(), &mps);
			check_mp(4, BOB, MPT7::Private, 4, vec![4], "Wartor".into(), &mps);

			// Check NFT mint fee
			assert_eq!(mp_mint_fee, 10000000000000000000000);
		})
	}
}

#[test]
fn upgrade_from_latest_to_latest() {
	ExtBuilder::default().build().execute_with(|| {
		let weight = <NFTs as OnRuntimeUpgrade>::on_runtime_upgrade();
		assert_eq!(weight, 0);
	})
}
 */
