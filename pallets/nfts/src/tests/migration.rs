use super::mock::*;
use crate::NFTId;
use frame_support::traits::{OnRuntimeUpgrade, StorageVersion};

frame_support::generate_storage_alias!(
    Nfts, Total => Value<NFTId>
);

mod version_6 {
    use super::*;

    #[test]
    fn upgrade_from_v5_to_v6() {
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
