#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use sp_std::vec::Vec;

/// How the NFT series id is encoded.
pub type NFTSeriesId = u32;

/// Data related to NFTs on the Ternoa Chain.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTDetails {
    /// ASCII encoded URI to fetch additional metadata
    pub offchain_uri: Vec<u8>,
    pub series_id: NFTSeriesId,
}

impl NFTDetails {
    pub const fn new(offchain_uri: Vec<u8>, series_id: NFTSeriesId) -> Self {
        Self {
            offchain_uri,
            series_id,
        }
    }
}
