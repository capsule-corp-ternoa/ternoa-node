use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

/// How the NFT series id is encoded.
pub type NFTSeriesId = u32;

/// Data related to NFTs on the Ternoa Chain.
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

impl NFTDetails {
    pub fn new(offchain_uri: Vec<u8>, series_id: NFTSeriesId, is_capsule: bool) -> Self {
        Self {
            offchain_uri,
            is_capsule,
            series_id,
        }
    }

    /// Checks if the nft is a part of an unique series.
    pub fn unique_series(&self) -> bool {
        self.series_id != NFTSeriesId::default()
    }
}

/// Data related to an NFT, such as who is its owner.
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

impl<AccountId> NFTData<AccountId> {
    pub fn new(owner: AccountId, details: NFTDetails, sealed: bool, locked: bool) -> Self {
        Self {
            owner,
            details,
            sealed,
            locked,
        }
    }
}

/// Data related to an NFT Series.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTSeriesDetails<AccountId, NFTId> {
    // Series owner.
    pub owner: AccountId,
    // NFTs that are part of the same series.
    pub nfts: Vec<NFTId>,
}

impl<AccountId, NFTId> NFTSeriesDetails<AccountId, NFTId> {
    pub fn new(owner: AccountId, nfts: Vec<NFTId>) -> Self {
        Self { owner, nfts }
    }
}