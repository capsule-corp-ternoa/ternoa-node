/* use super::v5::v5;
use super::Config;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use sp_std::vec::Vec;

pub mod v6 {
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

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    frame_support::generate_storage_alias!(
        Nfts, Series<T: Config> => Map<
            (Blake2_128Concat, NFTSeriesId),
            NFTSeriesDetails<T::AccountId>,
            OptionQuery
        >
    );

    frame_support::generate_storage_alias!(
        Nfts, Data<T: Config> => Map<
            (Blake2_128Concat, NFTId),
            NFTData<T::AccountId>,
            OptionQuery
        >
    );

    frame_support::generate_storage_alias!(
        Nfts, SeriesIdGenerator => Value<u32, ValueQuery>
    );

    frame_support::generate_storage_alias!(
        Nfts, NftMintFee<T: Config> => Value<BalanceOf<T>, ValueQuery>
    );

    // Define helper types
    pub type StorageSeries<AccountId> = BTreeMap<NFTSeriesId, NFTSeriesDetails<AccountId>>;
    pub type StorageNFTs<AccountId> = BTreeMap<NFTId, NFTData<AccountId>>;

    pub fn set_series_id_generator<T: Config>(value: u32) {
        SeriesIdGenerator::put(value);
    }

    pub fn is_series_id_free<T: Config>(value: &NFTSeriesId) -> bool {
        !Series::<T>::contains_key(value)
    }

    pub fn create_nft_mint_fee<T: Config>() {
        let fee: BalanceOf<T> = 10000000000000000000u128.try_into().ok().unwrap();
        NftMintFee::<T>::put(fee);
    }

    pub fn insert_series<T: Config>(id: NFTSeriesId, data: NFTSeriesDetails<T::AccountId>) {
        Series::<T>::insert(id, data);
    }

    pub fn insert_nft<T: Config>(id: NFTId, data: NFTData<T::AccountId>) {
        Data::<T>::insert(id, data);
    }

    #[allow(dead_code)]
    pub fn get_series<T: Config>() -> StorageSeries<T::AccountId> {
        Series::<T>::iter().map(|x| x).collect()
    }

    #[allow(dead_code)]
    pub fn get_nfts<T: Config>() -> StorageNFTs<T::AccountId> {
        Data::<T>::iter().map(|x| x).collect()
    }

    #[allow(dead_code)]
    pub fn get_nft_mint_fee<T: Config>() -> BalanceOf<T> {
        NftMintFee::<T>::get()
    }
}

pub fn migrate<T: Config>() -> Weight {
    let old_series = v5::get_series::<T>();
    let old_nfts = v5::get_nfts::<T>();

    // Kill old storage
    v5::kill_storage::<T>();

    // migrate series and nfts
    migrate_series::<T>(old_series);
    migrate_nfts::<T>(old_nfts);

    // Create NftMintFee
    v6::create_nft_mint_fee::<T>();

    T::BlockWeights::get().max_block
}

fn migrate_series<T: Config>(old_series: v5::StorageSeries<T::AccountId>) {
    // Migrate from old to new series
    for entry in old_series {
        let id = u32_to_text(entry.0);
        let data = v6::NFTSeriesDetails {
            owner: entry.1.owner,
            draft: false,
        };

        v6::insert_series::<T>(id, data)
    }
}

fn migrate_nfts<T: Config>(old_nfts: v5::StorageNFTs<T::AccountId>) {
    let mut stored_series_id = 0u32;

    // Migrate from old to new nfts
    for entry in old_nfts {
        let old_series_id: v5::NFTSeriesId = entry.1.details.series_id;
        let (new_series_id, exists) = convert_series_id::<T>(old_series_id, &mut stored_series_id);

        let (owner, locked) = (entry.1.owner, entry.1.locked);
        let offchain_uri = entry.1.details.offchain_uri;

        let data = v6::NFTData {
            owner: owner.clone(),
            ipfs_reference: offchain_uri,
            series_id: new_series_id.clone(),
            locked,
        };

        v6::insert_nft::<T>(entry.0, data);

        if !exists {
            let draft = false;
            let data = v6::NFTSeriesDetails { owner, draft };
            v6::insert_series::<T>(new_series_id, data)
        }
    }

    v6::set_series_id_generator::<T>(stored_series_id);
}

fn convert_series_id<T: Config>(
    old_series_id: v5::NFTSeriesId,
    stored_series_id: &mut u32,
) -> (Vec<u8>, bool) {
    if old_series_id != 0 {
        return (u32_to_text(old_series_id), true);
    }

    loop {
        let series_id = u32_to_text(*stored_series_id);
        *stored_series_id += 1;

        if v6::is_series_id_free::<T>(&series_id) {
            return (series_id, false);
        }
    }
}

fn u32_to_text(num: u32) -> Vec<u8> {
    let mut vec: Vec<u8> = Default::default();
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
 */
