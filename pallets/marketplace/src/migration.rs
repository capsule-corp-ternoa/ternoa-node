use crate::{NFTCurrency, SaleInformation};

use super::{Config, NFTsForSale, Pallet};
use frame_support::traits::{Get, StorageVersion};
use frame_support::weights::Weight;

/// Function that migrates our storage from pallet version 0.3.0 to 0.4.0
pub fn migration<T: Config>() -> Weight {
    let mut weight = Weight::from(0u64);

    let storage_version = StorageVersion::get::<Pallet<T>>();

    // TODO
    /*     panic!();
     */
    if storage_version <= 3 {
        weight = from_v3_to_v4::<T>();
    }

    weight
}

pub fn from_v3_to_v4<T: Config>() -> Weight {
    NFTsForSale::<T>::translate::<(T::AccountId, NFTCurrency<T>), _>(
        |_key, (account_id, price)| {
            return Some(SaleInformation::<T>::new(account_id, price, 0));
        },
    );

    T::BlockWeights::get().max_block
}
