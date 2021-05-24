use super::mock::*;
use crate::{migration, NFTData, NFTDetails};
use codec::{Decode, Encode};
use frame_support::migration::{get_storage_value, put_storage_value};
use frame_support::traits::{OnRuntimeUpgrade, PalletVersion, PALLET_VERSION_STORAGE_KEY_POSTFIX};
use frame_support::Hashable;
use sp_std::vec;

/// Returns the storage key for `PalletVersion` for the given `pallet`.
fn get_pallet_version_storage_key_for_pallet(pallet: &str) -> [u8; 32] {
    let pallet_name = sp_io::hashing::twox_128(pallet.as_bytes());
    let postfix = sp_io::hashing::twox_128(PALLET_VERSION_STORAGE_KEY_POSTFIX);

    let mut final_key = [0u8; 32];
    final_key[..16].copy_from_slice(&pallet_name);
    final_key[16..].copy_from_slice(&postfix);

    final_key
}

fn get_pallet_version(pallet: &str) -> PalletVersion {
    let key = get_pallet_version_storage_key_for_pallet(pallet);
    let value = sp_io::storage::get(&key).expect("Pallet version exists");
    PalletVersion::decode(&mut &value[..]).expect("Pallet version is encoded correctly")
}

fn set_pallet_version(pallet: &str, version: PalletVersion) {
    let key = get_pallet_version_storage_key_for_pallet(pallet);
    sp_io::storage::set(&key, &version.encode());
}

const fn new_pallet_version(major: u16, minor: u8, patch: u8) -> PalletVersion {
    PalletVersion {
        major,
        minor,
        patch,
    }
}

const PALLET_V020: PalletVersion = new_pallet_version(0, 2, 0);
const PALLET_V030: PalletVersion = new_pallet_version(0, 3, 0);
const PALLET_NAME: &str = "NFTs";
const MODULE_NAME: &[u8] = b"NFTs";

#[test]
fn upgrade_from_v020_to_v030() {
    ExtBuilder::default().build().execute_with(|| {
        const ITEM_NAME: &[u8] = b"Data";
        set_pallet_version(PALLET_NAME, PALLET_V020);

        let nft_ids = vec![0u64, 1u64];
        for i in nft_ids.iter() {
            let hash = i.blake2_128_concat();
            let details = migration::v020::NFTDetails {
                offchain_uri: vec![],
                series_id: 0,
            };
            let data = migration::v020::NFTData {
                owner: ALICE,
                details,
                sealed: false,
                locked: false,
            };

            put_storage_value(MODULE_NAME, ITEM_NAME, &hash, data);
        }

        <NFTs as OnRuntimeUpgrade>::on_runtime_upgrade();

        for i in nft_ids.iter() {
            let hash = i.blake2_128_concat();
            let details = NFTDetails::new(vec![], 0, false);

            let actual = get_storage_value::<NFTData<u64>>(MODULE_NAME, ITEM_NAME, &hash);
            let expected = Some(NFTData::new(ALICE, details, false, false));
            assert_eq!(actual, expected);
        }

        assert_eq!(get_pallet_version(PALLET_NAME), PALLET_V030);
    })
}

#[test]
fn upgrade_from_latest_to_latest() {
    ExtBuilder::default().build().execute_with(|| {
        let weight = <NFTs as OnRuntimeUpgrade>::on_runtime_upgrade();
        assert_eq!(weight, 0);
    })
}
