/* pub mod v5 {
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

    pub type StorageSeries<AccountId> = BTreeMap<NFTSeriesId, NFTSeriesDetails<AccountId, NFTId>>;
    pub type StorageNFTs<AccountId> = BTreeMap<NFTId, NFTData<AccountId>>;

    pub fn get_series<T: Config>() -> StorageSeries<T::AccountId> {
        Series::<T>::iter().map(|x| x).collect()
    }

    pub fn get_nfts<T: Config>() -> StorageNFTs<T::AccountId> {
        Data::<T>::iter().map(|x| x).collect()
    }

    #[allow(dead_code)]
    pub fn insert_series<T: Config>(
        series_id: NFTSeriesId,
        details: NFTSeriesDetails<T::AccountId, NFTId>,
    ) {
        Series::<T>::insert(series_id, details);
    }

    #[allow(dead_code)]
    pub fn insert_nft<T: Config>(
        owner: T::AccountId,
        nft_id: NFTId,
        offchain_uri: Vec<u8>,
        series_id: NFTSeriesId,
    ) {
        let details = NFTDetails {
            offchain_uri,
            series_id,
            is_capsule: false,
        };
        let data = NFTData {
            owner,
            details,
            locked: false,
            sealed: false,
        };

        Data::<T>::insert(nft_id, data);
    }

    pub fn kill_storage<T: Config>() {
        Series::<T>::remove_all(None);
        Data::<T>::remove_all(None);
    }
}
 */
