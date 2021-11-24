use super::v5::v5;
use super::Config;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use sp_std::vec::Vec;

pub mod v6 {
    use std::convert::TryInto;

    use crate::Config;
    use codec::{Decode, Encode};
    use frame_support::traits::Currency;
    use frame_support::Blake2_128Concat;
    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};
    use sp_runtime::RuntimeDebug;
    use sp_std::collections::btree_map::BTreeMap;
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
            NFTSeriesDetails<T::AccountId>
        >
    );

    frame_support::generate_storage_alias!(
        Nfts, Data<T: Config> => Map<
            (Blake2_128Concat, NFTId),
            NFTData<T::AccountId>
        >
    );

    frame_support::generate_storage_alias!(
        Nfts, SeriesIdGenerator => Value<u32>
    );

    frame_support::generate_storage_alias!(
        Nfts, NftMintFee<T: Config> => Value<BalanceOf<T>>
    );

    // Define types that we are going to receive
    pub type NewSeries<AccountId> = BTreeMap<NFTSeriesId, NFTSeriesDetails<AccountId>>;
    pub type NewData<AccountId> = BTreeMap<NFTId, NFTData<AccountId>>;

    pub fn set_series<T: Config>(data_map: NewSeries<T::AccountId>) {
        for data in data_map {
            Series::<T>::insert(data.0, data.1);
        }
    }

    pub fn set_data<T: Config>(data_map: NewData<T::AccountId>) {
        for data in data_map {
            Data::<T>::insert(data.0, data.1);
        }
    }

    pub fn set_series_id_generator<T: Config>(value: u32) {
        SeriesIdGenerator::put(value);
    }

    pub fn is_series_id_free<T: Config>(value: &NFTSeriesId) -> bool {
        Series::<T>::contains_key(value)
    }

    pub fn create_nft_mint_fee<T: Config>() {
        let fee: BalanceOf<T> = 10000000000000000000u128.try_into().ok().unwrap();
        NftMintFee::<T>::put(fee);
    }
}

pub fn migrate<T: Config>() -> Weight {
    let old_series = v5::get_series::<T>();
    let old_data = v5::get_data::<T>();

    // Kill old storage
    v5::kill_storage::<T>();

    // migrate series and data
    migrate_series::<T>(old_series);
    migrate_data::<T>(old_data);

    // Create NftMintFee
    v6::create_nft_mint_fee::<T>();

    // Insert it

    // Get all old series data

    /*     let total = Total::take().unwrap_or(0);
    Total::kill(); */
    /*
    NftIdGenerator::<T>::set(total); */

    T::BlockWeights::get().max_block
}

fn migrate_series<T: Config>(old_series: v5::OldSeries<T::AccountId>) {
    let mut new_series: v6::NewSeries<T::AccountId> = Default::default();

    // Migrate from old to new data
    for entry in old_series {
        let details = v6::NFTSeriesDetails {
            owner: entry.1,
            draft: false,
        };
        new_series.insert(u32_to_text(entry.0), details);
    }

    // Insert new data
    v6::set_series::<T>(new_series);
}

fn migrate_data<T: Config>(old_data: v5::OldData<T::AccountId>) {
    let mut new_data: v6::NewData<T::AccountId> = Default::default();
    let mut last_serial_generated_id = 0u32;

    // Migrate from old to new data
    for entry in old_data {
        // Convert series to string
        let old_series_id: v5::NFTSeriesId = entry.1.series_id;

        let new_series_id = if old_series_id != 0 {
            u32_to_text(old_series_id)
        } else {
            // If the old series id was zero, we need to generate a new unique one for it!
            generate_session_id::<T>(&mut last_serial_generated_id)
        };

        let details = v6::NFTData {
            owner: entry.1.owner,
            ipfs_reference: entry.1.offchain_uri,
            series_id: new_series_id,
            locked: entry.1.locked,
        };

        new_data.insert(entry.0, details);
    }

    // Insert new data
    v6::set_data::<T>(new_data);
    v6::set_series_id_generator::<T>(last_serial_generated_id);
}

fn generate_session_id<T: Config>(current_value: &mut u32) -> Vec<u8> {
    loop {
        let series_id = u32_to_text(*current_value);
        *current_value += 1;

        if v6::is_series_id_free::<T>(&series_id) {
            return series_id;
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
