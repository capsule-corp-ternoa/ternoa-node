use crate::{NFTCurrency, SaleInformation};

use super::{Config, NFTsForSale, Pallet};
use frame_support::traits::{Get, GetPalletVersion, PalletVersion};
use frame_support::weights::Weight;

/// Function that migrates our storage from pallet version 0.3.0 to 0.4.0
pub fn migration<T: Config>() -> Weight {
    let mut weight = Weight::from(0u64);

    let version: PalletVersion =
        <Pallet<T>>::storage_version().unwrap_or(<Pallet<T>>::current_version());

    if version.major == 0 && version.minor == 3 {
        weight = from_v030_to_v040::<T>()
    }

    weight
}

pub fn from_v030_to_v040<T: Config>() -> Weight {
    NFTsForSale::<T>::translate::<(T::AccountId, NFTCurrency<T>), _>(
        |_key, (account_id, price)| {
            return Some(SaleInformation::<T>::new(account_id, price, 0));
        },
    );

    T::BlockWeights::get().max_block
}
