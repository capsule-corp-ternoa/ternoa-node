mod v5;
mod v6;

use crate::{Config, Pallet};
use frame_support::traits::StorageVersion;
use frame_support::weights::Weight;

pub fn migrate<T: Config>() -> Weight {
    let mut weight: Weight = 0;

    let storage_version = StorageVersion::get::<Pallet<T>>();

    if storage_version == 6 {
        log::info!(target: "runtime::nfts", "No migration was run. Current storage version {:?}", storage_version);
        return weight;
    }

    if storage_version == 5 {
        log::info!("Migrating nfts pallet to StorageVersion::V6");

        weight = v6::migrate::<T>();
        StorageVersion::new(6).put::<Pallet<T>>();

        log::info!("Migration done.");
    }

    weight
}
