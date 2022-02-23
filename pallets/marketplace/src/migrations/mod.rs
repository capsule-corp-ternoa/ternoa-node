/* pub mod v6;
pub mod v7;

use crate::{Config, Pallet, Weight};
use frame_support::traits::StorageVersion;

pub fn migrate<T: Config>() -> Weight {
	let weight: Weight = 0;
	let storage_version = StorageVersion::get::<Pallet<T>>();

	if storage_version == 7 {
		log::info!(target: "runtime::marketplace", "Marketplace pallet: no migration was run.");
		return weight;
	}

	if storage_version == 6 {
		log::info!(target: "runtime::marketplace", "Marketplace pallet: migrating to StorageVersion V7");

		weight = v7::migrate::<T>();
		StorageVersion::new(7).put::<Pallet<T>>();

		log::info!(target: "runtime::marketplace", "Marketplace pallet: migration to StorageVersion V7 done");
	}

	weight
}
 */
