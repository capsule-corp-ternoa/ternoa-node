mod v3;

use crate::{Config, Pallet};
use frame_support::traits::StorageVersion;
use frame_support::weights::Weight;

pub fn migrate<T: Config>() -> Weight {
    let mut weight: Weight = 0;

    let storage_version = StorageVersion::get::<Pallet<T>>();
    if storage_version == 2 {
        log::info!(target: "runtime::nfts", "Running migration to v3 for nfts with storage version {:?}", storage_version);
        weight = v3::migrate::<T>();

        StorageVersion::new(3).put::<Pallet<T>>();
    } else {
        log::info!(target: "runtime::nfts", "No migration was run. Current storage version {:?}", storage_version);
    }

    weight
}
