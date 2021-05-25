use super::{Config, Data, NFTData, NFTDetails, Pallet};
use frame_support::traits::{Get, GetPalletVersion, PalletVersion};
use frame_support::weights::Weight;

pub mod v020 {
    use crate::NFTSeriesId;
    use codec::{Decode, Encode};
    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};
    use sp_runtime::RuntimeDebug;
    use sp_std::vec::Vec;

    /// NFTDetails structure that was present on pallet version 0.2.0
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct NFTDetails {
        pub offchain_uri: Vec<u8>,
        pub series_id: NFTSeriesId,
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
}

/// Function that migrates our storage from pallet version 0.2.0 to 0.3.0
pub fn migration<T: Config>() -> Weight {
    let mut weight = Weight::from(0u64);

    let version: PalletVersion =
        <Pallet<T>>::storage_version().unwrap_or(<Pallet<T>>::current_version());

    if version.major == 0 && version.minor == 2 {
        weight = from_v020_to_v030::<T>()
    }

    weight
}

pub fn from_v020_to_v030<T: Config>() -> Weight {
    Data::<T>::translate::<(T::AccountId, v020::NFTDetails, bool, bool), _>(
        |_key, (owner, old_details, sealed, locked)| {
            let new_details =
                NFTDetails::new(old_details.offchain_uri, old_details.series_id, false);
            let data = NFTData::new(owner, new_details, sealed, locked);
            Some(data)
        },
    );

    T::BlockWeights::get().max_block
}
