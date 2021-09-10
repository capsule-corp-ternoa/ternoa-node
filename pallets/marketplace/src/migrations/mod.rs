pub mod v5;
pub mod v6;

use crate::{Config, Pallet, Weight};
use frame_support::traits::StorageVersion;

pub fn migrate<T: Config>() -> Weight {
    let mut weight: Weight = 0;

    let storage_version = StorageVersion::get::<Pallet<T>>();
    if storage_version == 6 {
        log::info!(target: "runtime::marketplace", "No migration was run. Current storage version {:?}", storage_version);
        return weight;
    }

    /*     if storage_version == 4 {
        weight = v5::migrate::<T>();
        StorageVersion::new(5).put::<Pallet<T>>();
    } */

    if storage_version == 5 {
        weight = v6::migrate::<T>();
        StorageVersion::new(6).put::<Pallet<T>>();
    }

    log::info!("Migration done.");

    weight
}
