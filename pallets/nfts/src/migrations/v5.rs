use super::Config;
use crate::{NFTId, NftIdGenerator};
use frame_support::traits::Get;
use frame_support::weights::Weight;

frame_support::generate_storage_alias!(
    Nfts, Total => Value<NFTId>
);

pub fn migrate<T: Config>() -> Weight {
    log::info!("Migrating nfts to StorageVersion::V5");

    let total = Total::take().unwrap_or(0);
    Total::kill();

    NftIdGenerator::<T>::set(total);

    T::BlockWeights::get().max_block
}
