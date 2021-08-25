mod v4;

use crate::{Config, Pallet, Weight};
use frame_support::traits::StorageVersion;

pub fn migrate<T: Config>() -> Weight {
    let mut weight: Weight = 0;

    let storage_version = StorageVersion::get::<Pallet<T>>();
    if storage_version == 3 {
        log::info!(target: "runtime::marketplace", "Running migration to v4 for marketplace with storage version {:?}", storage_version);
        weight = v4::migrate::<T>();

        StorageVersion::new(4).put::<Pallet<T>>();
    } else {
        log::info!(target: "runtime::marketplace", "No migration was run. Current storage version {:?}", storage_version);
    }

    weight
}
