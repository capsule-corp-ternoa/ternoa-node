/* pub mod v5;
pub mod v6;

use crate::{Config, Pallet};
use frame_support::traits::StorageVersion;
use frame_support::weights::Weight;

pub fn migrate<T: Config>() -> Weight {
	let mut weight: Weight = 0;
	let storage_version = StorageVersion::get::<Pallet<T>>();

	if storage_version == 6 {
		log::info!(target: "runtime::nfts", "Nfts pallet: migration was run",);
		return weight;
	}

	if storage_version == 5 {
		log::info!(target: "runtime::nfts", "Nfts pallet: migrating to StorageVersion V6");

		weight = v6::migrate::<T>();
		StorageVersion::new(6).put::<Pallet<T>>();

		log::info!(target: "runtime::nfts", "Nfts pallet: migration to StorageVersion V6 done");
	}

	weight
}
 */
