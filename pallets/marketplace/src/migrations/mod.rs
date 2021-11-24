pub mod v6;
pub mod v7;

use crate::{Config, Pallet, Weight};
use frame_support::traits::StorageVersion;

pub fn migrate<T: Config>() -> Weight {
    let mut weight: Weight = 0;
    let storage_version = StorageVersion::get::<Pallet<T>>();

    if storage_version == 7 {
        log::info!(target: "runtime::marketplace", "No migration was run. Current storage version {:?}", storage_version);
        return weight;
    }

    if storage_version == 6 {
        log::info!(target: "runtime::marketplace", "Migrating marketplace pallet to StorageVersion V7");

        weight = v7::migrate::<T>();
        StorageVersion::new(7).put::<Pallet<T>>();

        log::info!(target: "runtime::marketplace", "Migration done.");
    }

    weight
}
