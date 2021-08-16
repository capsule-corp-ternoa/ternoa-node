use std::usize;

use super::mock::*;
use crate::{NFTCurrency, SaleInformation};
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

const PALLET_V030: PalletVersion = new_pallet_version(0, 3, 0);
const PALLET_V040: PalletVersion = new_pallet_version(0, 4, 0);
const PALLET_NAME: &str = "Marketplace";
const MODULE_NAME: &[u8] = b"Marketplace";

pub type AccountId = <Test as frame_system::Config>::AccountId;

#[test]
fn upgrade_from_v030_to_v040() {
    ExtBuilder::default().build().execute_with(|| {
        const ITEM_NAME: &[u8] = b"NFTsForSale";
        set_pallet_version(PALLET_NAME, PALLET_V030);

        let nft_ids = vec![0u64, 1u64];
        let accounts = vec![ALICE, BOB];
        let balances = vec![
            NFTCurrency::<Test>::CAPS(10),
            NFTCurrency::<Test>::TIIME(10),
        ];
        for i in nft_ids.iter() {
            let hash = i.blake2_128_concat();
            let account_id: AccountId = accounts[*i as usize].into();
            let balance: NFTCurrency<Test> = balances[*i as usize].clone();

            let data = (account_id, balance);

            put_storage_value(MODULE_NAME, ITEM_NAME, &hash, data);
        }

        <Marketplace as OnRuntimeUpgrade>::on_runtime_upgrade();

        for i in nft_ids.iter() {
            let hash = i.blake2_128_concat();
            let actual = get_storage_value::<SaleInformation<Test>>(MODULE_NAME, ITEM_NAME, &hash);

            let account_id = accounts[*i as usize];
            let price = balances[*i as usize];
            let expected = Some(SaleInformation::new(account_id, price, 0));

            assert_eq!(actual, expected);
        }

        assert_eq!(get_pallet_version(PALLET_NAME), PALLET_V040);
    })
}

#[test]
fn upgrade_from_latest_to_latest() {
    ExtBuilder::default().build().execute_with(|| {
        let weight = <NFTs as OnRuntimeUpgrade>::on_runtime_upgrade();
        assert_eq!(weight, 0);
    })
}
