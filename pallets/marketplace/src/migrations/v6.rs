use super::Config;
use crate::{MarketplaceInformation, Marketplaces};
use frame_support::traits::Get;
use frame_support::weights::Weight;

pub mod v5 {
    use crate::MarketplaceType;

    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};

    use crate::Config;
    use codec::{Decode, Encode};
    use sp_runtime::RuntimeDebug;
    use sp_std::vec::Vec;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct MarketplaceInformation<T: Config> {
        pub kind: MarketplaceType,
        pub commission_fee: u8,
        pub owner: T::AccountId,
        pub allow_list: Vec<T::AccountId>,
    }
}

pub fn migrate<T: Config>() -> Weight {
    log::info!("Migrating marketplace to StorageVersion::V6");
    Marketplaces::<T>::translate::<v5::MarketplaceInformation<T>, _>(|key, info| {
        if key == 0 {
            return Some(MarketplaceInformation::<T>::new(
                info.kind,
                info.commission_fee,
                info.owner,
                info.allow_list,
                "Ternoa Marketplace".into(),
            ));
        }

        return Some(MarketplaceInformation::<T>::new(
            info.kind,
            info.commission_fee,
            info.owner,
            info.allow_list,
            "User Marketplace".into(),
        ));
    });

    T::BlockWeights::get().max_block
}
