/* pub mod v6 {
    use crate::Config;
    use codec::{Decode, Encode};
    use frame_support::Blake2_128Concat;
    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};
    use sp_runtime::RuntimeDebug;
    use sp_std::vec::Vec;

    #[allow(dead_code)]
    pub type MarketplaceId = u32;

    #[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum MarketplaceType {
        Public,
        Private,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct MarketplaceInformation<T: Config> {
        pub kind: MarketplaceType,
        pub commission_fee: u8,
        pub owner: T::AccountId,
        pub allow_list: Vec<T::AccountId>,
        pub name: Vec<u8>,
    }

    frame_support::generate_storage_alias!(
        Marketplace, Marketplaces<T: Config> => Map<
            (Blake2_128Concat, MarketplaceId),
            MarketplaceInformation<T>
        >
    );

    #[allow(dead_code)]
    pub fn insert_marketplace<T: Config>(
        id: MarketplaceId,
        owner: T::AccountId,
        kind: MarketplaceType,
        commission_fee: u8,
        allow_list: Vec<T::AccountId>,
        name: Vec<u8>,
    ) {
        let data = MarketplaceInformation {
            kind,
            commission_fee,
            owner,
            allow_list,
            name,
        };

        Marketplaces::<T>::insert(id, data);
    }
}
 */
