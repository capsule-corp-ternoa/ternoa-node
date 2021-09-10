/* use super::Config;
use crate::MarketplaceId;
use crate::{MarketplaceIdGenerator, MarketplaceInformation, MarketplaceType, Marketplaces};
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::Blake2_128Concat;

frame_support::generate_storage_alias!(
    Marketplace,  MarketplaceCount => Value<MarketplaceId>
);

frame_support::generate_storage_alias!(
    Marketplace,  MarketplaceOwners<T: Config> => Map<(Blake2_128Concat, MarketplaceId), T::AccountId>
);

pub fn migrate<T: Config>() -> Weight {
    log::info!("Migrating marketplace to StorageVersion::V5");

    let count = MarketplaceOwners::<T>::iter().count();
    MarketplaceOwners::<T>::drain().for_each(|x| {
        Marketplaces::<T>::insert(
            x.0,
            MarketplaceInformation::<T>::new(MarketplaceType::Public, 0, x.1, Default::default()),
        );
    });

    Marketplaces::<T>::insert(
        0,
        MarketplaceInformation::<T>::new(
            MarketplaceType::Public,
            0,
            Default::default(),
            Default::default(),
        ),
    );

    MarketplaceCount::kill();
    MarketplaceIdGenerator::<T>::set(count as u32);

    T::BlockWeights::get().max_block
}
 */
