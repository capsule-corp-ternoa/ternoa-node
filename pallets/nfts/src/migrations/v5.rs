pub mod v5 {
    use crate::Config;
    use codec::{Decode, Encode};
    use frame_support::Blake2_128Concat;
    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};
    use sp_runtime::RuntimeDebug;
    use sp_std::collections::btree_map::BTreeMap;
    use sp_std::vec::Vec;

    // Define all types that were used for v5

    pub type NFTSeriesId = u32;
    pub type NFTId = u32;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct NFTSeriesDetails<AccountId, NFTId> {
        // Series owner.
        pub owner: AccountId,
        // NFTs that are part of the same series.
        pub nfts: Vec<NFTId>,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct NFTData<AccountId> {
        pub owner: AccountId,
        pub details: NFTDetails,
        /// Set to true to prevent further modifications to the details struct
        pub sealed: bool,
        /// Set to true to prevent changes to the owner variable
        pub locked: bool,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct NFTDetails {
        /// ASCII encoded URI to fetch additional metadata.
        pub offchain_uri: Vec<u8>,
        /// The series id that this nft belongs to.
        pub series_id: NFTSeriesId,
        /// Capsule flag.
        pub is_capsule: bool,
    }

    frame_support::generate_storage_alias!(
        Nfts, Series<T: Config> => Map<
            (Blake2_128Concat, NFTSeriesId),
            NFTSeriesDetails<T::AccountId, NFTId>
        >
    );

    frame_support::generate_storage_alias!(
        Nfts, Data<T: Config> => Map<
            (Blake2_128Concat, NFTId),
            NFTData<T::AccountId>
        >
    );

    // Define types that we are going to return
    pub struct OldNFTDetails<AccountId> {
        pub owner: AccountId,
        pub offchain_uri: Vec<u8>,
        pub series_id: NFTSeriesId,
        pub locked: bool,
    }
    pub type OldSeries<AccountId> = BTreeMap<NFTSeriesId, AccountId>;
    pub type OldData<AccountId> = BTreeMap<NFTId, OldNFTDetails<AccountId>>;

    //
    pub fn get_series<T: Config>() -> OldSeries<T::AccountId> {
        let mut old_values: OldSeries<T::AccountId> = Default::default();
        for (key, value) in Series::<T>::iter() {
            old_values.insert(key, value.owner);
        }

        old_values
    }

    pub fn get_data<T: Config>() -> OldData<T::AccountId> {
        let mut old_values: OldData<T::AccountId> = Default::default();
        for (key, value) in Data::<T>::iter() {
            let details = OldNFTDetails {
                owner: value.owner,
                offchain_uri: value.details.offchain_uri,
                series_id: value.details.series_id,
                locked: value.locked,
            };
            old_values.insert(key, details);
        }

        old_values
    }

    pub fn kill_storage<T: Config>() {
        Series::<T>::remove_all(None);
        Data::<T>::remove_all(None);

        /*         let a = map![22u32 => 3u32, 33u32 => 4u32]; */
    }
}
