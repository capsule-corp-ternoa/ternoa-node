use crate::{BalanceCaps, NFTCurrency};

use super::{Config, NFTsForSale, Pallet};
use frame_support::traits::{Get, GetPalletVersion, PalletVersion};
use frame_support::weights::Weight;

/// Function that migrates our storage from pallet version 0.2.0 to 0.3.0
pub fn migration<T: Config>() -> Weight {
    let mut weight = Weight::from(0u64);

    let version: PalletVersion =
        <Pallet<T>>::storage_version().unwrap_or(<Pallet<T>>::current_version());

    if version.major == 0 && version.minor == 2 {
        weight = from_v020_to_v030::<T>()
    }

    weight
}

pub fn from_v020_to_v030<T: Config>() -> Weight {
    NFTsForSale::<T>::translate::<(T::AccountId, BalanceCaps<T>), _>(|_key, (seller, balance)| {
        Some((seller, NFTCurrency::CAPS(balance)))
    });

    T::BlockWeights::get().max_block
}
