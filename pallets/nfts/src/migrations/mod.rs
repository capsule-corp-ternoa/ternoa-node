mod v5;
mod v6;

use crate::Config;
/* use frame_support::traits::StorageVersion; */
use frame_support::weights::Weight;

pub fn migrate<T: Config>() -> Weight {
    let weight: Weight = 0;

    /*     let storage_version = StorageVersion::get::<Pallet<T>>();
    if storage_version == 4 {
        weight = v5::migrate::<T>();

        StorageVersion::new(5).put::<Pallet<T>>();
        log::info!("Migration done.");
    } else {
        log::info!(target: "runtime::nfts", "No migration was run. Current storage version {:?}", storage_version);
    } */

    weight
}
