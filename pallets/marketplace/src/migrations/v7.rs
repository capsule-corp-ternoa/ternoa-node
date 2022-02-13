/* use super::Config;
use frame_support::traits::Get;
use frame_support::weights::Weight;

pub mod v7 {
    use super::super::v6::v6;
    use crate::Config;
    use codec::{Decode, Encode};
    use frame_support::pallet_prelude::{OptionQuery, ValueQuery};
    use frame_support::traits::Currency;
    use frame_support::Blake2_128Concat;
    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};
    use sp_runtime::RuntimeDebug;
    use sp_std::collections::btree_map::BTreeMap;
    use sp_std::convert::TryInto;
    use sp_std::vec::Vec;

    pub type MarketplaceId = u32;

    #[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum MarketplaceType {
        Public,
        Private,
    }

    impl From<v6::MarketplaceType> for MarketplaceType {
        fn from(value: v6::MarketplaceType) -> Self {
            match value {
                v6::MarketplaceType::Public => Self::Public,
                v6::MarketplaceType::Private => Self::Private,
            }
        }
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct MarketplaceInformation<T: Config> {
        pub kind: MarketplaceType,
        pub commission_fee: u8,
        pub owner: T::AccountId,
        pub allow_list: Vec<T::AccountId>,
        pub disallow_list: Vec<T::AccountId>,
        pub name: Vec<u8>,
        pub uri: Option<Vec<u8>>,
        pub logo_uri: Option<Vec<u8>>,
    }

    pub type BalanceCaps<T> =
        <<T as Config>::CurrencyCaps as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    frame_support::generate_storage_alias!(
        Marketplace, Marketplaces<T: Config> => Map<
            (Blake2_128Concat, MarketplaceId),
            MarketplaceInformation<T>,
            OptionQuery
        >
    );

    frame_support::generate_storage_alias!(
        Marketplace, MarketplaceMintFee<T: Config> => Value<BalanceCaps<T>, ValueQuery>
    );

    // Define helper types
    pub type StorageMarketplaces<T> = BTreeMap<MarketplaceId, MarketplaceInformation<T>>;

    pub fn migrate_marketplaces<T: Config>() {
        Marketplaces::<T>::translate::<v6::MarketplaceInformation<T>, _>(|_, value| {
            return Some(MarketplaceInformation {
                kind: value.kind.into(),
                commission_fee: value.commission_fee,
                owner: value.owner,
                allow_list: value.allow_list,
                disallow_list: Default::default(),
                name: value.name,
                uri: None,
                logo_uri: None,
            });
        });
    }

    pub fn create_marketplace_mint_fee<T: Config>() {
        let fee: BalanceCaps<T> = 10000000000000000000000u128.try_into().ok().unwrap();
        MarketplaceMintFee::<T>::put(fee);
    }

    #[allow(dead_code)]
    pub fn get_marketplaces<T: Config>() -> StorageMarketplaces<T> {
        Marketplaces::<T>::iter().map(|x| x).collect()
    }

    #[allow(dead_code)]
    pub fn get_nft_mint_fee<T: Config>() -> BalanceCaps<T> {
        MarketplaceMintFee::<T>::get()
    }
}

pub fn migrate<T: Config>() -> Weight {
    // Migrate Marketplaces
    v7::migrate_marketplaces::<T>();

    // Create MarketplaceMintFee
    v7::create_marketplace_mint_fee::<T>();

    T::BlockWeights::get().max_block
}
 */
