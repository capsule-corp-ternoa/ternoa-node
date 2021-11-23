use super::Config;
use codec::{Decode, Encode};
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::Blake2_128Concat;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

pub mod v5 {
    use super::*;

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
        owner: AccountId,
        offchain_uri: Vec<u8>,
        series_id: NFTSeriesId,
        locked: bool,
    }
    pub type OldSeriesData<AccountId> = BTreeMap<NFTSeriesId, AccountId>;
    pub type OldNftData<AccountId> = BTreeMap<NFTId, OldNFTDetails<AccountId>>;

    //
    pub fn series<T: Config>() -> OldSeriesData<T::AccountId> {
        let mut old_values: OldSeriesData<T::AccountId> = Default::default();
        for (key, value) in Series::<T>::iter() {
            old_values.insert(key, value.owner);
        }

        old_values
    }

    pub fn data<T: Config>() -> OldNftData<T::AccountId> {
        let mut old_values: OldNftData<T::AccountId> = Default::default();
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

pub mod v6 {
    use super::*;

    // Define all types that are used for v6

    pub type NFTSeriesId = Vec<u8>;
    pub type NFTId = u32;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct NFTSeriesDetails<AccountId> {
        pub owner: AccountId, // Series Owner
        pub draft: bool, // If Yes, the owner can add new nfts to that series but cannot list that nft for sale
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct NFTData<AccountId> {
        // NFT owner
        pub owner: AccountId,
        // IPFS reference
        pub ipfs_reference: Vec<u8>,
        // Series ID
        pub series_id: NFTSeriesId,
        // Is Locked
        pub locked: bool,
    }

    frame_support::generate_storage_alias!(
        Nfts, Series<T: Config> => Map<
            (Blake2_128Concat, NFTSeriesId),
            NFTSeriesDetails<T::AccountId>
        >
    );

    frame_support::generate_storage_alias!(
        Nfts, Data<T: Config> => Map<
            (Blake2_128Concat, NFTId),
            NFTData<T::AccountId>
        >
    );

    // Define types that we are going to receive
    pub type NewSeriesData<AccountId> = BTreeMap<NFTSeriesId, NFTSeriesDetails<AccountId>>;
    pub type NewNFTData<AccountId> = BTreeMap<NFTId, NFTData<AccountId>>;

    pub fn series<T: Config>(data_map: NewSeriesData<T::AccountId>) {
        for data in data_map {
            Series::<T>::insert(data.0, data.1);
        }
    }

    pub fn data<T: Config>(data_map: NewNFTData<T::AccountId>) {
        for data in data_map {
            Data::<T>::insert(data.0, data.1);
        }
    }
}

pub fn migrate<T: Config>() -> Weight {
    log::info!("Migrating nfts to StorageVersion::V6");

    // Convert to new data

    // Insert it

    // Get all old series data

    /*     let total = Total::take().unwrap_or(0);
    Total::kill(); */
    /*
    NftIdGenerator::<T>::set(total); */

    T::BlockWeights::get().max_block
}

/* fn migrate_series<T: Config>() {
    // Get old data
    let old_data = v5::get_old_series_data::<T>();

    // Kill Old storage
    v5::kill_storage::<T>();

    // Convert data
    let mut series_id_generator = 0u32;
}
 */
fn u32_to_text(num: u32) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![];
    let mut dc: usize = 0;

    fn inner(n: u32, vec: &mut Vec<u8>, dc: &mut usize) {
        *dc += 1;
        if n >= 10 {
            inner(n / 10, vec, dc);
        }

        if vec.is_empty() {
            *vec = Vec::with_capacity(*dc);
        }

        let char = u8_to_char((n % 10) as u8);
        vec.push(char);
    }

    inner(num, &mut vec, &mut dc);
    vec
}

const fn u8_to_char(num: u8) -> u8 {
    return num + 48;
}
