mod v5;

use crate::{Config, Pallet, Weight};
use frame_support::traits::StorageVersion;

pub fn migrate<T: Config>() -> Weight {
    let mut weight: Weight = 0;

    let storage_version = StorageVersion::get::<Pallet<T>>();
    if storage_version == 4 {
        log::info!(target: "runtime::marketplace", "Running migration to v5 for marketplace with storage version {:?}", storage_version);
        weight = v5::migrate::<T>();

        StorageVersion::new(5).put::<Pallet<T>>();
    } else {
        log::info!(target: "runtime::marketplace", "No migration was run. Current storage version {:?}", storage_version);
    }

    weight
}
