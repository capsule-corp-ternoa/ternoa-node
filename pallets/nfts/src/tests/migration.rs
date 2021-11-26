use super::mock::*;
use crate::migrations::v5::v5;
use crate::migrations::v6::v6;
use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};

mod version_6 {
    use super::*;
    use sp_std::collections::btree_map::BTreeMap;

    fn create_nft(owner: u64, id: v5::NFTId, uri: Vec<u8>, series_id: v5::NFTSeriesId) {
        v5::insert_nft::<Test>(owner, id, uri, series_id);
        if series_id != 0 {
            let details = v5::NFTSeriesDetails {
                owner,
                nfts: Vec::new(),
            };
            v5::insert_series::<Test>(series_id, details)
        }
    }

    fn check_nft(
        owner: u64,
        id: v6::NFTId,
        uri: Vec<u8>,
        series_id: v6::NFTSeriesId,
        map: &v6::NewData<u64>,
    ) {
        assert_eq!(map.get(&id).unwrap().owner, owner);
        assert_eq!(map.get(&id).unwrap().ipfs_reference, uri);
        assert_eq!(map.get(&id).unwrap().series_id, series_id);
        assert_eq!(map.get(&id).unwrap().locked, false);
    }

    #[test]
    fn upgrade_from_v5_to_v6() {
        ExtBuilder::default().build().execute_with(|| {
            create_nft(ALICE, 0, vec![48], 1);
            create_nft(ALICE, 1, vec![48], 2);
            create_nft(ALICE, 2, vec![48], 0);
            create_nft(BOB, 3, vec![48], 0);
            create_nft(ALICE, 4, vec![48], 1);
            create_nft(BOB, 5, vec![48], 4);
            create_nft(ALICE, 6, vec![48], 0);
            create_nft(ALICE, 7, vec![48], 0);

            StorageVersion::put::<NFTs>(&StorageVersion::new(5));
            let weight = <NFTs as OnRuntimeUpgrade>::on_runtime_upgrade();
            assert_ne!(weight, 0);

            let new_nfts = v6::get_nfts::<Test>();
            let new_series = v6::get_series::<Test>();

            assert_eq!(new_nfts.len(), 8);
            assert_eq!(new_series.len(), 7);

            check_nft(ALICE, 0, vec![48], "1".into(), &new_nfts);
            check_nft(ALICE, 1, vec![48], "2".into(), &new_nfts);
            check_nft(ALICE, 2, vec![48], "0".into(), &new_nfts);
            check_nft(BOB, 3, vec![48], "3".into(), &new_nfts);
            check_nft(ALICE, 4, vec![48], "1".into(), &new_nfts);
            check_nft(BOB, 5, vec![48], "4".into(), &new_nfts);
            check_nft(ALICE, 6, vec![48], "5".into(), &new_nfts);
            check_nft(ALICE, 7, vec![48], "6".into(), &new_nfts);
        })
    }
}

/* #[test]
fn upgrade_from_v4_to_v5() {
    ExtBuilder::default().build().execute_with(|| {
        StorageVersion::put::<NFTs>(&StorageVersion::new(4));

        let id = 3;
        Total::put(id);
        let weight = <NFTs as OnRuntimeUpgrade>::on_runtime_upgrade();

        assert_eq!(NftIdGenerator::<Test>::get(), id);
        assert_eq!(StorageVersion::get::<NFTs>(), 5);
        assert_ne!(weight, 0);
    })
}

#[test]
fn upgrade_from_latest_to_latest() {
    ExtBuilder::default().build().execute_with(|| {
        let weight = <NFTs as OnRuntimeUpgrade>::on_runtime_upgrade();
        assert_eq!(weight, 0);
    })
}
 */
